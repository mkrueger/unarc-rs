# unarc-rs

[![Crates.io](https://img.shields.io/crates/v/unarc-rs.svg)](https://crates.io/crates/unarc-rs)
[![License](https://img.shields.io/crates/l/unarc-rs.svg)](https://github.com/mkrueger/unarc-rs)

A Rust library for reading and extracting various archive formats, with a focus on legacy/retro formats from the BBS era, plus modern formats like 7z.

## Supported Formats

### Archive Formats

| Format | Extensions | Compression Support |
|--------|------------|---------------------|
| **7z** | `.7z` | Full support via sevenz-rust2 |
| **ZIP** | `.zip` | Full support via zip crate (including legacy methods) |
| **RAR** | `.rar` | Full support via unrar (RAR4 & RAR5) |
| **LHA/LZH** | `.lha`, `.lzh` | Full support via delharc |
| **TAR** | `.tar` | Full support via tar crate |
| **ACE** | `.ace` | ACE 1.0 (LZ77) and ACE 2.0 (Blocked) |
| **ARJ** | `.arj` | Store, Method 1-4 (full support) |
| **ARC** | `.arc` | Unpacked, Packed, Squeezed, Crunched, Squashed |
| **ZOO** | `.zoo` | Store, LZW, LH5 (full support) |
| **HA** | `.ha` | Store, ASC, HSC (full support) |
| **UC2** | `.uc2` | Full support |
| **SQ/SQ2** | `.sq`, `.sq2`, `.qqq`, `?q?` | Full support |
| **SQZ** | `.sqz` | Full support (stored + “Squeeze” methods 1–4) |
| **HYP** | `.hyp` | Full support |

### Single-File Compression

| Format | Extensions | Notes |
|--------|------------|-------|
| **Z** | `.Z` | Unix compress (LZW) |
| **GZ** | `.gz` | Gzip (Deflate) |
| **BZ2** | `.bz2` | Bzip2 |
| **ICE** | `.ice` | Ice |

### Compressed Archives

| Format | Extensions | Notes |
|--------|------------|-------|
| **TGZ** | `.tgz`, `.tar.gz` | Gzip-compressed TAR |
| **TBZ** | `.tbz`, `.tbz2`, `.tar.bz2` | Bzip2-compressed TAR |
| **TAR.Z** | `.tar.Z` | LZW-compressed TAR |

> **Note:** Single-file formats (`.Z`, `.gz`, `.bz2`) compress one file only. When a path like `file.tar.gz` is opened, the library detects it as a compressed TAR archive, returning all entries from the inner TAR.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
unarc-rs = "0.5"
```

## Quick Start

### Using the Unified API (Recommended)

```rust
use unarc_rs::unified::ArchiveFormat;

// Open archive directly from path
let mut archive = ArchiveFormat::open_path("archive.arj")?;

// Iterate over entries
for entry in archive.entries_iter() {
    let entry = entry?;
    println!("{}: {} bytes", entry.name(), entry.original_size());
}
```

### Extracting Files

```rust
use std::fs::File;
use unarc_rs::unified::ArchiveFormat;

let mut archive = ArchiveFormat::open_path("archive.zip")?;

while let Some(entry) = archive.next_entry()? {
    // Extract to a file
    let mut output = File::create(entry.file_name())?;
    archive.read_to(&entry, &mut output)?;
}
```

### Using a Specific Format

```rust
use std::fs::File;
use unarc_rs::unified::ArchiveFormat;

let file = File::open("data.arj")?;
let mut archive = ArchiveFormat::Arj.open(file)?;

for entry in archive.entries_iter() {
    let entry = entry?;
    let data = archive.read(&entry)?;
    // ... process data
}
```

### Format Detection

```rust
use std::path::Path;
use unarc_rs::unified::{ArchiveFormat, is_supported_archive};

// Check if a file is a supported archive
if is_supported_archive(Path::new("file.arj")) {
    println!("Supported!");
}

// Get format from path
if let Some(format) = ArchiveFormat::from_path(Path::new("archive.zoo")) {
    println!("Format: {}", format.name()); // "ZOO"
}
```

## API Overview

### `ArchiveFormat`

- `open_path(path)` - Open archive from file path (auto-detects format)
- `open(reader)` - Open archive from any `Read + Seek`
- `from_path(path)` - Detect format from path
- `name()` / `extension()` / `extensions()` - Format metadata

### `UnifiedArchive`

- `next_entry()` - Get next entry (returns `Option<ArchiveEntry>`)
- `entries_iter()` - Iterator over all entries
- `read(&entry)` - Read entry data into `Vec<u8>`
- `read_to(&entry, &mut writer)` - Stream entry data to writer
- `skip(&entry)` - Skip entry without reading

### `ArchiveEntry`

- `name()` / `file_name()` - Entry name (with/without path)
- `original_size()` / `compressed_size()` - Sizes
- `compression_method()` - Compression algorithm used
- `compression_ratio()` - Compression efficiency
- `modified_time()` - Modification timestamp
- `crc()` - Checksum
- `encryption()` - Encryption method used
- `is_encrypted()` - Whether the entry is encrypted

## Encryption Support

Several formats support encrypted archives:

| Format | Encryption Methods | Notes |
|--------|-------------------|-------|
| **7z** | AES-256 | Strong encryption |
| **ZIP** | ZipCrypto, AES-128/192/256 | ZipCrypto is weak |
| **RAR** | AES-128/256 | Legacy RAR encryption is weak |
| **ACE** | Blowfish | |
| **ARJ** | Garble, GOST40 | Both are weak by modern standards |

To decrypt, use `ArchiveOptions` with a password:

```rust
use unarc_rs::unified::{ArchiveFormat, ArchiveOptions};

let mut archive = ArchiveFormat::open_path("encrypted.arj")?;
let options = ArchiveOptions::new().with_password("secret");

while let Some(entry) = archive.next_entry()? {
    let data = archive.read_with_options(&entry, &options)?;
    // ... process decrypted data
}
```

## Format-Specific Notes

### 7z

Full support via the [sevenz-rust2](https://crates.io/crates/sevenz-rust2) crate. Supports LZMA, LZMA2, and other 7z compression methods. AES-256 encrypted archives are supported (password required).

### ZIP

Full support via the [zip](https://crates.io/crates/zip) crate with legacy compression methods enabled.

### RAR

Full support for RAR4 and RAR5 via the [unrar](https://crates.io/crates/unrar) crate (uses native unrar library).

### LHA/LZH

Full support via the excellent [delharc](https://crates.io/crates/delharc) crate.

### ACE

ACE archive support with LZ77+Huffman decompression. Supports both ACE 1.0 (LZ77 mode) and ACE 2.0 (Blocked mode). Blowfish encryption is supported (password required). Multi-volume archives are not supported.

https://github.com/droe/acefile


### ARC

Classic DOS archiver. Crushed & Distilled methods are not supported.

### ARJ

Popular in the BBS scene in the 90s. Supports Garble and GOST40 encryption (password required). Multi-volume archives are not supported.

### UC2

UltraCompressor II archive format. Supports decompression with SuperMaster dictionary and custom master entries.

### ZOO

Classic DOS/Amiga archiver from 1986. Native implementation supporting all compression methods.

### HA

Harri Hirvola's archiver (1993). Native implementation with ASC (arithmetic coding) and HSC (static Huffman) support.

### SQ/SQ2

CP/M and DOS "squeeze" format. Huffman-based compression used for single files, commonly seen as `?Q?` patterns (e.g., `.BQK` for `.BAS`).

### TAR Variants

TAR archives can be wrapped with compression. The library auto-detects `.tar.gz`, `.tar.bz2`, and `.tar.Z` from the file path and handles decompression transparently.

## Background

This library was written for my [icy_board](https://github.com/mkrueger/icy_board) BBS project. It focuses on extraction (not creation) of legacy archive formats.

Contributions welcome! Contact me on the icy_board repo or via email if I miss issues/PRs here.

## Related projects

https://github.com/temisu/ancient

## License

MIT OR Apache-2.0
