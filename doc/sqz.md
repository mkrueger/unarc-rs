# SQZ (HLSQZ) Archive Format

This file documents the on-disk *container* format of SQZ archives as parsed by `unarc-rs` (`src/sqz/*`) and how it aligns with the historical reference description in `doc/sqz.txt`.

## Overview

An SQZ archive consists of:

1. a fixed archive header (8 bytes)
2. a sequence of blocks
   - file entries (file header + file data)
   - optional meta blocks (comment, password, …)
   - end marker

All integer fields are read as **little-endian** by `unarc-rs`.

## 1) Archive Header (8 bytes)

Offset | Size | Type | Meaning
---|---:|---|---
0x00 | 5 | ASCII | Magic: `"HLSQZ"`
0x05 | 1 | ASCII | Version as an ASCII character (typically: `"1"`)
0x06 | 1 | u8 | OS id
0x07 | 1 | u8 | Flags (bitmapped)

### OS id (`src/sqz/sqz_header.rs`)

Values (per reference and code):

- `0` DOS
- `1` OS/2
- `2` MVS
- `3` HPFS (OS/2)
- `4` Amiga
- `5` Mac (in code: `OsX`)
- `6` Unix
- otherwise: unknown

### Flags

The historical reference (`doc/sqz.txt`) mentions, among others:

- Bit 0: Intel vs. Motorola byte order
- Bit 1: file time format
- Bit 2: security envelope present

`unarc-rs` currently reads the flags byte but does not interpret the bits further.

## 2) Block Sequence

Immediately after the archive header comes a sequence of blocks. Each block starts with **1 byte** `tag`.

`tag` | Meaning
---:|---
0 | End of archive
1 | Comment block
2 | Password block
3..=18 | “Other blocks” / reserved (skipped)
>= 18 | File entry: `tag` is the header length (see below)

## 3) File Entry

A file entry consists of:

1. `tag` (1 byte) with `tag >= 18`
2. `checksum` (1 byte)
3. `file_header_bytes` (`tag` bytes)
4. file data (`compressed_size` bytes)

Important: In `unarc-rs`, the `tag` value is interpreted as the **length of the file header excluding the checksum byte**.

This matches e.g. `tests/sqz/license.sqz`:

- `tag = 0x19` (25)
- then 1 byte `checksum`
- then 25 bytes of header from `flags` through the filename

### 3.1) File Header Fields (`src/sqz/file_header.rs`)

The header (after the separate checksum byte) starts with `flags`.

The relative offsets below are relative to the start of `file_header_bytes` (i.e. to `flags`).

Offset | Size | Type | Meaning
---|---:|---|---
0x00 | 1 | u8 | `flags`
0x01 | 4 | u32 | `compressed_size`
0x05 | 4 | u32 | `original_size`
0x09 | 4 | u32 | `date_time` (DOS DateTime)
0x0D | 1 | u8 | `attribute`
0x0E | 4 | u32 | `crc32` of the decompressed file
0x12 | … | bytes | filename (rest of header)

`unarc-rs` additionally derives:

- `method = flags & 0x0F`
  - `0` = Stored
  - `1..=5` = Compressed (SQZ “Squeeze” variants)
- `compression_method`: derived from `method`

Note: The leading `checksum` byte is currently read but not validated.

### 3.2) Filename

The filename length is `tag - 0x12` (i.e. `tag - 18`), because the fixed fields up to and including `crc32` occupy 18 bytes.

`unarc-rs` decodes the filename using `String::from_utf8_lossy(...)`.

### 3.3) File Data

Immediately after the file header come `compressed_size` bytes of file data.

- If `method == 0` (`Stored`), these bytes are already the uncompressed data.
- If `method != 0`, the data is decompressed by `src/sqz/unsqz.rs`.

## 4) Comment Block (`tag == 1`)

The historical reference (`doc/sqz.txt`) describes the comment block as:

Offset | Size | Type | Meaning
---|---:|---|---
0x00 | 2 | u16 | size of uncompressed comment
0x02 | 2 | u16 | size of compressed comment data (`LEN`)
0x04 | 1 | u8 | flags
0x05 | 4 | u32 | CRC-32
0x09 | LEN | bytes | compressed comment data

Implementation status in `unarc-rs`: the comment block is currently only **skipped** (not decoded).

## 5) Password Block (`tag == 2`)

Historical reference (`doc/sqz.txt`):

Offset | Size | Type | Meaning
---|---:|---|---
0x00 | 2 | u16 | block size (typically: 4)
0x02 | 4 | u32 | CRC-32 of the password

Implementation status in `unarc-rs`: the CRC-32 is read and stored, but currently not used for decryption.

## 6) Other blocks (`tag == 3..=18`)

These blocks have (per the reference) a generic format:

Offset | Size | Type | Meaning
---|---:|---|---
0x00 | 2 | u16 | size (`LEN`)
0x02 | LEN | bytes | block data

`unarc-rs` skips these blocks.

## 7) Compression Methods (high-level)

`method = flags & 0x0F`:

- `0`: Stored (no compression)
- `1..=4`: “Squeeze” / compressed (format depends on SQZ.EXE)

In `unarc-rs`, `method == 4` is the most important path (default compression). The concrete bitstream specification of the method-4 payload is not part of this container document; see the implementation in `src/sqz/unsqz.rs` and the historical notes in `doc/sqz.txt`.
