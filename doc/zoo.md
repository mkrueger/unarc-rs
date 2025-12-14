# ZOO Archive Format Specification

## Overview

ZOO is an archive format created by Rahul Dhesi in 1986. It was one of the early compression formats for DOS and Unix systems, known for its portability across different operating systems. While superseded by more modern formats like ZIP, ZOO archives are still encountered in legacy software distributions.

The format supports multiple compression methods and includes features like file comments, long filenames, and directory preservation.

## File Structure

A ZOO archive consists of a main header followed by a linked list of directory entries, each pointing to its compressed data.

### Archive Structure

```text
┌─────────────────────────────────────┐
│ Archive Header (46 bytes)           │
│   ├─ Header Text (20 bytes)         │
│   ├─ Magic Number (4 bytes)         │
│   └─ Archive Metadata (22 bytes)    │
├─────────────────────────────────────┤
│ Directory Entry 1                   │
│   ├─ Entry Header (56+ bytes)       │
│   └─ Compressed Data                │
├─────────────────────────────────────┤
│ Directory Entry 2                   │
│   ├─ Entry Header (56+ bytes)       │
│   └─ Compressed Data                │
├─────────────────────────────────────┤
│ ...                                 │
└─────────────────────────────────────┘
```

Note: Directory entries form a linked list. The `next` field in each entry points to the file offset of the next directory entry.

## Archive Header

The archive header is 46 bytes and contains global archive information.

### Header Layout (46 bytes)

| Offset | Size | Field      | Description                           |
|--------|------|------------|---------------------------------------|
| 0      | 20   | text       | Header text, typically "ZOO 2.10 Archive.\x1a" |
| 20     | 4    | zoo_tag    | Magic number: `0xFDC4A7DC`            |
| 24     | 4    | zoo_start  | Offset to first directory entry       |
| 28     | 4    | zoo_minus  | Consistency check value (negative of zoo_start in older versions) |
| 32     | 1    | major_ver  | Major version needed to extract       |
| 33     | 1    | minor_ver  | Minor version needed to extract       |
| 34     | 4    | cmt_pos    | Position of archive comment (0 if none) |
| 38     | 4    | cmt_len    | Length of archive comment             |
| 42     | 4    | vdata      | Version data flags                    |

### Magic Number

The ZOO format uses the magic number `0xFDC4A7DC` (little-endian) for identification. This value appears both in the archive header and in each directory entry.

### Header Text

The header text is a 20-byte field containing an ASCII string identifying the archive type. Standard text is:

```text
"ZOO 2.10 Archive.\x1a"
```

The `\x1a` (Ctrl-Z) character marks the end of the text for DOS compatibility.

## Directory Entry

Each file in the archive has a directory entry header followed by its compressed data.

### Directory Entry Layout (56 bytes minimum + variable part)

| Offset | Size | Field       | Description                          |
|--------|------|-------------|--------------------------------------|
| 0      | 4    | zoo_tag     | Magic number: `0xFDC4A7DC`           |
| 4      | 1    | type        | Entry type (always 1)                |
| 5      | 1    | packing_method | Compression method (see below)    |
| 6      | 4    | next        | Offset to next directory entry (0 = last) |
| 10     | 4    | offset      | Offset to compressed file data       |
| 14     | 4    | date_time   | DOS date/time format                 |
| 18     | 2    | file_crc16  | CRC-16 of uncompressed data          |
| 20     | 4    | org_size    | Original uncompressed size           |
| 24     | 4    | size_now    | Compressed size                      |
| 28     | 1    | major_ver   | Major version that created this entry |
| 29     | 1    | minor_ver   | Minimum version needed to extract    |
| 30     | 1    | deleted     | Deletion flag (0=active, 1=deleted)  |
| 31     | 1    | struc       | File structure indicator             |
| 32     | 4    | comment     | Offset to file comment (0 if none)   |
| 36     | 2    | cmt_size    | Length of file comment               |
| 38     | 13   | fname       | DOS 8.3 filename (null-terminated)   |
| 51     | 1    | var_dir_len | Length of variable directory part    |
| 52     | 1    | tz          | Timezone offset                      |
| 53     | 4    | dir_crc     | CRC of directory entry               |
| 57     | 1    | namlen      | Length of long filename              |
| 58     | 1    | dirlen      | Length of directory name             |

Total fixed size: 59 bytes (DIRENT_HEADER_SIZE in code)

### Variable Part (following fixed header)

If `namlen > 0` or `dirlen > 0`, additional data follows:

| Offset | Size    | Field      | Description                       |
|--------|---------|------------|-----------------------------------|
| 59     | namlen  | lfname     | Long filename                     |
| 59+n   | dirlen  | dirname    | Directory path                    |
| ...    | 2       | system_id  | System identifier                 |
| ...    | 3       | fattr      | File attributes (24 bits)         |
| ...    | 1       | vflag      | Version flag bits                 |
| ...    | 2       | version_no | File version number               |

Note: The original zoo.txt documentation has an error at offset 0x0D - it should be 0x0E (14 decimal) for the date_time field.

### Compression Methods

| Value | Name           | Description                              |
|-------|----------------|------------------------------------------|
| 0     | Stored         | No compression                           |
| 1     | Compressed     | LZW compression, 4K buffer, 9-13 bit codes |
| 2     | CompressedLh5  | LH5 compression (from LHA)               |

### Date/Time Format

Standard DOS date/time format packed into 32 bits:

```text
Bits 0-4:   Seconds / 2 (0-29)
Bits 5-10:  Minutes (0-59)
Bits 11-15: Hours (0-23)
Bits 16-20: Day (1-31)
Bits 21-24: Month (1-12)
Bits 25-31: Year - 1980
```

### System Identifiers

| Value | System    | Description              |
|-------|-----------|--------------------------|
| 0     | Unix      | Unix/Linux systems       |
| 1     | DOS       | MS-DOS/PC-DOS            |
| 2     | Portable  | Portable format          |

### CRC-16 Checksum

The file CRC uses the CRC-16-ARC polynomial (also known as CRC-16-IBM):

- Polynomial: `0x8005` (reversed: `0xA001`)
- Initial value: `0x0000`
- Reflects input and output

**Important:** The CRC is calculated on the **uncompressed** data, not the compressed stream.

## LZW Compression (Method 1)

Zoo uses LZW (Lempel-Ziv-Welch) compression with the following parameters:

### Parameters

| Parameter      | Value        |
|----------------|--------------|
| Initial code size | 9 bits    |
| Maximum code size | 13 bits   |
| Dictionary size | 8192 entries |
| Bit order      | LSB-first    |

### Algorithm

1. Start with 9-bit codes
2. Output codes in little-endian bit order
3. Increase code size when dictionary fills current capacity
4. Maximum of 13 bits (8192 codes)
5. Uses variable-length code strategy

## LH5 Compression (Method 2)

Zoo version 2.1 and later support LH5 compression, borrowed from the LHA archiver. This provides better compression than LZW for most file types.

LH5 uses:

- Sliding window dictionary (8192 bytes)
- Huffman coding for literals and distances
- Static Huffman trees per block

## Linked List Navigation

Directory entries form a linked list:

1. Start at `zoo_start` offset from archive header
2. Read directory entry
3. If `next == 0`, this is the last entry
4. Otherwise, seek to `next` offset and repeat

```text
Archive Header
     │
     └──► zoo_start
              │
              ▼
         ┌─────────────────┐
         │ Directory Entry │──► offset ──► [Compressed Data]
         │ next = 0x1234   │
         └────────┬────────┘
                  │
                  ▼ (seek to 0x1234)
         ┌─────────────────┐
         │ Directory Entry │──► offset ──► [Compressed Data]
         │ next = 0x5678   │
         └────────┬────────┘
                  │
                  ▼
                 ...
         ┌─────────────────┐
         │ Directory Entry │──► offset ──► [Compressed Data]
         │ next = 0        │ (last entry)
         └─────────────────┘
```

## Deleted Files

Zoo supports "deleting" files without physically removing them:

- Set `deleted` flag to 1
- File remains in archive but is skipped during extraction
- Use zoo "pack" command to physically remove deleted entries

## File Comments

Both the archive and individual files can have comments:

- Archive comment: Located at `cmt_pos` with length `cmt_len`
- File comment: Located at `comment` offset with length `cmt_size`
- Comments are plain text, typically null-terminated

## Implementation Notes

### Version History

| Version | Features                                |
|---------|------------------------------------------|
| 1.x     | Basic LZW compression                   |
| 2.0     | Extended directory structure            |
| 2.1     | Added LH5 compression support           |

### Portability

ZOO was designed for cross-platform use:

- Uses explicit byte order (little-endian)
- Stores system ID for platform-specific attributes
- Supports both DOS 8.3 and long filenames

### Memory Requirements

The decompressor requires approximately:

- ~16 KB for LZW dictionary (8192 entries × 2 bytes)
- ~8 KB for LH5 sliding window
- Additional buffer space for I/O

### Endianness

All multi-byte values are stored in little-endian format.

## References

- Original ZOO source code by Rahul Dhesi
- unarc-rs implementation: `src/zoo/`
- 