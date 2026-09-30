# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.7.0]: https://github.com/mkrueger/unarc-rs/compare/v0.6.3...v0.7.0
[0.6.3]: https://github.com/mkrueger/unarc-rs/releases/tag/v0.6.3
