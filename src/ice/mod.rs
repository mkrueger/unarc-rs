//! ICE archive format support
//!
//! ICE is a simple compression format that uses LHA compression (typically -lh1-)
//! but without the standard LHA header. The file starts with 4 bytes indicating
//! the uncompressed size, followed by the raw compressed data.

use std::io::Read;

use delharc::decode::{Decoder, Lh1Decoder};

use crate::error::Result;

/// ICE archive reader
///
/// ICE files contain:
/// - 4 bytes: uncompressed size (little-endian)
/// - Remaining bytes: LHA-compressed data (typically -lh1- method)
pub struct IceArchive {
    #[allow(dead_code)]
    original_size: u32,
    data: Vec<u8>,
}

impl IceArchive {
    /// Returns the original (uncompressed) size of the file
    pub fn original_size(&self) -> u32 {
        self.original_size
    }

    /// Create a new ICE archive reader
    ///
    /// This reads the compressed data and attempts to decompress it using
    /// various LHA compression methods until one succeeds.
    pub fn new<T: Read>(mut reader: T) -> Result<Self> {
        // Read the 4-byte uncompressed size
        let mut len_bytes = [0u8; 4];
        reader.read_exact(&mut len_bytes)?;
        let original_size = u32::from_le_bytes(len_bytes);

        // Read all remaining compressed data
        let mut compressed_data = Vec::new();
        reader.read_to_end(&mut compressed_data)?;

        // Try to decompress with different LHA methods
        // ICE typically uses -lh1- compression
        let data = Self::try_decompress(&compressed_data, original_size)?;

        Ok(Self {
            original_size,
            data,
        })
    }

    /// Try to decompress data using various LHA methods
    fn try_decompress(compressed: &[u8], expected_size: u32) -> Result<Vec<u8>> {
        // Try -lh1- (most common for ICE)
        if let Ok(data) = Self::decompress_lh1(compressed, expected_size) {
            return Ok(data);
        }

        Err(crate::error::ArchiveError::decompression_failed(
            "ICE",
            "Failed to decompress with any supported LHA method",
        ))
    }

    /// Decompress using -lh1- method
    fn decompress_lh1(compressed: &[u8], expected_size: u32) -> Result<Vec<u8>> {
        let cursor = std::io::Cursor::new(compressed);
        let mut decoder = Lh1Decoder::new(cursor);
        let mut decompressed = vec![0u8; expected_size as usize];

        decoder.fill_buffer(&mut decompressed).map_err(|e| {
            crate::error::ArchiveError::decompression_failed("ICE", format!("{:?}", e))
        })?;

        // Verify decompressed size matches expected
        if decompressed.len() as u32 != expected_size {
            return Err(crate::error::ArchiveError::corrupted_entry(
                "ICE",
                "Decompressed size does not match expected size",
            ));
        }

        Ok(decompressed)
    }

    /// Skip the current entry (ICE files only contain one file)
    pub fn skip(&mut self) -> Result<()> {
        // ICE archives contain only one file, nothing to skip
        Ok(())
    }

    /// Read the decompressed data
    pub fn read(&mut self) -> Result<Vec<u8>> {
        Ok(std::mem::take(&mut self.data))
    }
}
