# Self-authored Amiga fixtures

Generated with `amitools` 0.8.1, using only the text in `payload.txt` and zero
bytes. No AmigaOS or third-party disk content is included.

- `ofs.adf`: DD OFS (DOS0), a directory, a text file, an empty file, and a
  102400-byte zero-filled file spanning multiple file-extension blocks.
- `ffs.adf`: the same contents in directory-cache FFS (DOS5).
- `flat.hdf`: 16 cylinders, one head, 32 sectors, filesystem-only DOS3, with
  `docs/readme.txt`.
- `partitions.hdf`: 162 cylinders, two heads, 11 sectors, RDB with DH0 (OFS) and
  DH1 (directory-cache FFS), populated from the two ADFs.
- `blocks.hdf`: 32 cylinders, one head, 32 physical 512-byte sectors, RDB with
  one 1024-byte-block FFS partition containing `readme.txt`.

## Reproduction

With `xdftool` and `rdbtool` on PATH, run from the repository root. Image
timestamps other than the explicitly set text-file date reflect creation time.
Run these commands in a fresh directory, or use the tools' `-f` option only for
images you intend to replace.

```sh
dd if=/dev/zero of=large.bin bs=512 count=200
dd if=/dev/zero of=empty.bin count=0

xdftool ofs.adf format Retro DOS0 \
  + makedir docs \
  + write crates/unarc-rs/tests/amiga/payload.txt docs/readme.txt \
  + write large.bin docs/large.bin + write empty.bin empty
TZ=UTC xdftool ofs.adf time docs/readme.txt '16.05.2024 23:08:26'

xdftool ffs.adf format Retro DOS5 \
  + makedir docs \
  + write crates/unarc-rs/tests/amiga/payload.txt docs/readme.txt \
  + write large.bin docs/large.bin + write empty.bin empty

xdftool flat.hdf create chs=16,1,32 + format Hardfile DOS3 \
  + makedir docs + write crates/unarc-rs/tests/amiga/payload.txt docs/readme.txt

rdbtool partitions.hdf create chs=162,2,11 + init \
  + addimg ofs.adf name=DH0 + addimg ffs.adf name=DH1

rdbtool blocks.hdf create chs=32,1,32 + init + fill bs=1024
xdftool blocks.hdf open part=DH0 + format LargeBlocks DOS1 \
  + write crates/unarc-rs/tests/amiga/payload.txt readme.txt
```

`xdftool IMAGE list` verifies flat-image listings. For the RDB use
`xdftool partitions.hdf open part=DH0 + list` and repeat with DH1.
Extract each partition with `read / DESTINATION` and compare its files to the
input text and zero-filled payload. Rust tests additionally construct images
for all DOS0-DOS5 variants, HD geometry, larger block sizes, and links.
