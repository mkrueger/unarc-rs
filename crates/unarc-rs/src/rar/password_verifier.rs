//! RAR Password Verifier - password testing for RAR archives.
//!
//! This module provides a `Send + Sync` password verifier that can be used with
//! rayon for parallel password testing.

use std::sync::Arc;

use super::rar_archive::{decode_members, parse_encrypted_headers, read_independent_member, RarFileHeader, RawVolume, Volumes};

/// A standalone password verifier for RAR archives.
///
/// This struct holds all the data needed to verify a password.
/// It is `Send + Sync` and can be safely used from multiple threads with rayon.
///
/// A password is verified by decoding the entry, which lets the RAR integrity
/// checks (CRC32 / BLAKE2 / password check values) reject wrong passwords.
#[derive(Clone)]
pub struct RarPasswordVerifier {
    volumes: Volumes,
    /// Raw volume bytes if the headers are encrypted; the header password is checked first
    header_data: Option<Vec<RawVolume>>,
    raw_names: Arc<Vec<Vec<u8>>>,
    header: RarFileHeader,
}

impl RarPasswordVerifier {
    pub(super) fn new(volumes: Volumes, header_data: Option<Vec<RawVolume>>, raw_names: Arc<Vec<Vec<u8>>>, header: RarFileHeader) -> Self {
        Self {
            volumes,
            header_data,
            raw_names,
            header,
        }
    }

    /// Get the entry name this verifier was created for.
    pub fn entry_name(&self) -> &str {
        &self.header.name
    }

    /// Get the original (uncompressed) size.
    pub fn original_size(&self) -> u64 {
        self.header.original_size
    }

    /// Verify if the given password is correct.
    ///
    /// Returns `true` if the password produces valid decompressed data
    /// with matching checksum and size, `false` otherwise.
    pub fn verify(&self, password: &str) -> bool {
        let reparsed: Vec<rars::Archive>;
        let volumes: &[rars::Archive] = match &self.header_data {
            Some(data) => match data.iter().map(|volume| parse_encrypted_headers(volume, password.as_bytes())).collect() {
                Ok(volumes) => {
                    reparsed = volumes;
                    &reparsed
                }
                Err(_) => return false,
            },
            None => &self.volumes,
        };
        let password = Some(password.as_bytes());
        let data = match read_independent_member(volumes, &self.header, password) {
            Ok(Some(data)) => data,
            Ok(None) => match decode_members(volumes, &self.raw_names, &self.header, password, true) {
                Ok(mut members) => members.remove(&self.header.index).unwrap_or_default(),
                Err(_) => return false,
            },
            Err(_) => return false,
        };
        data.len() as u64 == self.header.original_size
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
