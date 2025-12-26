//! ACE archive header structures

use crate::date_time::DosDateTime;

/// ACE main header magic bytes
pub const ACE_MAGIC: &[u8; 7] = b"**ACE**";

/// Header type constants
#[allow(dead_code)]
pub mod header_type {
    pub const MAIN: u8 = 0;
    pub const FILE: u8 = 1;
    pub const RECOVERY32: u8 = 2;
    pub const RECOVERY64A: u8 = 3;
    pub const RECOVERY64B: u8 = 4;
}

/// Header flags for MAIN header
#[allow(dead_code)]
pub mod header_flags {
    pub const ADDSIZE: u16 = 0x0001;
    pub const COMMENT: u16 = 0x0002;
    pub const MEMORY_64BIT: u16 = 0x0004;
    pub const AV_STRING: u16 = 0x0008;
    pub const SOLID: u16 = 0x0010;
    pub const LOCKED: u16 = 0x0020;
    pub const PROTECTED: u16 = 0x0040;
    pub const PASSWORD: u16 = 0x0080;
    // Main header specific flags
    pub const V20FORMAT: u16 = 0x0100;
    pub const SFX: u16 = 0x0200;
    pub const LIMITSFXJR: u16 = 0x0400;
    pub const MULTIVOLUME: u16 = 0x0800;
    pub const ADVERT: u16 = 0x1000;
    pub const RECOVERY: u16 = 0x2000;
    pub const LOCKED_MAIN: u16 = 0x4000;
    pub const SOLID_MAIN: u16 = 0x8000;
    // File header specific flags (different meaning for same bits)
    pub const NTSECURITY: u16 = 0x0400;
    pub const CONTINUED_PREV: u16 = 0x1000;
    pub const CONTINUED_NEXT: u16 = 0x2000;
}

/// Compression types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    /// Stored (no compression)
    Stored,
    /// LZ77 compression (ACE 1.0)
    Lz77,
    /// Blocked compression (ACE 2.0)
    Blocked,
    /// Unknown compression type
    Unknown(u8),
}

impl From<u8> for CompressionType {
    fn from(value: u8) -> Self {
        match value {
            0 => CompressionType::Stored,
            1 => CompressionType::Lz77,
            2 => CompressionType::Blocked,
            n => CompressionType::Unknown(n),
        }
    }
}

impl std::fmt::Display for CompressionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompressionType::Stored => write!(f, "Stored"),
            CompressionType::Lz77 => write!(f, "LZ77"),
            CompressionType::Blocked => write!(f, "Blocked"),
            CompressionType::Unknown(n) => write!(f, "Unknown({})", n),
        }
    }
}

/// Compression quality
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionQuality {
    None,
    Fastest,
    Fast,
    Normal,
    Good,
    Best,
    Unknown(u8),
}

impl From<u8> for CompressionQuality {
    fn from(value: u8) -> Self {
        match value {
            0 => CompressionQuality::None,
            1 => CompressionQuality::Fastest,
            2 => CompressionQuality::Fast,
            3 => CompressionQuality::Normal,
            4 => CompressionQuality::Good,
            5 => CompressionQuality::Best,
            n => CompressionQuality::Unknown(n),
        }
    }
}

/// Host OS type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostOs {
    MsDos,
    Os2,
    Windows,
    Unix,
    MacOs,
    WinNt,
    Primos,
    AppleGs,
    Atari,
    Vax,
    Amiga,
    Next,
    Unknown(u8),
}

impl From<u8> for HostOs {
    fn from(value: u8) -> Self {
        match value {
            0 => HostOs::MsDos,
            1 => HostOs::Os2,
            2 => HostOs::Windows,
            3 => HostOs::Unix,
            4 => HostOs::MacOs,
            5 => HostOs::WinNt,
            6 => HostOs::Primos,
            7 => HostOs::AppleGs,
            8 => HostOs::Atari,
            9 => HostOs::Vax,
            10 => HostOs::Amiga,
            11 => HostOs::Next,
            n => HostOs::Unknown(n),
        }
    }
}

/// ACE main archive header
#[derive(Debug)]
pub struct MainHeader {
    pub header_crc: u16,
    pub header_size: u16,
    pub header_type: u8,
    pub header_flags: u16,
    pub extract_version: u8,
    pub creator_version: u8,
    pub host_os: HostOs,
    pub volume_number: u8,
    pub datetime: DosDateTime,
    pub advert: String,
    pub comment: Vec<u8>,
}

impl MainHeader {
    /// Check if archive uses 64-bit sizes
    pub fn is_64bit(&self) -> bool {
        self.header_flags & header_flags::MEMORY_64BIT != 0
    }

    /// Check if archive is solid
    pub fn is_solid(&self) -> bool {
        self.header_flags & header_flags::SOLID != 0
    }

    /// Check if archive is password protected
    pub fn is_encrypted(&self) -> bool {
        self.header_flags & header_flags::PASSWORD != 0
    }

    /// Check if archive is multi-volume
    pub fn is_multivolume(&self) -> bool {
        self.header_flags & header_flags::MULTIVOLUME != 0
    }
}

/// ACE file entry header
#[derive(Debug, Clone)]
pub struct FileHeader {
    pub header_crc: u16,
    pub header_size: u16,
    pub header_type: u8,
    pub header_flags: u16,
    pub packed_size: u64,
    pub original_size: u64,
    pub datetime: DosDateTime,
    pub attributes: u32,
    pub crc32: u32,
    pub compression_type: CompressionType,
    pub compression_quality: CompressionQuality,
    pub parameters: u16,
    pub filename: String,
    pub comment: Vec<u8>,
    /// File data offset in the archive
    pub data_offset: u64,
}

impl FileHeader {
    /// Check if entry is a directory
    pub fn is_directory(&self) -> bool {
        self.attributes & 0x10 != 0
    }

    /// Check if entry is encrypted
    pub fn is_encrypted(&self) -> bool {
        self.header_flags & header_flags::PASSWORD != 0
    }

    /// Check if entry continues from previous volume
    pub fn is_continued_from_prev(&self) -> bool {
        self.header_flags & header_flags::CONTINUED_PREV != 0
    }

    /// Check if entry continues to next volume
    pub fn is_continued_to_next(&self) -> bool {
        self.header_flags & header_flags::CONTINUED_NEXT != 0
    }

    /// Get dictionary size in bytes
    pub fn dictionary_size(&self) -> usize {
        let bits = (self.parameters & 0x0F) as usize;
        if bits < 10 {
            1024 // minimum 1K
        } else {
            1 << bits
        }
    }
}
