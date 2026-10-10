# Microsoft Cabinet (CAB)

Cabinet files are the archive format of Microsoft's setup tools (`makecab`,
`cabarc`, `iexpress`), Windows installers, drivers and updates. The format is
documented in [MS-CAB]. Compressed data is decoded entirely in Rust: MSZIP with
flate2 and LZX with the [lzxd] crate.

[MS-CAB]: https://learn.microsoft.com/en-us/openspecs/exchange_server_protocols/ms-cab/
[lzxd]: https://crates.io/crates/lzxd

## Verified support

- Single cabinets, format version 1.x, with any number of folders and files.
- Compression types None, MSZIP and LZX (window sizes 2^15 to 2^21), including
  MSZIP history across data blocks and LZX E8 (x86 call) translation.
- Reserved header, folder and data-block areas.
- Data-block checksums (verified when non-zero).
- UTF-8 file names (`_A_NAME_IS_UTF`) and code-page names, which are mapped
  byte for byte as Latin-1; `\` separators are converted to `/`.
- DOS modification times, attributes, and the unified API's entry kinds.

Quantum compression is **not supported**: reading such an entry returns
`ArchiveError::UnsupportedMethod` with the method `Quantum:<window bits>`, as
do unknown compression types and LZX windows outside 2^15 to 2^21. Listing
still works.

Multi-cabinet sets are **not supported**. Files whose folder index marks them
as continued from or into another cabinet (`0xFFFD`, `0xFFFE`, `0xFFFF`) are
listed, but reading them returns `ArchiveError::UnsupportedFormat`; so does
reaching a data block that continues in the next cabinet. Self-extracting
cabinets inside executables are not detected.

## Archive structure

All integers are little-endian. Offsets are relative to the `MSCF` signature.

```text
CFHEADER     signature, sizes, counts, flags, optional reserve and set links
CFFOLDER[]   one per folder
CFFILE[]     one per file, at CFHEADER.coffFiles
CFDATA[]     data blocks of each folder, starting at CFFOLDER.coffCabStart
```

### CFHEADER

| Offset | Bytes | Field |
| --- | --- | --- |
| 0x00 | 4 | Signature `MSCF` |
| 0x08 | 4 | Size of the cabinet file |
| 0x10 | 4 | Offset of the first `CFFILE` |
| 0x18 | 1 | Minor version (3) |
| 0x19 | 1 | Major version (1) |
| 0x1A | 2 | Number of folders |
| 0x1C | 2 | Number of files |
| 0x1E | 2 | Flags: 1 previous cabinet, 2 next cabinet, 4 reserve present |
| 0x20 | 2 | Set ID |
| 0x22 | 2 | Index of this cabinet in the set |

With flag 4, a 16-bit header reserve size, an 8-bit folder reserve size and an
8-bit data reserve size follow, then the header reserve. Flags 1 and 2 each add
two NUL-terminated strings (cabinet name and disk name).

### CFFOLDER (8 bytes + folder reserve)

| Offset | Bytes | Field |
| --- | --- | --- |
| 0x00 | 4 | Offset of the first `CFDATA` block |
| 0x04 | 2 | Number of `CFDATA` blocks in this cabinet |
| 0x06 | 2 | Compression type |

The compression type holds the method in bits 0-3 (0 None, 1 MSZIP,
2 Quantum, 3 LZX), the Quantum level in bits 4-7 and the Quantum/LZX window
size as a power of two in bits 8-12.

### CFFILE (16 bytes + name)

| Offset | Bytes | Field |
| --- | --- | --- |
| 0x00 | 4 | Uncompressed size |
| 0x04 | 4 | Offset of the file's data in the uncompressed folder |
| 0x08 | 2 | Folder index, or 0xFFFD/0xFFFE/0xFFFF for continued files |
| 0x0A | 2 | DOS date |
| 0x0C | 2 | DOS time |
| 0x0E | 2 | Attributes: 0x01 read-only, 0x02 hidden, 0x04 system, 0x20 archive, 0x40 execute, 0x80 UTF-8 name |
| 0x10 | n | NUL-terminated name |

### CFDATA (8 bytes + data reserve + data)

| Offset | Bytes | Field |
| --- | --- | --- |
| 0x00 | 4 | Checksum (0 = none) |
| 0x04 | 2 | Compressed size |
| 0x06 | 2 | Uncompressed size, at most 32768; 0 if the block continues in the next cabinet |

The checksum XORs the block as little-endian 32-bit words (a trailing partial
word is combined most significant byte first), then the two size fields. The
specification includes the data reserve; libmspack does not, so both are
accepted.

## Compression

Each folder is a single stream: a file's data is found by decoding its folder
from the first block up to the file's offset.

- **MSZIP**: each block is `CK` followed by a complete raw Deflate stream that
  may reference the last 32 KiB of the folder's previous blocks.
- **LZX**: each block is one LZX frame of 32 KiB (the last may be shorter); the
  decoder state persists across the folder's blocks.

## Reading and limits

Reading a file decodes its folder block by block and keeps the position, so
reading the files of a folder in order decodes the folder once. Reading an
earlier file, or a file of another folder, starts that folder over.

Memory use is bounded per read: one decoded block (32 KiB), one compressed
block (at most 38 KiB), the MSZIP history or the LZX window (at most 2 MiB),
and the file's output. A file's recorded size is checked against the unified
API's size limits before decoding, output never exceeds that size, and a file
that would extend past the folder's block capacity is rejected without
decoding. Skipped predecessors in a folder are decoded and discarded, so the
decoding work for one file is bounded by its folder (at most 65535 blocks of
32 KiB), not by the size limits.
