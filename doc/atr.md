# Atari 8-bit disk images (ATR)

Read-only extraction of files from ATR disk images of Atari 400/800/XL/XE floppy
disks, through `ArchiveFormat::Atr` and the `atr` module. No external tools or new
dependencies are needed.

## Supported images

- ATR files: a 16-byte header starting with `0x96 0x02`, followed by the sectors.
  Sectors hold 128 bytes (single and enhanced density) or 256 bytes (double
  density and larger disks), up to 65535 sectors (about 16 MiB).
- The paragraph count in the header is ignored, because many images record a
  wrong one; the sector count follows from the file length. A partial last
  sector is ignored.
- The first three (boot) sectors hold 128 bytes even on double-density disks.
  They are usually stored as 128 bytes; images whose length is a multiple of 256
  store them in 256-byte slots, either each padded or all three packed first and
  followed by 384 zero bytes. Both are recognised. If no filesystem is found that
  way, the image is read with 128-byte boot sectors and trailing data.

## Filesystems

**Atari DOS 2.0S, 2.0D and 2.5**, and DOS 2 compatible DOSes: the root directory
holds 64 entries of 16 bytes in sectors 361-368, and files are sector chains.
The last three bytes of each data sector hold the file number (the directory
index, upper six bits), a 10-bit link to the next sector and the number of data
bytes in the sector.

- Listed are entries in use (flag bit `$40`), including locked files (`$20`) and
  files opened for writing and never closed (bit `$01`: `$41`, `$43`, `$61`, `$63`),
  which DOS hides; DOS 2.5 files that use sectors above 719 (`$03`, `$23`), which
  DOS 2.5 hides from DOS 2.0 by leaving bit `$40` clear; and MyDOS subdirectories
  (`$10`, `$30`).
- Not listed: deleted entries (bit `$80`), and entries without bit `$40` other than
  the DOS 2.5 and MyDOS patterns above (for example `$01`, `$02`, `$21`, `$31`-`$3F`).
- The directory ends at the first entry that was never used (`$00`), as in DOS and
  the `atr` tool; entries after it are not listed even if they look valid.
- Entries whose start sector is 0 are directory art: title and separator lines
  without data, written by disk editors and menu makers. They are listed as empty
  files, the way DOS shows them; duplicate names are not made unique.
- If a file's first sector carries the file's directory index, every sector must
  carry it, as DOS checks (error 164). Some DOSes and directory sorters leave other
  numbers in the sectors; then file numbers are not checked. A final sector that
  holds no data and has an all-zero trailer is accepted whatever its file number:
  files whose length is a multiple of the sector data size often link to such a
  never-written sector.

**MyDOS** extends the DOS 2 layout:

- Subdirectories (flag `$10`, locked `$30`): eight sectors of 64 entries each, laid
  out like the root directory; file numbers count within the subdirectory.
- Files with flag bit `$04` use the two link bytes as a 16-bit sector number and
  carry no file number, which disks of more than 1023 sectors need.

**SpartaDOS** (and BW-DOS, which uses its format): the boot sector names the root
directory's sector map, and byte 32 the filesystem version (`$11`, `$20`, `$21`).

- Files and directories are located through chains of sector maps; a map holds
  links to the next and previous maps and then sector numbers. Sector number 0
  is an unallocated sector of a sparse file and reads as zeros.
- Directories are read as files of 23-byte entries; the first entry describes
  the directory itself and records its length. Entries with status bit `$08`
  (in use) and without `$10` (deleted) are listed; `$20` marks a subdirectory
  and `$01` a write-protected file. A directory ends at the first entry with
  status `$00`; entries after it are not listed.
- A subdirectory that cannot be read (its sector map or length is damaged, it is
  sparse, or its sectors belong to another directory) is listed as an empty
  directory, so the rest of the disk stays listable. A damaged root directory
  fails with `ArchiveError::CorruptedEntry`.
  Failed directory reads release their sector claims so a later valid directory
  can use those sectors.
- Each entry records its size and a date and time (day, month, two-digit year,
  hour, minute, second). Years 80-99 are 1980-1999 and 0-79 are 2000-2079, as
  SpartaDOS X counts them; impossible values are reported as missing.

## Names

Paths use `/` separators, and directories end with `/`, for example
`DOCS/README.TXT`. 8.3 names are joined with a dot, without trailing spaces.
ATASCII is converted to Unicode: printable ASCII stays as it is, the graphics
characters `$00`-`$1F`, `$60` and `$7B`-`$7F` become their Unicode equivalents
(`♥`, `┏`, `━`, `♦`, `♠`, arrows, ...), and inverse-video characters (`$80`-`$FF`)
become their normal form. `/` and `\` become `_`, and names that would be empty
or consist only of dots (directory art) have each character replaced by `_`.

## Images without a supported filesystem

Many disks, especially games, boot their own loader and have no DOS directory;
the bytes in sectors 360-368 are then arbitrary. Such images fail to open with
`ArchiveError::UnsupportedFormat`, as Amiga non-DOS disks do, instead of
returning a raw sector dump or garbage entries.

A DOS 2 or MyDOS directory is only accepted if at least one of its root entries
is a non-empty file with an intact sector chain whose sectors carry the file's
number (or MyDOS 16-bit links). A directory without any file data (empty, or
only directory art) is accepted if the VTOC in sector 360 looks like one DOS
writes. Atari DOS 1.0 disks (VTOC code 1), which use the byte count of a sector
differently, fail with `ArchiveError::UnsupportedFormat`.

## Not supported

Atari DOS 1.0 and 3, DOS XE, 512-byte-sector images, headerless XFD images,
and compressed or copy-protected formats (DCM, ATX, PRO) are not supported.
Deleted files are not recovered; this is not a repair tool. Files never closed
after writing (flag `$43`) are listed, although DOS hides them: on real disks
they are often complete (`DOS.SYS` among them), and a damaged sector chain still
fails when the file is read. D64 treats unclosed files the same way.

## Usage

```sh
unarc list --json disk.atr
unarc extract DISK.ATR DOCS/README.TXT -o output
```

```rust,no_run
use std::fs::File;
use unarc_rs::atr::AtrArchive;

let mut image = AtrArchive::new(File::open("disk.atr")?)?;
println!("{}", image.filesystem());
while let Some(entry) = image.get_next_entry()? {
    println!("{} ({} bytes)", entry.name, entry.size);
    if !entry.is_directory {
        let data = image.read(&entry)?;
        // Write data only after validating the destination path.
    }
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

Text files keep the ATASCII end-of-line character `$9B`; no line endings are
converted.

## Detection and resource bounds

ATR files are detected by `0x96 0x02` and a sector size of 128, 256 or 512
bytes in the header, and by the `.atr` extension.

The image is read into memory when opened (at most 65535 sectors). Opening reads
the directories and follows every DOS 2 sector chain to find the file sizes; a
damaged chain gives the size of its readable part, and reading the entry reports
the damage. Chains and sector maps may not loop or leave the image, and a sector
may not claim more data bytes than it holds. No sector may belong to two
directories: MyDOS subdirectories may not overlap another directory, and a
SpartaDOS directory's sector maps and data sectors may not belong to another
directory or appear twice, and may not be unallocated. So directory trees cannot
loop, and all directories together hold at most the image's bytes; the number of
entries is limited to one per 16 bytes of the image, and the directory data held
while listing to the image size. Listing reads at most 16 sectors per sector of
the image; images whose files share sectors beyond that fail with
`ArchiveError::CorruptedEntry`. Entry paths are limited to `MAX_PATH_BYTES`
(4096 bytes); only the last path component is stored per entry. The direct and
unified APIs enforce per-entry size limits; the unified API also enforces
cumulative limits.

## References and tests

The layouts follow *Inside Atari DOS* (Bill Wilkinson, 1982) for DOS 2.0, and the
descriptions of DOS 2.5, MyDOS 4.5 and SpartaDOS in "Atari 8-bit Disk Formats"
(Nir Dary's collection of technical information). The ATR header is described in
the SIO2PC documentation by Nick Kennedy. The ATASCII mapping follows the
Unicode equivalents of the Atari character set.

Fixtures in `crates/unarc-rs/tests/atr` are self-authored and were made with the
`atr` tool from jhallen/atari-tools, which also extracts them byte for byte as
this reader does; see the README there. Rust tests build MyDOS and SpartaDOS
disks, boot sector layouts, DOS 2 directory details and corruption cases.
