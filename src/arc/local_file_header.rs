use crate::date_time::DosDateTime;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Binary = 0,
    Text7Bit = 1,
    CommentHeader = 2,
    Directory = 3,
    VolumeLabel = 4,
    ChapterLabel = 5,
    Unknown(u8),
}

impl From<u8> for FileType {
    fn from(value: u8) -> Self {
        match value {
            0 => FileType::Binary,
            1 => FileType::Text7Bit,
            2 => FileType::CommentHeader,
            3 => FileType::Directory,
            4 => FileType::VolumeLabel,
            5 => FileType::ChapterLabel,
            _ => FileType::Unknown(value),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMethod {
    Unpacked(u8),
    /// Run Length Encoding
    RLE,
    /// RLE + Huffman
    Squeezed,
    /// Lempel-Zev compression
    Crunched(u8),
    Squashed,
    Crushed,
    Distilled,
    Unknown(u8),
}

impl From<u8> for CompressionMethod {
    fn from(value: u8) -> Self {
        match value {
            1..=2 => CompressionMethod::Unpacked(0),
            3 => CompressionMethod::RLE,
            4 => CompressionMethod::Squeezed,
            5..=8 => CompressionMethod::Crunched(0),
            9 => CompressionMethod::Squashed,
            10 => CompressionMethod::Crushed,
            11 => CompressionMethod::Distilled,
            _ => CompressionMethod::Unknown(value),
        }
    }
}

pub struct LocalFileHeader {
    pub compression_method: CompressionMethod,
    pub name: String,
    pub compressed_size: u32,
    pub date_time: DosDateTime,
    pub crc16: u16,
    pub original_size: u32,
}

impl LocalFileHeader {
    pub fn load_from(mut header_bytes: &[u8]) -> Option<Self> {
        convert_u8!(compression_method, header_bytes);
        if compression_method == 0 {
            return None;
        }
        let idx = header_bytes.iter().position(|&x| x == 0).unwrap_or(13);
        let name = String::from_utf8_lossy(&header_bytes[0..idx]).to_string();
        header_bytes = &header_bytes[13..];
        convert_u32!(compressed_size, header_bytes);
        convert_u32!(date_time, header_bytes);
        convert_u16!(crc16, header_bytes);
        convert_u32!(original_size, header_bytes);

        Some(LocalFileHeader {
            compression_method: compression_method.into(),
            name,
            compressed_size,
            date_time: DosDateTime::new(date_time),
            crc16,
            original_size,
        })
    }
}
