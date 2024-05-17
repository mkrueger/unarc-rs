use super::{
    file_header::{CompressionMethod, FileHeader},
    sqz_header::{SqzHeader, SQZ_HEADER_SIZE},
};
use std::io::{self, Read, Seek};

pub struct SqzArchieve<T: Read + Seek> {
    _header: SqzHeader,
    password_crc32: u32,
    reader: T,
}

impl<T: Read + Seek> SqzArchieve<T> {
    pub fn new(mut reader: T) -> io::Result<Self> {
        let mut header_bytes = [0; SQZ_HEADER_SIZE];
        reader.read_exact(&mut header_bytes)?;
        let header = SqzHeader::load_from(&header_bytes)?;

        Ok(Self {
            _header: header,
            reader,
            password_crc32: 0,
        })
    }

    pub fn skip(&mut self, header: &FileHeader) -> io::Result<()> {
        self.reader
            .seek(io::SeekFrom::Current(header.compressed_size as i64))?;
        Ok(())
    }

    pub fn read(&mut self, header: &FileHeader) -> io::Result<Vec<u8>> {
        let mut compressed_buffer = vec![0; header.compressed_size as usize];
        self.reader.read_exact(&mut compressed_buffer)?;

        let uncompressed = match header.compression_method {
            CompressionMethod::Stored => compressed_buffer,
            CompressionMethod::Compressed => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Unsupported compression method {:?}",
                        header.compression_method
                    ),
                ))
            }

            CompressionMethod::Unknown(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Unsupported compression method {:?}",
                        header.compression_method
                    ),
                ))
            }
        };
        let checksum = crc32fast::hash(&uncompressed);
        if checksum != header.crc32 {
            Err(io::Error::new(io::ErrorKind::InvalidData, "CRC mismatch"))
        } else {
            Ok(uncompressed)
        }
    }

    pub fn get_next_entry(&mut self) -> io::Result<Option<FileHeader>> {
        let mut next_header = [0; 1];
        self.reader.read_exact(&mut next_header)?;
        match next_header[0] {
            0 => return Ok(None),
            1 => {
                // comment block
                let mut size = [0; 2];
                self.reader.read_exact(&mut size)?;
                let size = u16::from_le_bytes(size);
                self.reader
                    .seek(io::SeekFrom::Current(size as i64 + 2 + 1 + 4))?;
                return self.get_next_entry();
            }
            2 => {
                // password block
                let mut size = [0; 2];
                self.reader.read_exact(&mut size)?;
                let mut password_crc32 = [0; 4];
                self.reader.read_exact(&mut password_crc32)?;
                self.password_crc32 = u32::from_le_bytes(password_crc32);
                return self.get_next_entry();
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
                self.reader.seek(io::SeekFrom::Current(size as i64))?;
                return self.get_next_entry();
            }
        }
    }
}
