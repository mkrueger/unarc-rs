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
//!
//! # Opening with Options
//!
//! ```no_run
//! use unarc_rs::unified::{ArchiveFormat, ArchiveOptions};
//!
//! // Open a password-protected archive with CRC verification
//! let options = ArchiveOptions::new()
//!     .with_password("secret")
//!     .with_verify_crc(true);
//!
//! let mut archive = ArchiveFormat::open_path_with_options("encrypted.zip", options).unwrap();
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
use crate::encryption::{EncryptionMethod, RarEncryption, ZipEncryption};
use crate::error::{ArchiveError, Result};
use crate::gz::GzArchive;
use crate::ha::ha_archive::HaArchive;
use crate::ha::header::FileHeader as HaHeader;
use crate::hyp::header::Header as HypHeader;
use crate::hyp::hyp_archive::HypArchive;
use crate::ice::IceArchive;
use crate::lha::lha_archive::{LhaArchiveSeekable, LhaFileHeader};
use crate::packice::PackIceArchive;
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

/// Options for opening and reading archives
///
/// `ArchiveOptions` provides a builder-style API for configuring archive operations,
/// such as setting passwords for encrypted archives or enabling CRC verification.
///
/// # Example
///
/// ```
/// use unarc_rs::unified::ArchiveOptions;
///
/// let options = ArchiveOptions::new()
///     .with_password("secret")
///     .with_verify_crc(true);
///
/// assert!(options.has_password());
/// assert!(options.verify_crc());
/// ```
#[derive(Debug, Clone, Default)]
pub struct ArchiveOptions {
    /// Password for encrypted archives
    password: Option<String>,
    /// Whether to verify CRC checksums during extraction
    verify_crc: bool,
}

impl ArchiveOptions {
    /// Create new default options
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the password for encrypted archives
    pub fn with_password<S: Into<String>>(mut self, password: S) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Enable or disable CRC verification during extraction
    pub fn with_verify_crc(mut self, verify: bool) -> Self {
        self.verify_crc = verify;
        self
    }

    /// Returns the password if set
    pub fn password(&self) -> Option<&str> {
        self.password.as_deref()
    }

    /// Returns whether a password is set
    pub fn has_password(&self) -> bool {
        self.password.is_some()
    }

    /// Returns whether CRC verification is enabled
    pub fn verify_crc(&self) -> bool {
        self.verify_crc
    }
}

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
    /// ICE compressed file format (.ice) - Legacy DOS ICE
    Ice,
    /// Pack-Ice compressed format (.pi9, etc.) - Atari ST Pack-Ice
    PackIce,
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
        ArchiveFormat::PackIce,
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
            // Pack-Ice compressed pictures commonly use .PI9
            "pi9" => Some(ArchiveFormat::PackIce),
            "uc2" => Some(ArchiveFormat::Uc2),
            "ue2" => Some(ArchiveFormat::Uc2),
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
            ArchiveFormat::PackIce => "pi9",
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
            ArchiveFormat::Ice => "ICE (Legacy DOS)",
            ArchiveFormat::PackIce => "Pack-Ice (Atari ST)",
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
            ArchiveFormat::PackIce => &["pi9"],
            ArchiveFormat::Hyp => &["hyp"],
            ArchiveFormat::Ha => &["ha"],
            // UE2 is UltraCrypt-encrypted UC2 (detected, but not decrypted).
            ArchiveFormat::Uc2 => &["uc2", "ue2"],
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

    /// Returns the magic bytes (preamble) for this archive format, if available.
    ///
    /// Some formats have magic bytes at a specific offset rather than at the start,
    /// in which case this returns `None` and `detect_from_bytes` should be used instead.
    ///
    /// # Example
    /// ```
    /// use unarc_rs::unified::ArchiveFormat;
    ///
    /// assert_eq!(ArchiveFormat::Arj.preambles(), Some(&[&[0x60u8, 0xEA][..]][..]));
    /// assert_eq!(ArchiveFormat::Zip.preambles(), Some(&[b"PK\x03\x04".as_slice(), b"PK\x05\x06".as_slice()][..]));
    /// ```
    pub fn preambles(&self) -> Option<&'static [&'static [u8]]> {
        match self {
            // ACE: "**ACE**" at offset 7
            ArchiveFormat::Ace => Some(&[b"**ACE**"]),
            // ARC: 0x1A followed by method byte (1-11)
            ArchiveFormat::Arc => Some(&[&[0x1A]]),
            // ARJ: 0x60 0xEA
            ArchiveFormat::Arj => Some(&[&[0x60, 0xEA]]),
            // ZOO: "ZOO " text header
            ArchiveFormat::Zoo => Some(&[b"ZOO "]),
            // SQ/SQ2: two possible signatures
            ArchiveFormat::Sq => Some(&[&[0x76, 0xFF], &[0xFA, 0xFF]]),
            // SQZ: "HLSQZ"
            ArchiveFormat::Sqz => Some(&[b"HLSQZ"]),
            // Unix compress: 0x1F 0x9D
            ArchiveFormat::Z => Some(&[&[0x1F, 0x9D]]),
            // Gzip: 0x1F 0x8B
            ArchiveFormat::Gz => Some(&[&[0x1F, 0x8B]]),
            // Bzip2: "BZh"
            ArchiveFormat::Bz2 => Some(&[b"BZh"]),
            // ICE: no fixed magic, starts with size (Legacy DOS format)
            ArchiveFormat::Ice => None,
            // Pack-Ice: "ICE!", "Ice!", "TMM!", "TSM!", "SHE!" at offset 0
            ArchiveFormat::PackIce => Some(&[b"ICE!", b"Ice!", b"TMM!", b"TSM!", b"SHE!"]),
            // HYP: "HP" (compressed) or "ST" (stored)
            ArchiveFormat::Hyp => Some(&[b"HP", b"ST"]),
            // HA: "HA"
            ArchiveFormat::Ha => Some(&[b"HA"]),
            // UC2: "UC2\x1A" (normal) or "UE2" (UltraCrypt-encrypted)
            ArchiveFormat::Uc2 => Some(&[b"UC2\x1a", b"UE2"]),
            // LHA: "-lh" or "-lz" at offset 2
            ArchiveFormat::Lha => Some(&[b"-lh", b"-lz"]),
            // ZIP: "PK\x03\x04" or "PK\x05\x06" (empty)
            ArchiveFormat::Zip => Some(&[b"PK\x03\x04", b"PK\x05\x06"]),
            // RAR: "Rar!\x1A\x07"
            ArchiveFormat::Rar => Some(&[b"Rar!\x1a\x07"]),
            // 7z: "7z\xBC\xAF\x27\x1C"
            ArchiveFormat::SevenZ => Some(&[&[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C]]),
            // TAR: "ustar" at offset 257
            ArchiveFormat::Tar => Some(&[b"ustar"]),
            // TGZ: gzip magic (contains TAR inside)
            ArchiveFormat::Tgz => Some(&[&[0x1F, 0x8B]]),
            // TBZ: bzip2 magic (contains TAR inside)
            ArchiveFormat::Tbz => Some(&[b"BZh"]),
            // TAR.Z: Unix compress magic (contains TAR inside)
            ArchiveFormat::TarZ => Some(&[&[0x1F, 0x9D]]),
        }
    }

    /// Returns the offset where the magic bytes are located.
    ///
    /// Most formats have magic at offset 0, but some (like LHA, ACE, TAR) have it elsewhere.
    pub fn preamble_offset(&self) -> usize {
        match self {
            ArchiveFormat::Lha => 2,   // "-lh" or "-lz" at offset 2
            ArchiveFormat::Ace => 7,   // "**ACE**" at offset 7
            ArchiveFormat::Tar => 257, // "ustar" at offset 257
            _ => 0,
        }
    }

    /// Detect archive format from file content (magic bytes).
    ///
    /// This function attempts to identify the archive format by examining
    /// the first bytes of the file content. It checks for known magic bytes
    /// and signatures at various offsets.
    ///
    /// Returns `Some(format)` if a known format is detected, `None` otherwise.
    ///
    /// # Example
    /// ```
    /// use unarc_rs::unified::ArchiveFormat;
    ///
    /// let zip_data = b"PK\x03\x04rest of zip...";
    /// assert_eq!(ArchiveFormat::detect_from_bytes(zip_data), Some(ArchiveFormat::Zip));
    ///
    /// let unknown = b"random data";
    /// assert_eq!(ArchiveFormat::detect_from_bytes(unknown), None);
    /// ```
    pub fn detect_from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < 2 {
            return None;
        }

        // Check simple prefix-based formats first (most specific to least)

        // 7z: 6 bytes "7z\xBC\xAF\x27\x1C"
        if data.len() >= 6 && data.starts_with(&[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C]) {
            return Some(ArchiveFormat::SevenZ);
        }

        // RAR: "Rar!\x1A\x07" (6 bytes for RAR4, 7 bytes for RAR5)
        if data.len() >= 6 && data.starts_with(b"Rar!\x1a\x07") {
            return Some(ArchiveFormat::Rar);
        }

        // SQZ: "HLSQZ" (5 bytes)
        if data.len() >= 5 && data.starts_with(b"HLSQZ") {
            return Some(ArchiveFormat::Sqz);
        }

        // ZIP: "PK\x03\x04" or "PK\x05\x06" (empty archive)
        if data.len() >= 4 && (data.starts_with(b"PK\x03\x04") || data.starts_with(b"PK\x05\x06")) {
            return Some(ArchiveFormat::Zip);
        }

        // Pack-Ice: several 4-byte IDs at offset 0
        if data.len() >= 4
            && (data.starts_with(b"ICE!")
                || data.starts_with(b"Ice!")
                || data.starts_with(b"TMM!")
                || data.starts_with(b"TSM!")
                || data.starts_with(b"SHE!"))
        {
            return Some(ArchiveFormat::PackIce);
        }

        // UC2: "UC2\x1A"
        if data.len() >= 4 && data.starts_with(b"UC2\x1a") {
            return Some(ArchiveFormat::Uc2);
        }

        // UE2: UltraCrypt-encrypted UC2 wrapper
        if data.len() >= 3 && data.starts_with(b"UE2") {
            return Some(ArchiveFormat::Uc2);
        }

        // ZOO: "ZOO " (then version text)
        if data.len() >= 4 && data.starts_with(b"ZOO ") {
            return Some(ArchiveFormat::Zoo);
        }

        // BZ2: "BZh" (3 bytes)
        if data.len() >= 3 && data.starts_with(b"BZh") {
            // Could be TBZ if followed by TAR, but we can't know without decompressing
            // Return Bz2 as default, let caller use extension for TBZ
            return Some(ArchiveFormat::Bz2);
        }

        // Gzip: 0x1F 0x8B (2 bytes)
        if data.len() >= 2 && data[0] == 0x1F && data[1] == 0x8B {
            // Could be TGZ if contains TAR, but we can't know without decompressing
            // Return Gz as default, let caller use extension for TGZ
            return Some(ArchiveFormat::Gz);
        }

        // Unix compress: 0x1F 0x9D (2 bytes)
        if data.len() >= 2 && data[0] == 0x1F && data[1] == 0x9D {
            // Could be TAR.Z, but we can't know without decompressing
            return Some(ArchiveFormat::Z);
        }

        // ARJ: 0x60 0xEA (2 bytes)
        if data.len() >= 2 && data[0] == 0x60 && data[1] == 0xEA {
            return Some(ArchiveFormat::Arj);
        }

        // HA: "HA" (2 bytes)
        if data.len() >= 2 && data.starts_with(b"HA") {
            return Some(ArchiveFormat::Ha);
        }

        // HYP: "HP" (compressed) or "ST" (stored)
        if data.len() >= 2 && (data.starts_with(b"HP") || data.starts_with(b"ST")) {
            return Some(ArchiveFormat::Hyp);
        }

        // SQ: 0x76 0xFF or 0xFA 0xFF
        if data.len() >= 2 && data[1] == 0xFF && (data[0] == 0x76 || data[0] == 0xFA) {
            return Some(ArchiveFormat::Sq);
        }

        // ARC: 0x1A followed by method byte (1-11)
        if data.len() >= 2 && data[0] == 0x1A && data[1] >= 1 && data[1] <= 11 {
            return Some(ArchiveFormat::Arc);
        }

        // ACE: "**ACE**" at offset 7
        if data.len() >= 14 && &data[7..14] == b"**ACE**" {
            return Some(ArchiveFormat::Ace);
        }

        // LHA/LZH: "-lh" or "-lz" at offset 2
        if data.len() >= 5
            && data[2] == b'-'
            && data[3] == b'l'
            && (data[4] == b'h' || data[4] == b'z')
        {
            return Some(ArchiveFormat::Lha);
        }

        // TAR: "ustar" at offset 257 (POSIX format)
        if data.len() >= 263 && &data[257..262] == b"ustar" {
            return Some(ArchiveFormat::Tar);
        }

        // TAR: Alternative detection - check if it looks like a TAR header
        // TAR headers have null-terminated filename at start, mode at 100, etc.
        if data.len() >= 512 {
            // Check for valid TAR: name ends with null, reasonable checksum area
            let has_null_in_name = data[..100].contains(&0);
            let checksum_area = &data[148..156];
            let is_checksum_space_or_digit = checksum_area
                .iter()
                .all(|&b| b == b' ' || b == b'0' || (b >= b'1' && b <= b'7'));
            if has_null_in_name && is_checksum_space_or_digit {
                // Could be TAR, but this is a weak heuristic
                // Only return TAR if nothing else matched
                return Some(ArchiveFormat::Tar);
            }
        }

        None
    }

    /// Detect archive format from a reader.
    ///
    /// Reads up to 512 bytes from the reader, then seeks back to the original position.
    /// Returns `Some(format)` if detected, `None` otherwise.
    pub fn detect_from_reader<R: Read + Seek>(reader: &mut R) -> std::io::Result<Option<Self>> {
        let pos = reader.stream_position()?;
        let mut buffer = [0u8; 512];
        let bytes_read = reader.read(&mut buffer)?;
        reader.seek(std::io::SeekFrom::Start(pos))?;
        Ok(Self::detect_from_bytes(&buffer[..bytes_read]))
    }

    /// Detect archive format, trying preamble first, then falling back to extension.
    ///
    /// This is the recommended method for detecting archive formats:
    /// 1. First tries to detect from file content (magic bytes)
    /// 2. Falls back to extension-based detection if content detection fails
    ///
    /// # Example
    /// ```no_run
    /// use std::path::Path;
    /// use std::fs::File;
    /// use std::io::Read;
    /// use unarc_rs::unified::ArchiveFormat;
    ///
    /// let path = Path::new("archive.arj");
    /// let mut file = File::open(path).unwrap();
    /// let format = ArchiveFormat::detect(&mut file, Some(path)).unwrap();
    /// ```
    pub fn detect<R: Read + Seek>(
        reader: &mut R,
        path: Option<&Path>,
    ) -> std::io::Result<Option<Self>> {
        // First try content-based detection
        if let Some(format) = Self::detect_from_reader(reader)? {
            return Ok(Some(format));
        }

        // Fall back to extension-based detection
        if let Some(p) = path {
            return Ok(Self::from_path(p));
        }

        Ok(None)
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

    /// Open an archive with this format and options
    ///
    /// # Example
    /// ```no_run
    /// use std::fs::File;
    /// use unarc_rs::unified::{ArchiveFormat, ArchiveOptions};
    ///
    /// let file = File::open("encrypted.zip").unwrap();
    /// let options = ArchiveOptions::new().with_password("secret");
    /// let mut archive = ArchiveFormat::Zip.open_with_options(file, options).unwrap();
    /// ```
    pub fn open_with_options<T: Read + Seek>(
        self,
        reader: T,
        options: ArchiveOptions,
    ) -> Result<UnifiedArchive<T>> {
        UnifiedArchive::open_with_format_and_options(reader, self, options)
    }

    /// Open an archive file directly from a path with options
    ///
    /// # Example
    /// ```no_run
    /// use unarc_rs::unified::{ArchiveFormat, ArchiveOptions};
    ///
    /// let options = ArchiveOptions::new()
    ///     .with_password("secret")
    ///     .with_verify_crc(true);
    ///
    /// let mut archive = ArchiveFormat::open_path_with_options("encrypted.zip", options).unwrap();
    /// ```
    pub fn open_path_with_options<P: AsRef<Path>>(
        path: P,
        options: ArchiveOptions,
    ) -> Result<UnifiedArchive<BufReader<File>>> {
        let path = path.as_ref();
        let format = Self::from_path(path).ok_or_else(|| {
            ArchiveError::UnsupportedFormat(format!(
                "Unsupported archive format: {}",
                path.display()
            ))
        })?;
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut archive = format.open_with_options(reader, options)?;

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
    /// Encryption method (if any)
    encryption: EncryptionMethod,
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
    /// ICE format has no header per file, just one file (Legacy DOS)
    Ice,
    /// Pack-Ice format has no header per file, just one file (Atari ST)
    PackIce,
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

    /// Returns the encryption method used for this entry
    pub fn encryption(&self) -> EncryptionMethod {
        self.encryption
    }

    /// Returns true if this entry is encrypted
    pub fn is_encrypted(&self) -> bool {
        self.encryption.is_encrypted()
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
    Z(ZArchive<T>, bool),          // bool = already read
    Gz(GzArchive<T>, bool),        // bool = already read
    Bz2(Bz2Archive<T>, bool),      // bool = already read
    Ice(IceArchive, bool),         // bool = already read (Legacy DOS)
    PackIce(PackIceArchive, bool), // bool = already read (Atari ST)
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
    /// Options for archive operations (password, CRC verification, etc.)
    options: ArchiveOptions,
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
            ArchiveFormat::PackIce => {
                ArchiveInner::PackIce(PackIceArchive::from_reader(reader)?, false)
            }
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
            options: ArchiveOptions::default(),
        })
    }

    /// Open an archive with a specific format and options
    ///
    /// # Example
    /// ```no_run
    /// use std::fs::File;
    /// use unarc_rs::unified::{UnifiedArchive, ArchiveFormat, ArchiveOptions};
    ///
    /// let file = File::open("encrypted.zip").unwrap();
    /// let options = ArchiveOptions::new().with_password("secret");
    /// let archive = UnifiedArchive::open_with_format_and_options(file, ArchiveFormat::Zip, options).unwrap();
    /// ```
    pub fn open_with_format_and_options(
        reader: T,
        format: ArchiveFormat,
        options: ArchiveOptions,
    ) -> Result<Self> {
        let password = options.password().map(|s| s.to_string());

        let inner = match format {
            ArchiveFormat::Ace => {
                let mut archive = AceArchive::new(reader)?;
                if let Some(ref pwd) = password {
                    archive.set_password(pwd);
                }
                ArchiveInner::Ace(archive)
            }
            ArchiveFormat::Arc => {
                let mut archive = ArcArchive::new(reader)?;
                if let Some(ref pwd) = password {
                    archive.set_password(pwd);
                }
                ArchiveInner::Arc(archive)
            }
            ArchiveFormat::Arj => {
                let mut archive = ArjArchive::new(reader)?;
                if let Some(ref pwd) = password {
                    archive.set_password(pwd);
                }
                ArchiveInner::Arj(archive)
            }
            ArchiveFormat::Zoo => ArchiveInner::Zoo(ZooArchive::new(reader)?),
            ArchiveFormat::Sq => ArchiveInner::Sq(SqArchive::new(reader)?),
            ArchiveFormat::Sqz => ArchiveInner::Sqz(SqzArchive::new(reader)?),
            ArchiveFormat::Z => ArchiveInner::Z(ZArchive::new(reader)?, false),
            ArchiveFormat::Gz => ArchiveInner::Gz(GzArchive::new(reader)?, false),
            ArchiveFormat::Bz2 => ArchiveInner::Bz2(Bz2Archive::new(reader)?, false),
            ArchiveFormat::Ice => ArchiveInner::Ice(IceArchive::new(reader)?, false),
            ArchiveFormat::PackIce => {
                ArchiveInner::PackIce(PackIceArchive::from_reader(reader)?, false)
            }
            ArchiveFormat::Hyp => ArchiveInner::Hyp(HypArchive::new(reader)?),
            ArchiveFormat::Ha => ArchiveInner::Ha(HaArchive::new(reader)?),
            ArchiveFormat::Uc2 => ArchiveInner::Uc2(Uc2Archive::new(reader)?),
            ArchiveFormat::Lha => ArchiveInner::Lha(LhaArchiveSeekable::new(reader)?),
            ArchiveFormat::Zip => {
                let mut archive = ZipArchive::new(reader)?;
                if let Some(ref pwd) = password {
                    archive.set_password(pwd.as_bytes());
                }
                ArchiveInner::Zip(archive)
            }
            ArchiveFormat::Rar => {
                let mut archive = RarArchive::new(reader)?;
                if let Some(ref pwd) = password {
                    archive.set_password(pwd);
                }
                ArchiveInner::Rar(archive)
            }
            ArchiveFormat::SevenZ => {
                // 7z needs password at open time for encrypted archives
                ArchiveInner::SevenZ(SevenZArchive::new_with_password(reader, password)?)
            }
            ArchiveFormat::Tar => ArchiveInner::Tar(TarArchive::new(reader)?),
            ArchiveFormat::Tgz => ArchiveInner::Tgz(TgzArchive::new(reader)?),
            ArchiveFormat::Tbz => ArchiveInner::Tbz(TbzArchive::new(reader)?),
            ArchiveFormat::TarZ => ArchiveInner::TarZ(TarZArchive::new(reader)?),
        };

        Ok(Self {
            inner,
            format,
            single_file_name: None,
            options,
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

    /// Returns a reference to the archive options
    pub fn options(&self) -> &ArchiveOptions {
        &self.options
    }

    /// Returns a mutable reference to the archive options
    pub fn options_mut(&mut self) -> &mut ArchiveOptions {
        &mut self.options
    }

    /// Set the archive options
    pub fn set_options(&mut self, options: ArchiveOptions) {
        self.options = options;
    }

    /// Set the password for encrypted entries
    pub fn set_password<S: Into<String>>(&mut self, password: S) {
        self.options = std::mem::take(&mut self.options).with_password(password);
    }

    /// Clear the password
    pub fn clear_password(&mut self) {
        self.options.password = None;
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
                        encryption: if header.is_encrypted() {
                            EncryptionMethod::Ace
                        } else {
                            EncryptionMethod::None
                        },
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
                        encryption: EncryptionMethod::None,
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
                        encryption: match archive.get_encryption_type() {
                            Some(enc) => EncryptionMethod::Arj(enc),
                            None => EncryptionMethod::None,
                        },
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
                        encryption: EncryptionMethod::None,
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
                        encryption: EncryptionMethod::None,
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
                        encryption: EncryptionMethod::None,
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
                        encryption: EncryptionMethod::None,
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
                        encryption: EncryptionMethod::None,
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
                        encryption: EncryptionMethod::None,
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
                        encryption: EncryptionMethod::None,
                        index: EntryIndex::Ice,
                    }))
                }
            }
            ArchiveInner::PackIce(ref archive, ref mut read) => {
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
                        compressed_size: 0, // Not stored in Pack-Ice format
                        original_size: archive.original_size() as u64,
                        compression_method: "Pack-Ice".to_string(),
                        modified_time: None,
                        crc: 0,
                        encryption: EncryptionMethod::None,
                        index: EntryIndex::PackIce,
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
                        encryption: EncryptionMethod::None,
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
                        encryption: EncryptionMethod::None,
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
                        encryption: EncryptionMethod::None,
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
                        encryption: EncryptionMethod::None,
                        index: EntryIndex::Lha(header),
                    }))
                } else {
                    Ok(None)
                }
            }
            ArchiveInner::Zip(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    let encryption = if header.is_encrypted {
                        EncryptionMethod::Zip(ZipEncryption::Unknown)
                    } else {
                        EncryptionMethod::None
                    };
                    Ok(Some(ArchiveEntry {
                        name: header.name.clone(),
                        compressed_size: header.compressed_size,
                        original_size: header.original_size,
                        compression_method: header.compression_method.clone(),
                        modified_time: header.date_time,
                        crc: header.crc32 as u64,
                        encryption,
                        index: EntryIndex::Zip(header),
                    }))
                } else {
                    Ok(None)
                }
            }
            ArchiveInner::Rar(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    let encryption = if header.is_encrypted {
                        EncryptionMethod::Rar(RarEncryption::Unknown)
                    } else {
                        EncryptionMethod::None
                    };
                    Ok(Some(ArchiveEntry {
                        name: header.name.clone(),
                        compressed_size: header.compressed_size,
                        original_size: header.original_size,
                        compression_method: header.compression_method.clone(),
                        modified_time: header.date_time,
                        crc: header.crc32 as u64,
                        encryption,
                        index: EntryIndex::Rar(header),
                    }))
                } else {
                    Ok(None)
                }
            }
            ArchiveInner::SevenZ(archive) => {
                if let Some(header) = archive.get_next_entry()? {
                    // 7z encrypts at block level - check if this file's block is encrypted
                    let encryption = if header.is_encrypted {
                        EncryptionMethod::SevenZ(crate::encryption::SevenZEncryption::Aes256)
                    } else {
                        EncryptionMethod::None
                    };
                    Ok(Some(ArchiveEntry {
                        name: header.name.clone(),
                        compressed_size: header.compressed_size,
                        original_size: header.original_size,
                        compression_method: header.compression_method.clone(),
                        modified_time: header.date_time,
                        crc: header.crc32 as u64,
                        encryption,
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
                        encryption: EncryptionMethod::None,
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
                        encryption: EncryptionMethod::None,
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
                        encryption: EncryptionMethod::None,
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
                        encryption: EncryptionMethod::None,
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
            (ArchiveInner::PackIce(archive, _), EntryIndex::PackIce) => archive.read(),
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

    /// Read (decompress) an entry's data with specific options
    ///
    /// This allows passing a different password or CRC verification setting
    /// for a specific read operation without changing the archive's default options.
    ///
    /// # Example
    /// ```no_run
    /// use std::fs::File;
    /// use unarc_rs::unified::{UnifiedArchive, ArchiveFormat, ArchiveOptions};
    ///
    /// let file = File::open("archive.zip").unwrap();
    /// let mut archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Zip).unwrap();
    ///
    /// while let Some(entry) = archive.next_entry().unwrap() {
    ///     // Read with a specific password for this entry
    ///     let options = ArchiveOptions::new()
    ///         .with_password("file_specific_password")
    ///         .with_verify_crc(true);
    ///     let data = archive.read_with_options(&entry, &options).unwrap();
    ///     println!("Read {} bytes from {}", data.len(), entry.name());
    /// }
    /// ```
    pub fn read_with_options(
        &mut self,
        entry: &ArchiveEntry,
        options: &ArchiveOptions,
    ) -> Result<Vec<u8>> {
        let password = options.password().map(|s| s.to_string());

        match (&mut self.inner, &entry.index) {
            (ArchiveInner::Ace(archive), EntryIndex::Ace(header)) => {
                archive.read_with_password(header, password)
            }
            (ArchiveInner::Zip(archive), EntryIndex::Zip(header)) => {
                archive.read_with_password(header, password.as_ref().map(|s| s.as_bytes()))
            }
            (ArchiveInner::Rar(archive), EntryIndex::Rar(header)) => {
                archive.read_with_password(header, password)
            }
            (ArchiveInner::SevenZ(archive), EntryIndex::SevenZ(header)) => {
                archive.read_with_password(header, password)
            }
            (ArchiveInner::Arj(archive), EntryIndex::Arj(header)) => {
                archive.read_with_password(header, password)
            }
            // Other formats don't support encryption, use normal read
            _ => self.read(entry),
        }
    }

    /// Read (decompress) an entry's data with options and write directly to a writer
    ///
    /// This combines the memory efficiency of `read_to` with the flexibility
    /// of per-read options.
    pub fn read_to_with_options<W: Write>(
        &mut self,
        entry: &ArchiveEntry,
        writer: &mut W,
        options: &ArchiveOptions,
    ) -> Result<u64> {
        let data = self.read_with_options(entry, options)?;
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
            (ArchiveInner::PackIce(archive, _), EntryIndex::PackIce) => archive.skip(),
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
            encryption: EncryptionMethod::None,
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
            encryption: EncryptionMethod::None,
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
            encryption: EncryptionMethod::None,
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
            encryption: EncryptionMethod::None,
            index: EntryIndex::Z,
        };
        assert!((empty.compression_ratio() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_preambles() {
        // Test formats with single preamble
        assert_eq!(
            ArchiveFormat::Arj.preambles(),
            Some(&[&[0x60u8, 0xEA][..]][..])
        );
        assert_eq!(
            ArchiveFormat::Sqz.preambles(),
            Some(&[b"HLSQZ".as_slice()][..])
        );
        assert_eq!(ArchiveFormat::Ha.preambles(), Some(&[b"HA".as_slice()][..]));

        // Test formats with multiple preambles
        assert_eq!(
            ArchiveFormat::Sq.preambles(),
            Some(&[&[0x76u8, 0xFF][..], &[0xFAu8, 0xFF][..]][..])
        );
        assert_eq!(
            ArchiveFormat::Hyp.preambles(),
            Some(&[b"HP".as_slice(), b"ST".as_slice()][..])
        );
        assert_eq!(
            ArchiveFormat::Lha.preambles(),
            Some(&[b"-lh".as_slice(), b"-lz".as_slice()][..])
        );
        assert_eq!(
            ArchiveFormat::Zip.preambles(),
            Some(&[b"PK\x03\x04".as_slice(), b"PK\x05\x06".as_slice()][..])
        );

        // Test formats with offset > 0
        assert_eq!(ArchiveFormat::Lha.preamble_offset(), 2);
        assert_eq!(ArchiveFormat::Ace.preamble_offset(), 7);
        assert_eq!(ArchiveFormat::Tar.preamble_offset(), 257);
        assert_eq!(ArchiveFormat::Arj.preamble_offset(), 0);

        // ICE has no fixed magic
        assert_eq!(ArchiveFormat::Ice.preambles(), None);
    }

    #[test]
    fn test_detect_from_bytes() {
        // Test ZIP detection
        assert_eq!(
            ArchiveFormat::detect_from_bytes(b"PK\x03\x04rest of data"),
            Some(ArchiveFormat::Zip)
        );

        // Test ARJ detection
        assert_eq!(
            ArchiveFormat::detect_from_bytes(&[0x60, 0xEA, 0x00, 0x00]),
            Some(ArchiveFormat::Arj)
        );

        // Test RAR detection
        assert_eq!(
            ArchiveFormat::detect_from_bytes(b"Rar!\x1a\x07\x00rest"),
            Some(ArchiveFormat::Rar)
        );

        // Test 7z detection
        assert_eq!(
            ArchiveFormat::detect_from_bytes(&[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C, 0x00]),
            Some(ArchiveFormat::SevenZ)
        );

        // Test gzip detection
        assert_eq!(
            ArchiveFormat::detect_from_bytes(&[0x1F, 0x8B, 0x08, 0x00]),
            Some(ArchiveFormat::Gz)
        );

        // Test bzip2 detection
        assert_eq!(
            ArchiveFormat::detect_from_bytes(b"BZh9data"),
            Some(ArchiveFormat::Bz2)
        );

        // Test SQZ detection
        assert_eq!(
            ArchiveFormat::detect_from_bytes(b"HLSQZrest"),
            Some(ArchiveFormat::Sqz)
        );

        // Test HA detection
        assert_eq!(
            ArchiveFormat::detect_from_bytes(b"HArest"),
            Some(ArchiveFormat::Ha)
        );

        // Test ARC detection (0x1A + method 1-11)
        assert_eq!(
            ArchiveFormat::detect_from_bytes(&[0x1A, 0x02, 0x00]),
            Some(ArchiveFormat::Arc)
        );

        // Test ZOO detection
        assert_eq!(
            ArchiveFormat::detect_from_bytes(b"ZOO 2.10 Archive."),
            Some(ArchiveFormat::Zoo)
        );

        // Test ACE detection (magic at offset 7)
        let mut ace_data = vec![0u8; 20];
        ace_data[7..14].copy_from_slice(b"**ACE**");
        assert_eq!(
            ArchiveFormat::detect_from_bytes(&ace_data),
            Some(ArchiveFormat::Ace)
        );

        // Test LHA detection (magic at offset 2)
        assert_eq!(
            ArchiveFormat::detect_from_bytes(b"\x00\x00-lh5-rest"),
            Some(ArchiveFormat::Lha)
        );

        // Test UC2 detection
        assert_eq!(
            ArchiveFormat::detect_from_bytes(b"UC2\x1arest"),
            Some(ArchiveFormat::Uc2)
        );

        // Test UE2 (UltraCrypt) detection as UC2
        assert_eq!(
            ArchiveFormat::detect_from_bytes(b"UE2\x01rest"),
            Some(ArchiveFormat::Uc2)
        );

        // Test unknown format
        assert_eq!(ArchiveFormat::detect_from_bytes(b"random data here"), None);

        // Test too short data
        assert_eq!(ArchiveFormat::detect_from_bytes(b"X"), None);
        assert_eq!(ArchiveFormat::detect_from_bytes(b""), None);
    }
}
