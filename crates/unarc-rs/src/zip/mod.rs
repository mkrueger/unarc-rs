//! ZIP archive support
//!
//! Uses the `zip` crate for decompression.

pub mod multi_volume;
pub mod password_verifier;
pub mod zip_archive;

pub use multi_volume::MultiVolumeReader;
pub use password_verifier::ZipPasswordVerifier;
