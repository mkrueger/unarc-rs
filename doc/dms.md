# DMS (DiskMasher) archives

DMS stores an Amiga floppy disk track by track, each track compressed on its own.
`ArchiveFormat::Dms` and the `dms` module decode the disk in memory and list what
is on it: the files of its AmigaDOS filesystem, read through the
[Amiga reader](amiga.md), or the raw disk image. Texts attached to the archive
(FILE_ID.DIZ and banners) are listed as extra entries, so a BBS can pick up the
description without unpacking the disk.

## Entries

- **Disk files.** A complete disk (tracks 0-79 in order, 11264 bytes each for
  double density or 22528 bytes for high density) that opens as an OFS/FFS ADF
  is listed file by file, exactly as `ArchiveFormat::Adf` lists it.
- **Disk image.** Any other disk is listed as one entry holding the decoded
  tracks. This covers non-AmigaDOS (NDOS) game disks with their own track
  loaders, disks with a `DOS` boot block but no root block, filesystems the Amiga
  reader does not support or rejects as damaged, and partial archives. Opened with
  `ArchiveFormat::open_path` (or the CLI), the entry is named after the archive,
  `game.dms` becoming `game.adf`; otherwise it is `disk.adf`.
  `UnifiedArchive::set_single_file_name` sets the stem as for `.gz` files.
- **FILE_ID.DIZ** (track 80) and **banners** (track 0xFFFF) are listed after the
  disk entries as `FILE_ID.DIZ` and `BANNER.TXT`. Archives often carry more than
  one banner (a BBS may append its own); later ones are `BANNER2.TXT`,
  `BANNER3.TXT`, and so on, and the same numbering avoids a clash with a file of
  the same name in the disk's root (`FILE_ID2.DIZ`). The texts are returned as
  stored, typically Latin-1 text with Amiga console escape sequences. Like xDMS,
  each text track is decoded on its own, not with the history of the disk tracks.

The 1024-byte "fake boot block" track that some archives store as track 0 is not
part of the disk and is not listed, as xDMS skips it too.

### Partial archives

DMS can store a subset of the tracks, such as the first or second half of a disk
or a few tracks of a game ("appends" archives). Like `xdms u`, the image holds
the stored disk tracks one after another in archive order. Missing tracks are not
zero-filled, so the image is shorter than a disk and track positions are not
preserved. (The Unarchiver instead lists each run of consecutive tracks as a
separate image; neither tool pads.) Such an image is never read as a filesystem.

## Compression modes

All seven modes are supported: NOCOMP, SIMPLE (run-length), QUICK, MEDIUM, DEEP
(adaptive Huffman), HEAVY1 and HEAVY2 (static Huffman, 4 and 8 KiB history).
QUICK, MEDIUM and DEEP are followed by the run-length stage, and HEAVY optionally.
Archives may switch mode per track and may carry the decoder history from one
track to the next; both are handled. Tracks with an unknown mode report
`UnsupportedMethod`.

## Integrity

The archive header CRC and each track header CRC are verified when the archive is
opened; a mismatch, a track larger than 32000 bytes or a truncated track fails
`open`. Data after the last track that is not a track header is ignored, as by
xDMS (some archives carry text or padding there).

Each track's packed data CRC and its unpacked checksum are verified while the
disk is decoded. A failure does not fail `open`: the disk is listed as an image
whose read returns `CrcMismatch` (packed data), `CorruptedEntry` (checksum) or
`DecompressionFailed`, while FILE_ID.DIZ and banners stay readable. xDMS stops at
the first damaged track and so never shows a FILE_ID.DIZ stored after it.

## Encryption

DMS encryption XORs the packed data with a 16-bit key that starts as the CRC of
the password and is updated from each encrypted byte. FILE_ID.DIZ is never
encrypted. Pass the password when opening (`ArchiveOptions::with_password`);
the encrypted entries report `EncryptionMethod::Dms`.

- Without a password, the disk is listed as an image whose read returns
  `EncryptionRequired`, as do the banners; FILE_ID.DIZ is readable.
- With a wrong password, decoding fails with `InvalidPassword` only until the
  key resynchronizes: because the key is updated from the encrypted bytes, it no
  longer depends on the password after a few bytes. In practice a wrong password
  garbles only a leading banner, and the disk behind it decodes, as with xDMS.

## Not supported

FMS archives (disk type 7, DMS's file-archive format) are rejected with
`UnsupportedFormat`. Several DMS archives concatenated into one file are not
split: reading stops at the second `DMS!` header.

## Resource bounds

Tracks are at most 32000 bytes packed and unpacked, and at most 80 disk tracks
are decoded (more fail the image with `CorruptedEntry`), so opening holds at most
about 2.5 MiB of decoded disk plus a 16 KiB history, whatever the headers claim.
Packed data is read only as far as the file goes. Each decoder stops when its
track is full. Banner and FILE_ID.DIZ tracks are kept packed until read. Reads
apply the per-entry and total limits of `ArchiveOptions`; listing reads no
file data beyond the decoded disk.

## Usage

```rust,no_run
use std::fs::File;
use unarc_rs::dms::{DmsArchive, DmsEntryKind};

let mut archive = DmsArchive::open(File::open("game.dms")?, None)?;
while let Some(entry) = archive.get_next_entry()? {
    if entry.kind == DmsEntryKind::FileId {
        let description = archive.read(&entry)?;
        println!("{}", String::from_utf8_lossy(&description));
    }
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

## References and validation

The decoders follow [xDMS](http://zakalwe.fi/~shd/foss/xdms/) 1.3.2 by Andre
Rodrigues de la Rocha and Heikki Orsila, which is public domain:

> xDMS is released as public domain software. You can spread it, modify it and
> use it in any way you like. You can do anything with it without even asking me
> first.

xDMS notes that its DEEP decoder derives from LZHUF by Haruyasu Yoshizaki and its
HEAVY decoder from UNIX LHA by Masaru Oki. The HEAVY Huffman tables here are
built with an independent canonical-code decoder rather than LHA's table builder.

The decoded images match `xdms u` byte for byte on 734 real-world archives
(HEAVY1, HEAVY2, SIMPLE and NOCOMP tracks, every archive xDMS unpacks without
error); the remaining 17 have a damaged track, reported by both. QUICK, MEDIUM
and DEEP, which that collection does not use, and encryption are covered by the
self-made fixtures, which xDMS unpacks to the original disks. See the
[fixture notes](../crates/unarc-rs/tests/dms/README.md).
