//! CAB header structures: `CFHEADER`, `CFFOLDER` and `CFFILE`.

use std::fmt;
use std::io::Read;

use crate::date_time::DosDateTime;
use crate::error::{ArchiveError, Result};

/// Signature at offset 0 of every cabinet
pub const CAB_MAGIC: &[u8; 4] = b"MSCF";

/// Size of the fixed part of `CFHEADER`
const HEADER_SIZE: usize = 36;

/// `CFHEADER.flags`: the cabinet continues a previous one
pub const FLAG_PREV_CABINET: u16 = 0x0001;
/// `CFHEADER.flags`: the cabinet is continued by a next one
pub const FLAG_NEXT_CABINET: u16 = 0x0002;
/// `CFHEADER.flags`: reserve fields are present
pub const FLAG_RESERVE_PRESENT: u16 = 0x0004;

/// `CFFILE.iFolder`: the file starts in the previous cabinet
pub const IFOLD_CONTINUED_FROM_PREV: u16 = 0xFFFD;
/// `CFFILE.iFolder`: the file continues in the next cabinet
pub const IFOLD_CONTINUED_TO_NEXT: u16 = 0xFFFE;
/// `CFFILE.iFolder`: the file starts in the previous and continues in the next cabinet
pub const IFOLD_CONTINUED_PREV_AND_NEXT: u16 = 0xFFFF;

/// `CFFILE.attribs`: the name is UTF-8 rather than in an unspecified code page
pub const ATTR_NAME_IS_UTF: u16 = 0x80;

/// Longest name/string field accepted (the format allows 255 bytes plus terminator)
const MAX_STRING: usize = 256;

/// Compression method of a folder (`CFFOLDER.typeCompress`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMethod {
    /// Stored without compression
    None,
    /// MSZIP (Deflate with a 32 KiB history carried across data blocks)
    MsZip,
    /// Quantum (not supported)
    Quantum {
        /// Compression level (1-7)
        level: u8,
        /// Window size as a power of two (10-21)
        window_bits: u8,
    },
    /// LZX
    Lzx {
        /// Window size as a power of two (15-21 for cabinets)
        window_bits: u8,
    },
    /// Unknown compression type
    Unknown(u16),
}

impl CompressionMethod {
    /// Decodes a `typeCompress` field
    #[must_use]
    pub const fn from_type_compress(value: u16) -> Self {
        let parameter = ((value >> 8) & 0x1F) as u8;
        match value & 0x000F {
            0 => Self::None,
            1 => Self::MsZip,
            2 => Self::Quantum {
                level: ((value >> 4) & 0x0F) as u8,
                window_bits: parameter,
            },
            3 => Self::Lzx { window_bits: parameter },
            _ => Self::Unknown(value),
        }
    }
}

impl fmt::Display for CompressionMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "Stored"),
            Self::MsZip => write!(f, "MSZIP"),
            Self::Quantum { window_bits, .. } => write!(f, "Quantum:{window_bits}"),
            Self::Lzx { window_bits } => write!(f, "LZX:{window_bits}"),
            Self::Unknown(value) => write!(f, "Unknown({value:#06x})"),
        }
    }
}

/// Cabinet header (`CFHEADER`)
#[derive(Debug, Clone)]
pub struct CabHeader {
    /// Total size of the cabinet file as recorded in the header
    pub cabinet_size: u32,
    /// Offset of the first `CFFILE` entry
    pub files_offset: u32,
    /// Format version (major, minor), normally 1.3
    pub version: (u8, u8),
    /// Number of `CFFOLDER` entries
    pub folder_count: u16,
    /// Number of `CFFILE` entries
    pub file_count: u16,
    /// Header flags
    pub flags: u16,
    /// Identifier shared by all cabinets of a set
    pub set_id: u16,
    /// Index of this cabinet within its set
    pub cabinet_index: u16,
    /// Size of the per-folder reserved area
    pub folder_reserve: u8,
    /// Size of the per-data-block reserved area
    pub data_reserve: u8,
    /// Name of the previous cabinet in the set
    pub previous_cabinet: Option<String>,
    /// Name of the next cabinet in the set
    pub next_cabinet: Option<String>,
}

impl CabHeader {
    /// Reads a cabinet header, leaving `reader` at the first `CFFOLDER` entry
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; HEADER_SIZE];
        reader.read_exact(&mut buf)?;
        if &buf[0..4] != CAB_MAGIC {
            return Err(ArchiveError::invalid_header("CAB"));
        }
        let u16_at = |pos: usize| u16::from_le_bytes([buf[pos], buf[pos + 1]]);
        let u32_at = |pos: usize| u32::from_le_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);

        let version = (buf[25], buf[24]);
        if version.0 != 1 {
            return Err(ArchiveError::unsupported_format(format!("CAB version {}.{}", version.0, version.1)));
        }
        let flags = u16_at(30);
        let mut header = Self {
            cabinet_size: u32_at(8),
            files_offset: u32_at(16),
            version,
            folder_count: u16_at(26),
            file_count: u16_at(28),
            flags,
            set_id: u16_at(32),
            cabinet_index: u16_at(34),
            folder_reserve: 0,
            data_reserve: 0,
            previous_cabinet: None,
            next_cabinet: None,
        };

        if flags & FLAG_RESERVE_PRESENT != 0 {
            let mut reserve = [0u8; 4];
            reader.read_exact(&mut reserve)?;
            let header_reserve = u16::from_le_bytes([reserve[0], reserve[1]]);
            header.folder_reserve = reserve[2];
            header.data_reserve = reserve[3];
            std::io::copy(&mut reader.take(u64::from(header_reserve)), &mut std::io::sink())?;
        }
        if flags & FLAG_PREV_CABINET != 0 {
            header.previous_cabinet = Some(latin1(&read_string(reader)?));
            read_string(reader)?; // disk name
        }
        if flags & FLAG_NEXT_CABINET != 0 {
            header.next_cabinet = Some(latin1(&read_string(reader)?));
            read_string(reader)?; // disk name
        }
        Ok(header)
    }
}

/// Folder entry (`CFFOLDER`): a compressed stream holding one or more files
#[derive(Debug, Clone)]
pub struct CabFolder {
    /// Offset of the folder's first `CFDATA` block
    pub data_offset: u32,
    /// Number of `CFDATA` blocks in this cabinet
    pub block_count: u16,
    /// Compression method
    pub compression_method: CompressionMethod,
}

impl CabFolder {
    /// Reads a folder entry and skips its reserved area
    pub fn read<R: Read>(reader: &mut R, reserve: u8) -> Result<Self> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        std::io::copy(&mut reader.take(u64::from(reserve)), &mut std::io::sink())?;
        Ok(Self {
            data_offset: u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]),
            block_count: u16::from_le_bytes([buf[4], buf[5]]),
            compression_method: CompressionMethod::from_type_compress(u16::from_le_bytes([buf[6], buf[7]])),
        })
    }
}

/// File entry (`CFFILE`)
#[derive(Debug, Clone)]
pub struct CabFileHeader {
    /// File name; `\` separators are converted to `/`
    pub name: String,
    /// Uncompressed size
    pub original_size: u32,
    /// Offset of the file's data within the uncompressed folder
    pub folder_offset: u32,
    /// Folder index, or one of the `IFOLD_CONTINUED_*` values
    pub folder_index: u16,
    /// DOS date and time
    pub date_time: DosDateTime,
    /// Attributes (`_A_RDONLY`, `_A_HIDDEN`, `_A_SYSTEM`, `_A_ARCH`, `_A_EXEC`, `_A_NAME_IS_UTF`)
    pub attributes: u16,
    /// Compression method of the file's folder
    pub compression_method: CompressionMethod,
    /// Index of this entry in the file table
    pub index: usize,
}

impl CabFileHeader {
    /// Reads a file entry
    pub fn read<R: Read>(reader: &mut R, index: usize) -> Result<Self> {
        let mut buf = [0u8; 16];
        reader.read_exact(&mut buf)?;
        let attributes = u16::from_le_bytes([buf[14], buf[15]]);
        let raw_name = read_string(reader)?;
        // Without the UTF-8 flag the code page is unspecified; Latin-1 keeps every byte.
        let name = if attributes & ATTR_NAME_IS_UTF != 0 {
            String::from_utf8_lossy(&raw_name).into_owned()
        } else {
            latin1(&raw_name)
        };
        Ok(Self {
            name: name.replace('\\', "/"),
            original_size: u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]),
            folder_offset: u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]),
            folder_index: u16::from_le_bytes([buf[8], buf[9]]),
            date_time: DosDateTime::from((u16::from_le_bytes([buf[10], buf[11]]), u16::from_le_bytes([buf[12], buf[13]]))),
            attributes,
            compression_method: CompressionMethod::None,
            index,
        })
    }

    /// Returns true if the file's data starts in a previous cabinet or continues in the next one
    #[must_use]
    pub const fn is_continued(&self) -> bool {
        self.folder_index >= IFOLD_CONTINUED_FROM_PREV
    }
}

/// Reads a NUL-terminated string of at most [`MAX_STRING`] bytes
fn read_string<R: Read>(reader: &mut R) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    let mut byte = [0u8; 1];
    loop {
        reader.read_exact(&mut byte)?;
        if byte[0] == 0 {
            return Ok(bytes);
        }
        if bytes.len() == MAX_STRING {
            return Err(ArchiveError::corrupted_entry("CAB", "unterminated name"));
        }
        bytes.push(byte[0]);
    }
}

fn latin1(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| char::from(b)).collect()
}
