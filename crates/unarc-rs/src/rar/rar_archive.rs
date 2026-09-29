//! RAR archive reader
//!
//! Uses the pure Rust [`rars`] crate, which supports every RAR generation from
//! RAR 1.3 up to RAR 7, including encrypted payloads and encrypted headers.

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

use chrono::{Datelike, Local, TimeZone, Timelike};
use rars::{ArchiveMember, ArchiveMemberDetail, ArchiveReadOptions, ArchiveReader};

use crate::date_time::DosDateTime;
use crate::error::{ArchiveError, Result};

/// Header information for a RAR entry
#[derive(Debug, Clone)]
pub struct RarFileHeader {
    /// File name (may include path)
    pub name: String,
    /// Compressed size in bytes
    pub compressed_size: u64,
    /// Original (uncompressed) size in bytes
    pub original_size: u64,
    /// Compression method name
    pub compression_method: String,
    /// Modification date/time
    pub date_time: Option<DosDateTime>,
    /// CRC32 checksum (RAR 1.3/1.4 archives only carry a 16-bit checksum)
    pub crc32: u32,
    /// Whether this entry is a directory
    pub is_directory: bool,
    /// Whether this entry is encrypted
    pub is_encrypted: bool,
    /// Position of the entry in archive order
    pub(crate) index: usize,
}

/// RAR archive reader
///
/// Supports RAR 1.3 through RAR 7 archives with full decompression.
pub struct RarArchive<T: Read + Seek> {
    state: ParseState,
    /// Raw name bytes of each entry (same order as `entries`), used to match extraction callbacks
    raw_names: Vec<Vec<u8>>,
    /// RAR5 link/copy redirections (same order as `entries`)
    redirections: Vec<Option<rars::rar50::FileRedirection>>,
    entries: Vec<RarFileHeader>,
    current_index: usize,
    /// Password for encrypted entries (and encrypted headers)
    password: Option<String>,
    /// Decoded members that were produced by a sequential extraction pass but not consumed yet
    cache: DecodeCache,
    _reader: PhantomData<T>,
}

enum ParseState {
    /// `header_data` holds the raw archive when its headers are encrypted, so that
    /// password verifiers can re-check the header password.
    Parsed {
        archive: Arc<rars::Archive>,
        header_data: Option<Arc<Vec<u8>>>,
    },
    /// Headers are encrypted; parsing is deferred until a password is known
    NeedsPassword(Arc<Vec<u8>>),
}

#[derive(Default)]
struct DecodeCache {
    password: Option<String>,
    members: HashMap<usize, Vec<u8>>,
}

impl<T: Read + Seek> RarArchive<T> {
    /// Create a new RAR archive reader from a Read+Seek source
    ///
    /// The whole archive is read into memory.
    pub fn new(mut reader: T) -> Result<Self> {
        reader.seek(SeekFrom::Start(0))?;
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        Self::from_bytes(data)
    }

    /// Create a RAR archive reader directly from a file path
    ///
    /// Member data is read from the file on demand instead of being loaded into memory up front.
    pub fn from_path(path: &std::path::Path) -> Result<Self> {
        match ArchiveReader::read_path(path) {
            Ok(archive) => Ok(Self::from_state(ParseState::Parsed {
                archive: Arc::new(archive),
                header_data: None,
            })),
            Err(rars::Error::NeedPassword) => Self::from_bytes(std::fs::read(path)?),
            Err(e) => Err(map_parse_error(e)),
        }
    }

    fn from_bytes(data: Vec<u8>) -> Result<Self> {
        // Encrypted headers can only be parsed once a password is known, so keep a copy of the bytes around.
        let needs_password = matches!(ArchiveReader::read(&data), Err(rars::Error::NeedPassword));
        if needs_password {
            return Ok(Self::from_state(ParseState::NeedsPassword(Arc::new(data))));
        }
        let archive = ArchiveReader::read_owned(data).map_err(map_parse_error)?;
        Ok(Self::from_state(ParseState::Parsed {
            archive: Arc::new(archive),
            header_data: None,
        }))
    }

    fn from_state(state: ParseState) -> Self {
        let mut result = Self {
            state,
            raw_names: Vec::new(),
            redirections: Vec::new(),
            entries: Vec::new(),
            current_index: 0,
            password: None,
            cache: DecodeCache::default(),
            _reader: PhantomData,
        };
        result.load_entries();
        result
    }

    /// Set the password for encrypted entries
    pub fn set_password<P: Into<String>>(&mut self, password: P) {
        self.password = Some(password.into());
    }

    /// Clear the password
    pub fn clear_password(&mut self) {
        self.password = None;
    }

    fn ensure_parsed(&mut self) -> Result<Arc<rars::Archive>> {
        let data = match &self.state {
            ParseState::Parsed { archive, .. } => return Ok(archive.clone()),
            ParseState::NeedsPassword(data) => data.clone(),
        };
        let Some(password) = self.password.as_deref() else {
            return Err(ArchiveError::encryption_required("archive headers", "RAR"));
        };
        let archive = Arc::new(parse_encrypted_headers(&data, password.as_bytes())?);
        self.state = ParseState::Parsed {
            archive: archive.clone(),
            header_data: Some(data),
        };
        self.load_entries();
        Ok(archive)
    }

    fn load_entries(&mut self) {
        let ParseState::Parsed { archive, .. } = &self.state else {
            return;
        };
        let rar50_times: Vec<Option<u32>> = archive
            .as_rar50()
            .map(|a| a.files().map(|f| f.mtime.or(f.htime_mtime)).collect())
            .unwrap_or_default();
        self.redirections = archive
            .as_rar50()
            .map(|a| a.files().map(|f| f.redirection.clone()).collect())
            .unwrap_or_default();

        self.raw_names.clear();
        self.entries.clear();
        for (index, member) in archive.members().enumerate() {
            let date_time = match member.detail {
                ArchiveMemberDetail::Rar50Plus { .. } => rar50_times.get(index).copied().flatten().and_then(unix_to_dos),
                _ => member.meta.file_time.filter(|&t| t != 0).map(DosDateTime::new),
            };
            let mut name = member.meta.name_lossy();
            if !matches!(member.detail, ArchiveMemberDetail::Rar50Plus { .. }) {
                // RAR 1.x-4.x store DOS style path separators, RAR 5 always uses '/'.
                name = name.replace('\\', "/");
            }
            self.entries.push(RarFileHeader {
                name,
                compressed_size: member.meta.packed_size,
                original_size: member.meta.unpacked_size,
                compression_method: compression_method(&member),
                date_time,
                crc32: member_crc(&member),
                is_directory: member.meta.is_directory,
                is_encrypted: member.meta.is_encrypted,
                index,
            });
            self.raw_names.push(member.meta.name);
        }
    }

    /// Get the next entry in the archive
    pub fn get_next_entry(&mut self) -> Result<Option<RarFileHeader>> {
        self.ensure_parsed()?;
        if self.current_index >= self.entries.len() {
            return Ok(None);
        }

        let entry = self.entries[self.current_index].clone();
        self.current_index += 1;
        Ok(Some(entry))
    }

    /// Skip the current entry without reading its data
    pub fn skip(&mut self, header: &RarFileHeader) -> Result<()> {
        self.cache.members.remove(&header.index);
        Ok(())
    }

    /// Read and decompress an entry's data
    pub fn read(&mut self, header: &RarFileHeader) -> Result<Vec<u8>> {
        self.read_with_password(header, self.password.clone())
    }

    /// Read and decompress an entry's data with a specific password
    pub fn read_with_password(&mut self, header: &RarFileHeader, password: Option<String>) -> Result<Vec<u8>> {
        if header.is_directory {
            return Ok(Vec::new());
        }
        if header.is_encrypted && password.is_none() {
            return Err(ArchiveError::encryption_required(&header.name, "RAR"));
        }
        let archive = self.ensure_parsed()?;
        let pwd = password.as_deref().map(str::as_bytes);

        if let Some(Some(redirection)) = self.redirections.get(header.index) {
            // Hard links and file copies carry the target's content; symlinks and junctions have none.
            let target = match redirection.redirection_type {
                REDIR_HARD_LINK | REDIR_FILE_COPY => self.raw_names[..header.index].iter().rposition(|name| *name == redirection.target_name),
                _ => None,
            };
            return match target {
                Some(target) => {
                    let target = self.entries[target].clone();
                    self.read_with_password(&target, password)
                }
                None => Ok(Vec::new()),
            };
        }

        if let Some(data) = read_independent_member(&archive, header, pwd)? {
            return Ok(data);
        }

        if self.cache.password == password {
            if let Some(data) = self.cache.members.remove(&header.index) {
                return Ok(data);
            }
        }

        // Solid archives (and older formats) can only be decoded front to back. Decode
        // every member from the requested one onwards in a single pass and keep the
        // rest around, so reading all entries sequentially stays a single pass.
        let mut members = decode_members(&archive, &self.raw_names, header, pwd, false)?;
        let data = members.remove(&header.index).unwrap_or_default();
        self.cache = DecodeCache { password, members };
        Ok(data)
    }

    /// Create a password verifier for the given encrypted entry.
    ///
    /// Returns a standalone verifier that can be used from multiple threads with rayon.
    pub fn create_password_verifier(&self, header: &RarFileHeader) -> Result<super::password_verifier::RarPasswordVerifier> {
        if !header.is_encrypted {
            return Err(ArchiveError::unsupported_method("RAR", "entry is not encrypted"));
        }
        let ParseState::Parsed { archive, header_data } = &self.state else {
            return Err(ArchiveError::encryption_required("archive headers", "RAR"));
        };
        Ok(super::password_verifier::RarPasswordVerifier::new(
            archive.clone(),
            header_data.clone(),
            Arc::new(self.raw_names.clone()),
            header.clone(),
        ))
    }
}

const REDIR_HARD_LINK: u64 = 4;
const REDIR_FILE_COPY: u64 = 5;

pub(super) fn parse_encrypted_headers(data: &[u8], password: &[u8]) -> Result<rars::Archive> {
    ArchiveReader::read_with_options(data, ArchiveReadOptions::with_password(password)).map_err(|e| match e {
        rars::Error::NeedPassword | rars::Error::WrongPasswordOrCorruptData => ArchiveError::invalid_password("archive headers", "RAR"),
        e => map_parse_error(e),
    })
}

/// Decodes one member of an archive whose members are independent of each other.
///
/// Returns `Ok(None)` if the archive needs sequential (solid) decoding.
pub(super) fn read_independent_member(archive: &rars::Archive, header: &RarFileHeader, password: Option<&[u8]>) -> Result<Option<Vec<u8>>> {
    let Some(rar50) = archive.as_rar50() else {
        return Ok(None);
    };
    let is_solid = |f: &rars::rar50::FileHeader| f.decoded_compression_info().is_ok_and(|info| info.solid);
    if rar50.main.is_solid() || rar50.files().any(is_solid) {
        return Ok(None);
    }
    let Some(file) = rar50.files().nth(header.index) else {
        return Err(ArchiveError::corrupted_entry_named("RAR", &header.name, "File not found in archive"));
    };
    if file.redirection.is_some() {
        return Ok(Some(Vec::new()));
    }
    let mut data = Vec::with_capacity(usize::try_from(header.original_size).unwrap_or(0).min(64 * 1024 * 1024));
    file.write_to(rar50, password, &mut data).map_err(|e| map_entry_error(&header.name, e))?;
    Ok(Some(data))
}

/// Decodes members in a single sequential pass, starting to collect at `header`.
///
/// With `stop_after_target` the pass ends once the requested member is decoded,
/// otherwise every later member is collected as well.
pub(super) fn decode_members(
    archive: &rars::Archive,
    raw_names: &[Vec<u8>],
    header: &RarFileHeader,
    password: Option<&[u8]>,
    stop_after_target: bool,
) -> Result<HashMap<usize, Vec<u8>>> {
    let mut buffers: Vec<(usize, SharedBuffer)> = Vec::new();
    let mut next_index = 0usize;
    let mut target_seen = false;
    let result = archive.extract_to(password, |meta| {
        // Some members (e.g. RAR5 links) are never reported, so resync by name.
        let mut index = next_index;
        while index < raw_names.len() && raw_names[index] != meta.name {
            index += 1;
        }
        if index >= raw_names.len() {
            index = next_index;
        }
        next_index = index + 1;

        if target_seen && stop_after_target {
            return Err(rars::Error::Cancelled);
        }
        if index < header.index || meta.is_directory {
            return Ok(Box::new(std::io::sink()) as Box<dyn Write>);
        }
        target_seen |= index == header.index;
        let buffer = SharedBuffer::default();
        buffers.push((index, buffer.clone()));
        Ok(Box::new(buffer) as Box<dyn Write>)
    });

    let mut members: HashMap<usize, Vec<u8>> = buffers.into_iter().map(|(index, buffer)| (index, buffer.0.take())).collect();

    match result {
        Ok(()) | Err(rars::Error::Cancelled) => {}
        Err(e) => {
            // A failure in a later member must not hide the data of the requested one;
            // the requested member is complete once a later member has started.
            let failed_after_target = members.keys().any(|&i| i > header.index);
            if !failed_after_target {
                return Err(map_entry_error(&header.name, e));
            }
            members.retain(|&i, _| i <= header.index);
        }
    }

    if !members.contains_key(&header.index) {
        return Err(ArchiveError::corrupted_entry_named("RAR", &header.name, "File not found in archive"));
    }
    Ok(members)
}

#[derive(Clone, Default)]
struct SharedBuffer(Rc<RefCell<Vec<u8>>>);

impl Write for SharedBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.borrow_mut().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn map_parse_error(e: rars::Error) -> ArchiveError {
    match e {
        rars::Error::Io(io) => ArchiveError::io_error(io.message),
        rars::Error::TooShort | rars::Error::UnsupportedSignature | rars::Error::InvalidHeader(_) => ArchiveError::invalid_header("RAR"),
        e => ArchiveError::external_library("rars", e.to_string()),
    }
}

pub(super) fn map_entry_error(entry: &str, e: rars::Error) -> ArchiveError {
    match e {
        rars::Error::AtEntry { source, .. } | rars::Error::AtArchiveOffset { source, .. } => map_entry_error(entry, *source),
        rars::Error::NeedPassword => ArchiveError::encryption_required(entry, "RAR"),
        rars::Error::WrongPasswordOrCorruptData => ArchiveError::invalid_password(entry, "RAR"),
        rars::Error::Crc32Mismatch { expected, actual } => ArchiveError::crc_mismatch(entry, expected, actual),
        rars::Error::CrcMismatch { expected, actual } => ArchiveError::crc_mismatch(entry, u32::from(expected), u32::from(actual)),
        rars::Error::Io(io) => ArchiveError::io_error(io.message),
        e => ArchiveError::decompression_failed(entry, e.to_string()),
    }
}

fn member_crc(member: &ArchiveMember) -> u32 {
    match member.detail {
        ArchiveMemberDetail::Rar13 { file_checksum, .. } => u32::from(file_checksum),
        ArchiveMemberDetail::Rar15To40 { crc32, .. } => crc32,
        ArchiveMemberDetail::Rar50Plus { crc32, .. } => crc32.unwrap_or(0),
        _ => 0,
    }
}

fn compression_method(member: &ArchiveMember) -> String {
    if member.meta.is_directory {
        return "Directory".to_string();
    }
    if member.meta.is_stored {
        return "Stored".to_string();
    }
    // Map the family specific method to RAR's level 0 (store) .. 5 (best).
    let level = match member.detail {
        ArchiveMemberDetail::Rar13 { method, .. } => Some(u64::from(method)),
        ArchiveMemberDetail::Rar15To40 { method, .. } => u64::from(method).checked_sub(0x30),
        ArchiveMemberDetail::Rar50Plus { compression_info, .. } => Some((compression_info >> 7) & 7),
        _ => None,
    };
    match level {
        Some(0) => "Stored",
        Some(1) => "Fastest",
        Some(2) => "Fast",
        Some(3) => "Normal",
        Some(4) => "Good",
        Some(5) => "Best",
        _ => "Compressed",
    }
    .to_string()
}

/// Converts a RAR5 (UTC) timestamp to a DOS timestamp, which is local time by convention.
fn unix_to_dos(unix_secs: u32) -> Option<DosDateTime> {
    let dt = Local.timestamp_opt(i64::from(unix_secs), 0).single()?;
    let year = u32::try_from(dt.year()).ok()?;
    if !(1980..=2107).contains(&year) {
        return None;
    }
    let date = ((year - 1980) << 9) | (dt.month() << 5) | dt.day();
    let time = (dt.hour() << 11) | (dt.minute() << 5) | (dt.second() / 2);
    Some(DosDateTime::new((date << 16) | time))
}
