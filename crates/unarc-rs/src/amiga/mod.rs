//! Read-only Amiga OFS/FFS filesystems in ADF and HDF images.
//!
//! See `doc/amiga.md` for supported layouts and limitations.
mod archive;

pub(crate) use archive::is_rdb_block;
pub use archive::{AmigaArchive, AmigaEntry, AmigaEntryKind, AmigaVolume, MAX_PATH_BYTES};
