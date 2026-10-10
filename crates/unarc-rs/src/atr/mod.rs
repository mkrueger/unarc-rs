//! Read-only Atari 8-bit ATR disk images with Atari DOS 2.x, MyDOS or SpartaDOS filesystems.
//!
//! See `doc/atr.md` for supported layouts and limitations.
mod archive;
mod dos2;
mod sparta;

pub use archive::{AtrArchive, AtrEntry, AtrFilesystem, MAX_PATH_BYTES};
