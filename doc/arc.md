# ARC Archive Format Specification

## Overview

ARC is one of the earliest archive file formats for personal computers, created by System Enhancement Associates (SEA) in 1985. It was one of the first widely-used file compression programs for MS-DOS and became a de facto standard for bulletin board systems (BBS) in the late 1980s.

The format has been implemented by various programs including SEA ARC, PAK, and PkPAK, with some variations and extensions.

## File Structure

ARC archives have a simple sequential structure with no central directory:

```text
┌─────────────────────────────────┐
│  Entry 1                        │
│    ├─ Header (29 bytes)         │
│    └─ Compressed Data           │
├─────────────────────────────────┤
│  Entry 2                        │
│    ├─ Header (29 bytes)         │
│    └─ Compressed Data           │
├─────────────────────────────────┤
│  ...                            │
├─────────────────────────────────┤
│  End Marker (2 bytes: 1A 00)    │
└─────────────────────────────────┘
```

## Entry Header Format

All multi-byte values are little-endian.

| Offset | Size | Field |
|--------|------|-------|
| 0x00 | 1 | Magic byte (`0x1A`) |
| 0x01 | 1 | Compression method |
| 0x02 | 13 | Filename (null-terminated, max 12 chars + null) |
| 0x0F | 4 | Compressed size (bytes) |
| 0x13 | 4 | Timestamp (DOS format) |
| 0x17 | 2 | CRC-16 checksum |
| 0x19 | 4 | Original size (bytes) |

**Total header size:** 29 bytes

### Magic Byte

Each entry begins with `0x1A` (ASCII SUB character). This byte is used to identify the start of an entry header.

**Note:** Other archivers also use `0x1A` at offset 0:

- **Hyper archives:** Check bytes 1-2 for "HP" or "ST"
- **ZOO archives:** Also starts with `0x1A`

### Compression Methods

| Value | Method | Description |
|-------|--------|-------------|
| 0 | End of archive | Archive terminator marker |
| 1 | Unpacked (v1) | Stored without compression (obsolete, ARC 1.0) |
| 2 | Unpacked (v2) | Stored without compression (ARC 3.1) |
| 3 | Packed | RLE encoding only |
| 4 | Squeezed | RLE + Huffman coding |
| 5 | Crunched (v1) | LZW compression (obsolete, ARC 4.0) |
| 6 | Crunched (v2) | LZW after RLE packing (obsolete, ARC 4.1) |
| 7 | Crunched (v3) | LZW + RLE with faster hash (ARC 4.6) |
| 8 | Crunched (v4) | Dynamic LZW variations + RLE (ARC 5.0) |
| 9 | Squashed | LZW without RLE packing (Phil Katz) |
| 10 | Crushed | PAK only |
| 11 | Distilled | PAK only |
| 12-19 | Reserved | ARC 6.0/7.0 extensions |
| 20-29 | Informational | ARC 6.0 informational items |
| 30-39 | Control | ARC 6.0 control items |
| 40+ | Reserved | Future use |

## Compression Algorithms

### Method 1-2: Unpacked (Stored)

Data is stored without any compression.

### Method 3: Packed (RLE)

Run-Length Encoding using `0x90` (DLE) as the escape character:

- `<byte> 0x90 <count>` - Repeat `<byte>` `<count>` times
- `0x90 0x00` - Literal `0x90` byte

### Method 4: Squeezed (RLE + Huffman)

1. Data is first RLE-encoded
2. Result is compressed with adaptive Huffman coding
3. Huffman tree is stored at the beginning of compressed data

The Huffman tree format:

| Field | Size | Description |
|-------|------|-------------|
| Node count | 2 bytes | Number of nodes (max 256) |
| Nodes | 4 bytes each | Two 16-bit signed integers per node |

Node interpretation:

- Positive value: Index of child node
- Negative value: `-1 - character` (leaf node)
- Special value `256` = End of file marker

### Method 5-8: Crunched (LZW + RLE)

LZW (Lempel-Ziv-Welch) compression with RLE pre-processing:

- **Method 5-6:** Original LZW implementation (obsolete)
- **Method 7:** Faster hash algorithm
- **Method 8:** Dynamic LZW with improved compression

LZW parameters:

- Initial code size: 9 bits
- Maximum code size: 12 bits
- Clear code: 256
- First available code: 257
- Maximum codes: 4096 (2^12)

For method 8, a header byte indicates the actual bit width used.

### Method 9: Squashed

LZW compression without RLE pre-processing:

- Maximum code size: 13 bits
- Maximum codes: 8192 (2^13)
- No RLE layer

### Method 10-11: Crushed/Distilled (PAK only)

These methods are specific to the PAK archiver and are not part of the original ARC specification.

## CRC-16

Standard CRC-16 with polynomial `0x8005` (ARC variant).

- Calculated over the uncompressed data
- Initial value: `0x0000`

## Timestamp Format (DOS)

The 32-bit timestamp uses the standard MS-DOS format:

| Bits | Field | Range |
|------|-------|-------|
| 0-4 | Second / 2 | 0-29 (0-58 seconds) |
| 5-10 | Minute | 0-59 |
| 11-15 | Hour | 0-23 |
| 16-20 | Day | 1-31 |
| 21-24 | Month | 1-12 |
| 25-31 | Year - 1980 | 0-127 (1980-2107) |

```text
  31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16
 |<---- year-1980 --->|<- month ->|<--- day ---->|

  15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
 |<--- hour --->|<---- minute --->|<- second/2 ->|
```

## ARC 6.0 Extensions

### Informational Items (Types 20-29)

Extended header format for informational items:

| Offset | Size | Field |
|--------|------|-------|
| 0 | 2 | Header length |
| 2 | 1 | Subtype |
| 3 | ? | Data |

#### Type 20: Archive Info

| Subtype | Description |
|---------|-------------|
| 0 | Archive description (ASCIIZ) |
| 1 | Creator program name (ASCIIZ) |
| 2 | Modifier program name (ASCIIZ) |

#### Type 21: File Info

| Subtype | Description |
|---------|-------------|
| 0 | File description (ASCIIZ) |
| 1 | Long filename (if not 8.3 format) |
| 2 | Extended date-time (reserved) |
| 3 | Icon (reserved) |
| 4 | File attributes (ASCIIZ) |

File attribute characters:

- `R` - Read access
- `W` - Write access
- `H` - Hidden file
- `S` - System file
- `N` - Network shareable

#### Type 22: OS Info (Reserved)

Reserved for operating system information.

## Limitations

- Maximum filename length: 12 characters (8.3 format)
- No directory support in original format
- No encryption support
- Sequential access only (no central directory)
- 32-bit size limit (~4 GB per file)

## Implementation Notes

This implementation supports:

- Methods 1-2: Unpacked (stored)
- Method 3: RLE packed
- Method 4: Squeezed (RLE + Huffman)
- Methods 5-8: Crunched (LZW + RLE)
- Method 9: Squashed (LZW only)

Not implemented:

- Method 10: Crushed (PAK only)
- Method 11: Distilled (PAK only)
- ARC 6.0 informational/control items

## References

- SEA ARC Technical Memo
- Original ARC file format documentation
- PAK/PkPAK documentation
