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
use std::io::{BufReader, Read, Seek, Write};
use std::path::Path;

use crate::ace::{AceArchive, FileHeader as AceFileHeader};
use crate::arc::arc_archive::ArcArchive;
use crate::arc::local_file_header::LocalFileHeader as ArcHeader;
use crate::arj::arj_archive::ArjArchive;
use crate::arj::local_file_header::LocalFileHeader as ArjHeader;
use crate::bz2::Bz2Archive;
use crate::date_time::DosDateTime;
use crate::error::{ArchiveError, Result};
use crate::gz::GzArchive;
use crate::ha::ha_archive::HaArchive;
use crate::ha::header::FileHeader as HaHeader;
use crate::hyp::header::Header as HypHeader;
use crate::hyp::hyp_archive::HypArchive;
use crate::ice::IceArchive;
use crate::lha::lha_archive::{LhaArchiveSeekable, LhaFileHeader};
use crate::rar::rar_archive::{RarArchive, RarFileHeader};
use crate::sevenz::sevenz_archive::{SevenZArchive, SevenZFileHeader};
use crate::sq::header::Header as SqHeader;
use crate::sq::sq_archive::SqArchive;
use crate::sqz::file_header::FileHeader as SqzFileHeader;
use crate::sqz::sqz_archive::SqzArchive;
use crate::tar::{TarArchive, TarFileHeader};
use crate::tarz::TarZArchive;
use crate::tbz::TbzArchive;
use crate::tgz::TgzArchive;
use crate::uc2::uc2_archive::{Uc2Archive, Uc2ArchiveEntry as Uc2Header};
use crate::z::ZArchive;
use crate::zip::zip_archive::{ZipArchive, ZipFileHeader};
use crate::zoo::dirent::DirectoryEntry as ZooEntry;
use crate::zoo::zoo_archive::ZooArchive;

/// Supported archive formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArchiveFormat {
    /// ACE archive format (.ace)
    Ace,
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
    /// Gzip single file format (.gz)
    Gz,
    /// Bzip2 single file format (.bz2)
    Bz2,
    /// ICE compressed file format (.ice)
    Ice,
    /// Hyper archive format (.hyp)
    Hyp,
    /// HA (Harri Archiver) format (.ha)
    Ha,
    /// LHA/LZH archive format (.lha, .lzh)
    Lha,
    /// ZIP archive format (.zip)
    Zip,
    /// RAR archive format (.rar) - RAR5 only
    Rar,
    /// 7z archive format (.7z)
    SevenZ,
    /// TAR archive format (.tar)
    Tar,
    /// TGZ (tar.gz) archive format (.tgz, .tar.gz)
    Tgz,
    /// TBZ (tar.bz2) archive format (.tbz, .tbz2, .tar.bz2)
    Tbz,
    /// TAR.Z (tar + Unix compress) archive format (.tar.Z)
    TarZ,
    //
    Uc2,
}

impl ArchiveFormat {
    /// All supported archive formats
    pub const ALL: &'static [ArchiveFormat] = &[
        ArchiveFormat::Ace,
        ArchiveFormat::Arc,
        ArchiveFormat::Arj,
        ArchiveFormat::Zoo,
        ArchiveFormat::Sq,
        ArchiveFormat::Sqz,
        ArchiveFormat::Z,
        ArchiveFormat::Gz,
        ArchiveFormat::Bz2,
        ArchiveFormat::Ice,
        ArchiveFormat::Hyp,
        ArchiveFormat::Ha,
        ArchiveFormat::Uc2,
        ArchiveFormat::Lha,
        ArchiveFormat::Zip,
        ArchiveFormat::Rar,
        ArchiveFormat::SevenZ,
        ArchiveFormat::Tar,
        ArchiveFormat::Tgz,
        ArchiveFormat::Tbz,
        ArchiveFormat::TarZ,
    ];

    /// Try to detect the archive format from a file extension (internal use only)
    fn from_extension(ext: &str) -> Option<Self> {
        let ext_lower = ext.to_lowercase();
        match ext_lower.as_str() {
            "ace" => Some(ArchiveFormat::Ace),
            "arc" => Some(ArchiveFormat::Arc),
            "arj" => Some(ArchiveFormat::Arj),
            "zoo" => Some(ArchiveFormat::Zoo),
            "sq" | "sq2" | "qqq" => Some(ArchiveFormat::Sq),
            "sqz" => Some(ArchiveFormat::Sqz),
            "z" => Some(ArchiveFormat::Z),
            "gz" => Some(ArchiveFormat::Gz),
            "bz2" => Some(ArchiveFormat::Bz2),
            "ice" => Some(ArchiveFormat::Ice),
            "uc2" => Some(ArchiveFormat::Uc2),
            "hyp" => Some(ArchiveFormat::Hyp),
            "ha" => Some(ArchiveFormat::Ha),
            "lha" | "lzh" => Some(ArchiveFormat::Lha),
            "zip" => Some(ArchiveFormat::Zip),
            "rar" => Some(ArchiveFormat::Rar),
            "7z" => Some(ArchiveFormat::SevenZ),
            "tar" => Some(ArchiveFormat::Tar),
            "tgz" => Some(ArchiveFormat::Tgz),
            "tbz" | "tbz2" => Some(ArchiveFormat::Tbz),
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
        // Check for .tar.gz first (double extension)
        let filename = path.file_name()?.to_str()?;
        let filename_lower = filename.to_lowercase();
        if filename_lower.ends_with(".tar.gz") {
            return Some(ArchiveFormat::Tgz);
        }
        if filename_lower.ends_with(".tar.bz2") {
            return Some(ArchiveFormat::Tbz);
        }
        if filename_lower.ends_with(".tar.z") {
            return Some(ArchiveFormat::TarZ);
        }

        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::from_extension)
    }

    /// Returns the typical file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            ArchiveFormat::Ace => "ace",
            ArchiveFormat::Arc => "arc",
            ArchiveFormat::Arj => "arj",
            ArchiveFormat::Zoo => "zoo",
            ArchiveFormat::Sq => "sq",
            ArchiveFormat::Sqz => "sqz",
            ArchiveFormat::Z => "Z",
            ArchiveFormat::Gz => "gz",
            ArchiveFormat::Bz2 => "bz2",
            ArchiveFormat::Ice => "ice",
            ArchiveFormat::Hyp => "hyp",
            ArchiveFormat::Ha => "ha",
            ArchiveFormat::Uc2 => "uc2",
            ArchiveFormat::Lha => "lha",
            ArchiveFormat::Zip => "zip",
            ArchiveFormat::Rar => "rar",
            ArchiveFormat::SevenZ => "7z",
            ArchiveFormat::Tar => "tar",
            ArchiveFormat::Tgz => "tgz",
            ArchiveFormat::Tbz => "tbz2",
            ArchiveFormat::TarZ => "tar.Z",
        }
    }

    /// Returns a human-readable name for this format
    pub fn name(&self) -> &'static str {
        match self {
            ArchiveFormat::Ace => "ACE",
            ArchiveFormat::Arc => "ARC",
            ArchiveFormat::Arj => "ARJ",
            ArchiveFormat::Zoo => "ZOO",
            ArchiveFormat::Sq => "SQ (Squeezed)",
            ArchiveFormat::Sqz => "SQZ (Squeeze It)",
            ArchiveFormat::Z => "Z (Unix compress)",
            ArchiveFormat::Gz => "GZ (gzip)",
            ArchiveFormat::Bz2 => "BZ2 (bzip2)",
            ArchiveFormat::Ice => "ICE",
            ArchiveFormat::Hyp => "HYP (Hyper)",
            ArchiveFormat::Ha => "HA (Harri Archiver)",
            ArchiveFormat::Uc2 => "UC2 (Ultra Compressor II)",
            ArchiveFormat::Lha => "LHA/LZH",
            ArchiveFormat::Zip => "ZIP",
            ArchiveFormat::Rar => "RAR",
            ArchiveFormat::SevenZ => "7z",
            ArchiveFormat::Tar => "TAR",
            ArchiveFormat::Tgz => "TGZ (tar.gz)",
            ArchiveFormat::Tbz => "TBZ (tar.bz2)",
            ArchiveFormat::TarZ => "TAR.Z (tar + Unix compress)",
        }
    }

    /// Returns all typical file extensions for this format
    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            ArchiveFormat::Ace => &["ace"],
            ArchiveFormat::Arc => &["arc"],
            ArchiveFormat::Arj => &["arj"],
            ArchiveFormat::Zoo => &["zoo"],
            ArchiveFormat::Sq => &["sq", "sq2", "qqq"],
            ArchiveFormat::Sqz => &["sqz"],
            ArchiveFormat::Z => &["Z"],
            ArchiveFormat::Gz => &["gz"],
            ArchiveFormat::Bz2 => &["bz2"],
            ArchiveFormat::Ice => &["ice"],
            ArchiveFormat::Hyp => &["hyp"],
            ArchiveFormat::Ha => &["ha"],
            ArchiveFormat::Uc2 => &["uc2"],
            ArchiveFormat::Lha => &["lha", "lzh"],
            ArchiveFormat::Zip => &["zip"],
            ArchiveFormat::Rar => &["rar"],
            ArchiveFormat::SevenZ => &["7z"],
            ArchiveFormat::Tar => &["tar"],
            ArchiveFormat::Tgz => &["tgz", "tar.gz"],
            ArchiveFormat::Tbz => &["tbz", "tbz2", "tar.bz2"],
            ArchiveFormat::TarZ => &["tar.Z"],
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
    pub fn open<T: Read + Seek>(self, reader: T) -> Result<UnifiedArchive<T>> {
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
    pub fn open_path<P: AsRef<Path>>(path: P) -> Result<UnifiedArchive<BufReader<File>>> {
        let path = path.as_ref();
        let format = Self::from_path(path).ok_or_else(|| {
            ArchiveError::UnsupportedFormat(format!(
                "Unsupported archive format: {}",
                path.display()
            ))
        })?;
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut archive = format.open(reader)?;

        // For single-file formats (.Z, .gz, .bz2), derive the output filename from the archive name
        if matches!(
            format,
            ArchiveFormat::Z | ArchiveFormat::Gz | ArchiveFormat::Bz2
        ) {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                archive.set_single_file_name(stem.to_string());
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
    Ace(AceFileHeader),
    Arc(ArcHeader),
    Arj(ArjHeader),
    Zoo(ZooEntry),
    Sq(SqHeader),
    Sqz(SqzFileHeader),
    /// Z format has no header per file, just one file
    Z,
    /// GZ format has no header per file, just one file
    Gz,
    /// BZ2 format has no header per file, just one file
    Bz2,
    /// ICE format has no header per file, just one file
    Ice,
    Hyp(HypHeader),
    Ha(HaHeader),
    Uc2(Uc2Header),
    Lha(LhaFileHeader),
    Zip(ZipFileHeader),
    Rar(RarFileHeader),
    SevenZ(SevenZFileHeader),
    Tar(TarFileHeader),
    /// TGZ uses the same header as TAR
    Tgz(TarFileHeader),
    /// TBZ uses the same header as TAR
    Tbz(TarFileHeader),
    /// TAR.Z uses the same header as TAR
    TarZ(TarFileHeader),
}

impl ArchiveEntry {
    /// Returns the file name (may include path components)
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns just the file name without any path components
    pub fn file_name(&self) -> &str {
        self.name.rsplit(['/', '\\']).next().unwrap_or(&self.name)
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
/// This iterator yields `Result<ArchiveEntry>` for each entry in the archive.
/// The iterator automatically skips entries after yielding them.
pub struct ArchiveEntryIter<'a, T: Read + Seek> {
    archive: &'a mut UnifiedArchive<T>,
}

impl<T: Read + Seek> Iterator for ArchiveEntryIter<'_, T> {
    type Item = Result<ArchiveEntry>;

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
    Ace(AceArchive<T>),
    Arc(ArcArchive<T>),
    Arj(ArjArchive<T>),
    Zoo(ZooArchive<T>),
    Sq(SqArchive<T>),
    Sqz(SqzArchive<T>),
    Z(ZArchive<T>, bool),     // bool = already read
    Gz(GzArchive<T>, bool),   // bool = already read
    Bz2(Bz2Archive<T>, bool), // bool = already read
    Ice(IceArchive, bool),    // bool = already read
    Hyp(HypArchive<T>),
    Ha(HaArchive<T>),
    Uc2(Uc2Archive<T>),
    Lha(LhaArchiveSeekable<T>),
    Zip(ZipArchive<T>),
    Rar(RarArchive<T>),
    SevenZ(SevenZArchive<T>),
    Tar(TarArchive<T>),
    /// TGZ decompresses to memory, so it doesn't need the generic reader type
    Tgz(TgzArchive),
    /// TBZ decompresses to memory, so it doesn't need the generic reader type
    Tbz(TbzArchive),
    /// TAR.Z decompresses to memory, so it doesn't need the generic reader type
    TarZ(TarZArchive),
}

/// Unified archive reader that provides a common interface for all supported formats
pub struct UnifiedArchive<T: Read + Seek> {
    inner: ArchiveInner<T>,
    format: ArchiveFormat,
    /// For single-file formats (.Z, .gz, .bz2): store the original filename if known
    single_file_name: Option<String>,
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
    pub fn open_with_format(reader: T, format: ArchiveFormat) -> Result<Self> {
        let inner = match format {
            ArchiveFormat::Ace => ArchiveInner::Ace(AceArchive::new(reader)?),
            ArchiveFormat::Arc => ArchiveInner::Arc(ArcArchive::new(reader)?),
            ArchiveFormat::Arj => ArchiveInner::Arj(ArjArchive::new(reader)?),
            ArchiveFormat::Zoo => ArchiveInner::Zoo(ZooArchive::new(reader)?),
            ArchiveFormat::Sq => ArchiveInner::Sq(SqArchive::new(reader)?),
            ArchiveFormat::Sqz => ArchiveInner::Sqz(SqzArchive::new(reader)?),
            ArchiveFormat::Z => ArchiveInner::Z(ZArchive::new(reader)?, false),
            ArchiveFormat::Gz => ArchiveInner::Gz(GzArchive::new(reader)?, false),
            ArchiveFormat::Bz2 => ArchiveInner::Bz2(Bz2Archive::new(reader)?, false),
            ArchiveFormat::Ice => ArchiveInner::Ice(IceArchive::new(reader)?, false),
            ArchiveFormat::Hyp => ArchiveInner::Hyp(HypArchive::new(reader)?),
            ArchiveFormat::Ha => ArchiveInner::Ha(HaArchive::new(reader)?),
            ArchiveFormat::Uc2 => ArchiveInner::Uc2(Uc2Archive::new(reader)?),
            ArchiveFormat::Lha => ArchiveInner::Lha(LhaArchiveSeekable::new(reader)?),
            ArchiveFormat::Zip => ArchiveInner::Zip(ZipArchive::new(reader)?),
            ArchiveFormat::Rar => ArchiveInner::Rar(RarArchive::new(reader)?),
            ArchiveFormat::SevenZ => ArchiveInner::SevenZ(SevenZArchive::new(reader)?),
            ArchiveFormat::Tar => ArchiveInner::Tar(TarArchive::new(reader)?),
            ArchiveFormat::Tgz => ArchiveInner::Tgz(TgzArchive::new(reader)?),
            ArchiveFormat::Tbz => ArchiveInner::Tbz(TbzArchive::new(reader)?),
            ArchiveFormat::TarZ => ArchiveInner::TarZ(TarZArchive::new(reader)?),
        };

        Ok(Self {
            inner,
            format,
            single_file_name: None,
        })
    }

    /// Returns the archive format
    pub fn format(&self) -> ArchiveFormat {
        self.format
    }

    /// Set the filename for single-file formats (.Z, .gz, .bz2) since they don't contain the filename
    ///
    /// This is typically derived from the archive filename by removing the extension.
    pub fn set_single_file_name(&mut self, name: String) {
        self.single_file_name = Some(name);
    }

    /// Get the next entry in the archive
    ///
    /// Returns `Ok(None)` when there are no more entries.
    pub fn next_entry(&mut self) -> Result<Option<ArchiveEntry>> {
        match &mut self.inner {
            ArchiveInner::Ace(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    Ok(Some(ArchiveEntry {
                        name: header.filename.clone(),
                        compressed_size: header.packed_size,
                        original_size: header.original_size,
                        compression_method: format!("{}", header.compression_type),
                        modified_time: Some(header.datetime),
                        crc: header.crc32 as u64,
                        index: EntryIndex::Ace(header),
                    }))
                } else {
                    Ok(None)
                }
            }
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
                        .single_file_name
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
            ArchiveInner::Gz(_, ref mut read) => {
                if *read {
                    Ok(None)
                } else {
                    *read = true;
                    let name = self
                        .single_file_name
                        .clone()
                        .unwrap_or_else(|| "compressed".to_string());
                    Ok(Some(ArchiveEntry {
                        name,
                        compressed_size: 0, // Would need to seek to get this
                        original_size: 0,   // Unknown until decompressed
                        compression_method: "Deflate".to_string(),
                        modified_time: None,
                        crc: 0,
                        index: EntryIndex::Gz,
                    }))
                }
            }
            ArchiveInner::Bz2(_, ref mut read) => {
                if *read {
                    Ok(None)
                } else {
                    *read = true;
                    let name = self
                        .single_file_name
                        .clone()
                        .unwrap_or_else(|| "compressed".to_string());
                    Ok(Some(ArchiveEntry {
                        name,
                        compressed_size: 0, // Would need to seek to get this
                        original_size: 0,   // Unknown until decompressed
                        compression_method: "Bzip2".to_string(),
                        modified_time: None,
                        crc: 0,
                        index: EntryIndex::Bz2,
                    }))
                }
            }
            ArchiveInner::Ice(ref archive, ref mut read) => {
                if *read {
                    Ok(None)
                } else {
                    *read = true;
                    let name = self
                        .single_file_name
                        .clone()
                        .unwrap_or_else(|| "compressed".to_string());
                    Ok(Some(ArchiveEntry {
                        name,
                        compressed_size: 0, // Not stored in ICE format
                        original_size: archive.original_size() as u64,
                        compression_method: "LH1".to_string(),
                        modified_time: None,
                        crc: 0,
                        index: EntryIndex::Ice,
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
            ArchiveInner::Ha(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    Ok(Some(ArchiveEntry {
                        name: header.full_path(),
                        compressed_size: header.compressed_size as u64,
                        original_size: header.original_size as u64,
                        compression_method: format!("{:?}", header.method),
                        modified_time: Some(header.timestamp),
                        crc: header.crc32 as u64,
                        index: EntryIndex::Ha(header),
                    }))
                } else {
                    Ok(None)
                }
            }
            ArchiveInner::Uc2(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    Ok(Some(ArchiveEntry {
                        name: header.name.clone(),
                        compressed_size: header.compress_info.compressed_length as u64,
                        original_size: header.original_size as u64,
                        compression_method: format!("{:?}", header.compress_info.method),
                        modified_time: Some(DosDateTime::new(header.dos_time)),
                        crc: header.crc as u64,
                        index: EntryIndex::Uc2(header),
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
            ArchiveInner::Tar(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    Ok(Some(ArchiveEntry {
                        name: header.name.clone(),
                        compressed_size: header.size, // TAR doesn't compress
                        original_size: header.size,
                        compression_method: "Stored".to_string(),
                        modified_time: header.modified_time(),
                        crc: 0, // TAR doesn't use CRC
                        index: EntryIndex::Tar(header),
                    }))
                } else {
                    Ok(None)
                }
            }
            ArchiveInner::Tgz(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    Ok(Some(ArchiveEntry {
                        name: header.name.clone(),
                        compressed_size: header.size, // Original TAR size (uncompressed)
                        original_size: header.size,
                        compression_method: "Gzip + Stored".to_string(),
                        modified_time: header.modified_time(),
                        crc: 0, // TAR doesn't use CRC
                        index: EntryIndex::Tgz(header),
                    }))
                } else {
                    Ok(None)
                }
            }
            ArchiveInner::Tbz(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    Ok(Some(ArchiveEntry {
                        name: header.name.clone(),
                        compressed_size: header.size, // Original TAR size (uncompressed)
                        original_size: header.size,
                        compression_method: "Bzip2 + Stored".to_string(),
                        modified_time: header.modified_time(),
                        crc: 0, // TAR doesn't use CRC
                        index: EntryIndex::Tbz(header),
                    }))
                } else {
                    Ok(None)
                }
            }
            ArchiveInner::TarZ(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    Ok(Some(ArchiveEntry {
                        name: header.name.clone(),
                        compressed_size: header.size, // Original TAR size (uncompressed)
                        original_size: header.size,
                        compression_method: "LZW + Stored".to_string(),
                        modified_time: header.modified_time(),
                        crc: 0, // TAR doesn't use CRC
                        index: EntryIndex::TarZ(header),
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
    pub fn read(&mut self, entry: &ArchiveEntry) -> Result<Vec<u8>> {
        match (&mut self.inner, &entry.index) {
            (ArchiveInner::Ace(archive), EntryIndex::Ace(header)) => archive.read(header),
            (ArchiveInner::Arc(archive), EntryIndex::Arc(header)) => archive.read(header),
            (ArchiveInner::Arj(archive), EntryIndex::Arj(header)) => archive.read(header),
            (ArchiveInner::Zoo(archive), EntryIndex::Zoo(header)) => archive.read(header),
            (ArchiveInner::Sq(archive), EntryIndex::Sq(header)) => archive.read(header),
            (ArchiveInner::Sqz(archive), EntryIndex::Sqz(header)) => archive.read(header),
            (ArchiveInner::Z(archive, _), EntryIndex::Z) => archive.read(),
            (ArchiveInner::Gz(archive, _), EntryIndex::Gz) => archive.read(),
            (ArchiveInner::Bz2(archive, _), EntryIndex::Bz2) => archive.read(),
            (ArchiveInner::Ice(archive, _), EntryIndex::Ice) => archive.read(),
            (ArchiveInner::Hyp(archive), EntryIndex::Hyp(header)) => archive.read(header),
            (ArchiveInner::Ha(archive), EntryIndex::Ha(header)) => archive.read(header),
            (ArchiveInner::Uc2(archive), EntryIndex::Uc2(header)) => archive.read(header),
            (ArchiveInner::Lha(archive), EntryIndex::Lha(header)) => archive.read(header),
            (ArchiveInner::Zip(archive), EntryIndex::Zip(header)) => archive.read(header),
            (ArchiveInner::Rar(archive), EntryIndex::Rar(header)) => archive.read(header),
            (ArchiveInner::SevenZ(archive), EntryIndex::SevenZ(header)) => archive.read(header),
            (ArchiveInner::Tar(archive), EntryIndex::Tar(header)) => archive.read(header),
            (ArchiveInner::Tgz(archive), EntryIndex::Tgz(header)) => archive.read(header),
            (ArchiveInner::Tbz(archive), EntryIndex::Tbz(header)) => archive.read(header),
            (ArchiveInner::TarZ(archive), EntryIndex::TarZ(header)) => archive.read(header),
            _ => Err(ArchiveError::IndexMismatch(
                "Entry does not belong to this archive".to_string(),
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
    pub fn read_to<W: Write>(&mut self, entry: &ArchiveEntry, writer: &mut W) -> Result<u64> {
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
    pub fn skip(&mut self, entry: &ArchiveEntry) -> Result<()> {
        match (&mut self.inner, &entry.index) {
            (ArchiveInner::Ace(archive), EntryIndex::Ace(header)) => archive.skip(header),
            (ArchiveInner::Arc(archive), EntryIndex::Arc(header)) => archive.skip(header),
            (ArchiveInner::Arj(archive), EntryIndex::Arj(header)) => archive.skip(header),
            (ArchiveInner::Zoo(archive), EntryIndex::Zoo(header)) => archive.skip(header),
            (ArchiveInner::Sq(archive), EntryIndex::Sq(header)) => archive.skip(header),
            (ArchiveInner::Sqz(archive), EntryIndex::Sqz(header)) => archive.skip(header),
            (ArchiveInner::Z(archive, _), EntryIndex::Z) => archive.skip(),
            (ArchiveInner::Gz(archive, _), EntryIndex::Gz) => archive.skip(),
            (ArchiveInner::Bz2(archive, _), EntryIndex::Bz2) => archive.skip(),
            (ArchiveInner::Ice(archive, _), EntryIndex::Ice) => archive.skip(),
            (ArchiveInner::Hyp(archive), EntryIndex::Hyp(header)) => archive.skip(header),
            (ArchiveInner::Ha(archive), EntryIndex::Ha(header)) => archive.skip(header),
            (ArchiveInner::Uc2(archive), EntryIndex::Uc2(header)) => archive.skip(header),
            (ArchiveInner::Lha(archive), EntryIndex::Lha(header)) => archive.skip(header),
            (ArchiveInner::Zip(archive), EntryIndex::Zip(header)) => archive.skip(header),
            (ArchiveInner::Rar(archive), EntryIndex::Rar(header)) => archive.skip(header),
            (ArchiveInner::SevenZ(archive), EntryIndex::SevenZ(header)) => archive.skip(header),
            (ArchiveInner::Tar(archive), EntryIndex::Tar(header)) => archive.skip(header),
            (ArchiveInner::Tgz(archive), EntryIndex::Tgz(header)) => archive.skip(header),
            (ArchiveInner::Tbz(archive), EntryIndex::Tbz(header)) => archive.skip(header),
            (ArchiveInner::TarZ(archive), EntryIndex::TarZ(header)) => archive.skip(header),
            _ => Err(ArchiveError::IndexMismatch(
                "Entry does not belong to this archive".to_string(),
            )),
        }
    }

    /// Collect all entries into a vector
    ///
    /// Note: After calling this, you'll need to re-open the archive to read entries.
    /// This method is useful when you need to know all entries upfront.
    pub fn entries(&mut self) -> Result<Vec<ArchiveEntry>> {
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
        assert!(exts.contains(&"gz"));
        assert!(exts.contains(&"bz2"));
        assert!(exts.contains(&"hyp"));
        assert!(exts.contains(&"rar"));
        assert!(exts.contains(&"7z"));
    }

    #[test]
    fn test_gz_bz2_format_detection() {
        // Test .gz as single file format
        assert_eq!(ArchiveFormat::from_extension("gz"), Some(ArchiveFormat::Gz));
        assert_eq!(ArchiveFormat::from_extension("GZ"), Some(ArchiveFormat::Gz));

        // Test .bz2 as single file format
        assert_eq!(
            ArchiveFormat::from_extension("bz2"),
            Some(ArchiveFormat::Bz2)
        );
        assert_eq!(
            ArchiveFormat::from_extension("BZ2"),
            Some(ArchiveFormat::Bz2)
        );

        // Test path detection
        assert_eq!(
            ArchiveFormat::from_path(Path::new("file.gz")),
            Some(ArchiveFormat::Gz)
        );
        assert_eq!(
            ArchiveFormat::from_path(Path::new("file.bz2")),
            Some(ArchiveFormat::Bz2)
        );

        // Test .tar.gz still detects as Tgz (tar archive)
        assert_eq!(
            ArchiveFormat::from_path(Path::new("file.tar.gz")),
            Some(ArchiveFormat::Tgz)
        );
        assert_eq!(
            ArchiveFormat::from_path(Path::new("file.tar.bz2")),
            Some(ArchiveFormat::Tbz)
        );
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
