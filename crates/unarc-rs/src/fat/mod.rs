//! Read-only FAT12 and FAT16 filesystems in PC and Atari ST disk images.
//!
//! See `doc/fat.md` for supported layouts and limitations.
mod archive;

pub use archive::{FatArchive, FatEntry, FatGeometry, FatType, MAX_PATH_BYTES};
