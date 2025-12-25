//! ACE archive format support
//!
//! ACE is an archive format created by Marcel Lemke in the late 1990s.
//! It supports LZ77-based compression with Huffman encoding.
//!
//! This implementation is based on the acefile Python library by
//! Daniel Roethlisberger (BSD-2-Clause license).

mod ace_archive;
mod bitstream;
mod crc16;
mod crypto;
mod header;
mod huffman;
mod lz77;
mod password_verifier;

pub use ace_archive::AceArchive;
pub use crypto::decrypt_ace_data;
pub use header::FileHeader;
pub use password_verifier::AcePasswordVerifier;
