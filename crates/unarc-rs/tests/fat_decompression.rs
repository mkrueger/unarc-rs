//! FAT12 floppy images: fixtures made with mtools/dosfstools, plus synthetic layouts.
use std::io::Cursor;
use std::path::Path;

use unarc_rs::error::ArchiveError;
use unarc_rs::fat::{FatArchive, FatEntry};
use unarc_rs::unified::{ArchiveEntryKind, ArchiveFormat, ArchiveOptions, UnifiedArchive};

#[path = "common/fat.rs"]
mod common;
use common::{ATARI720, PC720};

const PC720_IMG: &[u8] = include_bytes!("fat/pc720.img");
const PC360_IMG: &[u8] = include_bytes!("fat/pc360.img");
const ATARI_ST: &[u8] = include_bytes!("fat/atari.st");
const TEXT: &[u8] = include_bytes!("fat/payload.txt");

fn big() -> Vec<u8> {
    (0..20000usize).map(|i| ((i * 7 + i / 251) % 256) as u8).collect()
}

fn entries<T: std::io::Read + std::io::Seek>(archive: &mut FatArchive<T>) -> Vec<FatEntry> {
    let mut entries = Vec::new();
    while let Some(entry) = archive.get_next_entry().unwrap() {
        entries.push(entry);
    }
    entries
}

#[test]
fn pc720_fixture_with_long_names_and_nested_directories() {
    let mut archive = FatArchive::new(Cursor::new(PC720_IMG)).unwrap();
    assert_eq!(archive.volume_label(), Some("RETRO"));
    assert_eq!(archive.geometry().clusters(), 713);
    let entries = entries(&mut archive);
    let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "DOCS/",
            "BIG.BIN",
            "EMPTY.DAT",
            "DOCS/NESTED/",
            "DOCS/README.TXT",
            "DOCS/NESTED/A long file name.txt"
        ]
    );
    for entry in &entries {
        let data = archive.read(entry).unwrap();
        match entry.name.as_str() {
            "BIG.BIN" => assert_eq!(data, big()),
            "DOCS/README.TXT" | "DOCS/NESTED/A long file name.txt" => {
                assert_eq!(data, TEXT);
                let time = entry.modified_time.unwrap();
                assert_eq!(
                    (time.year(), time.month(), time.day(), time.hour(), time.minute(), time.second()),
                    (1994, 3, 12, 10, 22, 30)
                );
            }
            name => {
                assert!(data.is_empty(), "{name}");
                assert_eq!(entry.is_directory, name.ends_with('/'));
            }
        }
    }
}

#[test]
fn pc360_and_atari_fixtures() {
    let mut archive = FatArchive::new(Cursor::new(PC360_IMG)).unwrap();
    assert_eq!(archive.volume_label(), Some("SMALL"));
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(entry.name, "README.TXT");
    assert_eq!(archive.read(&entry).unwrap(), TEXT);

    // 68000 branch, no PC boot code and no 0x55AA signature
    assert_eq!(&ATARI_ST[..2], &[0x60, 0x38]);
    assert_eq!(&ATARI_ST[510..512], &[0, 0]);
    let mut archive = FatArchive::new(Cursor::new(ATARI_ST)).unwrap();
    assert_eq!(archive.volume_label(), None);
    assert_eq!(archive.geometry().media, 0xF8);
    let entries = entries(&mut archive);
    let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["AUTO/", "BIG.BIN", "AUTO/README.TXT"]);
    assert_eq!(archive.read(&entries[1]).unwrap(), big());
    assert_eq!(archive.read(&entries[2]).unwrap(), TEXT);
}

#[test]
fn dos1_disks_without_bpb_use_size_and_media_byte() {
    // DOS 1.x 360K disks: no BPB, FAT starting with FD FF FF
    let mut image = PC360_IMG.to_vec();
    assert_eq!(&image[512..515], &[0xFD, 0xFF, 0xFF]);
    image[11..36].fill(0);
    let mut archive = FatArchive::new(Cursor::new(image.clone())).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(archive.read(&entry).unwrap(), TEXT);
    // A different media byte does not match the 360K layout.
    image[512] = 0xF9;
    assert!(FatArchive::new(Cursor::new(image)).is_err());
}

#[test]
fn synthetic_layouts_atari_tos_and_larger_sectors() {
    let layouts = [
        (PC720, false),
        (ATARI720, true),
        (common::Layout { spc: 1, spf: 5, ..PC720 }, false),
        (common::Layout { spc: 4, total: 2880, ..PC720 }, true),
        (
            common::Layout {
                bps: 1024,
                spc: 1,
                root_entries: 192,
                spf: 2,
                total: 1232,
                ..PC720
            },
            false,
        ),
        (
            common::Layout {
                fats: 1,
                reserved: 4,
                ..ATARI720
            },
            true,
        ),
    ];
    for (layout, atari) in layouts {
        let image = common::simple(layout, atari);
        let mut archive = FatArchive::new(Cursor::new(image)).unwrap();
        let entries = entries(&mut archive);
        let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["DOCS/", "FILE.BIN", "DOCS/README.TXT"]);
        assert_eq!(archive.read(&entries[1]).unwrap(), common::file_bin(layout));
        assert_eq!(archive.read(&entries[2]).unwrap(), common::PAYLOAD);
    }
}

#[test]
fn long_names_short_names_and_code_page() {
    let layout = PC720;
    let mut image = common::blank(layout, false);
    let mut root = Vec::new();
    let alias = *b"LONGNA~1TXT";
    root.extend(common::long_name_entries("Long naïve name 🎵.txt", &alias));
    root.push(common::entry(&alias, 0x20, 2, 1));
    // Long name with the wrong checksum: the 8.3 name is used.
    let mut stale = common::long_name_entries("Ignored long name.txt", b"OTHER   TXT");
    let other = *b"STALE   TXT";
    root.append(&mut stale);
    root.push(common::entry(&other, 0x20, 3, 1));
    // A long name interrupted by a deleted entry is discarded.
    let mut broken = common::long_name_entries("Interrupted long file name entry.txt", b"BROKEN  TXT");
    broken[1][0] = 0xE5;
    root.append(&mut broken);
    root.push(common::entry(b"BROKEN  TXT", 0x20, 4, 1));
    // Windows NT lowercase flags, 0x05 for a leading 0xE5, and CP437 bytes.
    let mut lower = common::entry(b"LOWER   TXT", 0x20, 5, 1);
    lower[12] = 0x08 | 0x10;
    root.push(lower);
    root.push(common::entry(&[0x05, b'A', b'B', b' ', b' ', b' ', b' ', b' ', b'T', b'X', b'T'], 0x20, 6, 1));
    root.push(common::entry(&[b'M', 0x81, b'S', b'L', b'I', b' ', b' ', b' ', b' ', b' ', b' '], 0x20, 7, 1));
    root.push(common::entry(b"LABEL      ", 0x08, 0, 0));
    common::put_entries(&mut image, layout.root_offset(), &root);
    for cluster in 2..=7 {
        common::chain(&mut image, layout, &[cluster]);
        image[layout.cluster_offset(cluster)] = b'0' + cluster as u8;
    }
    let mut archive = FatArchive::new(Cursor::new(image)).unwrap();
    assert_eq!(archive.volume_label(), Some("LABEL"));
    let entries = entries(&mut archive);
    let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["Long naïve name 🎵.txt", "STALE.TXT", "BROKEN.TXT", "lower.txt", "σAB.TXT", "MüSLI"]);
    for (entry, cluster) in entries.iter().zip(2u8..) {
        assert_eq!(archive.read(entry).unwrap(), [b'0' + cluster]);
    }
}

#[test]
fn unified_api_detection_kinds_and_limits() {
    for (data, extension) in [(PC720_IMG, "img"), (PC360_IMG, "ima"), (ATARI_ST, "st")] {
        // Content detection without a name, at a nonzero reader position.
        let mut prefixed = b"prefix".to_vec();
        prefixed.extend_from_slice(data);
        let mut reader = Cursor::new(prefixed);
        reader.set_position(6);
        assert_eq!(ArchiveFormat::detect_from_reader(&mut reader).unwrap(), Some(ArchiveFormat::Fat));
        assert_eq!(reader.position(), 6);
        let path = format!("DISK.{}", extension.to_uppercase());
        assert_eq!(ArchiveFormat::from_path(Path::new(&path)), Some(ArchiveFormat::Fat));
        assert_eq!(
            ArchiveFormat::detect(&mut Cursor::new(data), Some(Path::new(&path))).unwrap(),
            Some(ArchiveFormat::Fat)
        );
        assert!(unarc_rs::unified::supported_extensions().contains(&extension));

        let mut archive = UnifiedArchive::open_with_format(Cursor::new(data), ArchiveFormat::Fat).unwrap();
        let mut total = 0;
        while let Some(entry) = archive.next_entry().unwrap() {
            assert!(entry.is_stored());
            let kind = if entry.name().ends_with('/') {
                ArchiveEntryKind::Directory
            } else {
                ArchiveEntryKind::File
            };
            assert_eq!(entry.kind(), kind);
            if entry.original_size() > 0 {
                let options = ArchiveOptions::new().with_max_entry_size(Some(entry.original_size() - 1));
                assert!(matches!(
                    archive.read_with_options(&entry, &options),
                    Err(ArchiveError::SizeLimitExceeded { .. })
                ));
                total += archive.read(&entry).unwrap().len();
            }
        }
        assert!(total > 0);
    }
}

#[test]
fn other_formats_and_noise_are_not_fat() {
    for data in [
        &include_bytes!("amiga/ofs.adf")[..],
        &include_bytes!("cab/license_mszip.cab")[..],
        &include_bytes!("arc/store.arc")[..],
        &vec![0u8; 737_280][..],
        &(0..737_280u32).map(|i| (i.wrapping_mul(2_654_435_761) >> 13) as u8).collect::<Vec<_>>()[..],
    ] {
        assert!(!FatArchive::probe(&mut Cursor::new(data)).unwrap());
        assert_ne!(ArchiveFormat::detect_from_reader(&mut Cursor::new(data)).unwrap(), Some(ArchiveFormat::Fat));
    }
    // An image whose length disagrees wildly with its boot sector is not detected by content.
    let mut padded = PC360_IMG.to_vec();
    padded.resize(PC360_IMG.len() + 1024 * 1024, 0);
    assert!(!FatArchive::probe(&mut Cursor::new(&padded)).unwrap());
    // A small amount of trailing padding is fine.
    padded.truncate(PC360_IMG.len() + 512);
    assert!(FatArchive::probe(&mut Cursor::new(&padded)).unwrap());
}

#[test]
fn atari_names_and_pc_compatible_atari_boot_sectors() {
    let mut image = common::blank(ATARI720, true);
    let mut name = *b"NAME    TXT";
    name[..4].copy_from_slice(&[0x9E, 0xB0, 0xC2, 0xFE]);
    let mut file = common::entry(&name, 0x20, 2, 1);
    file[12] = 0x18; // Reserved in GEMDOS, not Windows NT lowercase flags.
    common::put_entries(&mut image, ATARI720.root_offset(), &[file, common::entry(&name, 0x08, 0, 0)]);
    common::chain(&mut image, ATARI720, &[2]);
    image[ATARI720.cluster_offset(2)] = 42;
    // TOS boot code must not be interpreted as a PC 32-bit sector count.
    image[32..36].copy_from_slice(&[0xFF; 4]);
    // The BPB media byte need not match FAT[0] on Atari.
    image[21] = 0;
    let mut archive = FatArchive::new(Cursor::new(&image)).unwrap();
    assert_eq!(archive.volume_label(), Some("ßãא³    TXT"));
    let file = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(file.name, "ßãא³.TXT");
    assert_eq!(archive.read(&file).unwrap(), [42]);
    assert!(FatArchive::probe(&mut Cursor::new(&image)).unwrap());

    // Atari disks may also carry PC-compatible boot sectors.
    image[..3].copy_from_slice(&[0xEB, 0x3C, 0x90]);
    let mut archive = FatArchive::new_atari(Cursor::new(&image)).unwrap();
    assert_eq!(archive.get_next_entry().unwrap().unwrap().name, "ßãא³.TXT");
    let mut archive =
        UnifiedArchive::open_with_format_and_options(Cursor::new(&image), ArchiveFormat::Fat, ArchiveOptions::new().with_fat_atari_names(true)).unwrap();
    assert_eq!(archive.next_entry().unwrap().unwrap().name(), "ßãא³.TXT");
}

#[test]
fn truncated_images_and_invalid_fat_headers_are_not_detected() {
    let image = common::simple(ATARI720, true);
    for length in [ATARI720.data_offset() + ATARI720.cluster_size(), image.len() / 2, image.len() - 1] {
        let truncated = &image[..length];
        let mut reader = Cursor::new(truncated);
        assert!(!FatArchive::probe(&mut reader).unwrap());
        assert_eq!(reader.position(), 0);
        assert_ne!(ArchiveFormat::detect_from_reader(&mut reader).unwrap(), Some(ArchiveFormat::Fat));
    }
    for offset in [512, 513, 514] {
        let mut bad = image.clone();
        bad[offset] = 0;
        assert!(!FatArchive::probe(&mut Cursor::new(&bad)).unwrap());
        assert!(FatArchive::new(Cursor::new(bad)).is_err());
    }
}
