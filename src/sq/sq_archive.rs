use std::io::{self, Read, Seek};

use super::header::Header;

pub struct SqArchive<T: Read + Seek> {
    reader: T,
    read_header: bool,
}

impl<T: Read + Seek> SqArchive<T> {
    pub fn new(reader: T) -> io::Result<Self> {
        Ok(Self {
            reader,
            read_header: false,
        })
    }

    pub fn skip(&mut self, _header: &Header) -> io::Result<()> {
        // just 1 file in the archive
        Ok(())
    }

    pub fn read(&mut self, header: &Header) -> io::Result<Vec<u8>> {
        let mut compressed_buffer = Vec::new();
        self.reader.read_to_end(&mut compressed_buffer)?;

        let data = crate::arc::unsqueeze::unsqueeze(&compressed_buffer)?;

        let mut checksum: u16 = 0;
        for byte in data.iter() {
            checksum = checksum.wrapping_add(*byte as u16);
        }

        if checksum != header.checksum {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "checksum mismatch",
            ))
        } else {
            Ok(data)
        }
    }

    pub fn get_next_entry(&mut self) -> io::Result<Option<Header>> {
        if self.read_header {
            return Ok(None);
        }
        self.read_header = true;
        Ok(Some(Header::load_from(&mut self.reader)?))
    }
}

/// Hyper uses a non standard checksum algorithm.
pub fn calculate_checksum(data: &[u8]) -> u32 {
    let mut checksum: u32 = 0;
    for byte in data {
        checksum = checksum.wrapping_add(*byte as u32).rotate_left(1);
    }
    checksum
}
