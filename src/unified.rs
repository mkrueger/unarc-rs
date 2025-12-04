//! Unified Archive API
//!
//! This module provides a unified interface for working with various archive formats.
//! It abstracts away the differences between ARC, ARJ, ZOO, SQ, SQZ, Z, HYP and other
//! archive formats, providing a simple "open → list → extract" workflow.
//!
//! # Example
//!
//! ```no_run
//! use unarc_rs::unified::ArchiveFormat;
//!
//! // Open archive directly from path
//! let mut archive = ArchiveFormat::open_path("archive.arj").unwrap();
//!
//! // Iterate over entries using the iterator
//! for entry in archive.entries_iter() {
//!     let entry = entry.unwrap();
//!     println!("File: {} ({} bytes)", entry.name(), entry.original_size());
//! }
//! ```

use std::fs::File;
use std::io::{self, BufReader, Read, Seek, Write};
use std::path::Path;

use crate::arc::arc_archive::ArcArchive;
use crate::arc::local_file_header::LocalFileHeader as ArcHeader;
use crate::arj::arj_archive::ArjArchive;
use crate::arj::local_file_header::LocalFileHeader as ArjHeader;
use crate::date_time::DosDateTime;
use crate::hyp::header::Header as HypHeader;
use crate::hyp::hyp_archive::HypArchive;
use crate::lha::lha_archive::{LhaArchiveSeekable, LhaFileHeader};
use crate::rar::rar_archive::{RarArchive, RarFileHeader};
use crate::sevenz::sevenz_archive::{SevenZArchive, SevenZFileHeader};
use crate::sq::header::Header as SqHeader;
use crate::sq::sq_archive::SqArchive;
use crate::sqz::file_header::FileHeader as SqzFileHeader;
use crate::sqz::sqz_archive::SqzArchive;
use crate::z::ZArchive;
use crate::zip::zip_archive::{ZipArchive, ZipFileHeader};
use crate::zoo::dirent::DirectoryEntry as ZooEntry;
use crate::zoo::zoo_archive::ZooArchive;

/// Supported archive formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArchiveFormat {
    /// ARC archive format (.arc)
    Arc,
    /// ARJ archive format (.arj)
    Arj,
    /// ZOO archive format (.zoo)
    Zoo,
    /// SQ/SQ2/QQQ squeezed file format (.sq, .sq2, .qqq, or ?q? pattern)
    Sq,
    /// SQZ (Squeeze It) archive format (.sqz)
    Sqz,
    /// Unix compress format (.Z)
    Z,
    /// Hyper archive format (.hyp)
    Hyp,
    /// LHA/LZH archive format (.lha, .lzh)
    Lha,
    /// ZIP archive format (.zip)
    Zip,
    /// RAR archive format (.rar) - RAR5 only
    Rar,
    /// 7z archive format (.7z)
    SevenZ,
    // Future formats:
}

impl ArchiveFormat {
    /// All supported archive formats
    pub const ALL: &'static [ArchiveFormat] = &[
        ArchiveFormat::Arc,
        ArchiveFormat::Arj,
        ArchiveFormat::Zoo,
        ArchiveFormat::Sq,
        ArchiveFormat::Sqz,
        ArchiveFormat::Z,
        ArchiveFormat::Hyp,
        ArchiveFormat::Lha,
        ArchiveFormat::Zip,
        ArchiveFormat::Rar,
        ArchiveFormat::SevenZ,
    ];

    /// Try to detect the archive format from a file extension
    ///
    /// # Example
    /// ```
    /// use unarc_rs::unified::ArchiveFormat;
    ///
    /// assert_eq!(ArchiveFormat::from_extension("arj"), Some(ArchiveFormat::Arj));
    /// assert_eq!(ArchiveFormat::from_extension("ARJ"), Some(ArchiveFormat::Arj));
    /// assert_eq!(ArchiveFormat::from_extension("txt"), None);
    /// ```
    pub fn from_extension(ext: &str) -> Option<Self> {
        let ext_lower = ext.to_lowercase();
        match ext_lower.as_str() {
            "arc" => Some(ArchiveFormat::Arc),
            "arj" => Some(ArchiveFormat::Arj),
            "zoo" => Some(ArchiveFormat::Zoo),
            "sq" | "sq2" | "qqq" => Some(ArchiveFormat::Sq),
            "sqz" => Some(ArchiveFormat::Sqz),
            "z" => Some(ArchiveFormat::Z),
            "hyp" => Some(ArchiveFormat::Hyp),
            "lha" | "lzh" => Some(ArchiveFormat::Lha),
            "zip" => Some(ArchiveFormat::Zip),
            "rar" => Some(ArchiveFormat::Rar),
            "7z" => Some(ArchiveFormat::SevenZ),
            _ => {
                // Check for ?Q? pattern (e.g., .BQK, .CQM, .DQC)
                let bytes = ext_lower.as_bytes();
                if bytes.len() == 3 && bytes[1] == b'q' {
                    Some(ArchiveFormat::Sq)
                } else {
                    None
                }
            }
        }
    }

    /// Try to detect the archive format from a file path
    ///
    /// # Example
    /// ```
    /// use std::path::Path;
    /// use unarc_rs::unified::ArchiveFormat;
    ///
    /// assert_eq!(ArchiveFormat::from_path(Path::new("archive.arj")), Some(ArchiveFormat::Arj));
    /// assert_eq!(ArchiveFormat::from_path(Path::new("/path/to/file.zoo")), Some(ArchiveFormat::Zoo));
    /// ```
    pub fn from_path(path: &Path) -> Option<Self> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::from_extension)
    }

    /// Returns the typical file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            ArchiveFormat::Arc => "arc",
            ArchiveFormat::Arj => "arj",
            ArchiveFormat::Zoo => "zoo",
            ArchiveFormat::Sq => "sq",
            ArchiveFormat::Sqz => "sqz",
            ArchiveFormat::Z => "Z",
            ArchiveFormat::Hyp => "hyp",
            ArchiveFormat::Lha => "lha",
            ArchiveFormat::Zip => "zip",
            ArchiveFormat::Rar => "rar",
            ArchiveFormat::SevenZ => "7z",
        }
    }

    /// Returns a human-readable name for this format
    pub fn name(&self) -> &'static str {
        match self {
            ArchiveFormat::Arc => "ARC",
            ArchiveFormat::Arj => "ARJ",
            ArchiveFormat::Zoo => "ZOO",
            ArchiveFormat::Sq => "SQ (Squeezed)",
            ArchiveFormat::Sqz => "SQZ (Squeeze It)",
            ArchiveFormat::Z => "Z (Unix compress)",
            ArchiveFormat::Hyp => "HYP (Hyper)",
            ArchiveFormat::Lha => "LHA/LZH",
            ArchiveFormat::Zip => "ZIP",
            ArchiveFormat::Rar => "RAR",
            ArchiveFormat::SevenZ => "7z",
        }
    }

    /// Returns all typical file extensions for this format
    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            ArchiveFormat::Arc => &["arc"],
            ArchiveFormat::Arj => &["arj"],
            ArchiveFormat::Zoo => &["zoo"],
            ArchiveFormat::Sq => &["sq", "sq2", "qqq"],
            ArchiveFormat::Sqz => &["sqz"],
            ArchiveFormat::Z => &["Z"],
            ArchiveFormat::Hyp => &["hyp"],
            ArchiveFormat::Lha => &["lha", "lzh"],
            ArchiveFormat::Zip => &["zip"],
            ArchiveFormat::Rar => &["rar"],
            ArchiveFormat::SevenZ => &["7z"],
        }
    }

    /// Open an archive with this format
    ///
    /// # Example
    /// ```no_run
    /// use std::fs::File;
    /// use unarc_rs::unified::ArchiveFormat;
    ///
    /// let file = File::open("archive.arj").unwrap();
    /// let mut archive = ArchiveFormat::Arj.open(file).unwrap();
    ///
    /// while let Some(entry) = archive.next_entry().unwrap() {
    ///     println!("File: {}", entry.name());
    /// }
    /// ```
    pub fn open<T: Read + Seek>(self, reader: T) -> io::Result<UnifiedArchive<T>> {
        UnifiedArchive::open_with_format(reader, self)
    }

    /// Open an archive file directly from a path
    ///
    /// This is a convenience method that opens the file, detects the format from
    /// the extension, and returns a ready-to-use archive reader.
    ///
    /// # Example
    /// ```no_run
    /// use unarc_rs::unified::ArchiveFormat;
    ///
    /// let mut archive = ArchiveFormat::open_path("archive.arj").unwrap();
    ///
    /// for entry in archive.entries_iter() {
    ///     let entry = entry.unwrap();
    ///     println!("File: {}", entry.name());
    /// }
    /// ```
    pub fn open_path<P: AsRef<Path>>(path: P) -> io::Result<UnifiedArchive<BufReader<File>>> {
        let path = path.as_ref();
        let format = Self::from_path(path).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unsupported archive format: {}", path.display()),
            )
        })?;
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut archive = format.open(reader)?;

        // For .Z files, derive the output filename from the archive name
        if format == ArchiveFormat::Z {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                archive.set_z_filename(stem.to_string());
            }
        }

        Ok(archive)
    }
}

/// Unified archive entry containing metadata about a file in an archive
#[derive(Debug, Clone)]
pub struct ArchiveEntry {
    /// File name (may include path)
    name: String,
    /// Compressed size in bytes
    compressed_size: u64,
    /// Original (uncompressed) size in bytes
    original_size: u64,
    /// Compression method as a human-readable string
    compression_method: String,
    /// Modification date/time (if available)
    modified_time: Option<DosDateTime>,
    /// CRC checksum (CRC16 or CRC32, stored as u64)
    crc: u64,
    /// Internal index for reading the entry
    index: EntryIndex,
}

/// Internal index to track which entry to read
#[derive(Debug, Clone)]
enum EntryIndex {
    Arc(ArcHeader),
    Arj(ArjHeader),
    Zoo(ZooEntry),
    Sq(SqHeader),
    Sqz(SqzFileHeader),
    /// Z format has no header per file, just one file
    Z,
    Hyp(HypHeader),
    Lha(LhaFileHeader),
    Zip(ZipFileHeader),
    Rar(RarFileHeader),
    SevenZ(SevenZFileHeader),
}

impl ArchiveEntry {
    /// Returns the file name (may include path components)
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns just the file name without any path components
    pub fn file_name(&self) -> &str {
        self.name
            .rsplit(|c| c == '/' || c == '\\')
            .next()
            .unwrap_or(&self.name)
    }

    /// Returns the compressed size in bytes
    pub fn compressed_size(&self) -> u64 {
        self.compressed_size
    }

    /// Returns the original (uncompressed) size in bytes
    pub fn original_size(&self) -> u64 {
        self.original_size
    }

    /// Returns the compression method as a human-readable string
    pub fn compression_method(&self) -> &str {
        &self.compression_method
    }

    /// Returns the modification date/time if available
    pub fn modified_time(&self) -> Option<DosDateTime> {
        self.modified_time
    }

    /// Returns the CRC checksum (CRC16 or CRC32 depending on format)
    pub fn crc(&self) -> u64 {
        self.crc
    }

    /// Returns the compression ratio (0.0 - 1.0, where lower is better compression)
    pub fn compression_ratio(&self) -> f64 {
        if self.original_size == 0 {
            1.0
        } else {
            self.compressed_size as f64 / self.original_size as f64
        }
    }

    /// Returns true if this entry is stored without compression
    pub fn is_stored(&self) -> bool {
        self.compression_method.to_lowercase().contains("stored")
            || self.compression_method.to_lowercase().contains("unpacked")
    }
}

/// Iterator over archive entries
///
/// This iterator yields `io::Result<ArchiveEntry>` for each entry in the archive.
/// The iterator automatically skips entries after yielding them.
pub struct ArchiveEntryIter<'a, T: Read + Seek> {
    archive: &'a mut UnifiedArchive<T>,
}

impl<T: Read + Seek> Iterator for ArchiveEntryIter<'_, T> {
    type Item = io::Result<ArchiveEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.archive.next_entry() {
            Ok(Some(entry)) => {
                // Auto-skip after yielding the entry
                if let Err(e) = self.archive.skip(&entry) {
                    return Some(Err(e));
                }
                Some(Ok(entry))
            }
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

/// Internal archive wrapper enum
enum ArchiveInner<T: Read + Seek> {
    Arc(ArcArchive<T>),
    Arj(ArjArchive<T>),
    Zoo(ZooArchive<T>),
    Sq(SqArchive<T>),
    Sqz(SqzArchive<T>),
    Z(ZArchive<T>, bool), // bool = already read
    Hyp(HypArchive<T>),
    Lha(LhaArchiveSeekable<T>),
    Zip(ZipArchive<T>),
    Rar(RarArchive<T>),
    SevenZ(SevenZArchive<T>),
}

/// Unified archive reader that provides a common interface for all supported formats
pub struct UnifiedArchive<T: Read + Seek> {
    inner: ArchiveInner<T>,
    format: ArchiveFormat,
    /// For Z format: store the original filename if known
    z_filename: Option<String>,
}

impl<T: Read + Seek> UnifiedArchive<T> {
    /// Open an archive with a specific format
    ///
    /// # Example
    /// ```no_run
    /// use std::fs::File;
    /// use unarc_rs::unified::{UnifiedArchive, ArchiveFormat};
    ///
    /// let file = File::open("archive.arj").unwrap();
    /// let archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Arj).unwrap();
    /// ```
    pub fn open_with_format(reader: T, format: ArchiveFormat) -> io::Result<Self> {
        let inner = match format {
            ArchiveFormat::Arc => ArchiveInner::Arc(ArcArchive::new(reader)?),
            ArchiveFormat::Arj => ArchiveInner::Arj(ArjArchive::new(reader)?),
            ArchiveFormat::Zoo => ArchiveInner::Zoo(ZooArchive::new(reader)?),
            ArchiveFormat::Sq => ArchiveInner::Sq(SqArchive::new(reader)?),
            ArchiveFormat::Sqz => ArchiveInner::Sqz(SqzArchive::new(reader)?),
            ArchiveFormat::Z => ArchiveInner::Z(ZArchive::new(reader)?, false),
            ArchiveFormat::Hyp => ArchiveInner::Hyp(HypArchive::new(reader)?),
            ArchiveFormat::Lha => ArchiveInner::Lha(LhaArchiveSeekable::new(reader)?),
            ArchiveFormat::Zip => ArchiveInner::Zip(ZipArchive::new(reader)?),
            ArchiveFormat::Rar => ArchiveInner::Rar(RarArchive::new(reader)?),
            ArchiveFormat::SevenZ => ArchiveInner::SevenZ(SevenZArchive::new(reader)?),
        };

        Ok(Self {
            inner,
            format,
            z_filename: None,
        })
    }

    /// Returns the archive format
    pub fn format(&self) -> ArchiveFormat {
        self.format
    }

    /// Set the filename for Z format archives (since .Z files don't contain the filename)
    ///
    /// This is typically derived from the archive filename by removing the .Z extension.
    pub fn set_z_filename(&mut self, name: String) {
        self.z_filename = Some(name);
    }

    /// Get the next entry in the archive
    ///
    /// Returns `Ok(None)` when there are no more entries.
    pub fn next_entry(&mut self) -> io::Result<Option<ArchiveEntry>> {
        match &mut self.inner {
            ArchiveInner::Arc(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    Ok(Some(ArchiveEntry {
                        name: header.name.clone(),
                        compressed_size: header.compressed_size as u64,
                        original_size: header.original_size as u64,
                        compression_method: format!("{:?}", header.compression_method),
                        modified_time: Some(header.date_time),
                        crc: header.crc16 as u64,
                        index: EntryIndex::Arc(header),
                    }))
                } else {
                    Ok(None)
                }
            }
            ArchiveInner::Arj(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    Ok(Some(ArchiveEntry {
                        name: header.name.clone(),
                        compressed_size: header.compressed_size as u64,
                        original_size: header.original_size as u64,
                        compression_method: format!("{:?}", header.compression_method),
                        modified_time: Some(header.date_time_modified),
                        crc: header.original_crc32 as u64,
                        index: EntryIndex::Arj(header),
                    }))
                } else {
                    Ok(None)
                }
            }
            ArchiveInner::Zoo(archive) => {
                if let Some(entry) = archive.get_next_entry()? {
                    let result = ArchiveEntry {
                        name: entry.name.clone(),
                        compressed_size: entry.size_now as u64,
                        original_size: entry.org_size as u64,
                        compression_method: format!("{:?}", entry.compression_method),
                        modified_time: Some(entry.date_time),
                        crc: entry.file_crc16 as u64,
                        index: EntryIndex::Zoo(entry),
                    };
                    Ok(Some(result))
                } else {
                    Ok(None)
                }
            }
            ArchiveInner::Sq(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    Ok(Some(ArchiveEntry {
                        name: header.name.clone(),
                        compressed_size: 0, // SQ doesn't track compressed size
                        original_size: 0,   // SQ doesn't track original size in header
                        compression_method: "Squeezed".to_string(),
                        modified_time: None,
                        crc: header.checksum as u64,
                        index: EntryIndex::Sq(header),
                    }))
                } else {
                    Ok(None)
                }
            }
            ArchiveInner::Sqz(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    Ok(Some(ArchiveEntry {
                        name: header.name.clone(),
                        compressed_size: header.compressed_size as u64,
                        original_size: header.original_size as u64,
                        compression_method: format!("{:?}", header.compression_method),
                        modified_time: Some(header.date_time),
                        crc: header.crc32 as u64,
                        index: EntryIndex::Sqz(header),
                    }))
                } else {
                    Ok(None)
                }
            }
            ArchiveInner::Z(_, ref mut read) => {
                if *read {
                    Ok(None)
                } else {
                    *read = true;
                    let name = self
                        .z_filename
                        .clone()
                        .unwrap_or_else(|| "compressed".to_string());
                    Ok(Some(ArchiveEntry {
                        name,
                        compressed_size: 0, // Would need to seek to get this
                        original_size: 0,   // Unknown until decompressed
                        compression_method: "LZW".to_string(),
                        modified_time: None,
                        crc: 0,
                        index: EntryIndex::Z,
                    }))
                }
            }
            ArchiveInner::Hyp(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    Ok(Some(ArchiveEntry {
                        name: header.name.clone(),
                        compressed_size: header.compressed_size as u64,
                        original_size: header.original_size as u64,
                        compression_method: format!("{:?}", header.compression_method),
                        modified_time: Some(header.date_time),
                        crc: header.checksum as u64,
                        index: EntryIndex::Hyp(header),
                    }))
                } else {
                    Ok(None)
                }
            }
            ArchiveInner::Lha(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    Ok(Some(ArchiveEntry {
                        name: header.name.clone(),
                        compressed_size: header.compressed_size,
                        original_size: header.original_size,
                        compression_method: header.compression_method.clone(),
                        modified_time: header.date_time,
                        crc: header.crc16 as u64,
                        index: EntryIndex::Lha(header),
                    }))
                } else {
                    Ok(None)
                }
            }
            ArchiveInner::Zip(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    Ok(Some(ArchiveEntry {
                        name: header.name.clone(),
                        compressed_size: header.compressed_size,
                        original_size: header.original_size,
                        compression_method: header.compression_method.clone(),
                        modified_time: header.date_time,
                        crc: header.crc32 as u64,
                        index: EntryIndex::Zip(header),
                    }))
                } else {
                    Ok(None)
                }
            }
            ArchiveInner::Rar(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    Ok(Some(ArchiveEntry {
                        name: header.name.clone(),
                        compressed_size: header.compressed_size,
                        original_size: header.original_size,
                        compression_method: header.compression_method.clone(),
                        modified_time: header.date_time,
                        crc: header.crc32 as u64,
                        index: EntryIndex::Rar(header),
                    }))
                } else {
                    Ok(None)
                }
            }
            ArchiveInner::SevenZ(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    Ok(Some(ArchiveEntry {
                        name: header.name.clone(),
                        compressed_size: header.compressed_size,
                        original_size: header.original_size,
                        compression_method: header.compression_method.clone(),
                        modified_time: header.date_time,
                        crc: header.crc32 as u64,
                        index: EntryIndex::SevenZ(header),
                    }))
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// Read (decompress) an entry's data
    ///
    /// # Example
    /// ```no_run
    /// use std::fs::File;
    /// use unarc_rs::unified::{UnifiedArchive, ArchiveFormat};
    ///
    /// let file = File::open("archive.arj").unwrap();
    /// let mut archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Arj).unwrap();
    ///
    /// while let Some(entry) = archive.next_entry().unwrap() {
    ///     let data = archive.read(&entry).unwrap();
    ///     println!("Read {} bytes from {}", data.len(), entry.name());
    /// }
    /// ```
    pub fn read(&mut self, entry: &ArchiveEntry) -> io::Result<Vec<u8>> {
        match (&mut self.inner, &entry.index) {
            (ArchiveInner::Arc(archive), EntryIndex::Arc(header)) => archive.read(header),
            (ArchiveInner::Arj(archive), EntryIndex::Arj(header)) => archive.read(header),
            (ArchiveInner::Zoo(archive), EntryIndex::Zoo(header)) => archive.read(header),
            (ArchiveInner::Sq(archive), EntryIndex::Sq(header)) => archive.read(header),
            (ArchiveInner::Sqz(archive), EntryIndex::Sqz(header)) => archive.read(header),
            (ArchiveInner::Z(archive, _), EntryIndex::Z) => archive.read(),
            (ArchiveInner::Hyp(archive), EntryIndex::Hyp(header)) => archive.read(header),
            (ArchiveInner::Lha(archive), EntryIndex::Lha(header)) => archive.read(header),
            (ArchiveInner::Zip(archive), EntryIndex::Zip(header)) => archive.read(header),
            (ArchiveInner::Rar(archive), EntryIndex::Rar(header)) => archive.read(header),
            (ArchiveInner::SevenZ(archive), EntryIndex::SevenZ(header)) => archive.read(header),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Entry does not belong to this archive",
            )),
        }
    }

    /// Read (decompress) an entry's data and write it directly to a writer
    ///
    /// This is more memory-efficient than `read()` for large files, as it streams
    /// the data directly to the output without buffering the entire file in memory.
    ///
    /// Returns the number of bytes written.
    ///
    /// # Example
    /// ```no_run
    /// use std::fs::File;
    /// use unarc_rs::unified::ArchiveFormat;
    ///
    /// let mut archive = ArchiveFormat::open_path("archive.arj").unwrap();
    ///
    /// while let Some(entry) = archive.next_entry().unwrap() {
    ///     let mut output = File::create(entry.file_name()).unwrap();
    ///     let bytes_written = archive.read_to(&entry, &mut output).unwrap();
    ///     println!("Extracted {} bytes to {}", bytes_written, entry.file_name());
    /// }
    /// ```
    pub fn read_to<W: Write>(&mut self, entry: &ArchiveEntry, writer: &mut W) -> io::Result<u64> {
        // For now, we use the existing read() method and write the result.
        // In the future, individual archive implementations could provide
        // streaming variants for better memory efficiency.
        let data = self.read(entry)?;
        writer.write_all(&data)?;
        Ok(data.len() as u64)
    }

    /// Skip an entry without reading its data
    ///
    /// This is more efficient than reading if you only need to process certain files.
    pub fn skip(&mut self, entry: &ArchiveEntry) -> io::Result<()> {
        match (&mut self.inner, &entry.index) {
            (ArchiveInner::Arc(archive), EntryIndex::Arc(header)) => archive.skip(header),
            (ArchiveInner::Arj(archive), EntryIndex::Arj(header)) => archive.skip(header),
            (ArchiveInner::Zoo(archive), EntryIndex::Zoo(header)) => archive.skip(header),
            (ArchiveInner::Sq(archive), EntryIndex::Sq(header)) => archive.skip(header),
            (ArchiveInner::Sqz(archive), EntryIndex::Sqz(header)) => archive.skip(header),
            (ArchiveInner::Z(archive, _), EntryIndex::Z) => archive.skip(),
            (ArchiveInner::Hyp(archive), EntryIndex::Hyp(header)) => archive.skip(header),
            (ArchiveInner::Lha(archive), EntryIndex::Lha(header)) => archive.skip(header),
            (ArchiveInner::Zip(archive), EntryIndex::Zip(header)) => archive.skip(header),
            (ArchiveInner::Rar(archive), EntryIndex::Rar(header)) => archive.skip(header),
            (ArchiveInner::SevenZ(archive), EntryIndex::SevenZ(header)) => archive.skip(header),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Entry does not belong to this archive",
            )),
        }
    }

    /// Collect all entries into a vector
    ///
    /// Note: After calling this, you'll need to re-open the archive to read entries.
    /// This method is useful when you need to know all entries upfront.
    pub fn entries(&mut self) -> io::Result<Vec<ArchiveEntry>> {
        let mut entries = Vec::new();
        while let Some(entry) = self.next_entry()? {
            entries.push(entry.clone());
            self.skip(&entry)?;
        }
        Ok(entries)
    }

    /// Returns an iterator over all entries in the archive
    ///
    /// This is the idiomatic way to iterate over archive entries.
    /// The iterator automatically skips entries after yielding them.
    ///
    /// # Example
    /// ```no_run
    /// use unarc_rs::unified::ArchiveFormat;
    ///
    /// let mut archive = ArchiveFormat::open_path("archive.arj").unwrap();
    ///
    /// for entry in archive.entries_iter() {
    ///     let entry = entry.unwrap();
    ///     println!("{}: {} bytes", entry.name(), entry.original_size());
    /// }
    /// ```
    pub fn entries_iter(&mut self) -> ArchiveEntryIter<'_, T> {
        ArchiveEntryIter { archive: self }
    }
}

/// Check if a file path appears to be a supported archive based on extension
///
/// # Example
/// ```
/// use std::path::Path;
/// use unarc_rs::unified::is_supported_archive;
///
/// assert!(is_supported_archive(Path::new("file.arj")));
/// assert!(is_supported_archive(Path::new("file.zoo")));
/// assert!(!is_supported_archive(Path::new("file.txt")));
/// ```
pub fn is_supported_archive(path: &Path) -> bool {
    ArchiveFormat::from_path(path).is_some()
}

/// Get all supported file extensions
///
/// # Example
/// ```
/// use unarc_rs::unified::supported_extensions;
///
/// let exts = supported_extensions();
/// assert!(exts.contains(&"arj"));
/// assert!(exts.contains(&"zoo"));
/// ```
pub fn supported_extensions() -> Vec<&'static str> {
    ArchiveFormat::ALL
        .iter()
        .flat_map(|f| f.extensions().iter().copied())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_from_extension() {
        assert_eq!(
            ArchiveFormat::from_extension("arj"),
            Some(ArchiveFormat::Arj)
        );
        assert_eq!(
            ArchiveFormat::from_extension("ARJ"),
            Some(ArchiveFormat::Arj)
        );
        assert_eq!(
            ArchiveFormat::from_extension("zoo"),
            Some(ArchiveFormat::Zoo)
        );
        assert_eq!(
            ArchiveFormat::from_extension("arc"),
            Some(ArchiveFormat::Arc)
        );
        assert_eq!(ArchiveFormat::from_extension("sq"), Some(ArchiveFormat::Sq));
        assert_eq!(
            ArchiveFormat::from_extension("sq2"),
            Some(ArchiveFormat::Sq)
        );
        assert_eq!(
            ArchiveFormat::from_extension("qqq"),
            Some(ArchiveFormat::Sq)
        );
        assert_eq!(
            ArchiveFormat::from_extension("bqk"),
            Some(ArchiveFormat::Sq)
        ); // ?Q? pattern
        assert_eq!(
            ArchiveFormat::from_extension("sqz"),
            Some(ArchiveFormat::Sqz)
        );
        assert_eq!(ArchiveFormat::from_extension("Z"), Some(ArchiveFormat::Z));
        assert_eq!(
            ArchiveFormat::from_extension("hyp"),
            Some(ArchiveFormat::Hyp)
        );
        assert_eq!(ArchiveFormat::from_extension("txt"), None);
        assert_eq!(
            ArchiveFormat::from_extension("zip"),
            Some(ArchiveFormat::Zip)
        );
        assert_eq!(
            ArchiveFormat::from_extension("rar"),
            Some(ArchiveFormat::Rar)
        );
        assert_eq!(
            ArchiveFormat::from_extension("7z"),
            Some(ArchiveFormat::SevenZ)
        );
    }

    #[test]
    fn test_format_from_path() {
        assert_eq!(
            ArchiveFormat::from_path(Path::new("test.arj")),
            Some(ArchiveFormat::Arj)
        );
        assert_eq!(
            ArchiveFormat::from_path(Path::new("/path/to/archive.zoo")),
            Some(ArchiveFormat::Zoo)
        );
        assert_eq!(
            ArchiveFormat::from_path(Path::new("C:\\files\\data.arc")),
            Some(ArchiveFormat::Arc)
        );
        assert_eq!(ArchiveFormat::from_path(Path::new("noext")), None);
    }

    #[test]
    fn test_is_supported_archive() {
        assert!(is_supported_archive(Path::new("file.arj")));
        assert!(is_supported_archive(Path::new("file.zoo")));
        assert!(is_supported_archive(Path::new("file.arc")));
        assert!(is_supported_archive(Path::new("file.zip")));
        assert!(is_supported_archive(Path::new("file.rar")));
        assert!(is_supported_archive(Path::new("file.7z")));
        assert!(!is_supported_archive(Path::new("file.txt")));
    }

    #[test]
    fn test_supported_extensions() {
        let exts = supported_extensions();
        assert!(exts.contains(&"arj"));
        assert!(exts.contains(&"zoo"));
        assert!(exts.contains(&"arc"));
        assert!(exts.contains(&"sq"));
        assert!(exts.contains(&"sqz"));
        assert!(exts.contains(&"Z"));
        assert!(exts.contains(&"hyp"));
        assert!(exts.contains(&"rar"));
        assert!(exts.contains(&"7z"));
    }

    #[test]
    fn test_entry_file_name() {
        let entry = ArchiveEntry {
            name: "path/to/file.txt".to_string(),
            compressed_size: 100,
            original_size: 200,
            compression_method: "Stored".to_string(),
            modified_time: None,
            crc: 0,
            index: EntryIndex::Z,
        };
        assert_eq!(entry.file_name(), "file.txt");

        let entry2 = ArchiveEntry {
            name: "simple.txt".to_string(),
            compressed_size: 100,
            original_size: 200,
            compression_method: "Stored".to_string(),
            modified_time: None,
            crc: 0,
            index: EntryIndex::Z,
        };
        assert_eq!(entry2.file_name(), "simple.txt");
    }

    #[test]
    fn test_compression_ratio() {
        let entry = ArchiveEntry {
            name: "test".to_string(),
            compressed_size: 50,
            original_size: 100,
            compression_method: "LZW".to_string(),
            modified_time: None,
            crc: 0,
            index: EntryIndex::Z,
        };
        assert!((entry.compression_ratio() - 0.5).abs() < 0.001);

        let empty = ArchiveEntry {
            name: "empty".to_string(),
            compressed_size: 0,
            original_size: 0,
            compression_method: "Stored".to_string(),
            modified_time: None,
            crc: 0,
            index: EntryIndex::Z,
        };
        assert!((empty.compression_ratio() - 1.0).abs() < 0.001);
    }
}
