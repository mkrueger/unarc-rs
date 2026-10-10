# Lynx (LNX) Container Format

## Overview

Lynx was written by Will Corley for the Commodore 64 and cloned by many others
("Ultimate Lynx" and various "LYNX" versions). It links files together without
compression. Everything is organised in blocks of 254 bytes, the data part of a
1541 sector, so the container can be split again on a real C64 by rewriting
sector links.

## File Structure

```text
┌─────────────────────────────────────┐
│ BASIC loader (optional)             │  "USE LYNX TO DISSOLVE THIS FILE"
│ Header lines                        │
│ Directory entries                   │
│ Padding up to the directory size    │  directory = N blocks of 254 bytes
├─────────────────────────────────────┤
│ File 1 (padded to 254-byte blocks)  │
│ File 2 (padded to 254-byte blocks)  │
│ ...                                 │
│ Last file (usually not padded)      │
└─────────────────────────────────────┘
```

All offsets are relative to the start of the file, including the BASIC loader.

### BASIC loader

Most containers start with a small BASIC program (load address `$0801`) that
tells the user to run Lynx. Its length and text vary. It ends with the BASIC
end-of-line and end-of-program markers, three zero bytes, followed by a
carriage return (`$0D`).

### Header

The header and directory are text, written with `PRINT#`: numbers are decimal,
usually with a space on each side, and fields end with a carriage return.

```text
 1  *LYNX XV  BY WILL CORLEY<CR>     directory size in blocks, then the signature
 4 <CR>                              number of directory entries
```

The signature normally contains `LYNX`. The directory size counts 254-byte
blocks from the start of the file, so the first file starts at offset
`blocks * 254`.

### Directory entry

```text
GAME<12 x A0><CR>                    file name, PETSCII, up to 16 bytes, padded with $A0
 31 <CR>                             size in 254-byte blocks
P<CR>                                file type: P, S, U, R or D
 32 <CR>                             REL only: record length
 210 <CR>                            last block size + 1 (the 1541 "LSU" byte)
```

A file of `n > 0` blocks with last block value `l` is
`(n - 1) * 254 + l - 1` bytes long. Each file starts at the next block
boundary after the previous one.

REL files store their side sectors in front of the data. With `n` blocks
(side sectors included) there are `(n + 119) / 121` side sectors; they are
skipped, and the remaining blocks hold the record data.

## Implementation notes

- **Detection.** `detect_from_bytes` parses the header either at the start of
  the file or behind the first run of three zero bytes within the first 512
  bytes (the end of the BASIC loader), and requires the signature to contain
  `LYNX`. The header must be a positive block count followed by a positive
  entry count, each on its own line, so random data is very unlikely to match.
  Containers with a missing or different signature are still opened through
  the `.lnx` extension; the reader looks for the loader end in the first 1024
  bytes.
- **Variants.** Containers with and without the BASIC loader, with and without
  spaces around the numbers, and with REL and zero-length DEL entries are
  supported. If the last entry has no last block size, the last file extends
  to the end of the container (at most its block count). Containers created by
  "Ultimate Lynx" that are not aligned to 254-byte blocks cannot be detected
  as such and extract incorrectly, as with other tools.
- **Robustness.** Only the directory blocks (at most 1 MiB) are read into
  memory. Entry sizes come from the directory; a file extending past the end of
  the container fails with `ArchiveError::CorruptedEntry` when read, and no
  buffer is allocated from an untrusted size. The one exception is the last
  file: some archivers (Lynx XVI, Star Lynx) end the container a few bytes short
  of its recorded size. When the container ends inside that file's last block,
  the file is cut to the bytes present, as c1541 and cbmconvert do. All earlier
  data blocks and at least one byte of the last data block must be present;
  REL side sectors do not count as data blocks. A malformed entry returns
  `ArchiveError::CorruptedEntry` and ends the iteration, since the following
  entries cannot be located.
- **Contents.** PRG files include their load address, as on disk. REL files
  yield their record data; the record length is available in `LynxEntry`.
- **Names.** See [D64 entry names](d64.md#entry-names).

## References

- Peter Schepers, *LNX.TXT* (Lynx container format)
- VICE `c1541` `unlynx` command (`src/c1541.c`)
- cbmconvert `lynx.c`
