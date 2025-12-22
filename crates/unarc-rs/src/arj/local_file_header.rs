use crate::date_time::DosDateTime;

use super::main_header::HostOS;

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompressionMethod {
    Stored = 0,
    CompressedMost = 1,
    Compressed = 2,
    CompressedFaster = 3,
    CompressedFastest = 4,
    NoDataNoCrc = 8,
    NoData = 9,
    Unknown(u8),
}

impl From<u8> for CompressionMethod {
    fn from(value: u8) -> Self {
        match value {
            0 => CompressionMethod::Stored,
            1 => CompressionMethod::CompressedMost,
            2 => CompressionMethod::Compressed,
            3 => CompressionMethod::CompressedFaster,
            4 => CompressionMethod::CompressedFastest,
            8 => CompressionMethod::NoDataNoCrc,
            9 => CompressionMethod::NoData,
            _ => CompressionMethod::Unknown(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LocalFileHeader {
    pub archiver_version_number: u8,
    pub min_version_to_extract: u8,
    pub host_os: HostOS,
    pub arj_flags: u8,
    pub compression_method: CompressionMethod,
    pub file_type: FileType,
    /// Password modifier byte used for garble encryption
    pub password_modifier: u8,

    pub date_time_modified: DosDateTime,
    pub compressed_size: u32,
    pub original_size: u32,
    pub original_crc32: u32,

    pub file_spec_position: u16,
    pub file_access_mode: u16,

    pub first_chapter: u8,
    pub last_chapter: u8,

    pub extended_file_position: u32,

    pub date_time_accessed: DosDateTime,
    pub date_time_created: DosDateTime,

    pub original_size_even_for_volumes: u32,

    pub name: String,
    pub comment: String,
}

/// Standard header size
const STD_HDR_SIZE: u8 = 30;
/// Minimum size of header that holds DTA/DTC
const R9_HDR_SIZE: u8 = 46;

impl LocalFileHeader {
    pub fn load_from(mut header_bytes: &[u8]) -> Option<Self> {
        convert_u8!(header_size, header_bytes);
        convert_u8!(archiver_version_number, header_bytes);
        convert_u8!(min_version_to_extract, header_bytes);
        convert_u8!(host_os, header_bytes);
        convert_u8!(arj_flags, header_bytes);
        convert_u8!(compression_method, header_bytes);
        convert_u8!(file_type, header_bytes);
        convert_u8!(password_modifier, header_bytes);
        convert_u32!(date_time_modified, header_bytes);
        convert_u32!(compressed_size, header_bytes);
        convert_u32!(original_size, header_bytes);
        convert_u32!(original_crc32, header_bytes);
        convert_u16!(file_spec_position, header_bytes);
        convert_u16!(file_access_mode, header_bytes);
        convert_u8!(first_chapter, header_bytes);
        convert_u8!(last_chapter, header_bytes);

        let mut extended_file_position = 0;
        let mut date_time_accessed = 0;
        let mut date_time_created = 0;
        let mut original_size_even_for_volumes = 0;
        if header_size > STD_HDR_SIZE {
            convert_u32!(extended_file_position2, header_bytes);
            extended_file_position = extended_file_position2;
            if header_size >= R9_HDR_SIZE {
                convert_u32!(date_time_accessed2, header_bytes);
                convert_u32!(date_time_created2, header_bytes);
                convert_u32!(original_size_even2_for_volumes, header_bytes);
                date_time_accessed = date_time_accessed2;
                date_time_created = date_time_created2;
                original_size_even_for_volumes = original_size_even2_for_volumes;
            }
        }

        convert_string!(name, header_bytes);
        convert_string!(comment, header_bytes);

        Some(LocalFileHeader {
            archiver_version_number,
            min_version_to_extract,
            host_os: host_os.into(),
            arj_flags,
            compression_method: compression_method.into(),
            file_type: file_type.into(),
            password_modifier,
            date_time_modified: DosDateTime::new(date_time_modified),
            compressed_size,
            original_size,
            original_crc32,
            file_spec_position,
            file_access_mode,
            first_chapter,
            last_chapter,
            extended_file_position,
            date_time_accessed: DosDateTime::new(date_time_accessed),
            date_time_created: DosDateTime::new(date_time_created),
            original_size_even_for_volumes,
            name,
            comment,
        })
    }

    pub fn is_garbled(&self) -> bool {
        self.arj_flags & 0x01 != 0
    }

    pub fn is_volume(&self) -> bool {
        self.arj_flags & 0x04 != 0
    }

    pub fn is_ext_file(&self) -> bool {
        self.arj_flags & 0x08 != 0
    }

    pub fn is_path_sym(&self) -> bool {
        self.arj_flags & 0x10 != 0
    }

    pub fn is_backup(&self) -> bool {
        self.arj_flags & 0x20 != 0
    }
}
