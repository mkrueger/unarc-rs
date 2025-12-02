use std::io::{self, Read};

use crate::date_time::DosDateTime;

const COMPRESSED: u16 = 0x5048; // "HP"
const STORED: u16 = 0x5453; // "ST"

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMethod {
    Stored,
    Compressed,
}

#[derive(Debug, Clone)]
pub struct Header {
    pub compression_method: CompressionMethod,
    pub version: u8,
    pub compressed_size: u32,
    pub original_size: u32,
    pub date_time: DosDateTime,
    pub checksum: u32,
    pub attribute: u8,
    pub name: String,
}
pub const HEADER_SIZE: usize = 21;
impl Header {
    pub fn load_from<T: Read>(mut header_bytes: &[u8], reader: &mut T) -> io::Result<Self> {
        convert_u16!(compression_method, header_bytes);
        convert_u8!(version, header_bytes);
        convert_u32!(compressed_size, header_bytes);
        convert_u32!(original_size, header_bytes);
        convert_u32!(date_time, header_bytes);
        convert_u32!(checksum, header_bytes);
        convert_u8!(attribute, header_bytes);
        convert_u8!(name_length, header_bytes);
        let mut name_buffer = vec![0; name_length as usize];
        reader.read_exact(&mut name_buffer)?;
        let name = String::from_utf8_lossy(&name_buffer).to_string();
        Ok(Header {
            compression_method: match compression_method {
                STORED => CompressionMethod::Stored,
                COMPRESSED => CompressionMethod::Compressed,
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "invalid compression method",
                    ))
                }
            },
            version,
            compressed_size,
            original_size,
            date_time: DosDateTime::new(date_time),
            checksum,
            attribute,
            name,
        })
    }
}
