//! TGZ (tar.gz) archive format support
//!
//! TGZ is a combination of TAR archive format with gzip compression.
//! Files with extensions .tar.gz or .tgz are gzip-compressed TAR archives.
//!
//! This module provides read-only access to TGZ archives by first decompressing
//! the gzip layer and then parsing the TAR content.

use std::io::{Cursor, Read};

use flate2::read::GzDecoder;

use crate::error::Result;
use crate::tar::{TarArchive, TarFileHeader};

/// TGZ archive reader
///
/// This wraps a TAR archive after gzip decompression.
/// Note: The entire archive is decompressed into memory on construction.
pub struct TgzArchive {
    inner: TarArchive<Cursor<Vec<u8>>>,
}

impl TgzArchive {
    /// Create a new TGZ archive reader
    ///
    /// This will decompress the entire gzip stream into memory,
    /// then create a TAR archive reader from the decompressed data.
    pub fn new<T: Read>(reader: T) -> Result<Self> {
        Self::new_with_limit(reader, None)
    }

    /// Like [`Self::new`], but fails if the decompressed TAR stream exceeds `limit` bytes
    pub fn new_with_limit<T: Read>(reader: T, limit: Option<u64>) -> Result<Self> {
        let decoder = GzDecoder::new(reader);
        let decompressed = crate::limits::read_to_end_limited(decoder, limit, "TGZ").map_err(|e| match e {
            crate::error::ArchiveError::Io(e) => crate::error::ArchiveError::io_error(format!("Failed to decompress gzip: {}", e)),
            e => e,
        })?;

        // Create a cursor for the decompressed data
        let cursor = Cursor::new(decompressed);

        // Create TAR archive from decompressed data
        let inner = TarArchive::new(cursor)?;

        Ok(Self { inner })
    }

    /// Get the next entry in the archive
    pub fn get_next_entry(&mut self) -> Result<Option<TarFileHeader>> {
        self.inner.get_next_entry()
    }

    /// Skip the current entry
    pub fn skip(&mut self, header: &TarFileHeader) -> Result<()> {
        self.inner.skip(header)
    }

    /// Read the contents of the current entry
    pub fn read(&mut self, header: &TarFileHeader) -> Result<Vec<u8>> {
        self.inner.read(header)
    }

    /// Get the total number of entries
    pub fn entry_count(&self) -> usize {
        self.inner.entry_count()
    }
}
