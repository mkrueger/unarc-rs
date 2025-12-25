//! 7z archive support
//!
//! Uses the `sevenz-rust2` crate for decompression.

pub mod password_verifier;
pub mod sevenz_archive;

pub use password_verifier::SevenZPasswordVerifier;
