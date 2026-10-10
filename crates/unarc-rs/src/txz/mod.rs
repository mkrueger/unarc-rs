//! TXZ (tar.xz) archive format support
//!
//! TXZ is a combination of TAR archive format with xz compression.
//! Files with extensions .tar.xz or .txz are xz-compressed TAR archives.
//!
//! This module provides read-only access to TXZ archives by first decompressing
//! the xz layer and then parsing the TAR content.

use std::io::{Cursor, Read};

use crate::error::Result;
use crate::tar::{TarArchive, TarFileHeader};

/// TXZ archive reader
///
/// This wraps a TAR archive after xz decompression.
/// Note: The entire archive is decompressed into memory on construction.
pub struct TxzArchive {
    inner: TarArchive<Cursor<Vec<u8>>>,
}

impl TxzArchive {
    /// Create a new TXZ archive reader
    ///
    /// This will decompress the entire xz stream into memory,
    /// then create a TAR archive reader from the decompressed data.
    pub fn new<T: Read>(reader: T) -> Result<Self> {
        Self::new_with_limit(reader, None)
    }

    /// Like [`Self::new`], but fails if the decompressed TAR stream exceeds `limit` bytes
    pub fn new_with_limit<T: Read>(reader: T, limit: Option<u64>) -> Result<Self> {
        let decompressed = crate::xz::decompress(reader, limit, "TXZ")?;

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
