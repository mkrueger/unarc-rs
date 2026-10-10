//! Read-only FAT12 filesystems in PC and Atari ST floppy-disk images.
//!
//! See `doc/fat.md` for supported layouts and limitations.
mod archive;

pub use archive::{FatArchive, FatEntry, FatGeometry, MAX_PATH_BYTES};
