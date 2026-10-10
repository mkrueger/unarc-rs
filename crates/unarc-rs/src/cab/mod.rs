//! Microsoft Cabinet (CAB) archive support
//!
//! CAB is the archive format of Microsoft's setup tools (`makecab`, `cabarc`,
//! `iexpress`) and of Windows installers and updates. Files are stored in
//! *folders*: each folder is one compressed stream split into data blocks of at
//! most 32 KiB of uncompressed data, so a file's data is found by decoding its
//! folder up to the file's offset.
//!
//! ## Compression Methods
//!
//! | Method  | Type | Supported |
//! |---------|------|-----------|
//! | None    | 0    | Yes |
//! | MSZIP   | 1    | Yes (Deflate via flate2) |
//! | Quantum | 2    | No, reported as an unsupported method |
//! | LZX     | 3    | Yes, windows 2^15 to 2^21 (via lzxd) |
//!
//! ## Archive Format
//!
//! ```text
//! CFHEADER   "MSCF", sizes, counts, flags, optional reserve and set links
//! CFFOLDER[] first data block offset, block count, compression type
//! CFFILE[]   size, offset in folder, folder index, DOS date/time, attributes, name
//! CFDATA[]   checksum, compressed size, uncompressed size, data
//! ```
//!
//! Files that continue from or into another cabinet of a multi-cabinet set are
//! listed, but reading them fails. Self-extracting cabinets embedded in an
//! executable are not detected.

pub mod cab_archive;
pub mod header;
mod mszip;

pub use cab_archive::CabArchive;
pub use header::{CabFileHeader, CompressionMethod};
