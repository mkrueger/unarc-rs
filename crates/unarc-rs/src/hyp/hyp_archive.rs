use std::io::{Read, Seek, SeekFrom};

use super::header::{CompressionMethod, Header, HEADER_SIZE};
use super::hyp_unpack;
use crate::error::{ArchiveError, Result};

const HYP_ID: u8 = 0x1a;
pub struct HypArchive<T: Read + Seek> {
    reader: T,
}

impl<T: Read + Seek> HypArchive<T> {
    pub fn new(reader: T) -> Result<Self> {
        Ok(Self { reader })
    }

    pub fn skip(&mut self, header: &Header) -> Result<()> {
        self.reader.seek(SeekFrom::Current(header.compressed_size as i64))?;
        Ok(())
    }

    pub fn read(&mut self, header: &Header) -> Result<Vec<u8>> {
        let mut compressed_buffer = vec![0; header.compressed_size as usize];
        self.reader.read_exact(&mut compressed_buffer)?;

        // Checksum is calculated on compressed data
        let checksum = calculate_checksum(&compressed_buffer);
        if checksum != header.checksum {
            return Err(ArchiveError::crc_mismatch("HYP", header.checksum, checksum));
        }

        let uncompressed = match header.compression_method {
            CompressionMethod::Stored => compressed_buffer,
            CompressionMethod::Compressed => hyp_unpack::unpack_hyp(&compressed_buffer, header.original_size as usize, header.version)?,
        };
        Ok(uncompressed)
    }

    pub fn get_next_entry(&mut self) -> Result<Option<Header>> {
        let mut next_header = [0; 1];
        let Ok(_) = self.reader.read_exact(&mut next_header) else {
            return Ok(None);
        };

        if next_header[0] != HYP_ID {
            return Err(ArchiveError::invalid_header("HYP"));
        }
        let mut header_bytes = [0; HEADER_SIZE];
        self.reader.read_exact(&mut header_bytes)?;

        Ok(Some(Header::load_from(&header_bytes, &mut self.reader)?))
    }
}

/// Hyper checksum algorithm: add-then-rotate-left.
/// Applied to the compressed data (not the uncompressed content).
fn calculate_checksum(data: &[u8]) -> u32 {
    let mut checksum: u32 = 0;
    for &byte in data {
        checksum = checksum.wrapping_add(byte as u32).rotate_left(1);
    }
    checksum
}
