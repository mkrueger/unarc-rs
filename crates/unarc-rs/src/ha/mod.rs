//! HA Archive format support
//!
//! HA is a file archiver created by Harri Hirvola, first released in January 1993.
//! It is notable for its HSC (Harri's Statistical Compressor) compression method,
//! which uses PPM (Prediction by Partial Matching) with arithmetic coding.
//!
//! ## Compression Methods
//!
//! | Method | Code | Description |
//! |--------|------|-------------|
//! | CPY    | 0    | Store (no compression) |
//! | ASC    | 1    | LZ77 with arithmetic coding |
//! | HSC    | 2    | PPM with arithmetic coding |
//! | DIR    | 0xE  | Directory entry |
//! | SPECIAL| 0xF  | Special files (symlinks, devices, etc.) |
//!
//! ## Archive Format
//!
//! ```text
//! Offset  Size  Description
//! 0000    2     Magic "HA" (0x48 0x41)
//! 0002    2     Number of files in archive
//! 0004    ...   File headers and compressed data
//! ```
//!
//! ## Credits
//!
//! The original HA archiver was written by Harri Hirvola and released under
//! the GNU General Public License.
//!
//! This Rust re-implementation provides read-only support for HA archives.
//!

pub mod acoder;
pub mod asc;
pub mod ha_archive;
pub mod header;
pub mod hsc;
