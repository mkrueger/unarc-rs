//! TAR archive format support
//!
//! TAR (Tape Archive) is a file format used for storing multiple files in a single archive.
//! It was originally developed for tape backup systems but is now commonly used for
//! software distribution and file archiving, often in combination with compression
//! formats like gzip (.tar.gz) or bzip2 (.tar.bz2).
//!
//! This module provides read-only access to TAR archives using the `tar` crate.

use std::io::{Read, Seek, SeekFrom};

use crate::date_time::DosDateTime;
use crate::error::{ArchiveError, Result};

/// TAR file header information
#[derive(Debug, Clone)]
pub struct TarFileHeader {
    /// File name (may include path)
    pub name: String,
    /// File size in bytes
    pub size: u64,
    /// Modification time as Unix timestamp
    pub mtime: u64,
    /// File mode/permissions
    pub mode: u32,
    /// Entry type (file, directory, symlink, etc.)
    pub entry_type: TarEntryType,
    /// Link name for symlinks/hardlinks
    pub link_name: Option<String>,
}

/// TAR entry types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TarEntryType {
    /// Regular file
    Regular,
    /// Hard link
    HardLink,
    /// Symbolic link
    Symlink,
    /// Character device
    Char,
    /// Block device
    Block,
    /// Directory
    Directory,
    /// FIFO (named pipe)
    Fifo,
    /// Continuous file (GNU extension)
    Continuous,
    /// Other/unknown type
    Other(u8),
}

impl From<tar::EntryType> for TarEntryType {
    fn from(t: tar::EntryType) -> Self {
        match t {
            tar::EntryType::Regular => TarEntryType::Regular,
            tar::EntryType::Link => TarEntryType::HardLink,
            tar::EntryType::Symlink => TarEntryType::Symlink,
            tar::EntryType::Char => TarEntryType::Char,
            tar::EntryType::Block => TarEntryType::Block,
            tar::EntryType::Directory => TarEntryType::Directory,
            tar::EntryType::Fifo => TarEntryType::Fifo,
            tar::EntryType::Continuous => TarEntryType::Continuous,
            _ => TarEntryType::Other(t.as_byte()),
        }
    }
}

impl TarFileHeader {
    /// Convert Unix timestamp to DOS datetime
    pub fn modified_time(&self) -> Option<DosDateTime> {
        // Convert Unix timestamp to DOS datetime
        // Unix epoch is 1970-01-01, DOS epoch is 1980-01-01
        if self.mtime == 0 {
            return None;
        }

        // Use chrono to convert
        use chrono::{Datelike, TimeZone, Timelike, Utc};
        if let Some(dt) = Utc.timestamp_opt(self.mtime as i64, 0).single() {
            let year = dt.year() as u16;
            let month = dt.month() as u16;
            let day = dt.day() as u16;
            let hour = dt.hour() as u16;
            let minute = dt.minute() as u16;
            let second = dt.second() as u16;

            if year >= 1980 {
                // DOS datetime format: date in high 16 bits, time in low 16 bits
                let dos_date = ((year - 1980) << 9) | (month << 5) | day;
                let dos_time = (hour << 11) | (minute << 5) | (second / 2);
                // Combine into u32: date in high word, time in low word
                let combined = ((dos_date as u32) << 16) | (dos_time as u32);
                return Some(DosDateTime::new(combined));
            }
        }
        None
    }
}

/// Internal entry tracking for TAR archives
struct TarEntry {
    header: TarFileHeader,
    /// Offset in the archive where the file data starts
    data_offset: u64,
}

/// TAR archive reader
pub struct TarArchive<T: Read + Seek> {
    reader: T,
    /// List of all entries (pre-scanned)
    entries: Vec<TarEntry>,
    /// Current entry index
    current_index: usize,
}

impl<T: Read + Seek> TarArchive<T> {
    /// Create a new TAR archive reader
    pub fn new(mut reader: T) -> Result<Self> {
        // Pre-scan all entries to allow random access
        let mut entries = Vec::new();

        // Reset to start
        reader.seek(SeekFrom::Start(0))?;

        {
            let mut archive = tar::Archive::new(&mut reader);

            for entry_result in archive
                .entries()
                .map_err(|e| ArchiveError::io_error(format!("Failed to read TAR entries: {}", e)))?
            {
                let entry = entry_result.map_err(|e| {
                    ArchiveError::io_error(format!("Failed to read TAR entry: {}", e))
                })?;

                let header = entry.header();
                let name = entry
                    .path()
                    .map_err(|e| {
                        ArchiveError::io_error(format!("Failed to read entry path: {}", e))
                    })?
                    .to_string_lossy()
                    .to_string();

                let size = header.size().unwrap_or(0);
                let mtime = header.mtime().unwrap_or(0);
                let mode = header.mode().unwrap_or(0);
                let entry_type = header.entry_type().into();
                let link_name = header
                    .link_name()
                    .ok()
                    .flatten()
                    .map(|p| p.to_string_lossy().to_string());

                let raw_header_position = entry.raw_header_position();
                // Data starts after the 512-byte header
                let data_offset = raw_header_position + 512;

                entries.push(TarEntry {
                    header: TarFileHeader {
                        name,
                        size,
                        mtime,
                        mode,
                        entry_type,
                        link_name,
                    },
                    data_offset,
                });
            }
        }

        // Reset reader position
        reader.seek(SeekFrom::Start(0))?;

        Ok(Self {
            reader,
            entries,
            current_index: 0,
        })
    }

    /// Get the next entry in the archive
    pub fn get_next_entry(&mut self) -> Result<Option<TarFileHeader>> {
        if self.current_index >= self.entries.len() {
            return Ok(None);
        }

        let entry = &self.entries[self.current_index];
        Ok(Some(entry.header.clone()))
    }

    /// Skip the current entry
    pub fn skip(&mut self, _header: &TarFileHeader) -> Result<()> {
        if self.current_index < self.entries.len() {
            self.current_index += 1;
        }
        Ok(())
    }

    /// Read the contents of the current entry
    pub fn read(&mut self, header: &TarFileHeader) -> Result<Vec<u8>> {
        // Find the entry by name (in case entries were iterated out of order)
        let entry = self
            .entries
            .iter()
            .find(|e| e.header.name == header.name)
            .ok_or_else(|| ArchiveError::io_error(format!("Entry not found: {}", header.name)))?;

        // Seek to data position
        self.reader.seek(SeekFrom::Start(entry.data_offset))?;

        // Read the data
        let mut data = vec![0u8; entry.header.size as usize];
        self.reader.read_exact(&mut data)?;

        // Advance to next entry
        self.current_index += 1;

        Ok(data)
    }

    /// Get the total number of entries
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}
