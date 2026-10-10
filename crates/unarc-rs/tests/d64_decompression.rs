//! D64 extraction, checked against files written by VICE c1541 and cbmconvert (see tests/d64/README.md).
mod c64_common;

use std::io::Cursor;
use std::path::Path;

use unarc_rs::cbm::CbmFileType;
use unarc_rs::d64::D64Archive;
use unarc_rs::unified::{ArchiveEntryKind, ArchiveFormat};

const IMAGE: &[u8] = include_bytes!("d64/test.d64");

fn expected() -> Vec<(&'static str, CbmFileType, Vec<u8>)> {
    use c64_common::*;
    vec![
        ("HELLO.prg", CbmFileType::Prg, hello()),
        ("DATA.seq", CbmFileType::Seq, data()),
        ("EXACT.prg", CbmFileType::Prg, exact()),
        ("LONG.prg", CbmFileType::Prg, long()),
        ("NOTE.usr", CbmFileType::Usr, note()),
        ("EMPTY.seq", CbmFileType::Seq, Vec::new()),
        ("Mixed.prg", CbmFileType::Prg, hello()),
        ("RECS.rel", CbmFileType::Rel, records()),
        ("A_B.prg", CbmFileType::Prg, hello()),
        ("NINTH.seq", CbmFileType::Seq, data()),
    ]
}

fn check_image(image: &[u8]) {
    let mut archive = D64Archive::new(Cursor::new(image)).unwrap();
    assert_eq!(archive.disk_name(), "UNARC TEST");
    let mut count = 0;
    for (name, file_type, bytes) in expected() {
        let entry = archive.get_next_entry().unwrap().unwrap();
        assert_eq!(entry.name, name);
        assert_eq!(entry.file_type, file_type);
        assert!(entry.is_closed());
        assert_eq!(entry.size, bytes.len() as u64, "{name}");
        assert_eq!(archive.read(&entry).unwrap(), bytes, "{name}");
        count += 1;
    }
    assert!(archive.get_next_entry().unwrap().is_none());
    assert_eq!(count, 10);
}

#[test]
fn extracts_all_files() {
    check_image(IMAGE);
}

#[test]
fn rel_file_metadata() {
    let mut archive = D64Archive::new(Cursor::new(IMAGE)).unwrap();
    let rel = std::iter::from_fn(|| archive.get_next_entry().unwrap())
        .find(|e| e.file_type == CbmFileType::Rel)
        .unwrap();
    assert_eq!(rel.record_length, 32);
    // The directory block count includes the side sector.
    assert_eq!(rel.blocks, 27);
}

#[test]
fn error_bytes_and_40_tracks() {
    // 683 error bytes after the sectors
    let mut with_errors = IMAGE.to_vec();
    with_errors.resize(175_531, 1);
    check_image(&with_errors);

    // 40 tracks: 85 more sectors, optionally with 768 error bytes
    let mut forty = IMAGE.to_vec();
    forty.resize(196_608, 0);
    check_image(&forty);
    assert_eq!(D64Archive::new(Cursor::new(&forty)).unwrap().tracks(), 40);
    forty.resize(197_376, 1);
    check_image(&forty);
}

#[test]
fn unified_api() {
    let mut reader = Cursor::new(IMAGE);
    assert_eq!(ArchiveFormat::detect(&mut reader, None).unwrap(), Some(ArchiveFormat::D64));
    assert_eq!(reader.position(), 0);
    let mut archive = ArchiveFormat::D64.open(reader).unwrap();
    let mut expected = expected().into_iter();
    while let Some(entry) = archive.next_entry().unwrap() {
        let (name, _, bytes) = expected.next().unwrap();
        assert_eq!(entry.name(), name);
        assert_eq!(entry.original_size(), bytes.len() as u64);
        assert_eq!(entry.kind(), ArchiveEntryKind::File);
        assert!(entry.modified_time().is_none());
        assert!(entry.is_stored());
        assert_eq!(archive.read(&entry).unwrap(), bytes);
    }
    assert!(expected.next().is_none());
}

#[test]
fn detection_by_size_and_bam() {
    // Track 1 sector 0 is unused; make it look like an ARC header.
    let mut image = IMAGE.to_vec();
    image[0] = 0x1A;
    image[1] = 0x02;
    assert_eq!(ArchiveFormat::detect(&mut Cursor::new(&image), None).unwrap(), Some(ArchiveFormat::D64));

    // Without a plausible BAM the content wins, unless the name says D64.
    let bam = 357 * 256;
    image[bam + 2] = 0;
    image[bam + 0xA5] = 0;
    assert_eq!(ArchiveFormat::detect(&mut Cursor::new(&image), None).unwrap(), Some(ArchiveFormat::Arc));
    assert_eq!(
        ArchiveFormat::detect(&mut Cursor::new(&image), Some(Path::new("GAME.D64"))).unwrap(),
        Some(ArchiveFormat::D64)
    );
    assert_eq!(ArchiveFormat::from_path(Path::new("disk.d64")), Some(ArchiveFormat::D64));
}
