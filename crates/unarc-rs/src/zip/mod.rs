//! ZIP archive support
//!
//! Uses the `zip` crate for decompression.

pub mod password_verifier;
pub mod zip_archive;

pub use password_verifier::ZipPasswordVerifier;
