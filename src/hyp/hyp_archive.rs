use std::io::{self, Read, Seek};

use super::header::{CompressionMethod, Header, HEADER_SIZE};

const HYP_ID: u8 = 0x1a;
pub struct HypArchive<T: Read + Seek> {
    reader: T,
}

impl<T: Read + Seek> HypArchive<T> {
    pub fn new(reader: T) -> io::Result<Self> {
        Ok(Self { reader })
    }

    pub fn skip(&mut self, header: &Header) -> io::Result<()> {
        self.reader
            .seek(io::SeekFrom::Current(header.compressed_size as i64))?;
        Ok(())
    }

    pub fn read(&mut self, header: &Header) -> io::Result<Vec<u8>> {
        let mut compressed_buffer = vec![0; header.compressed_size as usize];
        self.reader.read_exact(&mut compressed_buffer)?;

        let uncompressed = match header.compression_method {
            CompressionMethod::Stored => compressed_buffer,
            CompressionMethod::Compressed => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "unsupported compression method {:?}",
                        header.compression_method
                    ),
                ))
            }
        };
        let checksum = calculate_checksum(&uncompressed);
        if checksum != header.checksum {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "checksum mismatch",
            ))
        } else {
            Ok(uncompressed)
        }
    }

    pub fn get_next_entry(&mut self) -> io::Result<Option<Header>> {
        let mut next_header = [0; 1];
        let Ok(_) = self.reader.read_exact(&mut next_header) else {
            return Ok(None);
        };

        if next_header[0] != HYP_ID {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid HYP ID"));
        }
        let mut header_bytes = [0; HEADER_SIZE];
        self.reader.read_exact(&mut header_bytes)?;

        Ok(Some(Header::load_from(&header_bytes, &mut self.reader)?))
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
