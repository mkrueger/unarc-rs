# HA Archive Format Specification

**Version:** 0.999 (final release by Harri Hirvola)  
**Author:** Harri Hirvola (1993-1995)

## 1. Overview

HA is a file archiver that achieves high compression ratios using advanced statistical compression methods. It was notable for its compression efficiency, often outperforming contemporary archivers like PKZIP and ARJ.

Key features:

- Two compression methods: ASC (LZ77-based) and HSC (PPM-based)
- Arithmetic coding for entropy encoding
- Simple sequential archive structure
- CRC32 integrity checking

## 2. Archive Structure

```text
┌──────────────────────────────────┐
│  Magic Number: "HA" (2 bytes)    │
├──────────────────────────────────┤
│  Entry 1                         │
│    ├─ Header                     │
│    └─ Compressed Data            │
├──────────────────────────────────┤
│  Entry 2                         │
│    ├─ Header                     │
│    └─ Compressed Data            │
├──────────────────────────────────┤
│  ...                             │
├──────────────────────────────────┤
│  End Marker (type = 0xFF)        │
└──────────────────────────────────┘
```

## 3. Entry Header Format

All multi-byte values are little-endian.

| Offset | Size | Field |
|--------|------|-------|
| 0 | 1 | Type byte (see below) |
| 1 | 4 | Compressed size (bytes) |
| 5 | 4 | Original size (bytes) |
| 9 | 4 | CRC-32 checksum (of uncompressed data) |
| 13 | 4 | Timestamp (DOS format, see below) |
| 17 | ? | Path (null-terminated string) |
| ? | ? | Filename (null-terminated string) |
| ? | n | Compressed data (n = compressed size) |

### Type Byte

#### Bits 0-3: Compression method

| Value | Method |
|-------|--------|
| 0 | CPY (store, uncompressed) |
| 1 | ASC (LZ77 + arithmetic) |
| 2 | HSC (PPM + arithmetic) |
| 14 | Directory entry |
| 15 | End marker (0x?F) |

#### Bits 4-7: Machine/OS type

| Value | OS |
|-------|-----|
| 0 | MS-DOS / generic |
| 1 | Unix |

### Timestamp (DOS format)

| Bits | Field |
|------|-------|
| 0-4 | Second / 2 (0-29) |
| 5-10 | Minute (0-59) |
| 11-15 | Hour (0-23) |
| 16-20 | Day (1-31) |
| 21-24 | Month (1-12) |
| 25-31 | Year - 1980 (0-127) |

## 4. Compression Methods

### 4.1 CPY (Method 0) - Store

No compression, data is stored verbatim.

### 4.2 ASC (Method 1) - Arithmetic Sliding-window Compression

- LZ77-style dictionary compression
- 31,200 byte sliding window
- Adaptive arithmetic coding
- Token types: Literal, Match (length+position), EOF
- See: `asc_compression.txt` for details

### 4.3 HSC (Method 2) - High Statistical Compression

- PPM (Prediction by Partial Matching)
- Order-4 context model
- 10,000 context slots, 32,760 frequency blocks
- Exclusion-based escape mechanism
- See: `hsc_compression.txt` for details

## 5. CRC-32

Standard CRC-32 with polynomial `0xEDB88320` (reflected form).

- Calculated over the uncompressed data
- Initial value: `0xFFFFFFFF`
- Final XOR: `0xFFFFFFFF`

## 6. Arithmetic Coding

Both ASC and HSC use the same 16-bit arithmetic coder:

- 16-bit precision for interval bounds
- E1/E2 renormalization (MSB convergence)
- E3 scaling for underflow prevention
- Big-endian bit ordering within bytes

**Reference:** Witten, Neal, Cleary (1987) "Arithmetic Coding for Data Compression"

## 7. Character Set

Filenames use the native character set of the originating system:

- **DOS:** CP437 or similar
- **Unix:** ISO-8859-1 or UTF-8

Path separator is stored as the native separator but typically normalized to forward slash (`/`) on extraction.

## 8. Notes

- Archives are strictly sequential (no central directory)
- No encryption support
- No multi-volume support
- No solid archive mode (each file compressed independently)
- Maximum path length: typically 255 bytes
- Directory entries have zero compressed/original size

## 9. References

- Original implementation: HA 0.999 by Harri Hirvola
- Arithmetic coding: Witten, Neal, Cleary (1987)
- PPM algorithm: Cleary & Witten (1984)

## Implementation Notes

This implementation was created using:

1. **Container format:** Analyzed from .ha archive files (used the DOS command line client)
2. **Arithmetic coding:** Witten, Neal, Cleary (1987) "Arithmetic Coding for Data Compression", CACM 30(6)
3. **PPM algorithm:** Cleary & Witten (1984) "Data Compression Using Adaptive Coding and Partial String Matching"
4. **Format details:** Reverse-engineered from test archive behavior
