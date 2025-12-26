# unarc-rs

[![Crates.io](https://img.shields.io/crates/v/unarc-rs.svg)](https://crates.io/crates/unarc-rs)
[![License](https://img.shields.io/crates/l/unarc-rs.svg)](https://github.com/mkrueger/unarc-rs)

A Rust library for reading and extracting various archive formats, with a focus on legacy/retro formats from the BBS era, plus modern formats like 7z.

## Supported Formats

### Archive Formats

| Format | Extensions | Compression | Encryption | Multi-Volume |
| ------ | ---------- | ----------- | ---------- | ------------ |
| **7z** | `.7z` | LZMA, LZMA2, etc. | AES-256 ✓ | ✓ |
| **ZIP** | `.zip` | Deflate, legacy methods | ZipCrypto, AES ✓ | ✓ |
| **RAR** | `.rar` | RAR4 & RAR5 | AES ✓ | — |
| **LHA/LZH** | `.lha`, `.lzh` | Full support | — | — |
| **TAR** | `.tar` | Full support | — | — |
| **ACE** | `.ace` | Stored, LZ77, Blocked | Blowfish ✓ | ✓ |
| **ARJ** | `.arj` | Full support | Garble, GOST40 ✓ | ✓ |
| **ARC/PAK** | `.arc`, `.pak` | Full support | XOR ✓ | — |
| **ZOO** | `.zoo` | Full support | — | — |
| **HA** | `.ha` | Full support | — | — |
| **UC2** | `.uc2` | Full support | — | — |
| **SQ/SQ2** | `.sq`, `.sq2`, `.qqq`, `?q?` | Full support | — | — |
| **SQZ** | `.sqz` | Full support | — | — |
| **HYP** | `.hyp` | Full support | — | — |

### Single-File Compression

| Format | Extensions | Notes |
| ------ | ---------- | ----- |
| **Z** | `.Z` | Full support |
| **GZ** | `.gz` | Gzip (Deflate) |
| **BZ2** | `.bz2` | Bzip2 |
| **ICE** | `.ice` | Full support |
| **Pack-Ice** | `.pi9` | Full support |

### Compressed Archives

| Format | Extensions | Notes |
| ------ | ---------- | ----- |
| **TGZ** | `.tgz`, `.tar.gz` | Gzip-compressed TAR |
| **TBZ** | `.tbz`, `.tbz2`, `.tar.bz2` | Bzip2-compressed TAR |
| **TAR.Z** | `.tar.Z` | LZW-compressed TAR |

> **Note:** Single-file formats (`.Z`, `.gz`, `.bz2`) compress one file only. When a path like `file.tar.gz` is opened, the library detects it as a compressed TAR archive, returning all entries from the inner TAR.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
unarc-rs = "0.6"
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
- `open_multi_volume_zip(paths, options)` - Open multi-volume ZIP archive
- `open_multi_volume_7z(paths, options)` - Open multi-volume 7z archive
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
| ------ | ------------------ | ----- |
| **7z** | AES-256 | Strong encryption |
| **ZIP** | ZipCrypto, AES-128/192/256 | ZipCrypto is weak |
| **RAR** | AES-128/256 | Legacy RAR encryption is weak |
| **ACE** | Blowfish | |
| **ARJ** | Garble, GOST40, GOST-256 | GOST-256 requires ARJCRYPT and is not supported |
| **ARC/PAK** | XOR (password) | No reliable encryption flag; wrong password typically yields CRC errors |
| **UC2** | UltraCrypt (UE2) | Detected but not supported (decrypt with UCRYPT first) |

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

**Multi-volume support:** Split 7z archives (`.7z.001`, `.7z.002`, etc.) are supported via `open_multi_volume_7z()`.

### ZIP

Full support via the [zip](https://crates.io/crates/zip) crate with legacy compression methods enabled.

**Multi-volume support:** Split ZIP archives (`.zip.001`, `.z01`/`.z02`/`.zip`, etc.) are supported via `open_multi_volume_zip()`.

### RAR

Full support for RAR4 and RAR5 via the [unrar](https://crates.io/crates/unrar) crate (uses native unrar library).

### LHA/LZH

Full support via the excellent [delharc](https://crates.io/crates/delharc) crate.

### ACE

ACE archive support with LZ77+Huffman decompression. Supports both ACE 1.0 (LZ77 mode) and ACE 2.0 (Blocked mode). Blowfish encryption is supported (password required). Multi-volume archives are not supported.

See <https://github.com/droe/acefile> for format documentation.

### ARC

Classic DOS archiver. Full support for all compression methods including Crushed (Method 10) and Distilled (Method 11).

ARC/PAK can be password-decrypted (simple XOR), but the format has no reliable encryption flag in headers.

### ARJ

Popular in the BBS scene in the 90s. Supports Garble and GOST40 encryption (password required). GOST-256 (ARJCRYPT) is detected but not supported. Multi-volume archives are not supported.

### UC2

UltraCompressor II archive format. Supports decompression with SuperMaster dictionary and custom master entries.

UltraCrypt (UE2) encrypted wrappers are detected but not supported.

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

- [ancient](https://github.com/temisu/ancient) - Decompression of many formats

## License

MIT OR Apache-2.0
