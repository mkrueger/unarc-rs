//! DMS (DiskMasher) compressed Amiga floppy images.
//!
//! The disk is decoded in memory and, when it carries an AmigaDOS filesystem, listed
//! through the [`amiga`](crate::amiga) reader. Attached FILE_ID.DIZ and banner texts
//! are listed as extra entries. See `doc/dms.md` for details and limitations.
//!
//! The decompressors follow xDMS 1.3.2 by Andre Rodrigues de la Rocha and Heikki
//! Orsila, which is public domain ("xDMS is released as public domain software. You
//! can spread it, modify it and use it in any way you like."). xDMS credits its DEEP
//! mode to LZHUF by Haruyasu Yoshizaki and its HEAVY modes to UNIX LHA by Masaru Oki;
//! the HEAVY Huffman tables here are built with an independent canonical decoder.
mod archive;
mod unpack;

pub use archive::{DmsArchive, DmsEntry, DmsEntryKind, DmsInfo};
