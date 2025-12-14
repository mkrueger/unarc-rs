# HYP (Hyper) Archive Format Specification

## Overview

HYP is a DOS-era archive format created by Peter Sawatzki and Klaus-Peter Nischke. It was distributed by the German publisher Sybex-Verlag. The format uses a combination of adaptive Huffman coding and LZ77-style string matching for compression.

The format was primarily distributed with German software products in the late 1980s and early 1990s.

## File Structure

A HYP archive consists of a sequence of file entries, each containing a header followed by the compressed or stored data.

### Archive Structure

```text
┌─────────────────────────────────┐
│ File Entry 1                    │
│   ├─ Marker (1 byte)            │
│   ├─ Header (21 bytes + name)   │
│   └─ Data (compressed_size)     │
├─────────────────────────────────┤
│ File Entry 2                    │
│   ├─ Marker (1 byte)            │
│   ├─ Header (21 bytes + name)   │
│   └─ Data (compressed_size)     │
├─────────────────────────────────┤
│ ...                             │
└─────────────────────────────────┘
```

## File Entry Header

Each file entry begins with a marker byte `0x1A`, followed by a 21-byte fixed header and a variable-length filename.

### Header Layout (21 bytes + filename)

| Offset | Size | Field              | Description                                    |
|--------|------|--------------------|------------------------------------------------|
| 0      | 2    | compression_method | Compression type (see below)                   |
| 2      | 1    | version            | BCD format version (e.g., `0x40` = version 4.0)|
| 3      | 4    | compressed_size    | Size of compressed data in bytes               |
| 7      | 4    | original_size      | Original uncompressed size in bytes            |
| 11     | 4    | date_time          | DOS date/time format                           |
| 15     | 4    | checksum           | File checksum (algorithm varies)               |
| 19     | 1    | attribute          | DOS file attribute                             |
| 20     | 1    | name_length        | Length of filename                             |
| 21     | n    | name               | Filename (name_length bytes, no null terminator)|

### Compression Methods

| Value    | ASCII | Description       |
|----------|-------|-------------------|
| `0x5453` | "ST"  | Stored (no compression) |
| `0x5048` | "HP"  | Hyper compression |

### Version Field

The version field uses BCD (Binary-Coded Decimal) format:

- High nibble: Major version
- Low nibble: Minor version

Common versions:

- `0x30` - Version 3.0 (mode 3)
- `0x40` - Version 4.0 (mode 4)

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

### Checksum

The checksum uses an add-then-rotate-left algorithm, applied to the **compressed data** (not the uncompressed content):

```rust
fn calculate_checksum(data: &[u8]) -> u32 {
    let mut checksum: u32 = 0;
    for &byte in data {
        checksum = checksum.wrapping_add(byte as u32).rotate_left(1);
    }
    checksum
}
```

For each byte:

1. Add the byte value to the 32-bit accumulator
2. Rotate the accumulator left by 1 bit

**Important:** The checksum is calculated over the compressed data stream, not the original file content. For stored files (method "ST"), the compressed data equals the original data.

## Hyper Compression Algorithm

The Hyper compression method combines adaptive Huffman coding with LZ77-style dictionary-based compression.

### Compression Stream Structure

```text
┌──────────────────────────────────────────────┐
│ Block Header (13 bits)                       │
│   └─ teststrings_index (number of entries)   │
├──────────────────────────────────────────────┤
│ Huffman-encoded symbol sequence              │
│   └─ Symbols encode characters and           │
│      back-references                         │
├──────────────────────────────────────────────┤
│ (More blocks until tsi == 255)               │
└──────────────────────────────────────────────┘
```

### Bit Reading Order

Bits are read LSB-first (least significant bit first) within each byte.

### Block Structure

Each compressed block begins with a 13-bit `teststrings_index` (tsi) value:

- Values 256-8191: Number of string table entries to read
- Value 255: End-of-stream marker

### Version-Dependent Parameters

The version affects compression parameters:

#### Version 3.x (mode 3)

| Parameter   | Value |
|-------------|-------|
| mindiff     | -8    |
| maxdiff     | 8     |
| maxlocal    | 0x96 (150) |

#### Version 4.x (mode 4)

| Parameter   | Value |
|-------------|-------|
| mindiff     | -4    |
| maxdiff     | 8     |
| maxlocal    | Calculated from tsi |

For version 4.x, maxlocal is calculated as:

```text
maxlocal = (((0x9C * (tsi - 0xFF)) / 0x1F00) & 0xFFFE) + 4
```

### Table Offsets

The Huffman decoding uses several offset values derived from the parameters:

```text
local_offset = 4 + maxdiff - mindiff
pos_offset   = local_offset + maxlocal + 2
char_offset  = pos_offset + 2 * (MAX_FREQ + 1)
```

Where `MAX_FREQ = 2`.

### Symbol Types

Decoded symbols are interpreted based on their value relative to the offsets:

| Symbol Range                     | Meaning                           |
|----------------------------------|-----------------------------------|
| `0` (LSEQUENCE_KEY × 2)          | Sequence marker (special handling)|
| `< local_offset`                 | Difference/local reference        |
| `local_offset ≤ s < pos_offset`  | Local string reference            |
| `pos_offset ≤ s < char_offset`   | Position reference (tab_decode)   |
| `≥ char_offset`                  | Literal character (byte value)    |

### Adaptive Huffman Trees

The decompressor maintains dynamic Huffman trees that are updated as symbols are decoded. Key data structures:

| Array       | Size            | Purpose                            |
|-------------|-----------------|-------------------------------------|
| vater       | 2 × HUFF_SIZE   | Parent pointers in Huffman tree    |
| sohn        | 2 × HUFF_SIZE   | Child pointers in Huffman tree     |
| the_freq    | 2 × HUFF_SIZE   | Frequency values for tree nodes    |
| nindex      | 8191            | Symbol to tree position mapping    |
| nvalue      | ~8400           | Tree position to symbol mapping    |
| frequencys  | ~8400           | Per-symbol frequency tracking      |
| nfreq       | 4               | Frequency offset table             |
| nfreqmax    | 4097            | Frequency maximum tracking         |

Constants:

- `HUFF_SIZE = 3200`
- `STR_IND_BUF_LEN = 8191`
- `MAX_REC_FREQUENCY = 4096`

### Huffman Decoding

The Huffman tree is traversed bit-by-bit:

1. Start at root (offset 0)
2. Read a bit: 0 = left child, 1 = right child
3. If the child node value has bit 0 set (odd), it's a leaf
4. Extract symbol from `sohn` array

### Tab Decode (Variable-Length Position Coding)

Position references use a special variable-length coding:

```text
1. Calculate base = freq - pos_offset
2. Get range: dx = (nfreq[base] - nfreq[base+2]) >> 1
3. Build value using adaptive bit reading:
   - Read bits until value > dx
   - XOR to extract final value
4. Look up actual position in nvalue table
5. Update frequency tables (adaptive)
```

### String Table (str_ind_buf)

The string table stores back-references for LZ77-style decompression:

- Size: 8191 entries
- Each entry is a 16-bit word
- Entries encode character/string relationships

### Output Generation

The `decode_data` phase traverses the string table to produce output:

1. Start from the last entry
2. Follow references backward
3. Push partial results onto a stack
4. Emit literal bytes when lowest bit is set
5. Continue until stack is empty

### Block Continuation

Multiple blocks may be present in the stream:

- After each block, `low_tsi` advances to continue from where the previous block ended
- Decompression continues until `tsi == 255` or all output bytes are produced
- The string table may be cleared when approaching capacity (`clear_when_full`)

## Implementation Notes

### Memory Requirements

The decompressor requires approximately:

- ~64 KB for Huffman tree arrays
- ~32 KB for string index buffer
- ~32 KB for nvalue/frequencys tables
- ~8 KB for other state

Total: ~140 KB working memory

### Original Implementation

The original implementation was written in Borland Pascal, later ported to 16-bit x86 assembly (UNPACK.ASM, COMM.ASM) and also to C for mainframe systems. The assembly version uses segment-based memory addressing with tables stored in separate 64KB segments.

### Endianness

All multi-byte values are stored in little-endian format.

## Acknowledgments

Special thanks to Peter Sawatzki, one of the original authors, who kindly provided the original Assembler source code. This was invaluable for understanding the compression algorithm and verifying the reimplementation.

## References

- Original source code provided by Peter Sawatzki
- HYPER.EXE disassembly
- unarc-rs implementation: `src/hyp/hyp_unpack.rs`
