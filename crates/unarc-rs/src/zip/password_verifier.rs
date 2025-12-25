//! ZIP Password Verifier - password testing for ZIP archives.
//!
//! This module provides a `Send + Sync` password verifier that can be used with
//! rayon for parallel password testing.

use std::io::{Cursor, Read};
use std::sync::Arc;

/// A standalone password verifier for ZIP archives.
///
/// This struct holds all the data needed to verify a password without
/// requiring access to the original archive. It is `Send + Sync` and
/// can be safely used from multiple threads with rayon.
#[derive(Clone)]
pub struct ZipPasswordVerifier {
    /// The complete archive data (needed because zip crate needs seekable reader)
    archive_data: Arc<[u8]>,
    /// Index of the encrypted file to test
    file_index: usize,
    /// Expected CRC32 of the uncompressed data
    expected_crc: u32,
    /// Original (uncompressed) size
    original_size: u64,
    /// Entry name (for debugging)
    entry_name: String,
}

// Arc<[u8]> is Send + Sync, other fields are Copy or Clone+Send+Sync
unsafe impl Send for ZipPasswordVerifier {}
unsafe impl Sync for ZipPasswordVerifier {}

impl ZipPasswordVerifier {
    /// Create a new password verifier from archive data.
    ///
    /// # Arguments
    /// * `archive_data` - The complete ZIP archive data
    /// * `file_index` - Index of the encrypted file to test against
    /// * `expected_crc` - The CRC32 from the header
    /// * `original_size` - The uncompressed size
    /// * `entry_name` - Name of the entry (for debugging)
    pub fn new(
        archive_data: Vec<u8>,
        file_index: usize,
        expected_crc: u32,
        original_size: u64,
        entry_name: String,
    ) -> Self {
        Self {
            archive_data: Arc::from(archive_data.into_boxed_slice()),
            file_index,
            expected_crc,
            original_size,
            entry_name,
        }
    }

    /// Get the entry name this verifier was created for.
    pub fn entry_name(&self) -> &str {
        &self.entry_name
    }

    /// Get the original (uncompressed) size.
    pub fn original_size(&self) -> u64 {
        self.original_size
    }

    /// Verify if the given password is correct.
    ///
    /// Returns `true` if the password produces valid decompressed data
    /// with matching CRC and size, `false` otherwise.
    pub fn verify(&self, password: &str) -> bool {
        // Create a cursor over the archive data
        let cursor = Cursor::new(&*self.archive_data);

        // Open the archive
        let mut archive = match zip::ZipArchive::new(cursor) {
            Ok(a) => a,
            Err(_) => return false,
        };

        // Try to decrypt and decompress with this password
        let mut file = match archive.by_index_decrypt(self.file_index, password.as_bytes()) {
            Ok(f) => f,
            Err(_) => return false,
        };

        // Read and decompress the data
        let mut data = Vec::with_capacity(self.original_size as usize);
        if file.read_to_end(&mut data).is_err() {
            return false;
        }

        // Check size first (fast rejection)
        if data.len() != self.original_size as usize {
            return false;
        }

        // Check CRC32
        let crc = crc32fast::hash(&data);
        crc == self.expected_crc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verifier_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ZipPasswordVerifier>();
    }
}
