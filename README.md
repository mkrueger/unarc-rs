# unarc-rs

[![Crates.io](https://img.shields.io/crates/v/unarc-rs.svg)](https://crates.io/crates/unarc-rs)
[![License](https://img.shields.io/crates/l/unarc-rs.svg)](https://github.com/mkrueger/unarc-rs)

A Rust library for reading and extracting various archive formats, with a focus on legacy/retro formats from the BBS era, plus modern formats like 7z.

## Supported Formats

| Format | Extensions | Compression Support |
|--------|------------|---------------------|
| **7z** | `.7z` | Full support via sevenz-rust2 |
| **ZIP** | `.zip` | Full support via zip crate (including legacy methods) |
| **RAR** | `.rar` | Full support via unrar (RAR4 & RAR5) |
| **LHA/LZH** | `.lha`, `.lzh` | Full support via delharc |
| **ARJ** | `.arj` | Store, Method 1-4 |
| **ARC** | `.arc` | Unpacked, Packed, Squeezed, Crunched, Squashed |
| **ZOO** | `.zoo` | Methods 0, 1, 2 |
| **UC2** | `.uc2` | Full LZ77+Huffman decompression |
| **SQ/SQ2** | `.sq`, `.sq2`, `.qqq`, `?q?` | Squeezed |
| **SQZ** | `.sqz` | Store only |
| **HYP** | `.hyp` | Store only |
| **Z** | `.Z` | LZW (Unix compress) |

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
- `from_path(path)` / `from_extension(ext)` - Detect format
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

## Format-Specific Notes

### 7z

Full support via the [sevenz-rust2](https://crates.io/crates/sevenz-rust2) crate. Supports LZMA, LZMA2, and other 7z compression methods.

### ZIP

Full support via the [zip](https://crates.io/crates/zip) crate with legacy compression methods enabled.

### RAR

Full support for RAR4 and RAR5 via the [unrar](https://crates.io/crates/unrar) crate (uses native unrar library).

### LHA/LZH

Full support via the excellent [delharc](https://crates.io/crates/delharc) crate.

### ARC

Classic DOS archiver. Crushed & Distilled methods are not supported.

### ARJ

Popular in the BBS scene in the 90s. Multi-volume and encrypted archives are not supported.

### UC2

UltraCompressor II archive format. Supports decompression with SuperMaster dictionary and custom master entries.

## Out of Scope

Not on the todo list:

- **TAR** - Use [tar](https://crates.io/crates/tar)

## Background

This library was written for my [icy_board](https://github.com/mkrueger/icy_board) BBS project. It focuses on extraction (not creation) of legacy archive formats.

Contributions welcome! Contact me on the icy_board repo or via email if I miss issues/PRs here.

## License

MIT OR Apache-2.0
