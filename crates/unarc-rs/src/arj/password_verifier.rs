//! ARJ Password Verifier - efficient password testing without repeated allocations.
//!
//! This module provides a `Send + Sync` password verifier that can be used with
//! rayon for parallel password testing.

use std::sync::Arc;

use delharc::decode::{Decoder, DecoderAny};

use super::crypto::decrypt_arj_data;
use super::decode_fastest::decode_fastest;
use super::local_file_header::CompressionMethod;
use crate::encryption::ArjEncryption;
use crate::error::Result;

/// A standalone password verifier for ARJ archives.
///
/// This struct holds all the data needed to verify a password without
/// requiring access to the original archive. It is `Send + Sync` and
/// can be safely used from multiple threads with rayon.
#[derive(Clone)]
pub struct ArjPasswordVerifier {
    /// The compressed (and possibly encrypted) data
    compressed_data: Arc<[u8]>,
    /// Compression method used
    compression_method: CompressionMethod,
    /// Expected CRC32 of the uncompressed data
    expected_crc: u32,
    /// Original (uncompressed) size
    original_size: u32,
    /// Entry name (for debugging)
    entry_name: String,
    /// Encryption type
    encryption_type: Option<ArjEncryption>,
    /// Password modifier from header
    password_modifier: u8,
    /// File time for GOST decryption
    file_time: u32,
}

// Arc<[u8]> is Send + Sync, other fields are Copy or Send+Sync
unsafe impl Send for ArjPasswordVerifier {}
unsafe impl Sync for ArjPasswordVerifier {}

impl ArjPasswordVerifier {
    /// Create a new password verifier from entry data.
    ///
    /// # Arguments
    /// * `compressed_data` - The raw compressed/encrypted data from the archive
    /// * `compression_method` - The compression method used
    /// * `expected_crc` - The CRC32 from the header
    /// * `original_size` - The uncompressed size
    /// * `entry_name` - Name of the entry (for debugging)
    /// * `encryption_type` - The type of encryption used
    /// * `password_modifier` - Modifier byte for garble encryption
    /// * `file_time` - DOS file time for GOST key derivation
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        compressed_data: Vec<u8>,
        compression_method: CompressionMethod,
        expected_crc: u32,
        original_size: u32,
        entry_name: String,
        encryption_type: Option<ArjEncryption>,
        password_modifier: u8,
        file_time: u32,
    ) -> Self {
        Self {
            compressed_data: Arc::from(compressed_data.into_boxed_slice()),
            compression_method,
            expected_crc,
            original_size,
            entry_name,
            encryption_type,
            password_modifier,
            file_time,
        }
    }

    /// Get the entry name this verifier was created for.
    pub fn entry_name(&self) -> &str {
        &self.entry_name
    }

    /// Get the compressed data size.
    pub fn compressed_size(&self) -> usize {
        self.compressed_data.len()
    }

    /// Get the original (uncompressed) size.
    pub fn original_size(&self) -> u32 {
        self.original_size
    }

    /// Get the encryption type.
    pub fn encryption_type(&self) -> Option<ArjEncryption> {
        self.encryption_type
    }

    /// Verify if the given password is correct.
    ///
    /// Returns `true` if the password produces valid decompressed data
    /// with matching CRC and size, `false` otherwise.
    pub fn verify(&self, password: &str) -> bool {
        // Thread-local buffer for decryption to avoid allocations
        thread_local! {
            static DECRYPT_BUF: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(Vec::with_capacity(64 * 1024));
        }

        DECRYPT_BUF.with(|buf| {
            let mut decrypt_buf = buf.borrow_mut();
            decrypt_buf.clear();
            decrypt_buf.extend_from_slice(&self.compressed_data);

            // Decrypt the data
            decrypt_arj_data(&mut decrypt_buf, self.encryption_type, password, self.password_modifier, self.file_time);

            // Decompress and verify
            self.decompress_and_verify(&decrypt_buf).unwrap_or(false)
        })
    }

    /// Decompress data and verify CRC and size.
    fn decompress_and_verify(&self, decrypted: &[u8]) -> Result<bool> {
        let uncompressed = match self.compression_method {
            CompressionMethod::Stored => decrypted.to_vec(),
            CompressionMethod::CompressedMost | CompressionMethod::Compressed | CompressionMethod::CompressedFaster => {
                let mut decoder = DecoderAny::new_from_compression(delharc::CompressionMethod::Lh6, decrypted);
                let mut decompressed_buffer = vec![0; self.original_size as usize];
                if decoder.fill_buffer(&mut decompressed_buffer).is_err() {
                    return Ok(false);
                }
                decompressed_buffer
            }
            CompressionMethod::CompressedFastest => match decode_fastest(decrypted, self.original_size as usize) {
                Ok(data) => data,
                Err(_) => return Ok(false),
            },
            CompressionMethod::NoDataNoCrc | CompressionMethod::NoData | CompressionMethod::Unknown(_) => {
                return Ok(false);
            }
        };

        // Check size first (fast rejection)
        if uncompressed.len() != self.original_size as usize {
            return Ok(false);
        }

        // Then check CRC32
        let checksum = crc32fast::hash(&uncompressed);
        Ok(checksum == self.expected_crc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verifier_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ArjPasswordVerifier>();
    }
}
