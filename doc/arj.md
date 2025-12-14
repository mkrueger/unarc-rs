# ARJ Archive Format Specification

## Overview

ARJ (Archived by Robert Jung) is a file archiver created by Robert K. Jung in 1991. It was one of the most popular compression utilities for DOS in the early 1990s, known for its high compression ratios and multi-volume archive support.

ARJ uses a variant of LZ77 compression combined with Huffman coding, similar to the LHA `-lh6-` method.

## File Structure

ARJ archives have a sequential structure with headers protected by CRC-32 checksums:

```text
┌─────────────────────────────────┐
│  Main Header                    │
│    ├─ Basic Header              │
│    └─ Extended Headers (0..n)   │
├─────────────────────────────────┤
│  File Entry 1                   │
│    ├─ Local File Header         │
│    ├─ Extended Headers (0..n)   │
│    └─ Compressed Data           │
├─────────────────────────────────┤
│  File Entry 2                   │
│    ├─ Local File Header         │
│    ├─ Extended Headers (0..n)   │
│    └─ Compressed Data           │
├─────────────────────────────────┤
│  ...                            │
├─────────────────────────────────┤
│  End Marker (header_size = 0)   │
└─────────────────────────────────┘
```

## Header Structure

All multi-byte values are little-endian.

### Basic Header Format

| Offset | Size | Field |
|--------|------|-------|
| 0x00 | 2 | Magic number (`0x60 0xEA`) |
| 0x02 | 2 | Header size (excluding this field and CRC) |
| 0x04 | n | Header data |
| 0x04+n | 4 | CRC-32 of header data |

**End of archive:** Indicated by `header_size = 0`

### Main Header Data

| Offset | Size | Field |
|--------|------|-------|
| 0 | 1 | Header size (first header size byte) |
| 1 | 1 | Archiver version number |
| 2 | 1 | Minimum version to extract |
| 3 | 1 | Host OS |
| 4 | 1 | ARJ flags |
| 5 | 1 | Security version |
| 6 | 1 | File type |
| 7 | 1 | Reserved |
| 8 | 4 | Creation date/time (DOS format) |
| 12 | 4 | Compressed size |
| 16 | 4 | Original archive size |
| 20 | 4 | Security envelope file position |
| 24 | 2 | File spec position in filename |
| 26 | 2 | Security envelope length |
| 28 | 1 | Encryption version |
| 29 | 1 | Last chapter |
| 30 | 1 | ARJ protection factor (if header_size >= 34) |
| 31 | 1 | ARJ flags 2 (if header_size >= 34) |
| 32 | 2 | Reserved |
| 34 | ? | Archive name (null-terminated) |
| ? | ? | Comment (null-terminated) |

### Local File Header Data

| Offset | Size | Field |
|--------|------|-------|
| 0 | 1 | Header size (first header size byte) |
| 1 | 1 | Archiver version number |
| 2 | 1 | Minimum version to extract |
| 3 | 1 | Host OS |
| 4 | 1 | ARJ flags |
| 5 | 1 | Compression method |
| 6 | 1 | File type |
| 7 | 1 | Reserved |
| 8 | 4 | Date/time modified (DOS format) |
| 12 | 4 | Compressed size |
| 16 | 4 | Original size |
| 20 | 4 | Original file CRC-32 |
| 24 | 2 | File spec position in filename |
| 26 | 2 | File access mode |
| 28 | 1 | First chapter |
| 29 | 1 | Last chapter |
| 30 | 4 | Extended file position (if header_size > 30) |
| 34 | 4 | Date/time accessed (if header_size >= 46) |
| 38 | 4 | Date/time created (if header_size >= 46) |
| 42 | 4 | Original size for volumes (if header_size >= 46) |
| ? | ? | Filename (null-terminated) |
| ? | ? | Comment (null-terminated) |

### Extended Headers

Extended headers follow the main/local header with the same structure:

| Offset | Size | Field |
|--------|------|-------|
| 0 | 2 | Extended header size (0 = no more) |
| 2 | n | Extended header data |
| 2+n | 4 | CRC-32 of extended header data |

## Host OS Values

| Value | OS |
|-------|-----|
| 0 | MS-DOS |
| 1 | PRIMOS |
| 2 | Unix |
| 3 | Amiga |
| 4 | Mac OS |
| 5 | OS/2 |
| 6 | Apple GS |
| 7 | Atari ST |
| 8 | NeXT |
| 9 | VAX VMS |
| 10 | Windows 95 |
| 11 | Windows 32-bit |

## File Types

| Value | Type |
|-------|------|
| 0 | Binary |
| 1 | 7-bit text |
| 2 | Comment header |
| 3 | Directory |
| 4 | Volume label |
| 5 | Chapter label |

## Compression Methods

| Value | Method | Description |
|-------|--------|-------------|
| 0 | Stored | No compression |
| 1 | Compressed (Most) | LZ77 + Huffman, maximum compression |
| 2 | Compressed | LZ77 + Huffman, normal compression |
| 3 | Compressed (Faster) | LZ77 + Huffman, faster compression |
| 4 | Compressed (Fastest) | Simple LZ77 with fixed codes |
| 8 | No data, no CRC | Special entry (no file data) |
| 9 | No data | Special entry (no file data) |

### Compression Algorithm Details

Methods 1-3 use LZ77 compression with Huffman coding, similar to LHA's `-lh6-` method:

- **Dictionary size:** 26,624 bytes
- **Minimum match length:** 3 bytes
- **Maximum match length:** 256+ bytes
- **Huffman-coded:** Literal bytes, match lengths, and distances

Method 4 (Fastest) uses a simpler encoding scheme with fixed-width codes.

## ARJ Flags (Byte 4)

| Bit | Meaning |
|-----|---------|
| 0 | Garbled (encrypted) |
| 1 | Reserved |
| 2 | Volume (multi-volume archive) |
| 3 | Extended file (ARJEXT) |
| 4 | Path symbol ('/' translated) |
| 5 | Backup |
| 6 | Reserved |
| 7 | Reserved |

## CRC-32

ARJ uses standard CRC-32 with polynomial `0xEDB88320` (reflected form).

- **Header CRC:** Covers header data only
- **File CRC:** Covers uncompressed file data
- Initial value: `0x00000000` (or `0xFFFFFFFF` with final XOR)

## Timestamp Format (DOS)

Same as standard DOS format:

| Bits | Field | Range |
|------|-------|-------|
| 0-4 | Second / 2 | 0-29 (0-58 seconds) |
| 5-10 | Minute | 0-59 |
| 11-15 | Hour | 0-23 |
| 16-20 | Day | 1-31 |
| 21-24 | Month | 1-12 |
| 25-31 | Year - 1980 | 0-127 (1980-2107) |

## Multi-Volume Archives

ARJ supports splitting archives across multiple volumes (disks):

- First volume: Normal archive with volume flag set
- Subsequent volumes: Continue file data
- Files can span volumes
- Each volume has its own header

## Security Envelope

ARJ supports optional security features:

- **Security envelope:** Digital signature area
- **Encryption:** Password-based encryption (garbled flag)
- **Protection factor:** Error recovery data

## Implementation Notes

This implementation supports:

- ✅ Method 0: Stored
- ✅ Method 1: Compressed (Most) - via LH6 decoder
- ✅ Method 2: Compressed - via LH6 decoder
- ✅ Method 3: Compressed (Faster) - via LH6 decoder
- ✅ Method 4: Compressed (Fastest) - custom decoder
- ❌ Method 8-9: No data entries (no decompression needed)
- ❌ Encrypted archives (garbled flag)
- ❌ Multi-volume archives

## Historical Context

ARJ was created by Robert K. Jung and first released in 1991. It quickly became one of the most popular compression utilities for DOS due to its:

- High compression ratios (often better than PKZIP at the time)
- Multi-volume archive support for floppy disks
- Self-extracting archive capability
- Advanced features like file comments and chapters

ARJ was shareware and remained popular until the mid-1990s when PKZIP and later WinZip became dominant.

## References

- ARJ Technical Documentation by Robert K. Jung
- ARJ 2.x file format specifications
