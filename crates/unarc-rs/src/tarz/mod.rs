//! TAR.Z (tar + Unix compress) archive format support
//!
//! TAR.Z is a combination of TAR archive format with Unix compress (LZW) compression.
//! Files with extension .tar.Z are LZW-compressed TAR archives.
//!
//! This module provides read-only access to TAR.Z archives by first decompressing
//! the LZW layer and then parsing the TAR content.

use std::io::{Cursor, Read};

use crate::error::Result;
use crate::tar::{TarArchive, TarFileHeader};
use crate::z::ZArchive;

/// TAR.Z archive reader
///
/// This wraps a TAR archive after LZW (Unix compress) decompression.
/// Note: The entire archive is decompressed into memory on construction.
pub struct TarZArchive {
    inner: TarArchive<Cursor<Vec<u8>>>,
}

impl TarZArchive {
    /// Create a new TAR.Z archive reader
    ///
    /// This will decompress the entire LZW stream into memory,
    /// then create a TAR archive reader from the decompressed data.
    pub fn new<T: Read>(reader: T) -> Result<Self> {
        // Use the existing Z archive to decompress
        let mut z_archive = ZArchive::new(reader)?;
        let decompressed = z_archive.read()?;

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
