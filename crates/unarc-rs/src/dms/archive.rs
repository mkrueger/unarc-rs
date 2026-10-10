use std::collections::HashSet;
use std::io::{Cursor, Read};
use std::sync::Arc;

use crc16::{State, ARC};

use super::unpack::Unpacker;
use crate::amiga::{AmigaArchive, AmigaEntry, AmigaEntryKind, AmigaVolume};
use crate::date_time::DosDateTime;
use crate::error::{ArchiveError, Result};

const FORMAT: &str = "DMS";
const HEADER_LEN: usize = 56;
const TRACK_HEADER_LEN: usize = 20;
/// Largest packed or unpacked track xDMS accepts.
const MAX_TRACK_LEN: usize = 32_000;
/// Disk tracks are numbered 0-79; each holds both sides of a cylinder.
const DISK_TRACKS: usize = 80;
const FILE_ID_TRACK: u16 = 80;
const BANNER_TRACK: u16 = 0xffff;
/// Shorter tracks below 80 are not disk data, such as the 1024-byte fake boot block.
const MIN_DISK_TRACK_LEN: usize = 2048;
/// Track sizes of double- and high-density disks (two sides of 11 or 22 sectors).
const TRACK_SIZES: [usize; 2] = [11 * 1024, 22 * 1024];
const FLAG_ENCRYPTED: u16 = 2;
const DISK_TYPE_FMS: u16 = 7;
const MODE_NAMES: [&str; 7] = ["NOCOMP", "SIMPLE", "QUICK", "MEDIUM", "DEEP", "HEAVY1", "HEAVY2"];
const DEFAULT_IMAGE_NAME: &str = "disk.adf";

fn mode_name(mode: u16) -> &'static str {
    MODE_NAMES.get(usize::from(mode)).copied().unwrap_or("unknown")
}

fn crc(data: &[u8]) -> u16 {
    State::<ARC>::calculate(data)
}

fn word(data: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes([data[offset], data[offset + 1]])
}

/// Information from the archive header.
#[derive(Debug, Clone)]
pub struct DmsInfo {
    /// Version of DMS that wrote the archive, times 100 (e.g. 203 for 2.03).
    pub creator_version: u16,
    /// General information flags: 1 no-zero, 2 encrypted, 4 appends, 8 banner,
    /// 16 high density, 32 MS-DOS, 64 device fixed, 128 registered, 256 FILE_ID.DIZ.
    pub flags: u16,
    /// Creation time as stored (seconds; xDMS treats it as a Unix time).
    pub created: u32,
    /// Lowest disk track according to the header.
    pub first_track: u16,
    /// Highest disk track according to the header.
    pub last_track: u16,
    /// Disk type: 0/1 OFS, 2 FFS, 3/4 international OFS/FFS, 5/6 directory-cache OFS/FFS.
    pub disk_type: u16,
    /// Main compression mode: 0 NOCOMP, 1 SIMPLE, 2 QUICK, 3 MEDIUM, 4 DEEP, 5 HEAVY1, 6 HEAVY2.
    pub mode: u16,
}

impl DmsInfo {
    pub fn is_encrypted(&self) -> bool {
        self.flags & FLAG_ENCRYPTED != 0
    }

    /// Name of the main compression mode, as DMS prints it.
    pub fn mode_name(&self) -> &'static str {
        mode_name(self.mode)
    }
}

/// What a [`DmsEntry`] holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmsEntryKind {
    /// A file, directory or link in the disk's AmigaDOS filesystem.
    Amiga(AmigaEntryKind),
    /// The decoded disk image, listed instead of files when it has no readable
    /// AmigaDOS filesystem or is not a complete disk.
    DiskImage,
    /// A FILE_ID.DIZ description attached to the archive (track 80).
    FileId,
    /// A banner text that DMS shows when unpacking (track 0xFFFF).
    Banner,
}

/// An entry of a DMS archive.
#[derive(Debug, Clone)]
pub struct DmsEntry {
    pub name: String,
    pub size: u64,
    /// Stored size: the packed track data for images and attached texts, 0 for files
    /// inside the disk (their tracks are compressed as a whole).
    pub packed_size: u64,
    pub kind: DmsEntryKind,
    /// Whether the data was encrypted in the archive. FILE_ID.DIZ never is.
    pub encrypted: bool,
    amiga: Option<AmigaEntry>,
    /// Position in the archive's attached texts.
    text: Option<usize>,
}

impl DmsEntry {
    /// The filesystem entry, for files, directories and links inside the disk.
    pub fn amiga_entry(&self) -> Option<&AmigaEntry> {
        self.amiga.as_ref()
    }

    pub fn modified_time(&self) -> Option<DosDateTime> {
        self.amiga.as_ref().and_then(|entry| entry.modified_time)
    }

    pub fn link_target(&self) -> Option<&str> {
        self.amiga.as_ref().and_then(|entry| entry.link_target.as_deref())
    }
}

/// Why the disk image cannot be read. Kept instead of failing the whole archive, so
/// attached texts stay readable.
#[derive(Debug, Clone)]
enum ImageError {
    PasswordRequired,
    BadPassword,
    Crc { track: u16, expected: u16, actual: u16 },
    Checksum { track: u16, expected: u16, actual: u16 },
    Decode { track: u16, reason: &'static str },
    UnknownMode { track: u16, mode: u8 },
    TooManyTracks,
}

impl ImageError {
    fn to_error(&self, entry: &str) -> ArchiveError {
        match *self {
            Self::PasswordRequired => ArchiveError::encryption_required(entry, FORMAT),
            Self::BadPassword => ArchiveError::invalid_password(entry, FORMAT),
            Self::Crc { track, expected, actual } => ArchiveError::crc_mismatch(format!("{entry} (track {track})"), expected.into(), actual.into()),
            Self::Checksum { track, expected, actual } => {
                ArchiveError::corrupted_entry_named(FORMAT, entry, format!("track {track} checksum {actual:#06x}, expected {expected:#06x}"))
            }
            Self::Decode { track, reason } => ArchiveError::decompression_failed(entry, format!("track {track}: {reason}")),
            Self::UnknownMode { track, mode } => ArchiveError::unsupported_method(FORMAT, format!("compression mode {mode} (track {track})")),
            Self::TooManyTracks => ArchiveError::corrupted_entry_named(FORMAT, entry, format!("more than {DISK_TRACKS} disk tracks")),
        }
    }
}

/// A track header and its packed data.
struct Track {
    number: u16,
    /// Size after the first decompression stage, before RLE.
    stage_len: usize,
    size: usize,
    flags: u8,
    mode: u8,
    checksum: u16,
    data: Vec<u8>,
    /// Stored and computed CRC when the packed data is damaged.
    crc_error: Option<(u16, u16)>,
}

impl Track {
    /// Decodes the track and verifies its checksum.
    fn unpack(&self, unpacker: &mut Unpacker, encrypted: bool) -> std::result::Result<Vec<u8>, ImageError> {
        let track = self.number;
        if let Some((expected, actual)) = self.crc_error {
            return Err(ImageError::Crc { track, expected, actual });
        }
        if usize::from(self.mode) >= MODE_NAMES.len() {
            return Err(ImageError::UnknownMode { track, mode: self.mode });
        }
        let data = unpacker
            .unpack(&self.data, self.mode, self.flags, self.stage_len, self.size)
            .map_err(|reason| {
                if encrypted {
                    ImageError::BadPassword
                } else {
                    ImageError::Decode { track, reason }
                }
            })?;
        let actual = data.iter().fold(0u16, |sum, &b| sum.wrapping_add(u16::from(b)));
        if actual != self.checksum {
            return Err(if encrypted {
                ImageError::BadPassword
            } else {
                ImageError::Checksum {
                    track,
                    expected: self.checksum,
                    actual,
                }
            });
        }
        Ok(data)
    }
}

/// An attached FILE_ID.DIZ or banner, decoded when read.
struct AttachedText {
    track: Track,
    encrypted: bool,
    /// Set when the text is encrypted and no password was given.
    locked: bool,
}

/// Reader for DMS (DiskMasher) compressed Amiga disk archives.
///
/// The disk tracks are decoded when the archive is opened. A complete disk with an
/// AmigaDOS filesystem is listed file by file; any other disk is listed as one raw
/// image. Attached FILE_ID.DIZ and banner texts are listed as extra entries.
pub struct DmsArchive {
    info: DmsInfo,
    image: Arc<[u8]>,
    image_error: Option<ImageError>,
    /// Unpacked and packed size of the disk tracks, from their headers.
    image_size: u64,
    image_packed_size: u64,
    filesystem: Option<AmigaArchive<Cursor<Arc<[u8]>>>>,
    texts: Vec<AttachedText>,
    entries: Vec<DmsEntry>,
    next: usize,
}

impl DmsArchive {
    /// Opens a DMS archive. `password` decrypts encrypted archives; without it, the
    /// disk of an encrypted archive is listed as an image that cannot be read.
    pub fn open<R: Read>(mut reader: R, password: Option<&str>) -> Result<Self> {
        let mut header = [0u8; HEADER_LEN];
        if crate::limits::read_up_to(&mut reader, &mut header)? != HEADER_LEN || &header[..4] != b"DMS!" {
            return Err(ArchiveError::invalid_header(FORMAT));
        }
        let (expected, actual) = (word(&header, HEADER_LEN - 2), crc(&header[4..HEADER_LEN - 2]));
        if expected != actual {
            return Err(ArchiveError::crc_mismatch("DMS header", expected.into(), actual.into()));
        }
        let info = DmsInfo {
            creator_version: word(&header, 46),
            flags: word(&header, 10),
            created: u32::from_be_bytes([header[12], header[13], header[14], header[15]]),
            first_track: word(&header, 16),
            last_track: word(&header, 18),
            disk_type: word(&header, 50),
            mode: word(&header, 52),
        };
        if info.disk_type == DISK_TYPE_FMS {
            return Err(ArchiveError::unsupported_format("DMS FMS file archive (disk type 7)"));
        }

        let encrypted = info.is_encrypted();
        let mut key = password.filter(|_| encrypted).map(|password| crc(password.as_bytes()));
        let mut unpacker = Unpacker::new();
        let mut image = Vec::new();
        let mut image_error = (encrypted && key.is_none()).then_some(ImageError::PasswordRequired);
        let (mut image_size, mut image_packed_size) = (0u64, 0u64);
        let mut disk_tracks = Vec::new();
        let mut texts = Vec::new();

        while let Some(mut track) = Self::read_track(&mut reader)? {
            // FILE_ID.DIZ is never encrypted; every other track continues the key stream.
            let track_encrypted = encrypted && track.number != FILE_ID_TRACK;
            if let Some(key) = key.as_mut().filter(|_| track_encrypted) {
                decrypt(&mut track.data, key);
            }
            match track.number {
                FILE_ID_TRACK | BANNER_TRACK => texts.push(AttachedText {
                    track,
                    encrypted: track_encrypted,
                    locked: track_encrypted && key.is_none(),
                }),
                number if usize::from(number) < DISK_TRACKS && track.size > MIN_DISK_TRACK_LEN => {
                    image_size += track.size as u64;
                    image_packed_size += track.data.len() as u64;
                    if disk_tracks.len() == DISK_TRACKS {
                        image_error.get_or_insert(ImageError::TooManyTracks);
                    }
                    if image_error.is_some() {
                        continue;
                    }
                    match track.unpack(&mut unpacker, encrypted) {
                        Ok(data) => {
                            image.extend_from_slice(&data);
                            disk_tracks.push((number, data.len()));
                        }
                        Err(error) => image_error = Some(error),
                    }
                }
                // Fake boot blocks and other tracks are not part of the disk.
                _ => {}
            }
        }

        let image: Arc<[u8]> = image.into();
        let complete = image_error.is_none()
            && disk_tracks.len() == DISK_TRACKS
            && disk_tracks
                .iter()
                .enumerate()
                .all(|(i, &(number, size))| usize::from(number) == i && size == disk_tracks[0].1)
            && TRACK_SIZES.contains(&disk_tracks[0].1);
        // A disk without a readable AmigaDOS filesystem (NDOS, trackloader, damaged or
        // unsupported filesystem) is listed as a raw image instead.
        let filesystem = if complete {
            AmigaArchive::open_adf(Cursor::new(Arc::clone(&image))).ok()
        } else {
            None
        };

        let mut archive = Self {
            info,
            image,
            image_error,
            image_size,
            image_packed_size,
            filesystem,
            texts,
            entries: Vec::new(),
            next: 0,
        };
        archive.list_entries()?;
        Ok(archive)
    }

    /// Reads the next track, or `None` at the end of the file or at data that is not a
    /// track header (some archives carry trailing text or padding).
    fn read_track<R: Read>(reader: &mut R) -> Result<Option<Track>> {
        let mut header = [0u8; TRACK_HEADER_LEN];
        let read = crate::limits::read_up_to(reader, &mut header)?;
        if read == 0 {
            return Ok(None);
        }
        if read < 2 || &header[..2] != b"TR" {
            return Ok(None);
        }
        if read < TRACK_HEADER_LEN {
            return Err(ArchiveError::corrupted_entry(FORMAT, "truncated track header"));
        }
        let number = word(&header, 2);
        let (expected, actual) = (word(&header, 18), crc(&header[..18]));
        if expected != actual {
            return Err(ArchiveError::crc_mismatch(format!("DMS track {number} header"), expected.into(), actual.into()));
        }
        let (packed_len, stage_len, size) = (usize::from(word(&header, 6)), usize::from(word(&header, 8)), usize::from(word(&header, 10)));
        if packed_len > MAX_TRACK_LEN || stage_len > MAX_TRACK_LEN || size > MAX_TRACK_LEN {
            return Err(ArchiveError::corrupted_entry(
                FORMAT,
                format!("track {number} is larger than {MAX_TRACK_LEN} bytes"),
            ));
        }
        let data = crate::limits::read_exact_vec(reader, packed_len as u64)
            .map_err(|_| ArchiveError::corrupted_entry(FORMAT, format!("track {number} data is truncated")))?;
        let (expected, actual) = (word(&header, 16), crc(&data));
        Ok(Some(Track {
            number,
            stage_len,
            size,
            flags: header[12],
            mode: header[13],
            checksum: word(&header, 14),
            data,
            crc_error: (expected != actual).then_some((expected, actual)),
        }))
    }

    fn list_entries(&mut self) -> Result<()> {
        let mut names = HashSet::new();
        if let Some(filesystem) = &mut self.filesystem {
            while let Some(entry) = filesystem.get_next_entry()? {
                names.insert(entry.name.to_ascii_lowercase());
                self.entries.push(DmsEntry {
                    name: entry.name.clone(),
                    size: entry.size,
                    packed_size: 0,
                    kind: DmsEntryKind::Amiga(entry.kind),
                    encrypted: self.info.is_encrypted(),
                    amiga: Some(entry),
                    text: None,
                });
            }
        } else {
            names.insert(DEFAULT_IMAGE_NAME.to_string());
            self.entries.push(DmsEntry {
                name: DEFAULT_IMAGE_NAME.to_string(),
                size: self.image_size,
                packed_size: self.image_packed_size,
                kind: DmsEntryKind::DiskImage,
                encrypted: self.info.is_encrypted(),
                amiga: None,
                text: None,
            });
        }
        for kind in [DmsEntryKind::FileId, DmsEntryKind::Banner] {
            let (base, extension, number) = if kind == DmsEntryKind::FileId {
                ("FILE_ID", "DIZ", FILE_ID_TRACK)
            } else {
                ("BANNER", "TXT", BANNER_TRACK)
            };
            // FILE_ID.DIZ, FILE_ID2.DIZ, ...: never the name of a file on the disk.
            let mut count = 0;
            for (text_index, text) in self.texts.iter().enumerate().filter(|(_, text)| text.track.number == number) {
                let name = loop {
                    count += 1;
                    let name = if count == 1 {
                        format!("{base}.{extension}")
                    } else {
                        format!("{base}{count}.{extension}")
                    };
                    if names.insert(name.to_ascii_lowercase()) {
                        break name;
                    }
                };
                self.entries.push(DmsEntry {
                    name,
                    size: text.track.size as u64,
                    packed_size: text.track.data.len() as u64,
                    kind,
                    encrypted: text.encrypted,
                    amiga: None,
                    text: Some(text_index),
                });
            }
        }
        Ok(())
    }

    pub fn info(&self) -> &DmsInfo {
        &self.info
    }

    /// The decoded disk tracks in archive order, as `xdms u` writes them. Tracks that
    /// are not stored are not filled in, so a partial archive yields a shorter image.
    /// Empty if the disk could not be decoded.
    pub fn image(&self) -> &[u8] {
        if self.image_error.is_some() {
            &[]
        } else {
            &self.image
        }
    }

    /// The AmigaDOS volume, when the disk is listed file by file.
    pub fn volume(&self) -> Option<&AmigaVolume> {
        self.filesystem.as_ref().and_then(|filesystem| filesystem.volumes().first())
    }

    /// Renames the raw image entry, e.g. after the archive's file stem plus `.adf`.
    pub fn set_image_name(&mut self, name: String) {
        if let Some(entry) = self.entries.iter_mut().find(|entry| entry.kind == DmsEntryKind::DiskImage) {
            entry.name = name;
        }
    }

    pub fn get_next_entry(&mut self) -> Result<Option<DmsEntry>> {
        let entry = self.entries.get(self.next).cloned();
        if entry.is_some() {
            self.next += 1;
        }
        Ok(entry)
    }

    /// Reads an entry with the default per-file limit.
    pub fn read(&mut self, entry: &DmsEntry) -> Result<Vec<u8>> {
        self.read_with_limit(entry, Some(crate::limits::DEFAULT_MAX_ENTRY_SIZE))
    }

    pub fn read_with_limit(&mut self, entry: &DmsEntry, limit: Option<u64>) -> Result<Vec<u8>> {
        match entry.kind {
            DmsEntryKind::Amiga(_) => {
                let (Some(filesystem), Some(amiga)) = (self.filesystem.as_mut(), entry.amiga.as_ref()) else {
                    return Err(Self::foreign());
                };
                filesystem.read_with_limit(amiga, limit)
            }
            DmsEntryKind::DiskImage => {
                if self.filesystem.is_some() {
                    return Err(Self::foreign());
                }
                if let Some(error) = &self.image_error {
                    return Err(error.to_error(&entry.name));
                }
                crate::limits::check_size(self.image.len() as u64, limit, &entry.name)?;
                Ok(self.image.to_vec())
            }
            DmsEntryKind::FileId | DmsEntryKind::Banner => {
                let text = entry
                    .text
                    .and_then(|index| self.texts.get(index))
                    .filter(|text| (text.track.number == FILE_ID_TRACK) == (entry.kind == DmsEntryKind::FileId))
                    .ok_or_else(Self::foreign)?;
                if text.locked {
                    return Err(ArchiveError::encryption_required(&entry.name, FORMAT));
                }
                crate::limits::check_size(text.track.size as u64, limit, &entry.name)?;
                // Attached texts are packed on their own, like xDMS decodes them.
                text.track
                    .unpack(&mut Unpacker::new(), text.encrypted)
                    .map_err(|error| error.to_error(&entry.name))
            }
        }
    }

    pub fn skip(&mut self, entry: &DmsEntry) -> Result<()> {
        match &entry.amiga {
            Some(amiga) => self.filesystem.as_mut().ok_or_else(Self::foreign)?.skip(amiga),
            None => Ok(()),
        }
    }

    fn foreign() -> ArchiveError {
        ArchiveError::IndexMismatch("Entry does not belong to this DMS archive".into())
    }
}

/// DMS encryption: XOR with a 16-bit key that starts as the password's CRC and is
/// updated with each encrypted byte.
fn decrypt(data: &mut [u8], key: &mut u16) {
    for byte in data {
        let encrypted = *byte;
        *byte ^= *key as u8;
        *key = (*key >> 1).wrapping_add(u16::from(encrypted));
    }
}
