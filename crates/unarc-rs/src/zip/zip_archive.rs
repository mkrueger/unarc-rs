//! ZIP archive reader
//!
//! Uses the `zip` crate for decompression.

use std::io::{Read, Seek};

use crate::date_time::DosDateTime;
use crate::error::{ArchiveError, Result};

/// Header information for a ZIP entry
#[derive(Debug, Clone)]
pub struct ZipFileHeader {
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
    /// Index of the file in the archive (for random access)
    pub index: usize,
}

/// ZIP archive reader
pub struct ZipArchive<T: Read + Seek> {
    archive: zip::ZipArchive<T>,
    current_index: usize,
}

impl<T: Read + Seek> ZipArchive<T> {
    /// Create a new ZIP archive reader
    pub fn new(reader: T) -> Result<Self> {
        let archive = zip::ZipArchive::new(reader)
            .map_err(|e| ArchiveError::external_library("zip", e.to_string()))?;

        Ok(Self {
            archive,
            current_index: 0,
        })
    }

    /// Get the next entry in the archive
    pub fn get_next_entry(&mut self) -> Result<Option<ZipFileHeader>> {
        if self.current_index >= self.archive.len() {
            return Ok(None);
        }

        let index = self.current_index;
        self.current_index += 1;

        let file = self
            .archive
            .by_index_raw(index)
            .map_err(|e| ArchiveError::external_library("zip", e.to_string()))?;

        let name = file.name().to_string();
        let compressed_size = file.compressed_size();
        let original_size = file.size();
        let compression_method = format!("{:?}", file.compression());
        let crc32 = file.crc32();
        let is_directory = file.is_dir();

        // Convert last_modified to DosDateTime
        let date_time = file.last_modified().map(|dt| {
            // zip crate uses its own DateTime type
            // DOS date format: bits 0-4: day, 5-8: month, 9-15: year-1980
            // DOS time format: bits 0-4: second/2, 5-10: minute, 11-15: hour
            let year = (dt.year() as u32).saturating_sub(1980) & 0x7F;
            let month = dt.month() as u32;
            let day = dt.day() as u32;
            let hour = dt.hour() as u32;
            let minute = dt.minute() as u32;
            let second = dt.second() as u32;

            let dos_date = (year << 9) | (month << 5) | day;
            let dos_time = (hour << 11) | (minute << 5) | (second / 2);

            DosDateTime::new((dos_date << 16) | dos_time)
        });

        Ok(Some(ZipFileHeader {
            name,
            compressed_size,
            original_size,
            compression_method,
            date_time,
            crc32,
            is_directory,
            index,
        }))
    }

    /// Skip the current entry without reading its data
    pub fn skip(&mut self, _header: &ZipFileHeader) -> Result<()> {
        // Nothing to do - we already advanced in get_next_entry
        Ok(())
    }

    /// Read and decompress an entry's data
    pub fn read(&mut self, header: &ZipFileHeader) -> Result<Vec<u8>> {
        if header.is_directory {
            return Ok(Vec::new());
        }

        let mut file = self
            .archive
            .by_index(header.index)
            .map_err(|e| ArchiveError::external_library("zip", e.to_string()))?;

        let mut data = Vec::with_capacity(header.original_size as usize);
        file.read_to_end(&mut data)?;

        Ok(data)
    }
}
