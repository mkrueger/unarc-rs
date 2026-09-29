//! RAR archive support
//!
//! Uses the pure Rust `rars` crate (RAR 1.3 through RAR 7).

pub mod password_verifier;
pub mod rar_archive;

pub use password_verifier::RarPasswordVerifier;
