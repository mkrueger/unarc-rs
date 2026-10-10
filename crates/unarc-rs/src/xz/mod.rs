//! XZ single file format support
//!
//! XZ is a compression format for single files based on LZMA2. Files with extension .xz
//! are xz-compressed single files (as opposed to .tar.xz which contains a TAR archive).
//!
//! This module provides read-only access to .xz files. Concatenated xz streams are
//! decoded one after another, as `xz -d` does.

use std::io::Read;

use lzma_rust2::XzReader;

use crate::error::{ArchiveError, Result};

/// XZ stream header magic: 0xFD "7zXZ" 0x00
const XZ_MAGIC: [u8; 6] = [0xFD, b'7', b'z', b'X', b'Z', 0x00];

/// Decoder memory needed besides the dictionary, in KiB (about 100 KiB for LZMA2)
const DECODER_OVERHEAD_KB: u32 = 1024;

/// XZ archive reader for single compressed files
///
/// Note: The entire file is decompressed into memory on read.
pub struct XzArchive<T: Read> {
    reader: Option<T>,
}

impl<T: Read> XzArchive<T> {
    /// Create a new XZ archive reader
    ///
    /// This validates the xz header but doesn't decompress until read() is called.
    pub fn new(mut reader: T) -> Result<Self> {
        let mut header = [0u8; 6];
        reader.read_exact(&mut header)?;

        if header != XZ_MAGIC {
            return Err(ArchiveError::invalid_header("XZ"));
        }

        Ok(Self { reader: Some(reader) })
    }

    /// Skip the file (XZ contains only one file)
    pub fn skip(&mut self) -> Result<()> {
        // Just one file in the archive, nothing to skip to
        Ok(())
    }

    /// Read and decompress the file
    pub fn read(&mut self) -> Result<Vec<u8>> {
        self.read_with_limit(None)
    }

    /// Read and decompress the file, failing if it decompresses to more than `limit` bytes
    pub fn read_with_limit(&mut self, limit: Option<u64>) -> Result<Vec<u8>> {
        let reader = self
            .reader
            .take()
            .ok_or_else(|| ArchiveError::io_error("XZ archive already read or in invalid state"))?;

        // Reconstruct with the header we already consumed
        let chained = std::io::Cursor::new(XZ_MAGIC).chain(reader);
        decompress(chained, limit, "XZ")
    }
}

/// Decompresses all xz streams in `reader`, failing once they produce more than `limit` bytes
///
/// The dictionary size is read from the block headers, so it is checked against
/// [`crate::limits::window_limit`] before the decoder allocates it.
pub(crate) fn decompress<R: Read>(reader: R, limit: Option<u64>, entry: &str) -> Result<Vec<u8>> {
    let window_kb = u32::try_from(crate::limits::window_limit(limit) / 1024).unwrap_or(u32::MAX);
    let decoder = XzReader::new_mem_limit(reader, true, window_kb.saturating_add(DECODER_OVERHEAD_KB));
    crate::limits::read_to_end_limited(decoder, limit, entry).map_err(|e| match e {
        ArchiveError::Io(e) => ArchiveError::io_error(format!("Failed to decompress xz: {e}")),
        e => e,
    })
}
