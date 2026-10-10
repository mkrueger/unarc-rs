# Amiga disk images (ADF / HDF)

Read-only extraction of AmigaDOS OFS and FFS filesystems is available through
`ArchiveFormat::Adf`, `ArchiveFormat::Hdf`, and the shared `amiga` module.
No emulator, filesystem driver, native library, or new dependency is required.

## Supported images

- Standard sector-dump ADFs: DD (901120 bytes) and HD (1802240 bytes), with
  512-byte sectors.
- Filesystem-only HDFs with power-of-two filesystem block sizes from 512 through
  65536 bytes. The block size is inferred from a valid midpoint root block;
  one or two reserved blocks are supported.
- RDB-partitioned HDFs with 512-byte physical sectors. The RDB can occupy any of
  the first sixteen sectors. PART chains and cylinder geometry locate each
  filesystem; larger logical blocks use `SizeBlock * 4 * SectorsPerBlock`.
  DOS environment tables of 11 (the minimum) to 31 longwords are accepted.
- DOS0/DOS1 (OFS/FFS), DOS2/DOS3 (international), and DOS4/DOS5
  (international plus directory cache). Listing follows the normal directory
  hash tables, not the directory-cache records.
- Nested directories, hash collisions, empty files, fragmented data, and
  file-extension chains. OFS data-block checksums, ownership, sequence numbers,
  sizes and links are verified. FFS data blocks have no checksums on disk.
- Latin-1 names and comments, protection flags and modification times.
  The unified API uses DOS timestamps: seconds are rounded down to an even
  second, and dates outside 1980-2107 are reported as unavailable. Invalid
  date stamps (out-of-range days, minutes or ticks) are also reported as
  unavailable rather than failing the image.
- Symbolic and hard links are identified in listings. Symbolic-link reads return
  the recorded target bytes; hard-file-link reads return the target file's data.
  Hard-directory links are listed without traversal, and reading one returns
  `UnsupportedMethod`, avoiding recursive directory aliases.

RDB entries are prefixed by the partition's device name, for example
`DH0/docs/readme.txt` and `DH1/docs/readme.txt`. Each partition also has a
directory entry (`DH0/`, `DH1/`). Filesystem-only images have no volume prefix.
The disk label is available through `AmigaArchive::volumes()`.

## Not supported

Extended/MFM ADFs, copy-protected or non-AmigaDOS game disks, DOS6/DOS7 long-name
filesystems, PFS, SFS, RDB bad-block replacement lists, and non-512-byte RDB
physical sectors are not supported. Unsupported filesystems, including one in
a mixed RDB image, cause opening to fail explicitly; partitions are not silently
omitted. Filesystem-only hardfiles with other reserved-block counts or a
nonstandard root location require external mount geometry and are not inferred.

Boot code is never executed. A boot checksum is not required: valid nonbootable
volumes may omit it. Allocation bitmaps and directory caches are not used for
extraction or recovery; this reader is not a filesystem repair tool. It cannot
detect removal of unused trailing sectors when the remaining flat HDF still
forms a valid filesystem.

## Usage

```sh
unarc list disk.adf
unarc list --json harddisk.hdf
unarc extract disk.adf docs/readme.txt -o output
unarc extract harddisk.hdf DH1/docs/readme.txt -o output
```

```rust,no_run
use std::fs::File;
use unarc_rs::amiga::AmigaArchive;

let mut image = AmigaArchive::open_hdf(File::open("harddisk.hdf")?)?;
while let Some(entry) = image.get_next_entry()? {
    println!("{} ({} bytes)", entry.name, entry.size);
    if entry.name.ends_with("readme.txt") {
        let data = image.read(&entry)?;
        // Write data only after validating the destination path.
    }
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Detection and resource bounds

Reader detection runs after the D64 probe and after POSIX TAR (`ustar`). A first
sector that is a TAR header with a valid checksum also suppresses Amiga probing,
so TAR archives whose first member is named `DOS...`, or is itself an ADF/HDF,
stay TAR. A `DOS` boot block is accepted only with a valid root block at the
volume midpoint; image length then distinguishes standard ADF sizes from HDFs.
Otherwise the first sixteen sectors are searched for an `RDSK` block with a
valid checksum. A `.hdf` name overrides the floppy-size classification for a
filesystem-only HDF with an ADF-sized payload. Prefix-only byte detection
requires a valid `RDSK` checksum at offset 0 and defaults `DOS` to ADF when there
is not enough data to determine the image size.

HDFs remain seek-based; opening reads reachable directory metadata, not the
entire disk. Directory walks are iterative and reject repeated header blocks.
Entry paths are limited to `MAX_PATH_BYTES` (4096 bytes, including partition
prefixes); a deeper tree fails to open. This keeps listing memory linear in
the number of entries instead of quadratic in the directory depth.
File reads reject out-of-volume pointers, repeated blocks, invalid extension
chains, and size/count disagreements. Reads locate entries and reject pointers
to header blocks in constant time, so extracting every file is linear.
File data is allocated only when read,
and the direct and unified APIs enforce per-entry limits; the unified API also
enforces cumulative limits. Skipping or listing does not read file payloads.

## References and tests

The layouts are described in Laurent Clevy's
[ADF format FAQ](http://lclevy.free.fr/adflib/adf_info.html), especially sections
4 (OFS/FFS), 6 (RDB) and 7 (hardfiles).

Self-authored fixtures are generated and independently read with
[`amitools` 0.8.1](https://amitools.readthedocs.io/en/latest/tools/xdftool.html).
See [fixture instructions](../crates/unarc-rs/tests/amiga/README.md).
