//! RAR archive support
//!
//! Uses the `rar` crate for parsing RAR5 archives.

pub mod password_verifier;
pub mod rar_archive;

pub use password_verifier::RarPasswordVerifier;
