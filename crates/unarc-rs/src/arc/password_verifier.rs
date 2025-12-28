//! ARC Password Verifier - efficient password testing without repeated allocations.
//!
//! This module provides a `Send + Sync` password verifier that can be used with
//! rayon for parallel password testing.

use crc16::{State, ARC};
use std::sync::Arc;

use super::local_file_header::CompressionMethod;
use super::{crushed, distilled, lzw, rle, unsqueeze};
use crate::error::Result;

/// Decrypt data using ARC's simple XOR encryption.
fn decrypt_into(data: &[u8], password: &[u8], output: &mut Vec<u8>) {
    output.clear();
    output.reserve(data.len());

    if password.is_empty() {
        output.extend_from_slice(data);
        return;
    }

    let mut key_pos = 0;
    for &byte in data {
        output.push(byte ^ password[key_pos]);
        key_pos += 1;
        if key_pos >= password.len() {
            key_pos = 0;
        }
    }
}

/// A standalone password verifier for ARC archives.
///
/// This struct holds all the data needed to verify a password without
/// requiring access to the original archive. It is `Send + Sync` and
/// can be safely used from multiple threads with rayon.
#[derive(Clone)]
pub struct ArcPasswordVerifier {
    /// The compressed (and possibly encrypted) data
    compressed_data: Arc<[u8]>,
    /// Compression method used
    compression_method: CompressionMethod,
    /// Expected CRC16 of the uncompressed data
    expected_crc: u16,
    /// Original (uncompressed) size - for buffer pre-allocation
    original_size: u32,
    /// Entry name (for debugging)
    entry_name: String,
}

// Arc<[u8]> is Send + Sync, CompressionMethod is Copy, so this is safe
unsafe impl Send for ArcPasswordVerifier {}
unsafe impl Sync for ArcPasswordVerifier {}

impl ArcPasswordVerifier {
    /// Create a new password verifier from entry data.
    ///
    /// # Arguments
    /// * `compressed_data` - The raw compressed/encrypted data from the archive
    /// * `compression_method` - The compression method used
    /// * `expected_crc` - The CRC16 from the header
    /// * `original_size` - The uncompressed size
    /// * `entry_name` - Name of the entry (for debugging)
    pub fn new(compressed_data: Vec<u8>, compression_method: CompressionMethod, expected_crc: u16, original_size: u32, entry_name: String) -> Self {
        Self {
            compressed_data: Arc::from(compressed_data.into_boxed_slice()),
            compression_method,
            expected_crc,
            original_size,
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
    pub fn original_size(&self) -> u32 {
        self.original_size
    }

    /// Verify if the given password is correct.
    ///
    /// Returns `true` if the password produces valid decompressed data
    /// with matching CRC, `false` otherwise.
    ///
    /// This method uses thread-local scratch buffers and decompressors
    /// to minimize allocations when called repeatedly.
    pub fn verify(&self, password: &str) -> bool {
        // Thread-local resources to avoid repeated allocations
        thread_local! {
            static DECRYPT_BUF: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(Vec::with_capacity(64 * 1024));
            static LZW_BUF: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(Vec::with_capacity(64 * 1024));
            static RLE_BUF: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(Vec::with_capacity(64 * 1024));
            static LZW: std::cell::RefCell<lzw::Lzw> = std::cell::RefCell::new(lzw::Lzw::new());
        }

        DECRYPT_BUF.with(|decrypt_buf| {
            LZW_BUF.with(|lzw_buf| {
                RLE_BUF.with(|rle_buf| {
                    LZW.with(|lzw_cell| {
                        let mut decrypt_buf = decrypt_buf.borrow_mut();
                        let mut lzw_buf = lzw_buf.borrow_mut();
                        let mut rle_buf = rle_buf.borrow_mut();
                        let mut lzw = lzw_cell.borrow_mut();

                        // Decrypt into thread-local buffer
                        decrypt_into(&self.compressed_data, password.as_bytes(), &mut decrypt_buf);

                        // Decompress and verify CRC using thread-local buffers
                        self.decompress_and_verify_reuse(&decrypt_buf, &mut lzw, &mut lzw_buf, &mut rle_buf)
                            .unwrap_or(false)
                    })
                })
            })
        })
    }

    /// Decompress data and verify CRC, reusing provided buffers.
    fn decompress_and_verify_reuse(&self, decrypted: &[u8], lzw: &mut lzw::Lzw, lzw_buf: &mut Vec<u8>, rle_buf: &mut Vec<u8>) -> Result<bool> {
        match self.compression_method {
            CompressionMethod::Unpacked(_) => {
                // No decompression needed, just check CRC directly
                Ok(self.check_crc_and_size(decrypted))
            }
            CompressionMethod::RLE90 => {
                rle::unpack_rle_into(decrypted, rle_buf);
                Ok(self.check_crc_and_size(rle_buf))
            }
            CompressionMethod::Squeezed => {
                // Squeezed still allocates internally - not optimized yet
                let uncompressed = unsqueeze::unsqueeze(decrypted)?;
                Ok(self.check_crc_and_size(&uncompressed))
            }
            CompressionMethod::Crunched(_) => {
                lzw.decomp_into(decrypted, true, lzw_buf)?;
                rle::unpack_rle_into(lzw_buf, rle_buf);
                Ok(self.check_crc_and_size(rle_buf))
            }
            CompressionMethod::Squashed => {
                lzw.decomp_into(decrypted, false, lzw_buf)?;
                Ok(self.check_crc_and_size(lzw_buf))
            }
            CompressionMethod::Crushed => {
                // Crushed still allocates internally - not optimized yet
                let decompressed = crushed::decompress(decrypted)?;
                rle::unpack_rle_into(&decompressed, rle_buf);
                Ok(self.check_crc_and_size(rle_buf))
            }
            CompressionMethod::Distilled => {
                // Distilled still allocates internally - not optimized yet
                let uncompressed = distilled::decompress(decrypted)?;
                Ok(self.check_crc_and_size(&uncompressed))
            }
            CompressionMethod::Unknown(_) => Ok(false),
        }
    }

    /// Decompress data and verify CRC, reusing provided LZW instance.
    #[allow(dead_code)]
    fn decompress_and_verify_with_lzw(&self, decrypted: &[u8], lzw: &mut lzw::Lzw) -> Result<bool> {
        let uncompressed = match self.compression_method {
            CompressionMethod::Unpacked(_) => {
                // No decompression needed, just check CRC directly
                return Ok(self.check_crc(decrypted));
            }
            CompressionMethod::RLE90 => rle::unpack_rle(decrypted),
            CompressionMethod::Squeezed => unsqueeze::unsqueeze(decrypted)?,
            CompressionMethod::Crunched(_) => {
                let decompressed = lzw.decomp(decrypted, true)?;
                rle::unpack_rle(&decompressed)
            }
            CompressionMethod::Squashed => lzw.decomp(decrypted, false)?,
            CompressionMethod::Crushed => {
                let decompressed = crushed::decompress(decrypted)?;
                rle::unpack_rle(&decompressed)
            }
            CompressionMethod::Distilled => distilled::decompress(decrypted)?,
            CompressionMethod::Unknown(_) => return Ok(false),
        };

        Ok(self.check_crc(&uncompressed))
    }

    /// Decompress data and verify CRC.
    #[allow(dead_code)]
    fn decompress_and_verify(&self, decrypted: &[u8]) -> Result<bool> {
        let uncompressed = match self.compression_method {
            CompressionMethod::Unpacked(_) => {
                // No decompression needed, just check CRC directly
                return Ok(self.check_crc(decrypted));
            }
            CompressionMethod::RLE90 => rle::unpack_rle(decrypted),
            CompressionMethod::Squeezed => unsqueeze::unsqueeze(decrypted)?,
            CompressionMethod::Crunched(_) => {
                let decompressed = lzw::Lzw::new().decomp(decrypted, true)?;
                rle::unpack_rle(&decompressed)
            }
            CompressionMethod::Squashed => lzw::Lzw::new().decomp(decrypted, false)?,
            CompressionMethod::Crushed => {
                let decompressed = crushed::decompress(decrypted)?;
                rle::unpack_rle(&decompressed)
            }
            CompressionMethod::Distilled => distilled::decompress(decrypted)?,
            CompressionMethod::Unknown(_) => return Ok(false),
        };

        Ok(self.check_crc(&uncompressed))
    }

    /// Check if data matches expected CRC AND expected size.
    ///
    /// This is more robust than CRC alone - with CRC16 and many password attempts,
    /// there's a statistical probability of CRC collisions (~1 in 65536).
    /// Checking the size as well dramatically reduces false positives.
    #[inline]
    fn check_crc_and_size(&self, data: &[u8]) -> bool {
        // First check size - fast rejection for most wrong passwords
        if data.len() != self.original_size as usize {
            return false;
        }
        // Then check CRC
        let mut state = State::<ARC>::new();
        state.update(data);
        state.get() == self.expected_crc
    }

    /// Check if data matches expected CRC.
    #[inline]
    fn check_crc(&self, data: &[u8]) -> bool {
        let mut state = State::<ARC>::new();
        state.update(data);
        state.get() == self.expected_crc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_into() {
        let data = b"Hello";
        let password = b"key";
        let mut output = Vec::new();

        decrypt_into(data, password, &mut output);

        // XOR should be reversible
        let mut decrypted = Vec::new();
        decrypt_into(&output, password, &mut decrypted);

        assert_eq!(&decrypted, data);
    }

    #[test]
    fn test_verifier_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ArcPasswordVerifier>();
    }
}
