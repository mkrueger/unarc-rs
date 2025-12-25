//! 7z Password Verifier - password testing for 7z archives.
//!
//! This module provides a `Send + Sync` password verifier that can be used with
//! rayon for parallel password testing.
//!
//! Note: The sevenz-rust2 crate requires a seekable reader, so this verifier
//! stores the complete archive data and re-opens it for each verification attempt.

use std::io::Cursor;
use std::sync::Arc;

/// A standalone password verifier for 7z archives.
///
/// This struct holds all the data needed to verify a password.
/// It is `Send + Sync` and can be safely used from multiple threads with rayon.
#[derive(Clone)]
pub struct SevenZPasswordVerifier {
    /// The complete archive data
    archive_data: Arc<[u8]>,
    /// Name of the encrypted file to test
    file_name: String,
    /// Expected CRC32 of the uncompressed data
    expected_crc: u32,
    /// Original (uncompressed) size
    original_size: u64,
}

// Arc<[u8]> is Send + Sync, other fields are Copy or Clone+Send+Sync
unsafe impl Send for SevenZPasswordVerifier {}
unsafe impl Sync for SevenZPasswordVerifier {}

impl SevenZPasswordVerifier {
    /// Create a new password verifier from archive data.
    ///
    /// # Arguments
    /// * `archive_data` - The complete 7z archive data
    /// * `file_name` - Name of the encrypted file to test against
    /// * `expected_crc` - The CRC32 from the header
    /// * `original_size` - The uncompressed size
    pub fn new(
        archive_data: Vec<u8>,
        file_name: String,
        expected_crc: u32,
        original_size: u64,
    ) -> Self {
        Self {
            archive_data: Arc::from(archive_data.into_boxed_slice()),
            file_name,
            expected_crc,
            original_size,
        }
    }

    /// Get the entry name this verifier was created for.
    pub fn entry_name(&self) -> &str {
        &self.file_name
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
        let mut cursor = Cursor::new(&*self.archive_data);

        // Open the archive with the password
        let pwd = sevenz_rust2::Password::from(password);
        let mut archive = match sevenz_rust2::ArchiveReader::new(&mut cursor, pwd) {
            Ok(a) => a,
            Err(_) => return false,
        };

        // Try to read the file
        let data = match archive.read_file(&self.file_name) {
            Ok(d) => d,
            Err(_) => return false,
        };

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
        assert_send_sync::<SevenZPasswordVerifier>();
    }
}
