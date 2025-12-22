//! ACE archive reader

use std::io::{Read, Seek, SeekFrom};

use crate::date_time::DosDateTime;
use crate::error::{ArchiveError, Result};

use super::bitstream::BitStream;
use super::crc16::ace_crc16;
use super::crypto::decrypt_ace_data;
use super::header::{
    header_flags, header_type, CompressionQuality, CompressionType, FileHeader, HostOs, MainHeader,
    ACE_MAGIC,
};
use super::lz77::Lz77Decoder;

/// ACE archive reader
pub struct AceArchive<R: Read + Seek> {
    reader: R,
    main_header: MainHeader,
    lz77: Lz77Decoder,
    password: Option<String>,
}

impl<R: Read + Seek> AceArchive<R> {
    /// Open an ACE archive
    pub fn new(mut reader: R) -> Result<Self> {
        let main_header = Self::find_and_parse_main_header(&mut reader)?;

        Ok(Self {
            reader,
            main_header,
            lz77: Lz77Decoder::new(),
            password: None,
        })
    }

    /// Set the password for encrypted entries
    pub fn set_password<P: Into<String>>(&mut self, password: P) {
        self.password = Some(password.into());
    }

    /// Clear the password
    pub fn clear_password(&mut self) {
        self.password = None;
    }

    /// Find the ACE magic and parse the main header
    fn find_and_parse_main_header(reader: &mut R) -> Result<MainHeader> {
        // Search for magic in first 512KB
        const SEARCH_SIZE: usize = 524288;
        reader.seek(SeekFrom::Start(0))?;

        let mut buffer = vec![0u8; SEARCH_SIZE.min(StreamLen::stream_len(&mut *reader)? as usize)];
        let bytes_read = reader.read(&mut buffer)?;

        // Search for **ACE** magic at offset 7 from header start
        let magic_pos = buffer[..bytes_read].windows(7).position(|w| w == ACE_MAGIC);

        let header_start = match magic_pos {
            Some(pos) if pos >= 7 => pos - 7,
            _ => {
                return Err(ArchiveError::invalid_header("ACE: magic not found"));
            }
        };

        reader.seek(SeekFrom::Start(header_start as u64))?;
        Self::parse_main_header(reader)
    }

    /// Parse the main header
    fn parse_main_header(reader: &mut R) -> Result<MainHeader> {
        let mut header_start = [0u8; 4];
        reader.read_exact(&mut header_start)?;

        let header_crc = u16::from_le_bytes([header_start[0], header_start[1]]);
        let header_size = u16::from_le_bytes([header_start[2], header_start[3]]);

        let mut header_data = vec![0u8; header_size as usize];
        reader.read_exact(&mut header_data)?;

        // Verify CRC
        let calculated_crc = ace_crc16(&header_data);
        if calculated_crc != header_crc {
            return Err(ArchiveError::crc_mismatch(
                "ACE main header",
                header_crc as u32,
                calculated_crc as u32,
            ));
        }

        let header_type = header_data[0];
        let header_flags = u16::from_le_bytes([header_data[1], header_data[2]]);

        if header_type != header_type::MAIN {
            return Err(ArchiveError::invalid_header("ACE: expected main header"));
        }

        // Verify magic
        if &header_data[3..10] != ACE_MAGIC {
            return Err(ArchiveError::invalid_header("ACE: invalid magic"));
        }

        let extract_version = header_data[10];
        let creator_version = header_data[11];
        let host_os = HostOs::from(header_data[12]);
        let volume_number = header_data[13];

        let datetime = DosDateTime::from(u32::from_le_bytes([
            header_data[14],
            header_data[15],
            header_data[16],
            header_data[17],
        ]));

        // Skip reserved1 (8 bytes)
        let mut pos = 26;

        // Read advert if present
        let advert = if header_flags & header_flags::AV_STRING != 0 && pos < header_data.len() {
            let len = header_data[pos] as usize;
            pos += 1;
            let s = String::from_utf8_lossy(&header_data[pos..pos + len]).to_string();
            pos += len;
            s
        } else {
            String::new()
        };

        // Read comment if present
        let comment = if header_flags & header_flags::COMMENT != 0 && pos + 2 <= header_data.len() {
            let len = u16::from_le_bytes([header_data[pos], header_data[pos + 1]]) as usize;
            pos += 2;
            if pos + len <= header_data.len() {
                header_data[pos..pos + len].to_vec()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        Ok(MainHeader {
            header_crc,
            header_size,
            header_type,
            header_flags,
            extract_version,
            creator_version,
            host_os,
            volume_number,
            datetime,
            advert,
            comment,
        })
    }

    /// Get the next file entry
    pub fn get_next_entry(&mut self) -> Result<Option<FileHeader>> {
        loop {
            let _start_pos = self.reader.stream_position()?;

            let mut header_start = [0u8; 4];
            if self.reader.read_exact(&mut header_start).is_err() {
                return Ok(None);
            }

            let header_crc = u16::from_le_bytes([header_start[0], header_start[1]]);
            let header_size = u16::from_le_bytes([header_start[2], header_start[3]]);

            if header_size == 0 {
                return Ok(None);
            }

            let mut header_data = vec![0u8; header_size as usize];
            self.reader.read_exact(&mut header_data)?;

            // Verify CRC
            if ace_crc16(&header_data) != header_crc {
                return Err(ArchiveError::crc_mismatch(
                    "ACE file header",
                    header_crc as u32,
                    0,
                ));
            }

            let header_type = header_data[0];
            let header_flags = u16::from_le_bytes([header_data[1], header_data[2]]);

            // Skip non-file headers
            if header_type != header_type::FILE {
                // Skip addsize if present
                if header_flags & header_flags::ADDSIZE != 0 {
                    let addsize = if header_flags & header_flags::MEMORY_64BIT != 0 {
                        if header_data.len() >= 11 {
                            u64::from_le_bytes([
                                header_data[3],
                                header_data[4],
                                header_data[5],
                                header_data[6],
                                header_data[7],
                                header_data[8],
                                header_data[9],
                                header_data[10],
                            ])
                        } else {
                            0
                        }
                    } else if header_data.len() >= 7 {
                        u32::from_le_bytes([
                            header_data[3],
                            header_data[4],
                            header_data[5],
                            header_data[6],
                        ]) as u64
                    } else {
                        0
                    };
                    self.reader.seek(SeekFrom::Current(addsize as i64))?;
                }
                continue;
            }

            // Parse file header
            let is_64bit = header_flags & header_flags::MEMORY_64BIT != 0;
            let (packed_size, original_size, pos) = if is_64bit {
                let packed = u64::from_le_bytes([
                    header_data[3],
                    header_data[4],
                    header_data[5],
                    header_data[6],
                    header_data[7],
                    header_data[8],
                    header_data[9],
                    header_data[10],
                ]);
                let original = u64::from_le_bytes([
                    header_data[11],
                    header_data[12],
                    header_data[13],
                    header_data[14],
                    header_data[15],
                    header_data[16],
                    header_data[17],
                    header_data[18],
                ]);
                (packed, original, 19)
            } else {
                let packed = u32::from_le_bytes([
                    header_data[3],
                    header_data[4],
                    header_data[5],
                    header_data[6],
                ]) as u64;
                let original = u32::from_le_bytes([
                    header_data[7],
                    header_data[8],
                    header_data[9],
                    header_data[10],
                ]) as u64;
                (packed, original, 11)
            };

            let datetime = DosDateTime::from(u32::from_le_bytes([
                header_data[pos],
                header_data[pos + 1],
                header_data[pos + 2],
                header_data[pos + 3],
            ]));

            let attributes = u32::from_le_bytes([
                header_data[pos + 4],
                header_data[pos + 5],
                header_data[pos + 6],
                header_data[pos + 7],
            ]);

            let crc32 = u32::from_le_bytes([
                header_data[pos + 8],
                header_data[pos + 9],
                header_data[pos + 10],
                header_data[pos + 11],
            ]);

            let compression_type = CompressionType::from(header_data[pos + 12]);
            let compression_quality = CompressionQuality::from(header_data[pos + 13]);
            let parameters = u16::from_le_bytes([header_data[pos + 14], header_data[pos + 15]]);

            // Skip reserved1 (2 bytes)
            let mut fpos = pos + 18;

            // Read filename
            let filename_len =
                u16::from_le_bytes([header_data[fpos], header_data[fpos + 1]]) as usize;
            fpos += 2;
            let filename =
                String::from_utf8_lossy(&header_data[fpos..fpos + filename_len]).to_string();
            fpos += filename_len;

            // Read comment if present
            let comment = if header_flags & header_flags::COMMENT != 0
                && fpos + 2 <= header_data.len()
            {
                let len = u16::from_le_bytes([header_data[fpos], header_data[fpos + 1]]) as usize;
                fpos += 2;
                if fpos + len <= header_data.len() {
                    header_data[fpos..fpos + len].to_vec()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };

            let data_offset = self.reader.stream_position()?;

            return Ok(Some(FileHeader {
                header_crc,
                header_size,
                header_type,
                header_flags,
                packed_size,
                original_size,
                datetime,
                attributes,
                crc32,
                compression_type,
                compression_quality,
                parameters,
                filename,
                comment,
                data_offset,
            }));
        }
    }

    /// Skip a file entry
    pub fn skip(&mut self, header: &FileHeader) -> Result<()> {
        self.reader
            .seek(SeekFrom::Start(header.data_offset + header.packed_size))?;
        Ok(())
    }

    /// Read and decompress a file entry
    pub fn read(&mut self, header: &FileHeader) -> Result<Vec<u8>> {
        self.read_with_password(header, self.password.clone())
    }

    /// Read and decompress a file entry with a specific password
    pub fn read_with_password(
        &mut self,
        header: &FileHeader,
        password: Option<String>,
    ) -> Result<Vec<u8>> {
        // Seek to data
        self.reader.seek(SeekFrom::Start(header.data_offset))?;

        // Read compressed data
        let mut compressed = vec![0u8; header.packed_size as usize];
        self.reader.read_exact(&mut compressed)?;

        // Decrypt if encrypted
        let data = if header.is_encrypted() {
            let pwd = password
                .ok_or_else(|| ArchiveError::encryption_required(&header.filename, "ACE"))?;
            decrypt_ace_data(&compressed, &pwd)
        } else {
            compressed
        };

        // Decompress based on type
        let decompressed = match header.compression_type {
            CompressionType::Stored => data,
            CompressionType::Lz77 | CompressionType::Blocked => {
                self.decompress_lz77(header, &data)?
            }
            CompressionType::Unknown(n) => {
                return Err(ArchiveError::unsupported_method(
                    "ACE",
                    format!("Unknown compression type {}", n),
                ));
            }
        };

        // Verify size
        if decompressed.len() != header.original_size as usize {
            return Err(ArchiveError::decompression_failed(
                &header.filename,
                format!(
                    "size mismatch: expected {}, got {}",
                    header.original_size,
                    decompressed.len()
                ),
            ));
        }

        // Verify CRC (ACE uses inverted CRC32)
        let crc = !crc32fast::hash(&decompressed);
        if crc != header.crc32 {
            return Err(ArchiveError::crc_mismatch(
                &header.filename,
                header.crc32,
                crc,
            ));
        }

        Ok(decompressed)
    }

    /// Decompress LZ77 data
    fn decompress_lz77(&mut self, header: &FileHeader, data: &[u8]) -> Result<Vec<u8>> {
        // Reset decoder for non-solid or first file
        if !self.main_header.is_solid() {
            self.lz77.reset();
        }

        // Set dictionary size
        self.lz77.set_dictionary_size(header.dictionary_size());

        // Create bitstream
        let cursor = std::io::Cursor::new(data);
        let mut bs = BitStream::new(cursor, data.len());

        // Decompress
        self.lz77.decompress(&mut bs, header.original_size as usize)
    }

    /// Get the main header
    pub fn main_header(&self) -> &MainHeader {
        &self.main_header
    }

    /// Check if archive is solid
    pub fn is_solid(&self) -> bool {
        self.main_header.is_solid()
    }
}

/// Helper trait for stream length
trait StreamLen {
    fn stream_len(&mut self) -> Result<u64>;
}

impl<R: Read + Seek> StreamLen for R {
    fn stream_len(&mut self) -> Result<u64> {
        let current = self.stream_position()?;
        let end = self.seek(SeekFrom::End(0))?;
        self.seek(SeekFrom::Start(current))?;
        Ok(end)
    }
}
