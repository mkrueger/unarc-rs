//! Corrupt, unsupported and hostile FAT12 images.
use std::io::Cursor;

use unarc_rs::error::ArchiveError;
use unarc_rs::fat::FatArchive;

#[path = "common/fat.rs"]
mod common;
use common::PC720;

fn open(image: Vec<u8>) -> unarc_rs::Result<FatArchive<Cursor<Vec<u8>>>> {
    FatArchive::new(Cursor::new(image))
}

/// Reads every entry; returns the first error.
fn read_all(image: Vec<u8>) -> unarc_rs::Result<()> {
    let mut archive = open(image)?;
    while let Some(entry) = archive.get_next_entry()? {
        archive.read(&entry)?;
    }
    Ok(())
}

fn corrupt_reason(result: unarc_rs::Result<()>) -> String {
    match result {
        Err(ArchiveError::CorruptedEntry { reason, .. }) => reason,
        other => panic!("expected a corrupt-entry error, got {other:?}"),
    }
}

#[test]
fn fat16_partitioned_and_garbage_boot_sectors() {
    // More than 4084 clusters is FAT16.
    let mut image = common::blank(
        common::Layout {
            spc: 1,
            spf: 32,
            total: 20_000,
            ..PC720
        },
        false,
    );
    image.truncate(64 * 1024);
    assert!(matches!(open(image), Err(ArchiveError::UnsupportedFormat(_))));
    // A hard-disk image with a partition table has no BPB in sector 0.
    let mut mbr = vec![0; 1024 * 1024];
    mbr[446 + 4] = 0x01;
    mbr[510..512].copy_from_slice(&[0x55, 0xAA]);
    assert!(matches!(open(mbr), Err(ArchiveError::InvalidHeader { .. })));
    for (offset, value) in [(11, 0x01), (12, 0x03), (13, 3), (13, 0), (14, 0), (16, 0), (16, 5), (22, 0), (19, 10)] {
        let mut image = common::simple(PC720, false);
        image[offset] = value;
        if offset == 19 {
            image[20] = 0;
        }
        assert!(open(image).is_err(), "offset {offset} = {value}");
    }
    // FAT too small for the cluster count.
    let mut image = common::simple(PC720, false);
    image[22] = 1;
    assert!(open(image).is_err());
    assert!(open(Vec::new()).is_err());
    assert!(open(vec![0; 600]).is_err());
}

#[test]
fn truncated_images() {
    let image = common::simple(PC720, false);
    // Before the end of the root directory: cannot be listed.
    assert!(open(image[..PC720.data_offset() - 1].to_vec()).is_err());
    // Directory cluster missing.
    assert!(open(image[..PC720.cluster_offset(2) + 100].to_vec()).is_err());
    // File data missing: listing works, reading the file fails.
    let mut archive = open(image[..PC720.cluster_offset(4)].to_vec()).unwrap();
    archive.get_next_entry().unwrap();
    let file = archive.get_next_entry().unwrap().unwrap();
    assert!(matches!(archive.read(&file), Err(ArchiveError::CorruptedEntry { .. })));
}

type Mutation = fn(&mut Vec<u8>);

#[test]
fn broken_cluster_chains() {
    let cases: [(&str, Mutation); 7] = [
        ("loops", |image| common::set_fat(image, PC720, 4, 3)),
        ("ends before", |image| common::set_fat(image, PC720, 4, 0xFFF)),
        ("not a data cluster", |image| common::set_fat(image, PC720, 4, 0)),
        ("not a data cluster", |image| common::set_fat(image, PC720, 4, 0xFF7)),
        ("not a data cluster", |image| common::set_fat(image, PC720, 4, 0x800)),
        ("not a data cluster", |image| {
            let offset = PC720.root_offset() + 32 + 26;
            image[offset..offset + 2].copy_from_slice(&1u16.to_le_bytes());
        }),
        ("shares clusters with a directory", |image| common::set_fat(image, PC720, 4, 2)),
    ];
    for (expected, mutate) in cases {
        let mut image = common::simple(PC720, false);
        mutate(&mut image);
        let reason = corrupt_reason(read_all(image));
        assert!(
            reason.contains(expected) || (expected == "shares clusters with a directory" && reason.contains("loops")),
            "{expected}: {reason}"
        );
    }

    // A size beyond the volume is rejected before following the chain, even without a size limit.
    let mut image = common::simple(PC720, false);
    let offset = PC720.root_offset() + 32 + 28;
    image[offset..offset + 4].copy_from_slice(&u32::MAX.to_le_bytes());
    let mut archive = open(image).unwrap();
    archive.get_next_entry().unwrap();
    let file = archive.get_next_entry().unwrap().unwrap();
    assert!(corrupt_reason(archive.read_with_limit(&file, None).map(|_| ())).contains("exceeds the volume"));
}

#[test]
fn reserved_clusters_and_unallocated_final_clusters_are_rejected() {
    // 4084 clusters remains FAT12, but 0xFF0..0xFF7 are markers, not data addresses.
    let layout = common::Layout {
        spc: 1,
        spf: 12,
        total: 4116,
        ..PC720
    };
    for marker in 0xFF0u16..=0xFF7 {
        let mut image = common::blank(layout, true);
        common::put_entries(&mut image, layout.root_offset(), &[common::entry(b"FILE    BIN", 0x20, marker, 1)]);
        common::set_fat(&mut image, layout, usize::from(marker), 0xFFF);
        assert!(corrupt_reason(read_all(image)).contains("not a data cluster"));
        let mut image = common::blank(layout, true);
        common::put_entries(&mut image, layout.root_offset(), &[common::entry(b"FILE    BIN", 0x20, 2, 513)]);
        common::set_fat(&mut image, layout, 2, marker);
        assert!(corrupt_reason(read_all(image)).contains("not a data cluster"));
    }
    for marker in [0, 1, 0xFF0, 0xFF7, 0x800] {
        let mut image = common::simple(PC720, true);
        common::set_fat(&mut image, PC720, 6, marker);
        assert!(corrupt_reason(read_all(image)).contains("not a data cluster"));
    }
}

#[test]
fn malformed_long_name_records_fall_back_to_short_names() {
    for case in 0..6 {
        let mut image = common::blank(PC720, true);
        let alias = *b"ALIAS   TXT";
        let overlong = "A".repeat(256);
        let mut records = common::long_name_entries(if case == 5 { &overlong } else { "Long.txt" }, &alias);
        match case {
            0 => records[0][12] = 1,
            1 => records[0][26] = 2,
            2 => records[0][0] |= 0x20,
            3 => records[0][0] |= 0x80,
            4 => records[0][28..30].copy_from_slice(&u16::from(b'X').to_le_bytes()),
            _ => {}
        }
        records.push(common::entry(&alias, 0x20, 0, 0));
        common::put_entries(&mut image, PC720.root_offset(), &records);
        let mut archive = open(image).unwrap();
        assert_eq!(archive.get_next_entry().unwrap().unwrap().name, "ALIAS.TXT", "case {case}");
    }
}

#[test]
fn directory_loops_and_shared_directories() {
    // DOCS contains a subdirectory pointing back at DOCS.
    let mut image = common::simple(PC720, false);
    common::put_entries(&mut image, PC720.cluster_offset(2) + 3 * 32, &[common::entry(b"LOOP       ", 0x10, 2, 0)]);
    assert!(corrupt_reason(open(image).map(|_| ())).contains("loops"));
    // Two directories sharing a cluster.
    let mut image = common::simple(PC720, false);
    common::put_entries(&mut image, PC720.root_offset() + 2 * 32, &[common::entry(b"TWIN       ", 0x10, 2, 0)]);
    assert!(open(image).is_err());
    // A directory without clusters.
    let mut image = common::simple(PC720, false);
    common::put_entries(&mut image, PC720.root_offset() + 2 * 32, &[common::entry(b"NOWHERE    ", 0x10, 0, 0)]);
    assert!(corrupt_reason(open(image).map(|_| ())).contains("no clusters"));
}

#[test]
fn unsafe_and_duplicate_names() {
    let names: [&[u8; 11]; 4] = [
        b"A/B     TXT",
        b"A\\B     TXT",
        &[b'A', 0x01, b' ', b' ', b' ', b' ', b' ', b' ', b'T', b'X', b'T'],
        b"           ",
    ];
    for name in names {
        let mut image = common::simple(PC720, false);
        common::put_entries(&mut image, PC720.root_offset() + 2 * 32, &[common::entry(name, 0x20, 0, 0)]);
        assert!(corrupt_reason(open(image).map(|_| ())).contains("invalid file name"), "{name:?}");
    }
    for long in ["..", ".", "a/b", "a\\b", "tab\there"] {
        let mut image = common::simple(PC720, false);
        let alias = *b"ALIAS   TXT";
        let mut entries = common::long_name_entries(long, &alias);
        entries.push(common::entry(&alias, 0x20, 0, 0));
        common::put_entries(&mut image, PC720.root_offset() + 2 * 32, &entries);
        assert!(corrupt_reason(open(image).map(|_| ())).contains("invalid file name"), "{long:?}");
    }
    let mut image = common::simple(PC720, false);
    let alias = *b"OTHER   BIN";
    let mut entries = common::long_name_entries("file.bin", &alias);
    entries.push(common::entry(&alias, 0x20, 0, 0));
    common::put_entries(&mut image, PC720.root_offset() + 2 * 32, &entries);
    assert!(corrupt_reason(open(image).map(|_| ())).contains("duplicate"));
}

#[test]
fn nested_paths_are_bounded() {
    // Each level adds "DDDDDDDD/" (9 bytes): 455 levels fit in 4096 bytes, 456 do not.
    let build = |depth: usize| {
        let mut image = common::blank(PC720, false);
        let mut offset = PC720.root_offset();
        for level in 0..depth {
            let cluster = level + 2;
            common::put_entries(&mut image, offset, &[common::entry(b"DDDDDDDD   ", 0x10, cluster as u16, 0)]);
            common::chain(&mut image, PC720, &[cluster]);
            offset = PC720.cluster_offset(cluster);
        }
        image
    };
    let mut archive = open(build(455)).unwrap();
    let mut longest = 0;
    while let Some(entry) = archive.get_next_entry().unwrap() {
        longest = longest.max(entry.name.len());
    }
    assert_eq!(longest, 455 * 9);
    assert!(corrupt_reason(open(build(456)).map(|_| ())).contains("path exceeds"));
}

#[test]
fn invalid_timestamps_are_missing() {
    for (date, time) in [
        (0u16, 0u16),
        ((14 << 9) | (13 << 5) | 1, 0),
        ((14 << 9) | (2 << 5) | 30, 0),
        ((14 << 9) | (1 << 5) | 1, 24 << 11),
        ((14 << 9) | (1 << 5) | 1, 30),
    ] {
        let mut image = common::simple(PC720, false);
        let offset = PC720.root_offset() + 32;
        image[offset + 22..offset + 24].copy_from_slice(&time.to_le_bytes());
        image[offset + 24..offset + 26].copy_from_slice(&date.to_le_bytes());
        let mut archive = open(image).unwrap();
        archive.get_next_entry().unwrap();
        let file = archive.get_next_entry().unwrap().unwrap();
        assert_eq!(file.modified_time, None, "{date:#x} {time:#x}");
        assert_eq!(archive.read(&file).unwrap(), common::file_bin(PC720));
    }
}

#[test]
fn limits_and_modified_entries() {
    let mut archive = open(common::simple(PC720, false)).unwrap();
    archive.get_next_entry().unwrap();
    let mut file = archive.get_next_entry().unwrap().unwrap();
    assert!(matches!(archive.read_with_limit(&file, Some(10)), Err(ArchiveError::SizeLimitExceeded { .. })));
    // A caller-modified size does not bypass the limit or change what is read.
    file.size = 1;
    assert!(matches!(archive.read_with_limit(&file, Some(10)), Err(ArchiveError::SizeLimitExceeded { .. })));
    assert_eq!(archive.read(&file).unwrap(), common::file_bin(PC720));
    let mut foreign = file.clone();
    foreign.name = "OTHER.BIN".into();
    assert!(matches!(archive.read(&foreign), Err(ArchiveError::IndexMismatch(_))));
    let mut moved = file.clone();
    moved.start_cluster = 6;
    assert!(matches!(archive.skip(&moved), Err(ArchiveError::IndexMismatch(_))));
}

#[test]
fn random_mutations_never_panic() {
    let fixtures = [
        common::simple(PC720, false),
        common::simple(common::ATARI720, true),
        include_bytes!("fat/pc720.img").to_vec(),
        include_bytes!("fat/atari.st").to_vec(),
    ];
    let mut state = 0x2545_F491_4F6C_DD1Du64;
    for fixture in fixtures {
        // Mutate the metadata areas, where parsing happens.
        let metadata = PC720.cluster_offset(8).min(fixture.len());
        for _ in 0..300 {
            let mut image = fixture.clone();
            for _ in 0..=state % 4 {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                let index = state as usize % metadata;
                image[index] = (state >> 32) as u8;
            }
            let _ = read_all(image.clone());
            let _ = FatArchive::probe(&mut Cursor::new(image));
        }
    }
}
