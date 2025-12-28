//! LHA/LZH archive reader
//!
//! Uses the delharc crate for decompression.

use std::io::{Read, Seek};

use chrono::{Datelike, Timelike};
use delharc::header::LhaHeader;
use delharc::LhaDecodeReader;

use crate::date_time::DosDateTime;
use crate::error::{ArchiveError, Result};

/// Header information for an LHA entry
#[derive(Debug, Clone)]
pub struct LhaFileHeader {
    /// File name (may include path)
    pub name: String,
    /// Compressed size in bytes
    pub compressed_size: u64,
    /// Original (uncompressed) size in bytes
    pub original_size: u64,
    /// Compression method identifier (e.g., "-lh5-", "-lh6-", "-lzs-")
    pub compression_method: String,
    /// Modification date/time
    pub date_time: Option<DosDateTime>,
    /// CRC16 checksum
    pub crc16: u16,
    /// Whether compression is supported by the decoder
    pub is_supported: bool,
    /// Whether this entry is a directory
    pub is_directory: bool,
}

impl LhaFileHeader {
    /// Create header from delharc's LhaHeader
    pub fn from_lha_header(header: &LhaHeader) -> Self {
        let path = header.parse_pathname();
        let name = path.to_string_lossy().to_string();

        // Parse modification time using TimestampResult's to_naive_utc method
        let modified = header.parse_last_modified();
        let date_time = modified.to_naive_utc().map(|dt| {
            // Convert to DOS datetime format
            let year = dt.year() as u32;
            let month = dt.month();
            let day = dt.day();
            let hour = dt.hour();
            let minute = dt.minute();
            let second = dt.second();

            // DOS date format: bits 0-4: day, 5-8: month, 9-15: year-1980
            // DOS time format: bits 0-4: second/2, 5-10: minute, 11-15: hour
            let dos_date = ((year.saturating_sub(1980) & 0x7F) << 9) | ((month & 0xF) << 5) | (day & 0x1F);
            let dos_time = ((hour & 0x1F) << 11) | ((minute & 0x3F) << 5) | ((second / 2) & 0x1F);

            DosDateTime::new((dos_date << 16) | dos_time)
        });

        LhaFileHeader {
            name,
            compressed_size: header.compressed_size,
            original_size: header.original_size,
            compression_method: match header.compression_method() {
                Ok(m) => format!("{:?}", m),
                Err(e) => e.to_string(),
            },
            date_time,
            crc16: header.file_crc,
            is_supported: true, // Will be updated when reading
            is_directory: header.is_directory(),
        }
    }
}

/// LHA/LZH archive reader
pub struct LhaArchive<T: Read> {
    reader: Option<LhaDecodeReader<T>>,
    current_header: Option<LhaFileHeader>,
    finished: bool,
}

impl<T: Read> LhaArchive<T> {
    /// Create a new LHA archive reader
    pub fn new(reader: T) -> Result<Self> {
        let lha_reader = LhaDecodeReader::new(reader)?;

        // Get the first header
        let header = LhaFileHeader::from_lha_header(lha_reader.header());
        let is_supported = lha_reader.is_decoder_supported();

        Ok(Self {
            reader: Some(lha_reader),
            current_header: Some(LhaFileHeader { is_supported, ..header }),
            finished: false,
        })
    }

    /// Get the next entry in the archive
    pub fn get_next_entry(&mut self) -> Result<Option<LhaFileHeader>> {
        if self.finished {
            return Ok(None);
        }

        // Return the current header if we have one queued
        if let Some(header) = self.current_header.take() {
            return Ok(Some(header));
        }

        // Try to advance to next file
        if let Some(ref mut reader) = self.reader {
            if reader.next_file()? {
                let header = LhaFileHeader::from_lha_header(reader.header());
                let is_supported = reader.is_decoder_supported();
                return Ok(Some(LhaFileHeader { is_supported, ..header }));
            }
        }

        self.finished = true;
        Ok(None)
    }

    /// Skip the current entry without reading its data
    pub fn skip(&mut self, _header: &LhaFileHeader) -> Result<()> {
        // In LHA, we just need to advance to the next file
        // The next call to get_next_entry will handle this
        if let Some(ref mut reader) = self.reader {
            if !reader.next_file()? {
                self.finished = true;
            } else {
                // Queue up the next header
                let header = LhaFileHeader::from_lha_header(reader.header());
                let is_supported = reader.is_decoder_supported();
                self.current_header = Some(LhaFileHeader { is_supported, ..header });
            }
        }
        Ok(())
    }

    /// Read and decompress the current entry's data
    pub fn read(&mut self, header: &LhaFileHeader) -> Result<Vec<u8>> {
        if header.is_directory {
            return Ok(Vec::new());
        }

        if !header.is_supported {
            return Err(ArchiveError::unsupported_method("LHA", &header.compression_method));
        }

        if let Some(ref mut reader) = self.reader {
            let mut data = Vec::with_capacity(header.original_size as usize);
            reader.read_to_end(&mut data)?;

            // Verify CRC
            reader.crc_check()?;

            // Advance to next file
            if !reader.next_file()? {
                self.finished = true;
            } else {
                // Queue up the next header
                let next_header = LhaFileHeader::from_lha_header(reader.header());
                let is_supported = reader.is_decoder_supported();
                self.current_header = Some(LhaFileHeader { is_supported, ..next_header });
            }

            Ok(data)
        } else {
            Err(ArchiveError::decompression_failed(&header.name, "Archive reader not available"))
        }
    }
}

/// Wrapper for LHA archive that supports Seek (needed for unified API)
pub struct LhaArchiveSeekable<T: Read + Seek> {
    inner: LhaArchive<T>,
}

impl<T: Read + Seek> LhaArchiveSeekable<T> {
    /// Create a new seekable LHA archive reader
    pub fn new(reader: T) -> Result<Self> {
        Ok(Self {
            inner: LhaArchive::new(reader)?,
        })
    }

    /// Get the next entry in the archive
    pub fn get_next_entry(&mut self) -> Result<Option<LhaFileHeader>> {
        self.inner.get_next_entry()
    }

    /// Skip the current entry without reading its data
    pub fn skip(&mut self, header: &LhaFileHeader) -> Result<()> {
        self.inner.skip(header)
    }

    /// Read and decompress the current entry's data
    pub fn read(&mut self, header: &LhaFileHeader) -> Result<Vec<u8>> {
        self.inner.read(header)
    }
}
