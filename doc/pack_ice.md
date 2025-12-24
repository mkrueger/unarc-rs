# Pack-Ice Compression Format

## Overview

Pack-Ice is a compression format developed for the Atari ST platform, primarily used for compressing executable files and picture data (e.g., Degas Elite `.PI?` formats). It was one of the most popular packers on the Atari ST demo scene in the late 1980s and early 1990s.

Pack-Ice uses a backward LZ77 variant with variable-length codes, reading the bitstream from the end of the file toward the beginning. This allows in-place decompression without requiring a separate output buffer in memory-constrained environments.

**Note:** This format is unrelated to the DOS-era "ICE" format by Michael Lamanuzzi (see [ice_dos.md](ice_dos.md)).

## Versions

Pack-Ice evolved through three major versions, each with different header formats and bitstream variations:

| Version | Magic/ID | Header Location | Notes |
|---------|----------|-----------------|-------|
| v0 (1.1–1.14) | `Ice!` | Footer (last 4 bytes) | Earliest format |
| v1 (2.0–2.20) | `Ice!`, `TMM!`, `TSM!`, `SHE!` | Header (first 4 bytes) | 32-bit bitstream |
| v2 (2.31+) | `ICE!` | Header (first 4 bytes) | 8-bit bitstream |

### Variant IDs

Several applications shipped their own Pack-Ice variants with custom magic bytes:

| Magic | Origin |
|-------|--------|
| `Ice!` | Standard Pack-Ice |
| `ICE!` | Pack-Ice 2.31+ |
| `TMM!` | Demo "Numb" by Movement |
| `TSM!` | Various Amiga games |
| `SHE!` | Demo "Overload2" by JetSet |

## File Structure

### Version 0 (Footer-based)

```text
┌────────────────────────────────────┐
│ Compressed data                    │
│ (variable length)                  │
├────────────────────────────────────┤
│ Raw size (4 bytes, big-endian)     │  offset: file_size - 8
├────────────────────────────────────┤
│ Magic "Ice!" (4 bytes)             │  offset: file_size - 4
└────────────────────────────────────┘
```

| Offset | Size | Description |
|--------|------|-------------|
| 0 | N-8 | Compressed bitstream |
| N-8 | 4 | Original (uncompressed) size, big-endian |
| N-4 | 4 | Magic bytes `Ice!` (0x49636521) |

### Version 1 & 2 (Header-based)

```text
┌────────────────────────────────────┐
│ Magic (4 bytes)                    │  "Ice!", "ICE!", "TMM!", etc.
├────────────────────────────────────┤
│ Packed size (4 bytes, big-endian)  │
├────────────────────────────────────┤
│ Raw size (4 bytes, big-endian)     │
├────────────────────────────────────┤
│ Compressed data                    │
│ (packed_size - 12 bytes)           │
└────────────────────────────────────┘
```

| Offset | Size | Description |
|--------|------|-------------|
| 0 | 4 | Magic bytes (see table above) |
| 4 | 4 | Total packed size including header, big-endian |
| 8 | 4 | Original (uncompressed) size, big-endian |
| 12 | varies | Compressed bitstream |

## Bitstream Format

Pack-Ice uses a backward bitstream: data is read from the end of the compressed block toward the beginning. Output is also written backward, from the end of the output buffer toward the start.

### Bit Reading

- **Version 0 & 1**: Bits are read from big-endian 32-bit words (MSB first)
- **Version 2**: Bits are read from individual bytes (MSB first)

### Anchor Bit

The bitstream begins with an "anchor bit" mechanism to synchronize the bit reader:

1. Read initial value (32-bit for v0/v1, 8-bit for v2)
2. Count leading zero bits from the MSB
3. The anchor bit (first `1` bit) is discarded
4. Remaining bits become the initial bit buffer

```text
Example (32-bit): 0x00123456
  Binary: 0000 0000 0001 0010 0011 0100 0101 0110
  Leading zeros: 11
  Anchor bit: bit 12 (the first '1')
  Initial buffer: remaining 11 bits
```

## Decompression Algorithm

The main loop alternates between literal runs and back-references:

```text
while not finished:
    if read_bit() == 1:
        lit_length = decode_literal_length()
        copy lit_length bytes from input to output
    
    if output_complete:
        break
    
    count_base = decode_count_base()
    count = decode_count(count_base) + 2
    distance = decode_distance(count)
    copy count bytes from output[pos + distance] to output[pos]
```

### Variable-Length Code Tables

Pack-Ice uses cascading variable-length codes. Each decoder tries successive bit patterns until one matches.

#### Literal Length (v1/v2: `lit_vlc_new`)

| Bits | Value Range | Offset |
|------|-------------|--------|
| 1 | 0 | 0 |
| 2 | 1–2 | +1 from previous |
| 2 | 3–4 | +3 |
| 3 | 5–8 | +5 |
| 8 | 9–264 | +9 |
| 15 | 265–32767+ | +265 |

Result = decoded_value + 1

#### Literal Length (v0: `lit_vlc_old`)

| Bits | Value Range |
|------|-------------|
| 1 | 0 |
| 2 | 1–2 |
| 2 | 3–4 |
| 3 | 5–8 |
| 10 | 9–1032 |

#### Count Base

| Bits | Value |
|------|-------|
| 1 | 0 |
| 1 | 1 |
| 1 | 2 |
| 1 | 3 |

Uses cascade: reads 1 bit at a time; if bit is `1` and not the last entry, continue to next.

#### Count (indexed by count_base)

| Base | Bits | Offset |
|------|------|--------|
| 0 | 0 | 0 |
| 1 | 0 | 1 |
| 2 | 1 | 2 |
| 3 | 2 | 4 |
| (overflow) | 10 | 8 |

Final count = decoded_value + 2

#### Distance

For `count == 2`:
```text
if read_bit() == 1:
    distance = read_bits(9) + 0x40
else:
    distance = read_bits(6)
distance += count - (1 if use_bytes else 0)
```

For `count > 2`:
```text
dist_base = decode_distance_base()  # cascade: 1,1 bits
if dist_base < 2: dist_base ^= 1    # swap 0↔1

distance = decode_distance_offset(dist_base)
```

Distance offset table:

| Base | Bits | Range |
|------|------|-------|
| 0 | 5 | 0–31 |
| 1 | 8 | 32–287 |
| 2 | 12 | 288–4383 |

Final adjustment:
- v0/v1: `distance += count`
- v2: `distance += count - 1` (or `distance = 1` if decoded as 0)

## Picture Mode

Versions 1 and 2 support a "picture mode" postprocessing step for Atari ST screen data. After decompression:

1. Check if picture mode flag is set (1 bit after main decompression)
2. Determine picture size:
   - v1: Fixed 32000 bytes (320×200×4 bitplanes)
   - v2: Can specify custom size via 16-bit field
3. Apply C2P (chunky-to-planar) style transformation on the last `picture_size` bytes

The transformation reorders interleaved bitplane data for direct display on the Atari ST's planar video memory.

```text
For each 8-byte block in the picture area:
    Read 4 × 16-bit words (planes 0-3)
    Interleave bits across planes
    Write back as 4 × 16-bit words
```

## Detection

To detect Pack-Ice files:

1. Check first 4 bytes for `ICE!`, `Ice!`, `TMM!`, `TSM!`, or `SHE!`
2. If not found, check last 4 bytes for `Ice!` (v0 format)
3. Validate sizes are non-zero and within bounds

## Common File Extensions

| Extension | Description |
|-----------|-------------|
| `.ICE` | Generic Pack-Ice compressed file |
| `.PI1` | Compressed Degas Elite low-res picture |
| `.PI2` | Compressed Degas Elite medium-res picture |
| `.PI3` | Compressed Degas Elite high-res picture |
| `.PI9` | Compressed Degas Elite picture (alternate) |

## Implementation Notes

### Memory Layout

The backward decompression scheme allows in-place decompression:
1. Load compressed data at end of destination buffer
2. Decompress backward, output overwrites input as it becomes available
3. No separate buffer needed if `raw_size >= packed_size`

### Error Handling

- Validate magic bytes match known variants
- Check `packed_size <= file_size`
- Check `raw_size` is reasonable (e.g., < 16 MB)
- Verify complete input consumption after decompression
- Handle v1/v2 ambiguity by trying v1 bitstream first, falling back to v2

## References

- Original Pack-Ice by Axe of Delight (Atari ST)
- Ancient library by Teemu Suutari: https://github.com/temisu/ancient
- Atari ST demo scene archives
