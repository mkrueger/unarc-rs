// SPDX-License-Identifier: MIT OR Apache-2.0
//! ARJ encryption/decryption module
//!
//! This module provides decryption for ARJ encrypted archives.
//!
//! ## Supported Encryption Types
//!
//! - **Garble**: Simple XOR-based cipher (very weak, trivially broken)
//! - **GOST40**: GOST 28147-89 with 40-bit key (export-restricted)
//! - **GOST256**: Full GOST 28147-89 (requires external module, not supported)
//!
//! ## Implementation Notes
//!
//! This is a clean-room implementation based on publicly available standards:
//!
//! - **GOST 28147-89**: RFC 5830, RFC 8891, Wikipedia
//!   - Soviet/Russian government block cipher standard
//!   - 64-bit block size, 256-bit key (ARJ restricts to 40 bits)
//!   - Feistel network with 32 rounds
//!   - 8 substitution boxes (S-boxes), each mapping 4 bits to 4 bits
//!   - Round function: add subkey mod 2^32 → apply S-boxes → rotate left 11 bits
//!
//! - **Garble**: Publicly documented in ARJ file format specifications
//!   - Simple XOR with (modifier + password_byte)
//!   - No cryptographic security
//!
//! ## References
//!
//! - RFC 5830: GOST 28147-89 Encryption, Decryption, and MAC Algorithms
//! - RFC 8891: GOST R 34.12-2015 Block Cipher "Magma"
//! - GOST R 34.12-2015: Russian Federation national standard

use crate::encryption::ArjEncryption;

// ============================================================================
// GOST 28147-89 S-boxes
// ============================================================================
//
// These are the "id-Gost28147-89-CryptoPro-A-ParamSet" S-boxes.
// S-boxes are publicly specified in RFC 4357 and various GOST standards.
// Each S-box maps a 4-bit input to a 4-bit output.

/// GOST 28147-89 S-boxes (CryptoPro-A parameter set)
/// Source: RFC 4357, Section 11.2
const SBOX: [[u8; 16]; 8] = [
    [0x01, 0x0F, 0x0D, 0x00, 0x05, 0x07, 0x0A, 0x04, 0x09, 0x02, 0x03, 0x0E, 0x06, 0x0B, 0x08, 0x0C],
    [0x0D, 0x0B, 0x04, 0x01, 0x03, 0x0F, 0x05, 0x09, 0x00, 0x0A, 0x0E, 0x07, 0x06, 0x08, 0x02, 0x0C],
    [0x04, 0x0B, 0x0A, 0x00, 0x07, 0x02, 0x01, 0x0D, 0x03, 0x06, 0x08, 0x05, 0x09, 0x0C, 0x0F, 0x0E],
    [0x06, 0x0C, 0x07, 0x01, 0x05, 0x0F, 0x0D, 0x08, 0x04, 0x0A, 0x09, 0x0E, 0x00, 0x03, 0x0B, 0x02],
    [0x07, 0x0D, 0x0A, 0x01, 0x00, 0x08, 0x09, 0x0F, 0x0E, 0x04, 0x06, 0x0C, 0x0B, 0x02, 0x05, 0x03],
    [0x05, 0x08, 0x01, 0x0D, 0x0A, 0x03, 0x04, 0x02, 0x0E, 0x0F, 0x0C, 0x07, 0x06, 0x00, 0x09, 0x0B],
    [0x0E, 0x0B, 0x04, 0x0C, 0x06, 0x0D, 0x0F, 0x0A, 0x02, 0x03, 0x08, 0x01, 0x00, 0x07, 0x05, 0x09],
    [0x04, 0x0A, 0x09, 0x02, 0x0D, 0x08, 0x00, 0x0E, 0x06, 0x0B, 0x01, 0x0C, 0x07, 0x0F, 0x05, 0x03],
];

// ============================================================================
// GOST Constants
// ============================================================================

/// GOST round function rotation amount (specified in standard)
const GOST_ROTATION: u32 = 11;

/// Number of key derivation iterations for key strengthening
const KEY_ITERATIONS: usize = 2048;

/// Initial key for bootstrapping key derivation
/// This allows the cipher to operate before the actual key is established
const BOOTSTRAP_KEY: [u32; 8] = [3, 10, 6, 12, 5, 9, 0, 7];

// ============================================================================
// GOST 28147-89 Implementation
// ============================================================================

/// Expanded S-box lookup table for optimized computation
///
/// Standard GOST optimization: combine pairs of 4-bit S-boxes into
/// 8-bit lookup tables, reducing the number of table lookups from 8 to 4.
struct SboxTable {
    table: [[u8; 256]; 4],
}

impl SboxTable {
    /// Create expanded S-box lookup table from the 8 small S-boxes
    fn new() -> Self {
        let mut table = [[0u8; 256]; 4];
        for i in 0..256 {
            // Combine pairs of 4-bit S-boxes: high nibble from even, low from odd
            table[0][i] = (SBOX[0][i >> 4] << 4) | SBOX[1][i & 0x0F];
            table[1][i] = (SBOX[2][i >> 4] << 4) | SBOX[3][i & 0x0F];
            table[2][i] = (SBOX[4][i >> 4] << 4) | SBOX[5][i & 0x0F];
            table[3][i] = (SBOX[6][i >> 4] << 4) | SBOX[7][i & 0x0F];
        }
        SboxTable { table }
    }

    /// GOST round function: S-box substitution + rotation
    ///
    /// Per GOST 28147-89 / RFC 5830:
    /// 1. Split 32-bit value into 8 nibbles (4 bits each)
    /// 2. Apply corresponding S-box to each nibble
    /// 3. Rotate the result left by 11 bits
    fn apply(&self, data: u32) -> u32 {
        let bytes = data.to_le_bytes();

        // Apply expanded S-boxes (2 nibbles per lookup)
        let s0 = self.table[3][bytes[0] as usize];
        let s1 = self.table[2][bytes[1] as usize];
        let s2 = self.table[1][bytes[2] as usize];
        let s3 = self.table[0][bytes[3] as usize];

        // Combine into 32-bit result
        let p1 = ((s3 as u16) << 8) | (s2 as u16);
        let p2 = ((s1 as u16) << 8) | (s0 as u16);

        // Rotate left by 11 bits (split across 16-bit halves)
        let hi = (p1 << GOST_ROTATION as u16) | (p2 >> (16 - GOST_ROTATION as u16));
        let lo = (p2 << GOST_ROTATION as u16) | (p1 >> (16 - GOST_ROTATION as u16));

        ((hi as u32) << 16) | (lo as u32)
    }
}

/// GOST 28147-89 block cipher with 40-bit key
///
/// This implements GOST 28147-89 as specified in RFC 5830,
/// but with the key limited to 40 bits for export compliance.
///
/// The cipher is used in a CFB-like streaming mode for ARJ archives.
pub struct Gost40 {
    /// Expanded S-box lookup table
    sbox: SboxTable,
    /// 256-bit key as 8 × 32-bit subkeys (only 40 bits used)
    subkeys: [u32; 8],
    /// Feedback register for CFB mode (64 bits = 2 × 32 bits)
    feedback: [u32; 2],
    /// Current byte position within 8-byte block (for unaligned data)
    byte_offset: usize,
}

impl Gost40 {
    /// Create a new GOST40 cipher for ARJ decryption
    ///
    /// # Arguments
    /// * `password` - Encryption password
    /// * `modifier` - Modifier byte from ARJ file header
    /// * `file_time` - File timestamp from ARJ header (used as IV)
    pub fn new(password: &str, modifier: u8, file_time: u32) -> Self {
        let sbox = SboxTable::new();

        // Build 40-bit key from password
        // Only 5 bytes (40 bits) are used for export compliance
        let initial_key = Self::build_key_from_password(password);

        // Initialization vector from header fields
        let iv: [u32; 2] = [file_time, modifier as i8 as i32 as u32];

        let mut cipher = Gost40 {
            sbox,
            subkeys: initial_key,
            feedback: [0, 0],
            byte_offset: 0,
        };

        // Strengthen the key through iterations
        cipher.derive_key(&iv);

        // Initialize feedback register for CFB mode
        cipher.feedback = cipher.encrypt_block(&iv);

        cipher
    }

    /// Build 40-bit key from password string
    ///
    /// The key is limited to 40 bits (5 bytes) for export compliance.
    /// Password bytes are accumulated with bit shifting for mixing.
    fn build_key_from_password(password: &str) -> [u32; 8] {
        let mut key = [0u32; 8];
        let key_bytes: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(key.as_mut_ptr() as *mut u8, 32) };

        let pwd_bytes = password.as_bytes();
        if pwd_bytes.is_empty() {
            return key;
        }

        let mut pwd_idx = 0;
        for round in 0..64 {
            // Only use first 5 bytes of key (40 bits)
            let byte_idx = round % 5;
            key_bytes[byte_idx] = key_bytes[byte_idx].wrapping_add(pwd_bytes[pwd_idx] << (round % 7));
            pwd_idx += 1;
            if pwd_idx >= pwd_bytes.len() {
                pwd_idx = 0;
            }
        }

        key
    }

    /// GOST 28147-89 encryption of a 64-bit block
    ///
    /// Implements the standard GOST Feistel network:
    /// - 32 rounds total
    /// - Rounds 1-24: subkeys used in order 0,1,2,3,4,5,6,7 (3 times)
    /// - Rounds 25-32: subkeys used in reverse order 7,6,5,4,3,2,1,0
    ///
    /// Round function: output = S-box(input + subkey) rotated left 11 bits
    fn encrypt_block(&self, block: &[u32; 2]) -> [u32; 2] {
        let mut left = block[0];
        let mut right = block[1];

        // Rounds 1-24: forward key schedule (3 iterations)
        for _ in 0..3 {
            for i in 0..8 {
                let round_output = self.sbox.apply(left.wrapping_add(self.subkeys[i]));
                let temp = right ^ round_output;
                right = left;
                left = temp;
            }
        }

        // Rounds 25-32: reverse key schedule (1 iteration)
        for i in (0..8).rev() {
            let round_output = self.sbox.apply(left.wrapping_add(self.subkeys[i]));
            let temp = right ^ round_output;
            right = left;
            left = temp;
        }

        // Final output (swapped)
        [right, left]
    }

    /// Key derivation function
    ///
    /// Strengthens a weak 40-bit key by iteratively encrypting
    /// the key material. This is a form of key stretching.
    fn derive_key(&mut self, iv: &[u32; 2]) {
        let mut work_key = self.subkeys;

        // Bootstrap: encrypt IV with initial bootstrap key
        let saved_keys = self.subkeys;
        self.subkeys = BOOTSTRAP_KEY;
        self.feedback = self.encrypt_block(iv);
        self.subkeys = saved_keys;

        // Key strengthening: repeatedly encrypt the key material
        for _ in 0..KEY_ITERATIONS {
            let work_bytes: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(work_key.as_mut_ptr() as *mut u8, 32) };
            self.cfb_encrypt(work_bytes);
        }

        self.subkeys = work_key;
    }

    /// CFB-mode encryption (used for key derivation)
    fn cfb_encrypt(&mut self, data: &mut [u8]) {
        for chunk in data.chunks_mut(8) {
            self.feedback = self.encrypt_block(&self.feedback);

            let fb_bytes: &[u8] = unsafe { std::slice::from_raw_parts(self.feedback.as_ptr() as *const u8, 8) };

            for (i, byte) in chunk.iter_mut().enumerate() {
                let encrypted = *byte ^ fb_bytes[i];
                *byte = encrypted;
                // CFB: feed ciphertext back
                unsafe {
                    let fb_ptr = self.feedback.as_mut_ptr() as *mut u8;
                    *fb_ptr.add(i) = encrypted;
                }
            }
        }
    }

    /// Decrypt data in CFB mode
    ///
    /// CFB decryption: plaintext = ciphertext XOR keystream
    /// The keystream is generated by encrypting the feedback register,
    /// then the feedback is updated with the ciphertext.
    pub fn decrypt(&mut self, data: &mut [u8]) {
        let len = data.len();

        // Fast path: 8-byte aligned blocks
        if len.is_multiple_of(8) && self.byte_offset == 0 {
            for chunk in data.chunks_mut(8) {
                // Generate keystream
                self.feedback = self.encrypt_block(&self.feedback);

                // Read ciphertext
                let ct_lo = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                let ct_hi = u32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);

                // Decrypt: XOR ciphertext with keystream
                let pt_lo = ct_lo ^ self.feedback[0];
                let pt_hi = ct_hi ^ self.feedback[1];

                // Update feedback with ciphertext (CFB mode)
                self.feedback = [ct_lo, ct_hi];

                // Write plaintext
                chunk[0..4].copy_from_slice(&pt_lo.to_le_bytes());
                chunk[4..8].copy_from_slice(&pt_hi.to_le_bytes());
            }
        } else {
            // Slow path: byte-by-byte for unaligned data
            for byte in data.iter_mut() {
                if self.byte_offset == 0 {
                    self.feedback = self.encrypt_block(&self.feedback);
                }

                let fb_bytes: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(self.feedback.as_mut_ptr() as *mut u8, 8) };

                let ciphertext = *byte;
                *byte = ciphertext ^ fb_bytes[self.byte_offset];
                fb_bytes[self.byte_offset] = ciphertext;

                self.byte_offset = (self.byte_offset + 1) % 8;
            }
        }
    }
}

// ============================================================================
// Garble Cipher (legacy XOR encryption)
// ============================================================================

/// Garble decryption - ARJ's legacy XOR cipher
///
/// This is an extremely weak cipher that provides no real security.
/// Each byte is XORed with (modifier + password_byte), cycling through
/// the password characters.
///
/// The algorithm is trivially reversible and offers no protection against
/// even basic cryptanalysis.
///
/// # Arguments
/// * `data` - Data to decrypt (in place)
/// * `password` - The encryption password
/// * `modifier` - Modifier byte from header
pub fn garble_decrypt(data: &mut [u8], password: &str, modifier: u8) {
    if password.is_empty() {
        return;
    }

    let pwd_bytes = password.as_bytes();
    let mut pwd_idx = 0;

    for byte in data.iter_mut() {
        *byte ^= modifier.wrapping_add(pwd_bytes[pwd_idx]);
        pwd_idx += 1;
        if pwd_idx >= pwd_bytes.len() {
            pwd_idx = 0;
        }
    }
}

// ============================================================================
// Public API
// ============================================================================

/// Decrypt ARJ data based on encryption type
///
/// Dispatches to the appropriate decryption algorithm based on the
/// encryption type specified in the ARJ header.
///
/// # Arguments
/// * `data` - Compressed data to decrypt (modified in place)
/// * `encryption_type` - Type of encryption (None = no encryption)
/// * `password` - The encryption password
/// * `modifier` - Modifier byte from ARJ header
/// * `file_time` - File timestamp (used as IV for GOST40)
pub fn decrypt_arj_data(data: &mut [u8], encryption_type: Option<ArjEncryption>, password: &str, modifier: u8, file_time: u32) {
    match encryption_type {
        None => {}
        Some(ArjEncryption::Garble) => {
            garble_decrypt(data, password, modifier);
        }
        Some(ArjEncryption::Gost40) => {
            let mut gost = Gost40::new(password, modifier, file_time);
            gost.decrypt(data);
        }
        Some(ArjEncryption::Gost256) | Some(ArjEncryption::Unknown) => {
            // GOST256 requires external ARJCRYPT module - cannot decrypt
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_garble_roundtrip() {
        let original = b"Hello, World! This is a test.";
        let mut data = original.to_vec();
        let password = "secret";
        let modifier = 0x42;

        // "Encrypt" with XOR
        garble_decrypt(&mut data, password, modifier);
        assert_ne!(&data[..], &original[..]);

        // "Decrypt" with XOR (same operation)
        garble_decrypt(&mut data, password, modifier);
        assert_eq!(&data[..], &original[..]);
    }

    #[test]
    fn test_sbox_expansion() {
        let sbox = SboxTable::new();
        // Verify the expanded table produces consistent results
        // First entry combines SBOX[0][0] and SBOX[1][0]
        assert_eq!(sbox.table[0][0], (SBOX[0][0] << 4) | SBOX[1][0]);
    }

    #[test]
    fn test_encryption_type_from_version() {
        assert_eq!(ArjEncryption::from_version(0, true), Some(ArjEncryption::Garble));
        assert_eq!(ArjEncryption::from_version(1, true), Some(ArjEncryption::Garble));
        assert_eq!(ArjEncryption::from_version(2, true), Some(ArjEncryption::Gost40));
        assert_eq!(ArjEncryption::from_version(0, false), None);
    }
}
