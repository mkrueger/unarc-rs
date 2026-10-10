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
| **RAR** | `.rar` | Full support | AES ✓ | ✓ |
| **LHA/LZH** | `.lha`, `.lzh` | Full support | — | — |
| **TAR** | `.tar` | Full support | — | — |
| **CAB** | `.cab` | None, MSZIP, LZX (no Quantum) | — | — |
| **ACE** | `.ace` | Stored, LZ77, Blocked | Blowfish ✓ | ✓ |
| **ARJ** | `.arj` | Full support | Garble, GOST40 ✓ | ✓ |
| **ARC/PAK** | `.arc`, `.pak` | Full support | XOR ✓ | — |
| **ZOO** | `.zoo` | Full support | — | — |
| **HA** | `.ha` | Full support | — | — |
| **JAR (DOS)** | `.j` | m1–m4, solid, word/binary transforms | — | — |
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
unarc-rs = "0.7"
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

### Entry types and links

`ArchiveEntry::kind()` returns `ArchiveEntryKind::{File, Directory, SymbolicLink,
HardLink, Special, Unknown}`. `is_directory()` uses this classification; names
are unchanged. `link_target()` returns `Option<&str>` and never resolves or
validates the destination. Consumers extracting only regular files should skip
other kinds explicitly:

```rust
use unarc_rs::{ArchiveEntryKind, unified::ArchiveFormat};

let mut archive = ArchiveFormat::open_path("archive.tar")?;
while let Some(entry) = archive.next_entry()? {
    if entry.kind() != ArchiveEntryKind::File {
        archive.skip(&entry)?;
        continue;
    }
    let data = archive.read(&entry)?;
    // ... process regular-file data
}
```

TAR and compressed TAR expose symbolic/hard links, device/FIFO types, and extended
link names. ZIP and 7z use Unix file-type attributes; RAR uses Unix modes and RAR5
redirections (Windows junctions are symbolic links, file copies are regular
files). LHA uses Unix permissions and the encoded `name|target` destination.
ACE, ARJ, CAB, HA, HYP, JAR, SQZ and UC2 use their available directory/special flags.
Formats without a type field retain their regular-file/name-suffix fallback.
An unrecognized explicit type is `Unknown`, not a regular file.

ZIP/7z link targets require decoding the link payload while listing. Targets are
limited to 64 KiB and the configured entry/total limits. Missing passwords,
decoding failures or non-UTF-8 targets yield `None` without hiding the link kind.
RAR 1.5–4.x Unix symlinks are identified, but their payload targets are not
decoded during listing. 7z Windows reparse points without Unix type information
are `Unknown`; hard-link destinations are not exposed by the 7z backend.
HA special entries lack enough interpreted metadata to distinguish symlinks
from devices/FIFOs/sockets; ARJ's Unix-special type likewise combines those kinds.
Listing does not create or follow links.

### Size-limit boundaries

`ArchiveOptions::with_max_entry_size` and `with_max_total_size` reject oversized
recorded sizes before entry decoding. GZ/BZ2/Z streams and 7z entry output are
also bounded while decoding; compressed TAR is bounded while opening.
CAB output never exceeds the recorded size, which is checked before decoding.
ZIP output is bounded by its central-directory size.

These are decompressed-output limits, **not a process memory/CPU budget**.
ACE, ARC, ARJ, ZOO, SQ, SQZ, HA, HYP, JAR, UC2 and LHA may allocate/decode their
output before the unified API checks its actual size. RAR independent reads and
solid/multi-volume caches can also allocate other members before that check.
The 7z decoder still allocates its own dictionaries and may decode earlier
solid members, though those members' output is discarded rather than buffered.
CAB likewise decodes and discards the data preceding an entry in its folder;
reading a folder's entries out of order restarts the folder each time.
Input archives, headers, compressed buffers, and caches are not covered by the
total-output limit. Direct format-specific APIs do not inherit unified options.

ACE `skip()` only advances over metadata/data; it does not reconstruct a solid
dictionary. Reading a later solid member requires decoding its predecessors in
order (including stored members). Reopen and replay predecessors after skipping
them or seeking backwards. `AceArchive::is_solid()` recognizes the canonical
main-header solid flag; Huffman trees and distance history reset per member,
while previously decoded dictionary bytes remain available.

## Building

```bash
git clone https://github.com/mkrueger/unarc-rs
cd unarc-rs
cargo build --release
```

The CLI binary will be at `target/release/unarc`. Building requires Rust 1.95 or newer.

Benchmarks for the native decoders use [Criterion](https://crates.io/crates/criterion):

```bash
cargo bench -p unarc-rs            # all decoders
cargo bench -p unarc-rs -- uc2     # only UC2
```

## Background

This library was written for the [icy_board](https://github.com/mkrueger/icy_board) BBS project. It focuses on extraction (not creation) of legacy archive formats commonly found in BBS file areas.

Contributions welcome! Contact me on the icy_board repo or via email.

## Related Projects

- [ancient](https://github.com/temisu/ancient) - C++ decompression library for ancient formats

## License

MIT OR Apache-2.0
