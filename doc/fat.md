# FAT12 floppy images (IMG / IMA / ST)

Read-only extraction of FAT12 filesystems from raw sector dumps of PC and
Atari ST floppy disks, through `ArchiveFormat::Fat` and the `fat` module.
No external tools or new dependencies are needed.

## Supported images

- Raw sector images (`.img`, `.ima`, `.st`) whose first sector holds a BIOS
  parameter block (BPB): 512 to 4096 bytes per sector, power-of-two clusters of
  up to 64 KiB, one to four FATs. This covers PC 160K to 2.88M floppies, PC-98
  1.2M disks with 1024-byte sectors, and Atari ST single- and double-sided disks.
- Atari ST boot sectors: a 68000 branch instead of an x86 jump, boot code where PC
  disks keep the 32-bit sector count, no `0x55AA` signature, and any media byte.
  The boot sector checksum is ignored; the initial 68000 branch selects Atari
  short-name decoding when no encoding is explicitly selected.
- DOS 1.x disks without a BPB (160K, 180K, 320K, 360K), recognised by their
  image size and the media byte at the start of the FAT.
- Nested directories, empty files, and fragmented files. Only the first FAT is
  read; clusters past a file's recorded size are ignored, as DOS does.
- VFAT long file names, used when their sequence and 8.3-alias checksum are
  intact, with valid record fields, UTF-16 and padding and at most 255 UTF-16
  units; otherwise the 8.3 name is used. PC 8.3 names use code page 437 and
  Windows NT lowercase flags; Atari names and volume labels use the Atari ST
  character set and ignore NT flags. Both support the `0x05` escape for a
  leading `0xE5`. Path-based opening and the CLI select Atari decoding for
  `.st` files even with PC-compatible boot code. For readers without a path,
  use `FatArchive::new_atari` or `ArchiveOptions::with_fat_atari_names(true)`;
  the latter also accepts `false` to force CP437.
- DOS modification times; impossible dates or times are reported as missing.
  The volume label is available through `FatArchive::volume_label()`.

Paths use `/` separators, and directories end with `/`, for example
`DOCS/NESTED/A long file name.txt`. `.` and `..` entries, deleted entries and
the volume label are not listed.

## Not supported

FAT16 and FAT32 (more than 4084 clusters) fail with
`ArchiveError::UnsupportedFormat`, and partitioned hard-disk images, which have
no BPB in their first sector, with `ArchiveError::InvalidHeader`. Compressed or
flux-level images (Atari MSA and STX, PC IMD/TD0/DMK) and non-FAT disks are not
supported. Deleted files are not recovered; this is not a repair tool.

## Usage

```sh
unarc list --json disk.img
unarc extract GAME.ST AUTO/README.TXT -o output
```

```rust,no_run
use std::fs::File;
use unarc_rs::fat::FatArchive;

let mut image = FatArchive::new(File::open("disk.img")?)?;
while let Some(entry) = image.get_next_entry()? {
    println!("{} ({} bytes)", entry.name, entry.size);
    if !entry.is_directory {
        let data = image.read(&entry)?;
        // Write data only after validating the destination path.
    }
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Detection and resource bounds

FAT images have no signature. Content detection therefore runs after every
format with magic bytes, and requires a consistent BPB (or a DOS 1.x layout)
whose filesystem size matches the image length, allowing up to 64 KiB of
trailing data, and valid reserved FAT entries. Truncated images are not
content-detected; explicit FAT opening can still list readable portions and
reports missing data when accessed. An `.img`, `.ima` or `.st` name skips the
other content checks when these checks pass.

Images are read with seeks; opening reads the boot sector, the used part of the
first FAT and all directories, not file data. Directory chains may not loop or
share clusters, and file chains may not loop or run into directories. Entry
clusters may not be free, bad or reserved, including the last cluster read.
Allocated clusters beyond the recorded file size are not traversed. Entry
paths are limited to `MAX_PATH_BYTES` (4096 bytes); only the last path
component is stored per entry, so memory stays linear in the number of entries.
Names containing `/`, `\`, control characters, or equal to `.` or `..` are
rejected, as are duplicate names within a directory (compared case-insensitively).
The direct and unified APIs enforce per-entry size limits; the unified API also
enforces cumulative limits.

## References and tests

The layout follows Microsoft's FAT specification (FAT12 cluster count limit,
BPB, VFAT long names) and the Atari ST GEMDOS boot sector description in the
Atari Compendium. Atari character decoding follows the
[Unicode Atari ST mapping](https://www.unicode.org/Public/MAPPINGS/VENDORS/MISC/ATARIST.TXT).

Fixtures in `crates/unarc-rs/tests/fat` are self-authored and verified with
mtools:

- `pc720.img`: `mformat -C -f 720 -v RETRO`, with nested directories, a long
  file name, a 20000-byte multi-cluster file and an empty file (mtools 4.0.49).
- `pc360.img`: `mkfs.fat -C -F 12 -n SMALL pc360.img 360` (dosfstools), plus
  `README.TXT` copied with mtools.
- `atari.st`: `mformat -C -t 80 -h 1 -n 9 -c 2 -r 7 -a`, single-sided, with an
  Atari-style boot sector (68000 branch, no PC code or signature) patched in
  afterwards; `mdir` and `mtype` still read it.

Text files are copies of `payload.txt` with `mcopy -m`, dated 1994-03-12 10:22:30.
Rust tests add synthetic layouts (Atari TOS FAT sizes, 1024-byte sectors, one FAT,
more reserved sectors), DOS 1.x disks, long-name edge cases, and corruption cases.
