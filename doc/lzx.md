# LZX (Amiga)

LZX is the Amiga archiver by Jonathan Forbes and Tomi Poutanen, released in
1995 and common on Aminet and Amiga BBSes. Archives are read through
`ArchiveFormat::Lzx` and the `lzx` module, with a pure Rust decoder.

Amiga LZX is **not** the LZX method of Microsoft Cabinet files. The CAB method
descends from it, but the bitstream differs in bit order, block types, tree
encoding, match symbols and repeated offsets, so the `lzxd` crate used for CAB
cannot read it.

## Verified support

- Pack modes 0 (stored) and 2 (LZX). Other modes
  are listed with the method `Unknown(n)`; reading them returns
  `ArchiveError::UnsupportedMethod`.
- Merged groups, where several files are compressed as one stream. Reading a
  file decodes its group up to the end of that file; reading the files of a
  group in order continues where the last read stopped, and reading an earlier
  file restarts the group. Skipping never decodes anything. See
  [Size limits](#size-limits) for how the decoded data is bounded.
- Block types 1 (reuse the previous tree), 2 (verbatim) and 3 (aligned offsets).
- The CRC-32 of every file (always verified, as in the other formats;
  `ArchiveOptions::with_verify_crc` is not consulted) and of every entry header.
- Amiga protection bits, file comments and dates. Names and comments are
  mapped byte for byte as Latin-1. Entries named with a trailing `/` are
  directories.

The unified API reports a compressed size of 0 and the method `LZX (merged)`
(or `Stored (merged)`) for files that share a stream with other files, since
they have no compressed size of their own. Dates before 1980 cannot be
represented as `DosDateTime` and are reported as missing.

### Size limits

A read never returns more than the entry's recorded size. To reach an entry of a
merged group, the decoder also decodes and discards the group's data before
it, unless an earlier read stopped there. That work is bounded too:

- The limit of a read (`read_with_limit`, or the unified API's entry limit,
  `ArchiveOptions::with_max_entry_size`) covers the discarded bytes as well as
  the entry. Both are known before decoding starts, so a read that would
  exceed the limit fails at once with `ArchiveError::SizeLimitExceeded`.
- The unified API charges discarded bytes, and the output of reads that fail
  (for example on a CRC mismatch), to `ArchiveOptions::with_max_total_size`.

Reading a group's files in order decodes the group once. Reading them in
reverse order decodes it once per file, so without a total limit the work grows
with the number of files times the group size; set a total limit when reading
untrusted archives out of order.

A damaged header ends the listing: the entries before it are returned, then
`get_next_entry` (or `next_entry`) returns the error. Files of a merged group
whose data never follows (because of a damaged header or a truncated archive)
are listed, but reading them fails unless they are empty. Fewer than 31
trailing bytes, such as transfer-protocol padding, are ignored.

## Archive structure

```text
info header      10 bytes
entry header     31 bytes, then the name and the comment
packed data      the entry's packed size, possibly 0 (see merged groups)
entry header ...
```

All multi-byte fields are little-endian, except the date.

### Info header

| Offset | Bytes | Field |
| --- | --- | --- |
| 0x00 | 3 | `LZX` |
| 0x03 | 7 | Version and flags, such as `00 0C 00 0A 04 00 00` |

Of this header, opening an archive checks only the signature (every entry header's
CRC is verified as the entries are read in). Format detection also requires a
first entry header with a valid header CRC, unless its name and comment extend
past the 512 bytes detection reads.

### Entry header (31 bytes)

| Offset | Bytes | Field |
| --- | --- | --- |
| 0x00 | 1 | Protection bits |
| 0x02 | 4 | Uncompressed size |
| 0x06 | 4 | Packed size |
| 0x0A | 1 | Machine type (10) |
| 0x0B | 1 | Pack mode: 0 stored, 2 LZX |
| 0x0C | 1 | Flags: bit 0 set on every entry of a merged group |
| 0x0E | 1 | Comment length |
| 0x0F | 1 | Extract version (10) |
| 0x12 | 4 | Date, big-endian |
| 0x16 | 4 | CRC-32 of the uncompressed data |
| 0x1A | 4 | CRC-32 of the header (with this field zeroed), name and comment |
| 0x1E | 1 | Name length |

The name and the comment follow the header. Both CRCs are the standard
(zlib) CRC-32.

Protection bits are stored with the read/write/execute/delete bits set when
the action is *allowed*: bit 0 read, 1 write, 2 delete, 3 execute, 4 archived,
5 hidden, 6 script, 7 pure. LZX writes `0x0F` (`----rwed`) for an ordinary
file. `LzxEntry::protection()` formats them as the Amiga `list` command does.

### Dates

The 32-bit big-endian date holds, from the most significant bit: day (5 bits),
month - 1 (4 bits), year (6 bits), hour (5 bits), minute (6 bits) and
second (6 bits). The 6-bit year wraps around; the year-2000 fixed versions of
LZX use this mapping, which is also correct for every pre-2000 archive:

| Year field | Years |
| --- | --- |
| 8-29 | 1978-1999 (field + 1970) |
| 58-63 | 2000-2005 (field + 1942) |
| 30-57 | 2006-2033 (field + 1976) |
| 0-7 | 2034-2041 (field + 2034) |

Unpatched tools, including `unlzx`, always add 1970, so they show an archive
from 2001 (field 59) as 2029.

### Merged groups

When LZX merges files, it compresses them as one stream. Every entry of the
group has flag bit 0 set and a packed size of 0, except the last, whose packed
size is that of the whole stream, which follows its header. The stream decodes
to the files' contents back to back. Its pack mode is taken from the last
entry. An entry without packed data joins the group of the next entry with
packed data, as in LZX's own extractor, except that an empty entry without
the merged flag (an empty file or a directory) stands alone.

## Compressed stream (pack mode 2)

Each stream (a file, or a merged group) is decoded independently. The window
is 64 KiB and starts zero-filled: LZX 1.21 does match against that zeroed
history at the start of a stream, so a decoder that keeps the previous
stream's window produces wrong data there (this is why `unlzx` reports CRC
errors for some files that unar and this crate extract correctly).

### Bit input

Packed data is read as big-endian 16-bit words, and the bits of each word from
the least significant up. Multi-bit values are read least significant bit first.
Huffman codes are canonical: codes are assigned in order of code length, then
symbol value, and the first bit read is the code's most significant bit.

### Blocks

| Bits | Field |
| --- | --- |
| 3 | Block type: 1 reuse the previous tree, 2 verbatim, 3 aligned offsets |
| 8 x 3 | Type 3 only: code lengths of the 8-symbol aligned-offset tree |
| 3 x 8 | Number of bytes the block decodes to, most significant byte first |
| ... | Types 2 and 3: the main tree (see below) |
| ... | Literals and matches |

A match may end past the end of its block; the next block starts after it.

### Main tree

The main tree has 768 symbols: 256 literals and 512 match symbols. Its code
lengths (0-16) persist from block to block, starting at 0, and each new tree
is sent as deltas against them, in two sections: lengths 0-255, then 256-767.
Each section starts with 20 four-bit lengths for a pretree, then pretree
symbols:

| Symbol | Meaning | Literal section | Match section |
| --- | --- | --- | --- |
| 0-16 | New length = (old length - symbol) mod 17 | | |
| 17 | Run of zero lengths | 4 bits + 4 | 4 bits + 3 |
| 18 | Run of zero lengths | 5 bits + 20 | 6 bits + 19 |
| 19 | Run of equal lengths: a pretree symbol 0-16 follows, applied to the first length of the run | 1 bit + 4 | 1 bit + 3 |

Runs that extend past their section are errors.

### Literals and matches

A main symbol below 256 is a literal byte. For a match, subtract 256: the low
5 bits are the position slot and the next 4 bits the length slot. Both slots
use the same tables:

| Slot | 0-3 | 4-5 | 6-7 | 8-9 | ... | 30-31 |
| --- | --- | --- | --- | --- | --- | --- |
| Base | slot | 4, 6 | 8, 12 | 16, 24 | ... | 32768, 49152 |
| Footer bits | 0 | 1 | 2 | 3 | ... | 14 |

The position footer follows the symbol, then the length footer. The match
length is the length slot's base plus its footer plus 3 (3-258 bytes). The
offset is the position slot's base plus its footer; in type 3 blocks, a footer
of 3 or more bits is sent as its upper bits, then its low 3 bits as an
aligned-offset tree symbol. An offset of 0 (position slot 0) repeats the
previous match offset, which starts at 1 and persists from block to block.

## Sources

The decoder and this description were written from the specification
documents (`ALGORITHM.md`, `CONSTANTS.md`) of the
[amiga-lzx](https://github.com/bitplane/amiga-lzx) project (WTFPL), and checked
against archives with `unar`/`lsar` 1.10 and `unlzx` 1.1 used only as
black-box references. No code from those projects was used: `unlzx.c` carries
no license, and unar's XAD code is LGPL.

The format details that the documents leave open were settled by testing
against real archives: the zero-filled initial window, the equal-length run
(symbol 19) taking its delta from the first length of the run, and empty
entries that are not merged.
