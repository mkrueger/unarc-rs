//! RAR archive reader
//!
//! Uses the `unrar` crate for parsing and extracting RAR archives (supports RAR4 and RAR5).
//! Note: The unrar crate requires a file path, so for streaming readers we write to a temp file.

use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

use crate::date_time::DosDateTime;
use crate::error::{ArchiveError, Result};

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
    /// Whether this entry is encrypted
    pub is_encrypted: bool,
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
    /// Password for encrypted entries
    password: Option<String>,
}

impl<T: Read + Seek> RarArchive<T> {
    /// Create a new RAR archive reader from a Read+Seek source
    ///
    /// This will write the data to a temporary file since unrar requires a file path.
    pub fn new(mut reader: T) -> Result<Self> {
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
            password: None,
        })
    }

    /// Create a RAR archive reader directly from a file path (more efficient)
    pub fn from_path(path: &std::path::Path) -> Result<Self>
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

    fn parse_entries_from_path(path: &std::path::Path) -> Result<Vec<RarFileHeader>> {
        let archive = unrar::Archive::new(path).open_for_listing().map_err(|e| {
            ArchiveError::external_library("unrar", format!("Failed to open RAR archive: {:?}", e))
        })?;

        let mut entries = Vec::new();
        for entry_result in archive {
            match entry_result {
                Ok(entry) => {
                    // Convert modification time from unrar's file_time
                    let date_time = {
                        let ft = entry.file_time;
                        // file_time is already in DOS format
                        Some(DosDateTime::new(ft))
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
                        is_encrypted: entry.is_encrypted(),
                    });
                }
                Err(e) => {
                    return Err(ArchiveError::external_library(
                        "unrar",
                        format!("Failed to read RAR entry: {:?}", e),
                    ));
                }
            }
        }

        Ok(entries)
    }

    /// Get the next entry in the archive
    pub fn get_next_entry(&mut self) -> Result<Option<RarFileHeader>> {
        if self.current_index >= self.entries.len() {
            return Ok(None);
        }

        let entry = self.entries[self.current_index].clone();
        self.current_index += 1;
        Ok(Some(entry))
    }

    /// Skip the current entry without reading its data
    pub fn skip(&mut self, _header: &RarFileHeader) -> Result<()> {
        Ok(())
    }

    /// Read and decompress an entry's data
    pub fn read(&mut self, header: &RarFileHeader) -> Result<Vec<u8>> {
        self.read_with_password(header, self.password.clone())
    }

    /// Read and decompress an entry's data with a specific password
    pub fn read_with_password(
        &mut self,
        header: &RarFileHeader,
        password: Option<String>,
    ) -> Result<Vec<u8>> {
        if header.is_directory {
            return Ok(Vec::new());
        }

        // Check if entry needs a password
        if header.is_encrypted && password.is_none() {
            return Err(ArchiveError::encryption_required(&header.name, "RAR"));
        }

        // Create temp directory for extraction
        let temp_dir =
            std::env::temp_dir().join(format!("unarc_rar_extract_{}", std::process::id()));
        std::fs::create_dir_all(&temp_dir)?;

        // Open archive for processing with optional password
        let archive = if let Some(ref pwd) = password {
            unrar::Archive::with_password(&self.archive_path, pwd)
                .open_for_processing()
                .map_err(|e| {
                    ArchiveError::external_library(
                        "unrar",
                        format!("Failed to open RAR for extraction: {:?}", e),
                    )
                })?
        } else {
            unrar::Archive::new(&self.archive_path)
                .open_for_processing()
                .map_err(|e| {
                    ArchiveError::external_library(
                        "unrar",
                        format!("Failed to open RAR for extraction: {:?}", e),
                    )
                })?
        };

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
                            ArchiveError::decompression_failed(
                                &header.name,
                                format!("Failed to extract file: {:?}", e),
                            )
                        })?;
                        result_data = Some(data);
                        break;
                    } else {
                        // Skip this file
                        current = header_cursor.skip().map_err(|e| {
                            ArchiveError::external_library(
                                "unrar",
                                format!("Failed to skip file: {:?}", e),
                            )
                        })?;
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    let _ = std::fs::remove_dir_all(&temp_dir);
                    return Err(ArchiveError::external_library(
                        "unrar",
                        format!("Failed to read RAR header: {:?}", e),
                    ));
                }
            }
        }

        // Clean up temp directory
        let _ = std::fs::remove_dir_all(&temp_dir);

        result_data.ok_or_else(|| {
            ArchiveError::corrupted_entry_named("RAR", &header.name, "File not found in archive")
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

impl<T: Read + Seek> RarArchive<T> {
    /// Create a password verifier for the given encrypted entry.
    ///
    /// Returns a standalone verifier that can be used from multiple threads with rayon.
    /// Note: RAR verification requires re-opening the archive file for each attempt.
    pub fn create_password_verifier(
        &self,
        header: &RarFileHeader,
    ) -> Result<super::password_verifier::RarPasswordVerifier> {
        if !header.is_encrypted {
            return Err(ArchiveError::unsupported_method(
                "RAR",
                "entry is not encrypted",
            ));
        }

        Ok(super::password_verifier::RarPasswordVerifier::new(
            self.archive_path.clone(),
            header.name.clone(),
            header.crc32,
            header.original_size,
        ))
    }
}
