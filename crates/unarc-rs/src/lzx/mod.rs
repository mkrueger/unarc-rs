//! Amiga LZX archive support
//!
//! LZX is the Amiga archiver by Jonathan Forbes and Tomi Poutanen (1995). Its
//! compression is the ancestor of the LZX method in Microsoft Cabinet files, but
//! the bitstream differs (bit order, block types, tree encoding, a single repeated
//! offset, a fixed 64 KiB window), so it has its own decoder here; CAB's LZX
//! decoder (`lzxd`) cannot read it.
//!
//! ## Pack modes
//!
//! | Mode | Name   | Supported |
//! |------|--------|-----------|
//! | 0    | Stored | Yes |
//! | 2    | Normal | Yes (LZX) |
//!
//! Other modes are listed, but reading them returns
//! [`ArchiveError::UnsupportedMethod`](crate::error::ArchiveError::UnsupportedMethod).
//!
//! ## Archive format
//!
//! ```text
//! "LZX" info header (10 bytes)
//! entry header (31 bytes), file name, comment, packed data
//! ...
//! ```
//!
//! Several files can be compressed as one stream (a *merged group*): every entry
//! of the group except the last records a packed size of 0, and the last records
//! the size of the whole stream, which follows its header. Each file still has its
//! own CRC-32. See `doc/lzx.md` for the full format description.
//!
//! The decoder was written from the format description in `doc/lzx.md`, which is
//! based on the specification documents of the
//! [amiga-lzx](https://github.com/bitplane/amiga-lzx) project.

pub(crate) mod decoder;
pub mod lzx_archive;

pub use lzx_archive::{LzxArchive, LzxEntry, PackMode};
