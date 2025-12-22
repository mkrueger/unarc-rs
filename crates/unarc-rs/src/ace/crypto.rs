//! ACE archive decryption using Blowfish
//!
//! ACE uses Blowfish in CBC mode for encryption.
//! The key is derived from the password using SHA-1.

use blowfish::cipher::{BlockDecrypt, KeyInit};
use blowfish::Blowfish;
use sha1::{Digest, Sha1};

/// Blowfish block size in bytes
const BLOCK_SIZE: usize = 8;

/// Derive a Blowfish key from a password using SHA-1
///
/// ACE uses SHA-1 to hash the password and uses the first 16 bytes as the key.
pub fn derive_key(password: &str) -> [u8; 16] {
    let mut hasher = Sha1::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();

    let mut key = [0u8; 16];
    key.copy_from_slice(&result[..16]);
    key
}

/// Decrypt data using Blowfish in CBC mode
///
/// ACE uses a zero IV for CBC mode.
pub fn decrypt_cbc(data: &[u8], key: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    // ACE uses big-endian Blowfish
    let cipher: Blowfish<byteorder::BigEndian> =
        Blowfish::new_from_slice(key).expect("Invalid key length");

    let mut result = data.to_vec();
    let mut prev_block = [0u8; BLOCK_SIZE]; // Zero IV

    // Process each block
    for chunk in result.chunks_exact_mut(BLOCK_SIZE) {
        let encrypted_block: [u8; BLOCK_SIZE] = chunk.try_into().unwrap();

        // Decrypt block
        let mut block = blowfish::cipher::generic_array::GenericArray::clone_from_slice(chunk);
        cipher.decrypt_block(&mut block);

        // XOR with previous ciphertext (CBC mode)
        for (i, byte) in chunk.iter_mut().enumerate() {
            *byte = block[i] ^ prev_block[i];
        }

        prev_block = encrypted_block;
    }

    result
}

/// Decrypt ACE file data
///
/// Decrypts the compressed data and returns the decrypted bytes.
pub fn decrypt_ace_data(encrypted_data: &[u8], password: &str) -> Vec<u8> {
    let key = derive_key(password);
    decrypt_cbc(encrypted_data, &key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key() {
        let key = derive_key("test");
        assert_eq!(key.len(), 16);
        // SHA-1 of "test" starts with a9993e36...
        assert_eq!(key[0], 0xa9);
    }

    #[test]
    fn test_decrypt_empty() {
        let result = decrypt_ace_data(&[], "test");
        assert!(result.is_empty());
    }
}
