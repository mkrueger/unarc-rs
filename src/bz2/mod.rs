//! BZ2 (bzip2) single file format support
//!
//! BZ2 is a compression format for single files. Files with extension .bz2
//! are bzip2-compressed single files (as opposed to .tar.bz2 which contains
//! a TAR archive).
//!
//! This module provides read-only access to .bz2 files.

use std::io::Read;

use bzip2::read::BzDecoder;

use crate::error::{ArchiveError, Result};

/// BZ2 archive reader for single compressed files
///
/// Note: The entire file is decompressed into memory on read.
pub struct Bz2Archive<T: Read> {
    reader: Option<T>,
}

impl<T: Read> Bz2Archive<T> {
    /// Create a new BZ2 archive reader
    ///
    /// This validates the bzip2 header but doesn't decompress until read() is called.
    pub fn new(mut reader: T) -> Result<Self> {
        // Peek at the header to validate it's a bzip2 file
        let mut header = [0u8; 3];
        reader.read_exact(&mut header)?;

        // Bzip2 magic: "BZh" (0x42 0x5a 0x68)
        if header[0] != b'B' || header[1] != b'Z' || header[2] != b'h' {
            return Err(ArchiveError::invalid_header("BZ2"));
        }

        Ok(Self {
            reader: Some(reader),
        })
    }

    /// Skip the file (BZ2 contains only one file)
    pub fn skip(&mut self) -> Result<()> {
        // Just one file in the archive, nothing to skip to
        Ok(())
    }

    /// Read and decompress the file
    pub fn read(&mut self) -> Result<Vec<u8>> {
        let reader = self.reader.take().ok_or_else(|| {
            ArchiveError::io_error("BZ2 archive already read or in invalid state")
        })?;

        // Reconstruct with the header we already consumed
        let header = [b'B', b'Z', b'h'];
        let chained = std::io::Cursor::new(header).chain(reader);

        let mut decoder = BzDecoder::new(chained);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .map_err(|e| ArchiveError::io_error(format!("Failed to decompress bzip2: {}", e)))?;

        Ok(decompressed)
    }
}
