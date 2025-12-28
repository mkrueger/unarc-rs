//! GZ (gzip) single file format support
//!
//! GZ is a compression format for single files. Files with extension .gz
//! are gzip-compressed single files (as opposed to .tar.gz which contains
//! a TAR archive).
//!
//! This module provides read-only access to .gz files.

use std::io::Read;

use flate2::read::GzDecoder;

use crate::error::{ArchiveError, Result};

/// GZ archive reader for single compressed files
///
/// Note: The entire file is decompressed into memory on read.
pub struct GzArchive<T: Read> {
    reader: Option<T>,
}

impl<T: Read> GzArchive<T> {
    /// Create a new GZ archive reader
    ///
    /// This validates the gzip header but doesn't decompress until read() is called.
    pub fn new(mut reader: T) -> Result<Self> {
        // Peek at the header to validate it's a gzip file
        let mut header = [0u8; 2];
        reader.read_exact(&mut header)?;

        // Gzip magic number: 0x1f 0x8b
        if header != [0x1f, 0x8b] {
            return Err(ArchiveError::invalid_header("GZ"));
        }

        // We need to reconstruct the reader with the header
        // Since we can't "unread", we'll store the reader and handle this in read()
        // For simplicity, we'll create a chain
        Ok(Self { reader: Some(reader) })
    }

    /// Skip the file (GZ contains only one file)
    pub fn skip(&mut self) -> Result<()> {
        // Just one file in the archive, nothing to skip to
        Ok(())
    }

    /// Read and decompress the file
    pub fn read(&mut self) -> Result<Vec<u8>> {
        let reader = self
            .reader
            .take()
            .ok_or_else(|| ArchiveError::io_error("GZ archive already read or in invalid state"))?;

        // Reconstruct with the header we already consumed
        let header = [0x1f, 0x8b];
        let chained = std::io::Cursor::new(header).chain(reader);

        let mut decoder = GzDecoder::new(chained);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .map_err(|e| ArchiveError::io_error(format!("Failed to decompress gzip: {}", e)))?;

        Ok(decompressed)
    }
}
