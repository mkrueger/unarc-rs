# unarc-rs

[![Crates.io](https://img.shields.io/crates/v/unarc-rs.svg)](https://crates.io/crates/unarc-rs)
[![License](https://img.shields.io/crates/l/unarc-rs.svg)](https://github.com/mkrueger/unarc-rs)

A Rust library and CLI tool for reading and extracting various archive formats, with a focus on legacy/retro formats from the BBS era, plus modern formats like 7z.

## Crates

This workspace contains two crates:

| Crate | Description |
| ----- | ----------- |
| [**unarc-rs**](crates/unarc-rs) | Library for reading archive formats |
| [**unarc-cli**](crates/unarc-cli) | Command-line tool (`unarc`) |

Install the CLI tool:

```bash
cargo install unarc-cli
```

## Supported Formats

### Archive Formats

| Format | Extensions | Compression | Encryption | Multi-Volume |
| ------ | ---------- | ----------- | ---------- | ------------ |
| **7z** | `.7z` | Full support | AES-256 ✓ | ✓ |
| **ZIP** | `.zip` | Full support | ZipCrypto, AES ✓ | ✓ |
| **RAR** | `.rar` | Full support | AES ✓ | — |
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
| **Z** | `.Z` | Unix compress (LZW) |
| **GZ** | `.gz` | Gzip (Deflate) |
| **BZ2** | `.bz2` | Bzip2 |
| **ICE** | `.ice` | Legacy DOS ICE (LH1) |
| **Pack-Ice** | `.pi9` | Atari ST Pack-Ice (v0/v1/v2) |

### Compressed Archives

| Format | Extensions |
| ------ | ---------- |
| **TGZ** | `.tgz`, `.tar.gz` |
| **TBZ** | `.tbz`, `.tar.bz2` |
| **TAR.Z** | `.tar.Z` |

## Quick Start

### CLI Tool

```bash
# Install
cargo install unarc-cli

# List archive contents
unarc list archive.arj

# Extract files
unarc extract archive.zip -o ./output

# Extract encrypted archive
unarc extract -p secret encrypted.arj

# Extract multi-volume archive (auto-detects all volumes)
unarc extract archive.zip.001 -o ./output
unarc extract archive.7z.001 -o ./output
```

### Library

Add to your `Cargo.toml`:

```toml
[dependencies]
unarc-rs = "0.6"
```

```rust
use unarc_rs::unified::ArchiveFormat;

// Open and iterate
let mut archive = ArchiveFormat::open_path("archive.arj")?;

while let Some(entry) = archive.next_entry()? {
    println!("{}: {} bytes", entry.name(), entry.original_size());
    let data = archive.read(&entry)?;
    // ... process data
}
```

### Encrypted Archives

```rust
use unarc_rs::unified::{ArchiveFormat, ArchiveOptions};

let mut archive = ArchiveFormat::open_path("encrypted.arj")?;
let options = ArchiveOptions::new().with_password("secret");

while let Some(entry) = archive.next_entry()? {
    let data = archive.read_with_options(&entry, &options)?;
    // ... process decrypted data
}
```

## Building

```bash
git clone https://github.com/mkrueger/unarc-rs
cd unarc-rs
cargo build --release
```

The CLI binary will be at `target/release/unarc`.

## Background

This library was written for the [icy_board](https://github.com/mkrueger/icy_board) BBS project. It focuses on extraction (not creation) of legacy archive formats commonly found in BBS file areas.

Contributions welcome! Contact me on the icy_board repo or via email.

## Related Projects

- [ancient](https://github.com/temisu/ancient) - C++ decompression library for ancient formats

## License

MIT OR Apache-2.0
