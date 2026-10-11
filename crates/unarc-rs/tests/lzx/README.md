# Self-authored LZX (Amiga) fixtures

The fixtures contain only the repository's LICENSE (11357 bytes) and the
synthetic files listed below. All compressed streams were written by the
independent [amiga-lzx](https://github.com/bitplane/amiga-lzx) encoder 0.1.5
(`lzx -2`). Both fixtures were checked with `unlzx` 1.1 (`crc good` for every
file, output compared byte for byte with the inputs) and with unar/lsar 1.10
(`lsar -t`: all passed; extracted files compared byte for byte).

## license.lzx

LICENSE as written by the encoder, unmodified:

```sh
cp LICENSE work/ && TZ=UTC touch -d '2026-10-10 12:34:56' work/LICENSE
cd work && TZ=UTC lzx -2 c ../license.lzx LICENSE
```

## multi.lzx

amiga-lzx never merges files, so a short script assembled this archive: each
compressed stream is the packed data of an archive written by `lzx -2 c` from
the stream's contents, placed after hand-written entry headers (info header
`LZX\0\x0C\0\x0A\x04\0\0`, machine type and extract version 10, both CRCs
computed as described in doc/lzx.md). The merged group's stream holds its four
files back to back.

| Entry | Pack mode | Contents | Protection | Date |
| --- | --- | --- | --- | --- |
| README.TXT | 0, comment `Stored entry with a comment` | `Self-authored LZX test fixture for unarc-rs.\n` | `----rwed` | 2026-10-10 12:34:56 |
| docs/ | 0 | Empty (directory) | `----rwed` | 2026-10-10 12:34:56 |
| docs/LICENSE | 2, merged | LICENSE | `----rwed` | 1999-12-31 23:59:58 |
| docs/words.txt | 2, merged | 30000 words, see below (194526 bytes) | `----rw-d` | 2001-02-03 04:05:06 |
| docs/empty | 2, merged | Empty | `----rwed` | 2026-10-10 12:34:56 |
| docs/binary.dat | 2, merged, last | Bytes 0..255, repeated 160 times | `hspa----` | 2041-12-31 00:00:00 |
| EMPTY | 2 | Empty, not merged | `----rwed` | 2026-10-10 12:34:56 |
| `caf\xE9.txt` | 2 | `Latin-1 name\n`, 20 times | `-s--rwed` | 2026-10-10 12:34:56 |

The dates cover all four ranges of the 6-bit year field. The merged stream is
246843 bytes long, so its matches wrap around the 64 KiB window.

words.txt is generated from the whitespace-separated words of LICENSE with
`state = (state * 1103515245 + 12345) mod 2^31`, starting from `state = 1`:
each step appends word number `state mod word_count`, followed by a newline
after every 12th word and a space otherwise (see `words()` in
lzx_decompression.rs).

The encoder writes only verbatim blocks (type 2). Aligned-offset (type 3) and
tree-reuse (type 1) blocks are covered by unit tests in `src/lzx/decoder.rs`
that build bitstreams directly.
