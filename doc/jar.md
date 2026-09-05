# JAR 1.02 (DOS/Windows, not Java)

JAR by ARJ Software uses a **Huffman → LZ symbols → word/binary transform**
decoder. Compression presets m1, m2, m3 and m4 share this wire format. It is
not ARJ method 4. This implementation runs entirely in Rust, without executing
the reference EXE or requiring Wine/DOSBox.

## Verified support

- Ordinary, unencrypted, single-volume JAR 1.02 archives.
- Solid files, empty files, directories and multiple Huffman blocks.
- Huffman literal/length codes, direct symbol distances, repeated distances,
  recent special symbols and recent matches.
- Fixed English word dictionary, case/suffix codes, CRLF and run-length codes.
- Zero-interleaving (UTF-16-like) and interleaved 16-bit delta transforms.
- Actual filenames, sizes, modification timestamps and directory attributes.
- Header, file-record and metadata-record CRC verification.
- Unified API listing, reading and skipping (solid data is decoded once and cached).

Encryption, multi-volume archives, chapter/update histories, recovery data and
nonstandard/custom dictionaries are **not claimed as supported**. Other engine
versions and unrecognized stream layouts return errors. DOS high-byte filename
encoding currently uses a byte-preserving Latin-1 mapping, not code-page autodetection.

## Archive header (64 bytes)

All integers are little-endian.

| Offset | Bytes | Meaning |
| --- | --- | --- |
| 0x00 | 4 | Rotated header CRC |
| 0x04 | 4 | Archive data offset (normally 64) |
| 0x08 | 4 | Data region size, including compressed metadata and trailing CRC |
| 0x0E | 4 | `1A 4A 61 72` (`\x1aJar`) |
| 0x12 | 2 | Format version, 27 |
| 0x18 | 4 | Auxiliary header field; not a file CRC |
| 0x1C | 4 | Header flags/auxiliary field |
| 0x20 | 4 | Metadata offset **relative to data offset** |
| 0x24 | 4 | Auxiliary checksum field; semantics not yet established |

The old `JarHeader::uncompressed_size` field name is retained for compatibility,
but its value is the metadata offset, **not** the expanded data size. Use
`directory_offset()` or `total_original_size()` instead. Per-file compressed
sizes are reported as zero because they are not independently attributable in
a solid stream. Original sizes come from validated metadata.

Header CRC: zero bytes 0..4, compute standard reflected CRC-32 over all 64
bytes, complement the finalized result, then rotate left by 21 bits.
Original routines: `0x42e790`, `0x42e870`, `0x417400`.

## Packet framing

Each solid stream starts with `19 96 05 30 16 30`. Packets follow:

| Bytes | Meaning |
| --- | --- |
| 2 | Payload length |
| 1 | Flags: bit 0 first packet, bit 1 last packet, bit 2 metadata |
| length | Payload bytes |

Remove the packet headers and concatenate their payloads before reading bits.
In the original license fixtures the first payload has 360 bytes; subsequent
packets often have 2432. Packet boundaries have **no Huffman significance**.

The concatenated payload starts with a ten-byte engine header. Byte 0 is 0x10,
byte 1 is the engine revision, bytes 2..4 hold a window size in Ki-symbols,
and bytes 4..6 contain engine flags. Bit 0 disables the word dictionary; bit 1
disables the recent-symbol/match models. The Huffman stream begins at byte 10.

The former interpretation of payload byte 20 as `'A'`, `'B'`, `'C'` or `'\''`
was incorrect: **those bytes are already Huffman table bits**, not compression
method indicators. In particular the original small `test.j` is not stored.

## Huffman/LZ stage

Original routines: table reader `0x422d70`, tree construction `0x416410`,
symbol decoder `0x422fb0`, match decoder `0x423140`.

Bits are LSB-first. Each block begins at a byte boundary with marker 1; marker
0 ends the stream. A run-coded code-length table for 768 symbols is followed
by a 16-bit symbol count, then the encoded operations. Code lengths map to
weights `0x100000 >> length`. The original tree's tie-breaking order matters.

- 0x000..0x107: literal 16-bit symbols.
- 0x108..0x10F: upper three bits of a 16-bit symbol plus 13 extra bits.
- 0x110..0x11F: match using the remembered distance.
- 0x120..0x2FF: low nibble selects a length or recent-history operation.
- Low nibble 0: recall one special symbol (>=0x108).
- Low nibble 15: recall a previous match's position and length.
- Other nibbles: direct symbol distance and match length.

The two recent histories each have 0x1010 slots. Only matches of length >=4,
or length 2/3 containing a symbol >=0x108, enter the recent-match history
(`0x40fd20`). Direct distances are **not** limited to 0x1010. Overlapping
copies must work. The final expanded symbol block contains zero padding to
an 8192-symbol boundary after the 0xA00 end record.

The implementation's full 8192-symbol output for `license_m1.j` was compared
byte-for-byte with isolated original routines executed in an x86 emulator.

## Word and binary transforms

Original routines: `0x42f380`, `0x42f540`, `0x41d2e0`, `0x41d390`.

- <0x100: byte literal; 0x104: CRLF.
- 0x800..0x8FF: repeat low byte six times.
- 0x900..0x9FF: repeat low byte 32 times.
- >=0x2000: fixed dictionary word, with optional suffix and case modifier.
- A following 0x100 suppresses a word's implicit trailing space; 0x101
  uppercases the whole word; 0x102 uppercases its first letter. Both case
  modifiers also suppress the implicit space.
- 0xB00 pairs enclose word-coded bytes to expand into `(byte, 0)` pairs.
- 0xB01 pairs enclose two initial LE16 samples followed by two arrays of
  signed 8-bit deltas. Reconstruct alternating wrapping LE16 samples.

### Fixed dictionary provenance

`crates/unarc-rs/src/jar/dictionary.bin` is the 65536-byte format dictionary
from the supplied JAR 1.02 reference. It contains the code-to-word data needed
for interoperability, not executable code. Its prefix index and suffix-coded
records are interpreted by the independent Rust decoder.

Reproduction details: in JAR32.EXE the named PE resource JARSPEC/JARSPEC has
RVA 401520 and size 81983. Its last 32 bytes encode eight resource sizes.
The decoded sizes are `(835, 65536, 480, 13102, 1998, 0, 0, 0)`. Resource 1,
starting 835 bytes into JARSPEC, is the dictionary. Decode the size table and
dictionary independently using original routine `0x421a50`: for byte `v` at
local offset `i`, let `n = i & 15`, then output
`((v ^ (255 - n)) + n + 1) & 255`.

## Logical records and metadata

0xE02 precedes file bytes; 0xE01 precedes a trailer consisting of year (LE16),
month, day, directory flag, and NUL-terminated name. Empty files/directories
may omit 0xE02. 0xE03 carries a four-byte CRC over the preceding file tags
(LE16), file bytes and trailer. 0xE04 separates files/blocks and is excluded
from the file CRC. JAR stores **unfinalized** reflected CRC-32 (`!crc32fast::hash`).
0xA00 terminates logical records; subsequent symbols are zero padding.

The metadata stream contains a tree: 0xE06 opens a group (LE32 id and optional
name), 0xE07 introduces a typed record (LE16 type), and 0xE08 closes a group.
File groups contain type 0x100 metadata and type 0x101 NUL-terminated names.
The hierarchy is `ABL -> solid block ID -> file ID`. File IDs restart at zero
in each block; the pair `(block ID, file ID)` identifies an entry. These IDs
give the solid stream/file order, which can differ from tree order. Named BIN
children contain block information, not files; VIN/CIN/AIN are separate trees.
Names use backslashes; the Rust API normalizes them to slashes.

Offsets within the 95-byte type-0x100 record, **including the type**:

| Offset | Meaning |
| --- | --- |
| 2, 13, 24 | Three 11-byte timestamps (year/month/day/hour/minute/second/fraction) |
| 35 | Attributes (LE32); bit 0 archive, bit 4 directory |
| 39 | Original size low DWORD |
| 43 | Original size high DWORD (currently must be zero) |
| 51 | File record CRC |

Metadata has its own 0xE03 checksum; the same checksum occurs as a trailing
raw DWORD after the packet-framed metadata stream. Both are checked.

## Limits and regression coverage

`JarArchive` defaults to a 256 MiB expanded-stream limit, configurable with
`set_output_limit()`. Metadata is limited to 16 MiB and 256 nesting levels.
Malformed packets, truncated bitstreams, oversized table runs, invalid
back-references and checksum failures return errors, never fabricated output.

Tests require all fixtures (no silent skips). They compare all four license
archives to the complete repository LICENSE and all four new multi-file
archives to their original bytes. The latter include random data, a 272568-byte
text, UTF-16LE, delta-coded samples, empty entries and a nested directory.
Corruption/truncation, size limits and unified listing/reading are also tested.