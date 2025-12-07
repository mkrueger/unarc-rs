use crate::date_time::DosDateTime;
use crate::error::Result;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMethod {
    Stored,
    Compressed,
    Unknown(u8),
}

impl From<u8> for CompressionMethod {
    fn from(value: u8) -> Self {
        match value {
            0 => CompressionMethod::Stored,
            1..=5 => CompressionMethod::Compressed,
            _ => CompressionMethod::Unknown(value),
        }
    }
}
#[derive(Debug, Clone)]
pub struct FileHeader {
    pub checksum: u8,
    pub compression_method: CompressionMethod,

    pub compressed_size: u32,
    pub original_size: u32,
    /// DOS format date
    pub date_time: DosDateTime,

    pub attribute: u8,
    pub crc32: u32,

    pub name: String,
}

impl FileHeader {
    pub fn load_from(mut header_bytes: &[u8]) -> Result<Self> {
        convert_u8!(checksum, header_bytes);
        convert_u8!(compression_method, header_bytes);
        convert_u32!(compressed_size, header_bytes);
        convert_u32!(original_size, header_bytes);
        convert_u32!(date_time2, header_bytes);
        convert_u8!(attribute, header_bytes);
        convert_u32!(crc32, header_bytes);

        let name = String::from_utf8_lossy(header_bytes).to_string();

        Ok(FileHeader {
            checksum,
            compression_method: compression_method.into(),
            compressed_size,
            original_size,
            date_time: DosDateTime::new(date_time2),
            attribute,
            crc32,
            name,
        })
    }
}
