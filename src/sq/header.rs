use std::io::{self, Read};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMethod {
    Stored,
    Compressed,
}

const SQ_SIG: [u8; 2] = [0x76, 0xFF];
const SQ2_SIG: [u8; 2] = [0xFA, 0xFF];

pub struct Header {
    pub name: String,
    pub checksum: u16,
}

pub const HEADER_SIZE: usize = 21;
impl Header {
    pub fn load_from<T: Read>(reader: &mut T) -> io::Result<Self> {
        let mut header: [u8; 2] = [0; 2];
        reader.read_exact(&mut header)?;
        if header == SQ_SIG {
            Self::load_sq1_header(reader)
        } else if header == SQ2_SIG {
            Self::load_sq2_header(reader)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid header signature",
            ))
        }
    }

    fn load_sq1_header<T: Read>(reader: &mut T) -> io::Result<Self> {
        let mut bytes: [u8; 2] = [0; 2];
        reader.read_exact(&mut bytes)?;
        let checksum = u16::from_le_bytes(bytes);
        let mut name = String::new();
        let mut byte = [0; 1];
        loop {
            reader.read_exact(&mut byte)?;
            if byte[0] == 0 {
                break;
            }
            name.push(byte[0] as char);
        }
        Ok(Self { name, checksum })
    }

    fn load_sq2_header<T: Read>(reader: &mut T) -> io::Result<Self> {
        let mut name = String::new();
        let mut byte = [0; 1];
        loop {
            reader.read_exact(&mut byte)?;
            if byte[0] == 0 {
                break;
            }
            name.push(byte[0] as char);
        }

        let mut date = String::new();
        loop {
            reader.read_exact(&mut byte)?;
            if byte[0] == 0 {
                break;
            }
            date.push(byte[0] as char);
        }

        // Maybe it's something connected to the checksum, seems that the checksum is larger in sq2
        // but doesn't seem to be u32 -> for now just skip the extra info.
        let mut unknown: [u8; 2] = [0; 2];
        reader.read_exact(&mut unknown)?;

        let mut bytes: [u8; 2] = [0; 2];
        reader.read_exact(&mut bytes)?;
        let checksum = u16::from_le_bytes(bytes);

        let mut unknown: [u8; 4] = [0; 4];
        reader.read_exact(&mut unknown)?;

        Ok(Self { name, checksum })
    }
}
