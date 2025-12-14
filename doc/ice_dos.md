# ICE-DOS Archive Format

## Overview

Note: Has nothing to do with the popular Atari ST ICE.

ICE is a simple single-file compression format created by Michael Lamanuzzi in 1995. It uses LHA compression (specifically the `-lh1-` method) but without the standard LHA file header, making it a very compact format for storing a single compressed file.

The design goal of ICE was to be an "extremely tiny compression utility" - a universal file compressor with good compression ratios, especially effective on text and graphics files.

## File Structure

An ICE file consists of only two parts:

| Offset | Size | Description |
|--------|------|-------------|
| 0 | 4 bytes | Original (uncompressed) size, little-endian uint32 |
| 4 | variable | LHA-compressed data using `-lh1-` method |

## Header Details

### Original Size Field (Offset 0, 4 bytes)

The first 4 bytes contain the uncompressed file size as a 32-bit unsigned integer in little-endian byte order.

**Example:**

```text
5D 2C 00 00  →  0x00002C5D = 11,357 bytes
```

## Compression Method

ICE files use the LHA `-lh1-` compression algorithm:

- **Algorithm**: LZ77 with Huffman encoding
- **Dictionary size**: 4 KB (4096 bytes)
- **Offset bits**: 12 bits for dictionary position
- **Length encoding**: Huffman-coded

The `-lh1-` method was one of the earlier LHA compression methods, predating the more common `-lh5-` and `-lh7-` methods used in modern LHA archives.

## Characteristics

- **Single file only**: ICE archives contain exactly one file
- **No filename**: The original filename is not stored in the archive
- **No metadata**: No timestamps, permissions, or other file attributes
- **No CRC/checksum**: Data integrity is not verified by the format itself
- **Minimal overhead**: Only 4 bytes of header data

## Comparison with Standard LHA

| Feature | ICE | Standard LHA |
|---------|-----|--------------|
| Header size | 4 bytes | 21+ bytes |
| Multiple files | No | Yes |
| Filename stored | No | Yes |
| Timestamps | No | Yes |
| CRC checksum | No | Yes |
| Compression methods | -lh1- only | Multiple (-lh0- to -lh7-) |

## Implementation Notes

### Reading ICE Files

1. Read the first 4 bytes as little-endian uint32 (original size)
2. Read remaining bytes as compressed data
3. Initialize LH1 decoder with the compressed data
4. Decompress into a buffer of the original size
5. Verify the decompressed size matches the expected size

### Potential Issues

- Since there is no CRC, data corruption cannot be detected by the format
- The original filename must be derived from the archive filename or provided externally
- Large files (> 4 GB) are not supported due to the 32-bit size field

## Historical Context

ICE was created by Michael Lamanuzzi and released as freeware in 1995. The initial public release was version 1.02c on April 30, 1995.

The design philosophy of ICE was to be an "extremely tiny compression utility." Due to size constraints of the executable, ICE was intentionally limited to compressing only a single file per archive. The author noted that adding multiple file support would make the program "considerably larger."

ICE was described as a "universal file compressor" with good compression ratios, especially effective on text and graphics files. The trade-off was somewhat slower compression speed, which the author planned to optimize in future versions.

### Planned Features (never released)

The author planned to add these features in future versions:

- Multiple files in one archive
- Faster compression through optimization  
- Spanning large files across multiple disks

### Version History

| Version | Date | Notes |
|---------|------|-------|
| 1.00 | (not released) | Initial compile, mediocre speed and compression |
| 1.02a | (not released) | Much improved compression |
| 1.02b | (not released) | Bug fix in Huffman code lookup tables |
| 1.02c | April 30, 1995 | Initial public release, speed improvements |

## References

- ICE.DOC - Original documentation by Michael Lamanuzzi (1995)
- LHA compression algorithm documentation
