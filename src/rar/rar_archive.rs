//! RAR archive reader
//!
//! Uses the `unrar` crate for parsing and extracting RAR archives (supports RAR4 and RAR5).
//! Note: The unrar crate requires a file path, so for streaming readers we write to a temp file.

use std::io::{self, Read, Seek, SeekFrom};
use std::path::PathBuf;

use crate::date_time::DosDateTime;

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
}

/// RAR archive reader
///
/// Provides access to RAR archives using the unrar library.
/// Supports both RAR4 and RAR5 formats with full decompression.
pub struct RarArchive<T: Read + Seek> {
    #[allow(dead_code)]
    reader: T,
    entries: Vec<RarFileHeader>,
    current_index: usize,
    /// Path to the archive (or temp file)
    archive_path: PathBuf,
    /// Whether we created a temp file
    is_temp_file: bool,
}

impl<T: Read + Seek> RarArchive<T> {
    /// Create a new RAR archive reader from a Read+Seek source
    ///
    /// This will write the data to a temporary file since unrar requires a file path.
    pub fn new(mut reader: T) -> io::Result<Self> {
        // Create temp file
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join(format!("unarc_rar_{}.rar", std::process::id()));

        // Write reader contents to temp file
        reader.seek(SeekFrom::Start(0))?;
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        std::fs::write(&temp_path, &data)?;

        // Reset reader
        reader.seek(SeekFrom::Start(0))?;

        // Parse entries using unrar
        let entries = Self::parse_entries_from_path(&temp_path)?;

        Ok(Self {
            reader,
            entries,
            current_index: 0,
            archive_path: temp_path,
            is_temp_file: true,
        })
    }

    /// Create a RAR archive reader directly from a file path (more efficient)
    pub fn from_path(path: &std::path::Path) -> io::Result<Self>
    where
        T: Default,
    {
        let entries = Self::parse_entries_from_path(path)?;

        Ok(Self {
            reader: T::default(),
            entries,
            current_index: 0,
            archive_path: path.to_path_buf(),
            is_temp_file: false,
        })
    }

    fn parse_entries_from_path(path: &std::path::Path) -> io::Result<Vec<RarFileHeader>> {
        let archive = unrar::Archive::new(path).open_for_listing().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to open RAR archive: {:?}", e),
            )
        })?;

        let mut entries = Vec::new();
        for entry_result in archive {
            match entry_result {
                Ok(entry) => {
                    // Convert modification time from unrar's file_time
                    let date_time = {
                        let ft = entry.file_time;
                        // file_time is already in DOS format
                        Some(DosDateTime::new(ft as u32))
                    };

                    entries.push(RarFileHeader {
                        name: entry.filename.to_string_lossy().to_string(),
                        compressed_size: entry.unpacked_size, // unrar doesn't expose packed size in listing mode
                        original_size: entry.unpacked_size,
                        compression_method: if entry.is_file() {
                            "Compressed".to_string()
                        } else {
                            "Directory".to_string()
                        },
                        date_time,
                        crc32: entry.file_crc,
                        is_directory: entry.is_directory(),
                    });
                }
                Err(e) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Failed to read RAR entry: {:?}", e),
                    ));
                }
            }
        }

        Ok(entries)
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
        Ok(())
    }

    /// Read and decompress an entry's data
    pub fn read(&mut self, header: &RarFileHeader) -> io::Result<Vec<u8>> {
        if header.is_directory {
            return Ok(Vec::new());
        }

        // Create temp directory for extraction
        let temp_dir =
            std::env::temp_dir().join(format!("unarc_rar_extract_{}", std::process::id()));
        std::fs::create_dir_all(&temp_dir)?;

        // Open archive for processing
        let archive = unrar::Archive::new(&self.archive_path)
            .open_for_processing()
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to open RAR for extraction: {:?}", e),
                )
            })?;

        // Find and extract the specific file
        let mut result_data = None;
        let mut current = archive;

        loop {
            match current.read_header() {
                Ok(Some(header_cursor)) => {
                    let entry_name = header_cursor.entry().filename.to_string_lossy().to_string();

                    if entry_name == header.name {
                        // Extract this file
                        let (data, _next) = header_cursor.read().map_err(|e| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("Failed to extract file: {:?}", e),
                            )
                        })?;
                        result_data = Some(data);
                        break;
                    } else {
                        // Skip this file
                        current = header_cursor.skip().map_err(|e| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("Failed to skip file: {:?}", e),
                            )
                        })?;
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    let _ = std::fs::remove_dir_all(&temp_dir);
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Failed to read RAR header: {:?}", e),
                    ));
                }
            }
        }

        // Clean up temp directory
        let _ = std::fs::remove_dir_all(&temp_dir);

        result_data.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("File not found in archive: {}", header.name),
            )
        })
    }
}

impl<T: Read + Seek> Drop for RarArchive<T> {
    fn drop(&mut self) {
        // Clean up temp file if we created one
        if self.is_temp_file {
            let _ = std::fs::remove_file(&self.archive_path);
        }
    }
}
