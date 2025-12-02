//! RAR archive reader
//!
//! Uses the `rar` crate for parsing RAR5 archives.
//! Note: The rar crate has a file-system oriented API. This wrapper provides
//! a streaming interface for listing entries, but full extraction requires
//! using the rar crate's extract_all() function directly.

use std::io::{self, Read, Seek, SeekFrom};

use crate::date_time::DosDateTime;

// RAR5 signature
const RAR5_SIGNATURE: [u8; 8] = [0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x01, 0x00];

/// Header information for a RAR entry
#[derive(Debug, Clone)]
pub struct RarFileHeader {
    /// File name (may include path)
    pub name: String,
    /// Compressed size in bytes
    pub compressed_size: u64,
    /// Original (uncompressed) size in bytes
    pub original_size: u64,
    /// Compression method name
    pub compression_method: String,
    /// Modification date/time
    pub date_time: Option<DosDateTime>,
    /// CRC32 checksum
    pub crc32: u32,
    /// Whether this entry is a directory
    pub is_directory: bool,
    /// Data offset in the archive
    pub data_offset: u64,
    /// Data size in the archive
    pub data_size: u64,
}

/// RAR archive reader
///
/// Note: This provides basic parsing of RAR5 archives. For full extraction
/// with decompression, use the `rar` crate's `Archive::extract_all()` function.
pub struct RarArchive<T: Read + Seek> {
    reader: T,
    entries: Vec<RarFileHeader>,
    current_index: usize,
}

impl<T: Read + Seek> RarArchive<T> {
    /// Create a new RAR archive reader
    pub fn new(mut reader: T) -> io::Result<Self> {
        // Verify RAR5 signature
        let mut sig = [0u8; 8];
        reader.read_exact(&mut sig)?;

        if sig != RAR5_SIGNATURE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not a valid RAR5 archive (only RAR5 format is supported)",
            ));
        }

        let entries = Self::parse_entries(&mut reader)?;

        // Reset reader position
        reader.seek(SeekFrom::Start(0))?;

        Ok(Self {
            reader,
            entries,
            current_index: 0,
        })
    }

    fn parse_entries(reader: &mut T) -> io::Result<Vec<RarFileHeader>> {
        let mut entries = Vec::new();

        // Skip archive header (simplified parsing)
        // In a full implementation, we'd parse all header types properly
        loop {
            let _pos = reader.stream_position()?;

            // Try to read a header
            match Self::try_parse_header(reader) {
                Ok(Some(header)) => {
                    entries.push(header);
                }
                Ok(None) => {
                    // End of archive or non-file header, continue
                }
                Err(_) => {
                    // End of archive or parse error
                    break;
                }
            }
        }

        Ok(entries)
    }

    fn try_parse_header(reader: &mut T) -> io::Result<Option<RarFileHeader>> {
        // Read header CRC (4 bytes)
        let mut crc_buf = [0u8; 4];
        if reader.read_exact(&mut crc_buf).is_err() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "End of archive",
            ));
        }

        // Read header size (vint - variable length integer)
        let _header_size = Self::read_vint(reader)?;

        // Read header type (vint)
        let header_type = Self::read_vint(reader)?;

        // Read header flags (vint)
        let header_flags = Self::read_vint(reader)?;

        let has_extra_area = (header_flags & 0x01) != 0;
        let has_data_area = (header_flags & 0x02) != 0;

        // Read extra area size if present
        let extra_area_size = if has_extra_area {
            Self::read_vint(reader)?
        } else {
            0
        };

        // Read data area size if present
        let data_area_size = if has_data_area {
            Self::read_vint(reader)?
        } else {
            0
        };

        // Header type 2 = File header
        if header_type == 2 {
            // Read file flags
            let file_flags = Self::read_vint(reader)?;
            let is_directory = (file_flags & 0x01) != 0;

            // Read unpacked size
            let unpacked_size = Self::read_vint(reader)?;

            // Read attributes
            let _attributes = Self::read_vint(reader)?;

            // Read mtime if present
            let mtime = if (file_flags & 0x02) != 0 {
                let mut buf = [0u8; 4];
                reader.read_exact(&mut buf)?;
                u32::from_le_bytes(buf)
            } else {
                0
            };

            // Read data CRC if present
            let data_crc = if (file_flags & 0x04) != 0 {
                let mut buf = [0u8; 4];
                reader.read_exact(&mut buf)?;
                u32::from_le_bytes(buf)
            } else {
                0
            };

            // Read compression info
            let compression = Self::read_vint(reader)?;
            let compression_method = match compression & 0x3F {
                0 => "Store",
                1..=5 => "Normal",
                _ => "Unknown",
            };

            // Read host OS
            let _host_os = Self::read_vint(reader)?;

            // Read name length
            let name_len = Self::read_vint(reader)?;

            // Read name
            let mut name_buf = vec![0u8; name_len as usize];
            reader.read_exact(&mut name_buf)?;
            let name = String::from_utf8_lossy(&name_buf).to_string();

            // Record data position
            let data_offset = reader.stream_position()?;

            // Skip extra area
            if extra_area_size > 0 {
                reader.seek(SeekFrom::Current(extra_area_size as i64))?;
            }

            // Skip data area
            if data_area_size > 0 {
                reader.seek(SeekFrom::Current(data_area_size as i64))?;
            }

            return Ok(Some(RarFileHeader {
                name,
                compressed_size: data_area_size,
                original_size: unpacked_size,
                compression_method: compression_method.to_string(),
                date_time: if mtime != 0 {
                    Some(DosDateTime::new(mtime))
                } else {
                    None
                },
                crc32: data_crc,
                is_directory,
                data_offset,
                data_size: data_area_size,
            }));
        }

        // Skip other header types
        // Skip data area if present
        if data_area_size > 0 {
            reader.seek(SeekFrom::Current(data_area_size as i64))?;
        }

        // Check for end block (type 5)
        if header_type == 5 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "End of archive",
            ));
        }

        Ok(None)
    }

    fn read_vint(reader: &mut T) -> io::Result<u64> {
        let mut result: u64 = 0;
        let mut shift = 0;

        loop {
            let mut byte = [0u8; 1];
            reader.read_exact(&mut byte)?;
            let b = byte[0];

            result |= ((b & 0x7F) as u64) << shift;

            if (b & 0x80) == 0 {
                break;
            }

            shift += 7;
            if shift > 63 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "VInt too large"));
            }
        }

        Ok(result)
    }

    /// Get the next entry in the archive
    pub fn get_next_entry(&mut self) -> io::Result<Option<RarFileHeader>> {
        if self.current_index >= self.entries.len() {
            return Ok(None);
        }

        let entry = self.entries[self.current_index].clone();
        self.current_index += 1;
        Ok(Some(entry))
    }

    /// Skip the current entry without reading its data
    pub fn skip(&mut self, _header: &RarFileHeader) -> io::Result<()> {
        // Nothing to do - we parse all entries upfront
        Ok(())
    }

    /// Read and decompress an entry's data
    ///
    /// Note: RAR decompression is complex. This only works for stored (uncompressed) entries.
    /// For compressed entries, use the `rar` crate's `Archive::extract_all()` function.
    pub fn read(&mut self, header: &RarFileHeader) -> io::Result<Vec<u8>> {
        if header.is_directory {
            return Ok(Vec::new());
        }

        // For stored entries, we can read directly
        if header.compression_method == "Store" {
            self.reader.seek(SeekFrom::Start(header.data_offset))?;
            let mut data = vec![0u8; header.data_size as usize];
            self.reader.read_exact(&mut data)?;
            return Ok(data);
        }

        // RAR decompression is not implemented
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            format!("RAR decompression not supported for method: {}. Use 'rar' crate's Archive::extract_all() for full extraction.", header.compression_method),
        ))
    }
}
