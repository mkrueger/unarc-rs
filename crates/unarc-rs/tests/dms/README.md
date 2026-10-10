# Self-authored DMS fixtures

The disks contain only the repository's `LICENSE`, `FILE_ID.DIZ` from this
directory, generated text and zero bytes. No Amiga software or third-party
disk content is included.

- `modes.dms`: the `ofs.adf` disk below. Tracks 40-48 (the file data) use
  NOCOMP, SIMPLE, QUICK, MEDIUM, DEEP, HEAVY1, HEAVY2, DEEP and MEDIUM; the
  other tracks use runs of seven tracks per mode that carry the decoder state
  to the next track. HEAVY tracks alternate with and without the RLE stage.
  It also has a banner, a 1024-byte fake boot block track, an attached
  FILE_ID.DIZ (track 80) and a second banner after the disk.
- `hd.dms`: the high-density `hd.adf` disk, DEEP (tracks 0-39) and HEAVY2.
- `ndos.dms`: an 80-track non-AmigaDOS disk (boot block `NDOS`, some text in
  tracks 0 and 40), HEAVY1, with an attached FILE_ID.DIZ.
- `partial.dms`: only tracks 0, 1 and 40 of `ofs.adf`, MEDIUM.
- `encrypted.dms`: `ofs.adf` in HEAVY2, encrypted with the password `unarc`,
  with a banner (encrypted) and a FILE_ID.DIZ (never encrypted).

The banner and attached FILE_ID.DIZ texts are listed in `tests/common/dms.rs`.

## Reproduction

The disks were made with `xdftool` from `amitools` 0.8.1, run from the
repository root in a fresh directory:

```sh
for i in $(seq 1 3000); do echo "line $i of the unarc-rs DMS test file"; done > lines.txt

xdftool ofs.adf format DMSTest DOS0 + write LICENSE \
  + write crates/unarc-rs/tests/dms/FILE_ID.DIZ FILE_ID.DIZ \
  + makedir docs + write lines.txt docs/lines.txt
TZ=UTC xdftool ofs.adf time LICENSE '16.05.2024 23:08:26'

xdftool hd.adf create type=adf_hd + format HDTest DOS1 + write LICENSE \
  + write crates/unarc-rs/tests/dms/FILE_ID.DIZ FILE_ID.DIZ
```

No open-source DMS packer exists, so the archives were written by a small
packer made for these tests from the decoder's behaviour. Each archive was
checked with independent decoders: `xdms u` (xDMS 1.3.2) unpacks `modes.dms`,
`hd.dms` and, with `-p unarc`, `encrypted.dms` to images identical to
`ofs.adf` and `hd.adf`, and `partial.dms` to tracks 0, 1 and 40 of `ofs.adf`;
`xdms d` and `xdms b` print the attached texts. `lsar -t` (The Unarchiver
1.10.1) also verifies every archive except `encrypted.dms`.
