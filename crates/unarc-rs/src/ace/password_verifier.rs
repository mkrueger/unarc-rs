//! ACE Password Verifier - efficient password testing without repeated allocations.
//!
//! This module provides a `Send + Sync` password verifier that can be used with
//! rayon for parallel password testing.

use std::io::Cursor;
use std::sync::Arc;

use super::bitstream::BitStream;
use super::crypto::decrypt_ace_data;
use super::header::CompressionType;
use super::lz77::Lz77Decoder;

/// A standalone password verifier for ACE archives.
///
/// This struct holds all the data needed to verify a password without
/// requiring access to the original archive. It is `Send + Sync` and
/// can be safely used from multiple threads with rayon.
///
/// Note: For solid archives, only the first encrypted file can be tested
/// in parallel since the LZ77 decoder state must be maintained across files.
#[derive(Clone)]
pub struct AcePasswordVerifier {
    /// The compressed (and encrypted) data
    compressed_data: Arc<[u8]>,
    /// Compression type used
    compression_type: CompressionType,
    /// Expected CRC32 of the uncompressed data (ACE uses inverted CRC)
    expected_crc: u32,
    /// Original (uncompressed) size
    original_size: u64,
    /// Dictionary size for LZ77/Blocked compression
    dictionary_size: usize,
    /// Entry name (for debugging)
    entry_name: String,
}

// Arc<[u8]> is Send + Sync, other fields are Copy or Clone+Send+Sync
unsafe impl Send for AcePasswordVerifier {}
unsafe impl Sync for AcePasswordVerifier {}

impl AcePasswordVerifier {
    /// Create a new password verifier from entry data.
    ///
    /// # Arguments
    /// * `compressed_data` - The raw compressed/encrypted data from the archive
    /// * `compression_type` - The compression type used
    /// * `expected_crc` - The CRC32 from the header
    /// * `original_size` - The uncompressed size
    /// * `dictionary_size` - Dictionary size for LZ77/Blocked compression
    /// * `entry_name` - Name of the entry (for debugging)
    pub fn new(
        compressed_data: Vec<u8>,
        compression_type: CompressionType,
        expected_crc: u32,
        original_size: u64,
        dictionary_size: usize,
        entry_name: String,
    ) -> Self {
        Self {
            compressed_data: Arc::from(compressed_data.into_boxed_slice()),
            compression_type,
            expected_crc,
            original_size,
            dictionary_size,
            entry_name,
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
    pub fn original_size(&self) -> u64 {
        self.original_size
    }

    /// Verify if the given password is correct.
    ///
    /// Returns `true` if the password produces valid decompressed data
    /// with matching CRC and size, `false` otherwise.
    pub fn verify(&self, password: &str) -> bool {
        // Decrypt the data
        let decrypted = decrypt_ace_data(&self.compressed_data, password);

        // Decompress and verify
        self.decompress_and_verify(&decrypted).unwrap_or(false)
    }

    /// Decompress data and verify CRC and size.
    fn decompress_and_verify(&self, decrypted: &[u8]) -> crate::error::Result<bool> {
        let decompressed = match self.compression_type {
            CompressionType::Stored => decrypted.to_vec(),
            CompressionType::Lz77 | CompressionType::Blocked => {
                let mut decoder = Lz77Decoder::new();
                decoder.set_dictionary_size(self.dictionary_size);

                let cursor = Cursor::new(decrypted);
                let mut bs = BitStream::new(cursor, decrypted.len());

                match decoder.decompress(&mut bs, self.original_size as usize) {
                    Ok(data) => data,
                    Err(_) => return Ok(false),
                }
            }
            CompressionType::Unknown(_) => {
                return Ok(false);
            }
        };

        // Check size first (fast rejection)
        if decompressed.len() != self.original_size as usize {
            return Ok(false);
        }

        // ACE uses inverted CRC32
        let checksum = !crc32fast::hash(&decompressed);
        Ok(checksum == self.expected_crc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verifier_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<AcePasswordVerifier>();
    }
}
