use crate::error::{ArchiveError, Result};

const TEXT: &[u8; 5] = b"HLSQZ";

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OS {
    DOS = 0,
    OS2 = 1,
    MVS = 2,
    HPFS = 3,
    Amiga = 4,
    OsX = 5,
    Unix = 6,
    Unknown(u8),
}

impl From<u8> for OS {
    fn from(value: u8) -> Self {
        match value {
            0 => OS::DOS,
            1 => OS::OS2,
            2 => OS::MVS,
            3 => OS::HPFS,
            4 => OS::Amiga,
            5 => OS::OsX,
            6 => OS::Unix,
            _ => OS::Unknown(value),
        }
    }
}

pub struct SqzHeader {
    pub os: OS,
    pub flags: u8,
}

pub const SQZ_HEADER_SIZE: usize = 8;
impl SqzHeader {
    pub fn load_from(mut header_bytes: &[u8]) -> Result<Self> {
        if !header_bytes.starts_with(TEXT) {
            return Err(ArchiveError::invalid_header("SQZ"));
        }
        header_bytes = &header_bytes[TEXT.len()..];

        convert_u8!(_version, header_bytes);
        convert_u8!(os, header_bytes);
        convert_u8!(flags, header_bytes);

        Ok(Self { os: os.into(), flags })
    }
}
