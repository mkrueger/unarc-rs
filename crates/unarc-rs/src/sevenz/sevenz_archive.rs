//! 7z archive reader
//!
//! Uses the `sevenz-rust2` crate for decompression.

use std::collections::HashSet;
use std::io::{Read, Seek};

use crate::date_time::DosDateTime;
use crate::error::{ArchiveError, Result};

/// AES-256-SHA-256 encryption method ID (from sevenz-rust2)
const AES_METHOD_ID: &[u8] = &[0x06, 0xF1, 0x07, 0x01];

/// Header information for a 7z entry
#[derive(Debug, Clone)]
pub struct SevenZFileHeader {
    /// File name (may include path)
    pub name: String,
    /// Compressed size in bytes (0 if solid block)
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
    /// Whether this entry is encrypted
    pub is_encrypted: bool,
    /// Index of the file in the archive (for random access)
    pub index: usize,
}

/// 7z archive reader
pub struct SevenZArchive<T: Read + Seek> {
    reader: T,
    entries: Vec<SevenZFileHeader>,
    current_index: usize,
    password: Option<String>,
    /// Whether the archive has any encrypted content
    is_encrypted: bool,
}

impl<T: Read + Seek> SevenZArchive<T> {
    /// Create a new 7z archive reader
    pub fn new(reader: T) -> Result<Self> {
        Self::new_with_password(reader, None)
    }

    /// Check which blocks use AES encryption
    fn get_encrypted_blocks(archive: &sevenz_rust2::Archive) -> HashSet<usize> {
        let mut encrypted = HashSet::new();
        for (idx, block) in archive.blocks.iter().enumerate() {
            for coder in &block.coders {
                if coder.encoder_method_id().starts_with(AES_METHOD_ID) {
                    encrypted.insert(idx);
                    break;
                }
            }
        }
        encrypted
    }

    /// Create a new 7z archive reader with an optional password
    pub fn new_with_password(mut reader: T, password: Option<String>) -> Result<Self> {
        // Parse the archive and collect all entries upfront
        let pwd = match &password {
            Some(p) => sevenz_rust2::Password::from(p.as_str()),
            None => sevenz_rust2::Password::empty(),
        };

        let archive = sevenz_rust2::ArchiveReader::new(&mut reader, pwd)
            .map_err(|e| ArchiveError::external_library("sevenz-rust2", e.to_string()))?;

        // Detect which blocks use AES encryption
        let encrypted_blocks = Self::get_encrypted_blocks(archive.archive());
        let is_encrypted = !encrypted_blocks.is_empty();

        let mut entries = Vec::new();

        for (index, entry) in archive.archive().files.iter().enumerate() {
            let name = entry.name.clone();
            let is_directory = entry.is_directory;
            let original_size = entry.size;

            // 7z doesn't always have per-file compressed size (solid archives)
            let compressed_size = entry.compressed_size;

            // Get CRC if available
            let crc32 = if entry.has_crc { entry.crc as u32 } else { 0 };

            // Convert modification time to DosDateTime
            let date_time = if entry.has_last_modified_date {
                // sevenz-rust2's NtTime contains a timestamp in FILETIME format
                let ts: u64 = entry.last_modified_date.into();
                convert_nt_time_to_dos_datetime(ts)
            } else {
                None
            };

            // Get compression method info
            let compression_method = "7z".to_string();

            // Check if this file is in an encrypted block
            let file_encrypted = if is_directory {
                false
            } else {
                archive
                    .archive()
                    .stream_map
                    .file_block_index
                    .get(index)
                    .and_then(|opt| *opt)
                    .map(|block_idx| encrypted_blocks.contains(&block_idx))
                    .unwrap_or(false)
            };

            entries.push(SevenZFileHeader {
                name,
                compressed_size,
                original_size,
                compression_method,
                date_time,
                crc32,
                is_directory,
                is_encrypted: file_encrypted,
                index,
            });
        }

        Ok(Self {
            reader,
            entries,
            current_index: 0,
            password,
            is_encrypted,
        })
    }

    /// Check if the archive contains any encrypted content
    pub fn is_encrypted(&self) -> bool {
        self.is_encrypted
    }

    /// Set the password for encrypted archives
    pub fn set_password<P: Into<String>>(&mut self, password: P) {
        self.password = Some(password.into());
    }

    /// Clear the password
    pub fn clear_password(&mut self) {
        self.password = None;
    }

    /// Get the next entry in the archive
    pub fn get_next_entry(&mut self) -> Result<Option<SevenZFileHeader>> {
        if self.current_index >= self.entries.len() {
            return Ok(None);
        }

        let entry = self.entries[self.current_index].clone();
        self.current_index += 1;

        Ok(Some(entry))
    }

    /// Skip the current entry without reading its data
    pub fn skip(&mut self, _header: &SevenZFileHeader) -> Result<()> {
        // Nothing to do - we already advanced in get_next_entry
        Ok(())
    }

    /// Read and decompress an entry's data
    pub fn read(&mut self, header: &SevenZFileHeader) -> Result<Vec<u8>> {
        self.read_with_password(header, self.password.clone())
    }

    /// Read and decompress an entry's data with a specific password
    pub fn read_with_password(
        &mut self,
        header: &SevenZFileHeader,
        password: Option<String>,
    ) -> Result<Vec<u8>> {
        if header.is_directory {
            return Ok(Vec::new());
        }

        // We need to re-open the archive and read the specific file
        let pwd = match &password {
            Some(p) => sevenz_rust2::Password::from(p.as_str()),
            None => sevenz_rust2::Password::empty(),
        };

        let mut archive = sevenz_rust2::ArchiveReader::new(&mut self.reader, pwd)
            .map_err(|e| ArchiveError::external_library("sevenz-rust2", e.to_string()))?;

        // Find and extract the file by name
        archive
            .read_file(&header.name)
            .map_err(|e| ArchiveError::decompression_failed(&header.name, e.to_string()))
    }
}

/// Convert Windows FILETIME (100-nanosecond intervals since 1601) to DOS date/time
fn convert_nt_time_to_dos_datetime(ts: u64) -> Option<DosDateTime> {
    let nanos_per_sec = 10_000_000u64;
    let secs_since_1601 = ts / nanos_per_sec;

    // Seconds from 1601 to 1970 (Unix epoch)
    let secs_1601_to_1970 = 11_644_473_600u64;

    if secs_since_1601 >= secs_1601_to_1970 {
        let unix_secs = secs_since_1601 - secs_1601_to_1970;
        // Convert Unix timestamp to date/time components
        let days_since_epoch = unix_secs / 86400;
        let time_of_day = unix_secs % 86400;

        let hour = (time_of_day / 3600) as u32;
        let minute = ((time_of_day % 3600) / 60) as u32;
        let second = (time_of_day % 60) as u32;

        // Simple date calculation (approximate - good enough for most uses)
        let mut days = days_since_epoch as i64;
        let mut year = 1970i32;

        loop {
            let days_in_year = if is_leap_year(year) { 366 } else { 365 };
            if days < days_in_year {
                break;
            }
            days -= days_in_year;
            year += 1;
        }

        let (month, day) = day_of_year_to_month_day(days as u32, is_leap_year(year));

        if (1980..=2107).contains(&year) {
            let dos_year = ((year - 1980) as u32) & 0x7F;
            let dos_date = (dos_year << 9) | ((month as u32) << 5) | (day as u32);
            let dos_time = (hour << 11) | (minute << 5) | (second / 2);

            Some(DosDateTime::new((dos_date << 16) | dos_time))
        } else {
            None
        }
    } else {
        None
    }
}

/// Check if a year is a leap year
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Convert day of year (0-based) to month and day
fn day_of_year_to_month_day(day_of_year: u32, is_leap: bool) -> (u8, u8) {
    let days_in_months: [u32; 12] = if is_leap {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut remaining = day_of_year;
    for (month, &days) in days_in_months.iter().enumerate() {
        if remaining < days {
            return ((month + 1) as u8, (remaining + 1) as u8);
        }
        remaining -= days;
    }

    // Fallback (shouldn't happen with valid input)
    (12, 31)
}
