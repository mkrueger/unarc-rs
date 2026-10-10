//! Extraction tests for Amiga LZX archives (see tests/lzx/README.md)
use std::io::Cursor;
use std::path::Path;

use unarc_rs::lzx::{LzxArchive, LzxEntry, PackMode};
use unarc_rs::unified::{ArchiveEntryKind, ArchiveFormat, UnifiedArchive};

const LICENSE: &[u8] = include_bytes!("../../../LICENSE");
const SINGLE: &[u8] = include_bytes!("lzx/license.lzx");
const MULTI: &[u8] = include_bytes!("lzx/multi.lzx");

/// Word salad drawn from LICENSE with a fixed LCG, as in tests/lzx/README.md
fn words(count: usize) -> Vec<u8> {
    let vocabulary: Vec<&[u8]> = LICENSE.split(u8::is_ascii_whitespace).filter(|word| !word.is_empty()).collect();
    let mut state = 1u64;
    let mut out = Vec::new();
    for index in 0..count {
        state = (state * 1_103_515_245 + 12345) % (1 << 31);
        out.extend_from_slice(vocabulary[state as usize % vocabulary.len()]);
        out.push(if index % 12 == 11 { b'\n' } else { b' ' });
    }
    out
}

fn binary() -> Vec<u8> {
    (0..=255u8).collect::<Vec<_>>().repeat(160)
}

fn date(entry: &LzxEntry) -> Option<(u16, u8, u8, u8, u8, u8)> {
    entry.modified_time.map(|t| (t.year(), t.month(), t.day(), t.hour(), t.minute(), t.second()))
}

#[test]
fn extract_single_file() {
    let mut archive = LzxArchive::new(Cursor::new(SINGLE)).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(entry.name, "LICENSE");
    assert_eq!((entry.original_size, entry.packed_size), (11357, 4014));
    assert_eq!(entry.pack_mode, PackMode::Normal);
    assert!(!entry.merged);
    assert_eq!(entry.protection(), "----rwed");
    assert_eq!(entry.crc32, 0x7B5D_04BC);
    assert_eq!(date(&entry), Some((2026, 10, 10, 12, 34, 56)));
    assert_eq!(archive.read(&entry).unwrap(), LICENSE);
    // A second read of the same entry restarts the stream.
    assert_eq!(archive.read(&entry).unwrap(), LICENSE);
    assert!(archive.get_next_entry().unwrap().is_none());

    let mut archive = UnifiedArchive::open_with_format(Cursor::new(SINGLE), ArchiveFormat::Lzx).unwrap();
    let entry = archive.next_entry().unwrap().unwrap();
    assert_eq!(entry.compression_method(), "LZX");
    assert_eq!((entry.compressed_size(), entry.original_size()), (4014, 11357));
    assert_eq!(entry.crc(), 0x7B5D_04BC);
    assert_eq!(entry.kind(), ArchiveEntryKind::File);
    assert!(!entry.is_stored());
    assert_eq!(archive.read(&entry).unwrap(), LICENSE);
    assert!(archive.next_entry().unwrap().is_none());
}

fn expected_multi() -> Vec<(&'static str, Vec<u8>, &'static str, u64, ArchiveEntryKind)> {
    use ArchiveEntryKind::{Directory, File};
    vec![
        ("README.TXT", b"Self-authored LZX test fixture for unarc-rs.\n".to_vec(), "Stored", 45, File),
        ("docs/", Vec::new(), "Stored", 0, Directory),
        ("docs/LICENSE", LICENSE.to_vec(), "LZX (merged)", 0, File),
        ("docs/words.txt", words(30000), "LZX (merged)", 0, File),
        ("docs/empty", Vec::new(), "LZX (merged)", 0, File),
        ("docs/binary.dat", binary(), "LZX (merged)", 0, File),
        ("EMPTY", Vec::new(), "LZX", 0, File),
        ("café.txt", b"Latin-1 name\n".repeat(20), "LZX", 52, File),
    ]
}

#[test]
fn extract_merged_group_and_stored_entries() {
    let mut archive = UnifiedArchive::open_with_format(Cursor::new(MULTI), ArchiveFormat::Lzx).unwrap();
    let mut seen = Vec::new();
    while let Some(entry) = archive.next_entry().unwrap() {
        let data = archive.read(&entry).unwrap();
        assert_eq!(data.len() as u64, entry.original_size());
        assert_eq!(u64::from(crc32fast::hash(&data)), entry.crc());
        seen.push((
            entry.name().to_string(),
            data,
            entry.compression_method().to_string(),
            entry.compressed_size(),
            entry.kind(),
        ));
    }
    let expected: Vec<_> = expected_multi()
        .into_iter()
        .map(|(n, d, m, c, k)| (n.to_string(), d, m.to_string(), c, k))
        .collect();
    assert_eq!(seen, expected);
}

#[test]
fn merged_group_metadata() {
    let mut archive = LzxArchive::new(Cursor::new(MULTI)).unwrap();
    let mut entries = Vec::new();
    while let Some(entry) = archive.get_next_entry().unwrap() {
        entries.push(entry);
    }
    let summary: Vec<_> = entries
        .iter()
        .map(|e| (e.name.as_str(), e.pack_mode, e.merged, e.packed_size, e.protection(), date(e)))
        .collect();
    let today = Some((2026, 10, 10, 12, 34, 56));
    assert_eq!(
        summary,
        [
            ("README.TXT", PackMode::Stored, false, 45, "----rwed".to_string(), today),
            ("docs/", PackMode::Stored, false, 0, "----rwed".to_string(), today),
            (
                "docs/LICENSE",
                PackMode::Normal,
                true,
                0,
                "----rwed".to_string(),
                Some((1999, 12, 31, 23, 59, 58))
            ),
            // Year field 59: written by a Y2K-fixed LZX, read by unpatched tools as 2029
            ("docs/words.txt", PackMode::Normal, true, 0, "----rw-d".to_string(), Some((2001, 2, 3, 4, 5, 6))),
            ("docs/empty", PackMode::Normal, true, 0, "----rwed".to_string(), today),
            (
                "docs/binary.dat",
                PackMode::Normal,
                true,
                61934,
                "hspa----".to_string(),
                Some((2041, 12, 31, 0, 0, 0))
            ),
            ("EMPTY", PackMode::Normal, false, 0, "----rwed".to_string(), today),
            ("café.txt", PackMode::Normal, false, 52, "-s--rwed".to_string(), today),
        ]
    );
    assert_eq!(entries[0].comment, "Stored entry with a comment");
    assert!(entries[1..].iter().all(|e| e.comment.is_empty()));
}

#[test]
fn read_merged_entries_out_of_order() {
    let mut archive = LzxArchive::new(Cursor::new(MULTI)).unwrap();
    let mut entries = Vec::new();
    while let Some(entry) = archive.get_next_entry().unwrap() {
        entries.push(entry);
    }
    let expected = expected_multi();
    // Backwards: every read within the merged group restarts it.
    for (entry, (name, data, ..)) in entries.iter().zip(&expected).rev() {
        assert_eq!(entry.name, *name);
        assert_eq!(&archive.read(entry).unwrap(), data, "{name}");
    }
    // Skipping ahead decodes and discards the preceding files of the group.
    assert_eq!(archive.read(&entries[5]).unwrap(), binary());
    assert_eq!(archive.read(&entries[2]).unwrap(), LICENSE);
    assert_eq!(archive.read(&entries[3]).unwrap(), words(30000));
    // Interleaving another group does not disturb the merged one.
    assert_eq!(archive.read(&entries[2]).unwrap(), LICENSE);
    assert_eq!(archive.read(&entries[7]).unwrap(), b"Latin-1 name\n".repeat(20));
    assert_eq!(archive.read(&entries[5]).unwrap(), binary());
}

#[test]
fn unified_skips_then_reads_a_later_merged_entry() {
    let mut archive = UnifiedArchive::open_with_format(Cursor::new(MULTI), ArchiveFormat::Lzx).unwrap();
    while let Some(entry) = archive.next_entry().unwrap() {
        if entry.name() == "docs/binary.dat" {
            assert_eq!(archive.read(&entry).unwrap(), binary());
        } else {
            archive.skip(&entry).unwrap();
        }
    }

    let mut archive = UnifiedArchive::open_with_format(Cursor::new(MULTI), ArchiveFormat::Lzx).unwrap();
    let names: Vec<_> = archive.entries_iter().map(|e| e.unwrap().name().to_string()).collect();
    assert_eq!(names, expected_multi().iter().map(|e| e.0).collect::<Vec<_>>());
}

#[test]
fn detect_lzx() {
    assert_eq!(ArchiveFormat::detect_from_bytes(SINGLE), Some(ArchiveFormat::Lzx));
    assert_eq!(
        ArchiveFormat::detect(&mut Cursor::new(MULTI), Some(Path::new("GAME.LZX"))).unwrap(),
        Some(ArchiveFormat::Lzx)
    );
    assert_eq!(ArchiveFormat::from_path(Path::new("demo.lzx")), Some(ArchiveFormat::Lzx));
    assert_eq!(ArchiveFormat::Lzx.name(), "LZX (Amiga)");
    assert!(unarc_rs::unified::supported_extensions().contains(&"lzx"));
}
