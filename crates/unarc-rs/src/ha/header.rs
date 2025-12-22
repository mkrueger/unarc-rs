use crate::date_time::DosDateTime;
use std::io::Read;

use crate::error::{ArchiveError, Result};

pub const HA_MAGIC: &[u8] = b"HA";

/// Compression methods supported by HA archives
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMethod {
    /// Store (no compression)
    Cpy = 0,
    /// LZ77 + Arithmetic coding
    Asc = 1,
    /// PPM + Arithmetic coding
    Hsc = 2,
    /// Directory entry
    Dir = 0x0E,
    /// Special entry
    Special = 0x0F,
    /// Unknown method
    Unknown(u8),
}

impl From<u8> for CompressionMethod {
    fn from(value: u8) -> Self {
        match value & 0x0F {
            0 => CompressionMethod::Cpy,
            1 => CompressionMethod::Asc,
            2 => CompressionMethod::Hsc,
            0x0E => CompressionMethod::Dir,
            0x0F => CompressionMethod::Special,
            v => CompressionMethod::Unknown(v),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArchiveHeader {
    pub file_count: u16,
}

impl ArchiveHeader {
    pub fn load_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut magic = [0u8; 2];
        reader.read_exact(&mut magic)?;

        if magic != HA_MAGIC {
            return Err(ArchiveError::invalid_header("HA"));
        }

        let mut count_buf = [0u8; 2];
        reader.read_exact(&mut count_buf)?;
        let file_count = u16::from_le_bytes(count_buf);

        Ok(Self { file_count })
    }
}

/// HA file header structure
#[derive(Debug, Clone)]
pub struct FileHeader {
    pub version: u8,
    pub method: CompressionMethod,
    pub compressed_size: u32,
    pub original_size: u32,
    pub crc32: u32,
    pub timestamp: DosDateTime,
    pub path: String,
    pub name: String,
    pub machine_info: Vec<u8>,
}

impl FileHeader {
    pub fn load_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut ver_type = [0u8; 1];
        reader.read_exact(&mut ver_type)?;
        let version = ver_type[0] >> 4;
        let method = CompressionMethod::from(ver_type[0]);

        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        let compressed_size = u32::from_le_bytes(buf);

        reader.read_exact(&mut buf)?;
        let original_size = u32::from_le_bytes(buf);

        reader.read_exact(&mut buf)?;
        let crc32 = u32::from_le_bytes(buf);

        reader.read_exact(&mut buf)?;
        let timestamp = DosDateTime::from(u32::from_le_bytes(buf));

        let path = read_null_string(reader)?;
        let name = read_null_string(reader)?;

        let mut len_buf = [0u8; 1];
        reader.read_exact(&mut len_buf)?;
        let machine_info_len = len_buf[0] as usize;

        let mut machine_info = vec![0u8; machine_info_len];
        if machine_info_len > 0 {
            reader.read_exact(&mut machine_info)?;
        }

        Ok(Self {
            version,
            method,
            compressed_size,
            original_size,
            crc32,
            timestamp,
            path,
            name,
            machine_info,
        })
    }

    pub fn full_path(&self) -> String {
        if self.path.is_empty() {
            self.name.clone()
        } else {
            format!("{}/{}", self.path, self.name)
        }
    }

    pub fn is_directory(&self) -> bool {
        self.method == CompressionMethod::Dir
    }
}

fn read_null_string<R: Read>(reader: &mut R) -> Result<String> {
    let mut bytes = Vec::new();
    let mut byte = [0u8; 1];

    loop {
        reader.read_exact(&mut byte)?;
        if byte[0] == 0 {
            break;
        }
        bytes.push(byte[0]);
    }

    Ok(String::from_utf8_lossy(&bytes).into_owned())
}
