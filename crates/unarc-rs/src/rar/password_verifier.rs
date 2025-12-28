//! RAR Password Verifier - password testing for RAR archives.
//!
//! This module provides a `Send + Sync` password verifier that can be used with
//! rayon for parallel password testing.
//!
//! Note: The unrar crate requires a file path, so this verifier stores the path
//! and re-opens the archive for each verification attempt. This is less efficient
//! than the ARC/ARJ verifiers but still enables parallel testing.

use std::path::PathBuf;
use std::sync::Arc;

/// A standalone password verifier for RAR archives.
///
/// This struct holds all the data needed to verify a password.
/// It is `Send + Sync` and can be safely used from multiple threads with rayon.
///
/// Note: Since unrar requires a file path, each verification attempt opens the archive.
#[derive(Clone)]
pub struct RarPasswordVerifier {
    /// Path to the archive file
    archive_path: Arc<PathBuf>,
    /// Name of the encrypted file to test
    file_name: String,
    /// Expected CRC32 of the uncompressed data
    expected_crc: u32,
    /// Original (uncompressed) size
    original_size: u64,
}

// Arc<PathBuf> is Send + Sync, other fields are Copy or Clone+Send+Sync
unsafe impl Send for RarPasswordVerifier {}
unsafe impl Sync for RarPasswordVerifier {}

impl RarPasswordVerifier {
    /// Create a new password verifier.
    ///
    /// # Arguments
    /// * `archive_path` - Path to the RAR archive
    /// * `file_name` - Name of the encrypted file to test against
    /// * `expected_crc` - The CRC32 from the header
    /// * `original_size` - The uncompressed size
    pub fn new(archive_path: PathBuf, file_name: String, expected_crc: u32, original_size: u64) -> Self {
        Self {
            archive_path: Arc::new(archive_path),
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
        // Open archive with password
        let archive = match unrar::Archive::with_password(&*self.archive_path, password).open_for_processing() {
            Ok(a) => a,
            Err(_) => return false,
        };

        // Find and extract the specific file
        let mut current = archive;
        loop {
            match current.read_header() {
                Ok(Some(header_cursor)) => {
                    let entry_name = header_cursor.entry().filename.to_string_lossy().to_string();

                    if entry_name == self.file_name {
                        // Try to extract this file
                        match header_cursor.read() {
                            Ok((data, _)) => {
                                // Check size first (fast rejection)
                                if data.len() != self.original_size as usize {
                                    return false;
                                }
                                // Check CRC32
                                let crc = crc32fast::hash(&data);
                                return crc == self.expected_crc;
                            }
                            Err(_) => return false,
                        }
                    } else {
                        // Skip this file
                        match header_cursor.skip() {
                            Ok(next) => current = next,
                            Err(_) => return false,
                        }
                    }
                }
                Ok(None) => return false, // File not found
                Err(_) => return false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verifier_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<RarPasswordVerifier>();
    }
}
