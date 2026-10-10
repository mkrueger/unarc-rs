//! Read-only Amiga OFS/FFS filesystems in ADF and HDF images.
//!
//! See `doc/amiga.md` for supported layouts and limitations.
mod archive;

pub use archive::{AmigaArchive, AmigaEntry, AmigaEntryKind, AmigaVolume};
