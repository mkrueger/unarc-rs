# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Breaking changes

- `ArchiveFormat` gained the variants `Xz`, `Zst`, `Txz` and `Tzst`. Exhaustive
  matches on it need new arms.

### Added

- Commodore 64 containers: Lynx (`.lnx`), T64 tape images (`.t64`) and D64 1541 disk
  images (`.d64`, 35/40/42 tracks, with or without error bytes), via the new `lynx`,
  `t64` and `d64` modules and `ArchiveFormat::Lynx`, `ArchiveFormat::T64` and
  `ArchiveFormat::D64`. Entries are named after their PETSCII name plus a CBM type
  extension (`GAME.prg`, `DATA.seq`); PRG files keep their load address. D64 images,
  which have no magic bytes, are detected by size and BAM. Since `ArchiveFormat` is not
  `#[non_exhaustive]`, the new variants break exhaustive `match`es on it.
- Self-authored D64, T64 and Lynx fixtures, cross-checked against VICE `c1541` and
  cbmconvert, and tests for truncated images, sector chain loops and bad links.
- xz (`.xz`) and Zstandard (`.zst`) single-file streams, and xz- and
  Zstandard-compressed TAR archives (`.txz`, `.tar.xz`, `.tzst`, `.tar.zst`), using
  the pure Rust [`lzma-rust2`](https://crates.io/crates/lzma-rust2) and
  [`ruzstd`](https://crates.io/crates/ruzstd) crates. Concatenated xz streams and
  multiple Zstandard frames are decoded in full; skippable frames are ignored.
  `ArchiveFormat::detect()` returns `Txz` or `Tzst` for xz or Zstandard content
  named `.txz`, `.tar.xz`, `.tzst` or `.tar.zst`, like the other compressed TARs.
- Size limits apply to xz and Zstandard output, and the dictionary (window) size a
  stream announces is checked against the limit before the decoder allocates it.

### Fixed

- ARC/PAK crunched (method 8) and squashed members that reset the LZW table
  failed with "infinite loop detected", and squashed archives could panic. The
  reader now skips the padding to the end of a group of eight codes when the
  table is cleared, as the `.Z` reader already does; squashed codes grow to
  13 bits instead of stopping at 12; and the tables hold all 8192 codes. Over
  46 ARC archives from a BBS file base, every member now matches nomarch, where
  59 of 349 failed before.
- Lynx short-final-file tolerance requires EOF inside the last data block,
  including for REL files. Missing final blocks and earlier truncations now fail
  instead of being returned as successfully shortened files.
- XZ and TXZ decoding handles short and interrupted input reads and rejects
  trailing stream padding that is not a multiple of four bytes.
- Zstandard and TZST decoding verifies each frame's declared uncompressed size,
  rejecting streams with mismatched content sizes.
- ARC and PAK entries report their real modification time. The header's date and
  time words were read in the wrong order, so every timestamp was garbage (a 2024
  file listed as 2072, or with month 0).
- LHA entry names use `/` path separators on every platform, fixing Windows
  listings and the LHA metadata regression test without changing link targets.
- `ArchiveFormat::detect()` returns `Tgz`, `Tbz` or `TarZ` when gzip, bzip2 or
  compress content has a matching compressed-TAR name (`.tgz`, `.tar.gz`, `.tbz`,
  `.tbz2`, `.tar.bz2`, `.tar.Z`). Previously such archives were detected as a single
  compressed file, including in the `unarc` command-line tool.

## [0.7.2] - 2026-10-05

### Fixed

- ACE solid archives are identified by the standard main-header flag `0x8000`.
- ACE stored members now populate the LZ77 dictionary, including stored members
  split across volumes, so subsequent compressed members can reference their bytes.
- ACE member reads reset Huffman trees and distance history for every compressed
  member while retaining dictionary data across members.
- Added self-authored synthetic regressions for canonical solid flags, stored
  predecessors, per-member tree resets and unchanged non-solid reads.

## [0.7.1] - 2026-10-05

### Added

- Unified `ArchiveEntryKind`, `ArchiveEntry::kind()` and `ArchiveEntry::link_target()`.
  TAR (including compressed TAR), ZIP Unix modes, RAR Unix modes/RAR5 redirections,
  7z Unix attributes, and LHA Unix permissions distinguish links and special entries.
  Legacy directory and special-entry flags are used where available.
- Regression tests for links, devices/FIFOs, unknown entry types, extended TAR link
  names, and unchanged regular-file reads.

### Fixed

- TAR link targets now include GNU/PAX extended link names rather than only the
  fixed-size header field.
- 7z unified size limits are propagated into the entry output reader. Solid
  predecessors are drained without collecting their contents, and reads identify
  entries by index rather than by possibly duplicated names.

## [0.7.0] - 2026-09-30

### Breaking changes

- RAR support now uses the pure Rust [`rars`](https://crates.io/crates/rars) crate instead of the C++ UnRAR library.
  - `RarPasswordVerifier::new` is no longer public.
  - `RarFileHeader` gained a private field.
  - `RarArchive::from_path` no longer requires `T: Default`.
- Unified API reads now reject entries larger than 1 GiB by default (`DEFAULT_MAX_ENTRY_SIZE`) with the new `ArchiveError::SizeLimitExceeded`. Use `ArchiveOptions::with_max_entry_size(None)` to turn the limit off.
- Zstandard ZIP entries moved behind the new default-on `zstd` feature. Builds using `default-features = false` must enable it again to read zstd entries.
- Minimum supported Rust version is now 1.95 (declared as `rust-version`).

### Added

- Multi-volume RAR archives (`.part1.rar`, … and `.rar`/`.r00`, …) through a `VolumeProvider` in `ArchiveOptions`. Entries split across volumes are listed once. Works with encrypted headers and the password verifier. A missing volume is reported when an entry that needs it is read, and listing still works.
- RAR 1.3/1.4 archives (including detection of the `RE~^` signature) and RAR archives with encrypted headers.
- Resolution of RAR5 hard links and file copies.
- Decompressed size limits: `ArchiveOptions::with_max_entry_size` and `ArchiveOptions::with_max_total_size`.
  - Entries whose recorded size exceeds the limit are rejected before decoding.
  - Streams without a recorded size (gz, bz2, Z) are bounded while decoding.
  - The limit also applies to the whole-archive decompression of `.tar.gz`, `.tar.bz2` and `.tar.Z`, and to the stored size of ICE/Pack-Ice files.
- With `default-features = false` the crate is pure Rust and builds for `wasm32-unknown-unknown`.
- Criterion benchmarks for the native decoders (`cargo bench -p unarc-rs`).
- CI:
  - tests on Linux, Windows and macOS, without default features, with the MSRV, and a wasm32 build
  - `cargo-deny` checks for advisories, licenses, bans and sources (also run weekly)
  - a release workflow with `cargo-semver-checks`, `cargo-deny` and `cargo package`

### Changed

- RAR data is read from memory instead of temporary files. Solid archives are decoded in a single pass, with the remaining members cached, so sequential reads stay linear.
- Compressed data buffers sized from header fields (ACE, ARC, ARJ, HA, HYP, JAR, SQZ, TAR, UC2, ZOO) now grow as data is read. A forged size can no longer force a huge allocation, and pre-allocation from header sizes is capped at 16 MiB.
- ZIP reads never produce more data than the size in the central directory.
- `open_with_format` now shares the options code path.
- The CLI opts out of the default entry size limit.
- Replaced the deprecated `sha-1` crate with `sha1` and dropped the direct `byteorder` dependency.
- Bumped dependencies: `sevenz-rust2` 0.23, `clap` 4.6, `rayon` 1.12, `log` 0.4.34, `crc32fast` 1.5.2.
- The library and the CLI are now `#![forbid(unsafe_code)]`.
  - Removed the manual `unsafe impl Send/Sync` for the password verifiers. They are `Send + Sync` automatically, and a compile-time assertion now checks this.
  - ARJ GOST40 decryption no longer reinterprets `u32` arrays as bytes through raw pointers. It now uses explicit little-endian conversion, which also makes it correct on big-endian targets.
- README: ACE and ARJ multi-volume support is now documented (the format notes wrongly said it was unsupported). Also documented the `detect*` functions and the `*_with_options` variants, and the crate description now lists ACE, JAR, ICE and Pack-Ice.

### Fixed

- The RAR password verifier pointed at a deleted temporary file.
- Iterating a ZOO archive past its last entry failed with "failed to fill whole buffer". This affected `unarc extract` on any `.zoo` file. The end-of-directory marker is now recognized.
- Test fixtures and `LICENSE` are no longer end-of-line converted on Windows.

### Removed

- The `bit_decode_test`, `jar_engine_probe` and `jar_probe_lha` debugging examples.

## [0.6.3] - 2026-09-05

[0.7.2]: https://github.com/mkrueger/unarc-rs/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/mkrueger/unarc-rs/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/mkrueger/unarc-rs/compare/v0.6.3...v0.7.0
[0.6.3]: https://github.com/mkrueger/unarc-rs/releases/tag/v0.6.3
