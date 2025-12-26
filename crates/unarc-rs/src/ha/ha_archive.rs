use crc32fast::Hasher;
use std::io::{Cursor, Read, Seek, SeekFrom};

use crate::error::{ArchiveError, Result};
use crate::ha::asc::decompress_asc;

use super::header::{ArchiveHeader, CompressionMethod, FileHeader};
use super::hsc::decompress_hsc;

/// HA Archive reader
pub struct HaArchive<T: Read + Seek> {
    reader: T,
    pub file_count: u16,
    files_read: u16,
}

impl<T: Read + Seek> HaArchive<T> {
    pub fn new(mut reader: T) -> Result<Self> {
        let header = ArchiveHeader::load_from(&mut reader)?;

        Ok(Self {
            reader,
            file_count: header.file_count,
            files_read: 0,
        })
    }

    pub fn get_next_entry(&mut self) -> Result<Option<FileHeader>> {
        if self.files_read >= self.file_count {
            return Ok(None);
        }

        let header = FileHeader::load_from(&mut self.reader)?;
        self.files_read += 1;
        Ok(Some(header))
    }

    pub fn skip(&mut self, header: &FileHeader) -> Result<()> {
        self.reader
            .seek(SeekFrom::Current(header.compressed_size as i64))?;
        Ok(())
    }

    pub fn read(&mut self, header: &FileHeader) -> Result<Vec<u8>> {
        let mut compressed = vec![0u8; header.compressed_size as usize];
        self.reader.read_exact(&mut compressed)?;

        let decompressed = match header.method {
            CompressionMethod::Cpy => compressed,
            CompressionMethod::Dir => Vec::new(),
            CompressionMethod::Asc => decompress_asc(Cursor::new(compressed))?,
            CompressionMethod::Hsc => decompress_hsc(Cursor::new(compressed))?,
            CompressionMethod::Special => {
                log::debug!(
                    "Skipping special file '{}' (symlink, device, fifo, or socket)",
                    header.full_path()
                );
                Vec::new()
            }
            CompressionMethod::Unknown(m) => {
                return Err(ArchiveError::unsupported_method(
                    "HA",
                    format!("Unknown({})", m),
                ));
            }
        };

        // Verify CRC-32 (skip for directories)
        if header.method != CompressionMethod::Dir {
            let mut hasher = Hasher::new();
            hasher.update(&decompressed);
            let calculated_crc = hasher.finalize();

            if calculated_crc != header.crc32 {
                return Err(ArchiveError::crc_mismatch(
                    header.full_path(),
                    header.crc32,
                    calculated_crc,
                ));
            }
        }

        Ok(decompressed)
    }
}
