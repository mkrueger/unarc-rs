use super::{
    file_header::{CompressionMethod, FileHeader},
    sqz_header::{SqzHeader, SQZ_HEADER_SIZE},
};
use crate::error::{ArchiveError, Result};
use std::io::{Read, Seek};

pub struct SqzArchive<T: Read + Seek> {
    _header: SqzHeader,
    password_crc32: u32,
    reader: T,
}

impl<T: Read + Seek> SqzArchive<T> {
    pub fn new(mut reader: T) -> Result<Self> {
        let mut header_bytes = [0; SQZ_HEADER_SIZE];
        reader.read_exact(&mut header_bytes)?;
        let header = SqzHeader::load_from(&header_bytes)?;

        Ok(Self {
            _header: header,
            reader,
            password_crc32: 0,
        })
    }

    pub fn skip(&mut self, header: &FileHeader) -> Result<()> {
        self.reader
            .seek(std::io::SeekFrom::Current(header.compressed_size as i64))?;
        Ok(())
    }

    pub fn read(&mut self, header: &FileHeader) -> Result<Vec<u8>> {
        let mut compressed_buffer = vec![0; header.compressed_size as usize];
        self.reader.read_exact(&mut compressed_buffer)?;

        let uncompressed = match header.compression_method {
            CompressionMethod::Stored => compressed_buffer,
            CompressionMethod::Compressed => super::unsqz::unsqz_compressed(
                &compressed_buffer,
                header.original_size as usize,
                header.method,
                header.crc32,
            )?,

            CompressionMethod::Unknown(m) => {
                return Err(ArchiveError::unsupported_method(
                    "SQZ",
                    format!("Unknown({})", m),
                ));
            }
        };

        let actual_crc = crc32fast::hash(&uncompressed);
        if actual_crc != header.crc32 {
            return Err(ArchiveError::crc_mismatch("SQZ", header.crc32, actual_crc));
        }

        Ok(uncompressed)
    }

    pub fn get_next_entry(&mut self) -> Result<Option<FileHeader>> {
        let mut next_header = [0; 1];
        self.reader.read_exact(&mut next_header)?;
        match next_header[0] {
            0 => Ok(None),
            1 => {
                // comment block
                // Format: uncompressed_size(2) + compressed_size(2) + flags(1) + crc32(4) + data(compressed_size)
                let mut uncompressed_size = [0; 2];
                self.reader.read_exact(&mut uncompressed_size)?;
                let _uncompressed_size = u16::from_le_bytes(uncompressed_size);
                let mut compressed_size = [0; 2];
                self.reader.read_exact(&mut compressed_size)?;
                let compressed_size = u16::from_le_bytes(compressed_size);
                // Skip: flags(1) + crc32(4) + compressed_data(compressed_size)
                self.reader
                    .seek(std::io::SeekFrom::Current(1 + 4 + compressed_size as i64))?;
                self.get_next_entry()
            }
            2 => {
                // password block
                let mut size = [0; 2];
                self.reader.read_exact(&mut size)?;
                let mut password_crc32 = [0; 4];
                self.reader.read_exact(&mut password_crc32)?;
                self.password_crc32 = u32::from_le_bytes(password_crc32);
                self.get_next_entry()
            }
            size => {
                if size >= 18 {
                    // + 1 for the checksum size & checksum bytes are not part of the length.
                    let mut header_bytes = vec![0; 1 + size as usize];
                    self.reader.read_exact(&mut header_bytes)?;
                    let current_local_file_header = FileHeader::load_from(&header_bytes)?;
                    return Ok(Some(current_local_file_header));
                }

                // 'other blocks'
                let mut size = [0; 2];
                self.reader.read_exact(&mut size)?;
                let size = u16::from_le_bytes(size);
                self.reader.seek(std::io::SeekFrom::Current(size as i64))?;
                self.get_next_entry()
            }
        }
    }
}
