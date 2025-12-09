use std::fs::File;
use std::io::BufReader;

use unarc_rs::tbz::TbzArchive;
use unarc_rs::unified::ArchiveFormat;

const LICENSE_CONTENT: &[u8] = include_bytes!("../LICENSE");

#[test]
fn extract_tbz() {
    let file = File::open("tests/tbz/license.tar.bz2").expect("Failed to open test file");
    let reader = BufReader::new(file);
    let mut archive = TbzArchive::new(reader).expect("Failed to create TBZ archive");

    let header = archive
        .get_next_entry()
        .expect("Failed to get entry")
        .expect("No entry found");

    assert_eq!(header.name, "LICENSE");
    assert_eq!(header.size, LICENSE_CONTENT.len() as u64);

    let data = archive.read(&header).expect("Failed to read entry");
    assert_eq!(data, LICENSE_CONTENT);
}

#[test]
fn extract_tbz_with_tbz2_extension() {
    let file = File::open("tests/tbz/license.tbz2").expect("Failed to open test file");
    let reader = BufReader::new(file);
    let mut archive = TbzArchive::new(reader).expect("Failed to create TBZ archive");

    let header = archive
        .get_next_entry()
        .expect("Failed to get entry")
        .expect("No entry found");

    assert_eq!(header.name, "LICENSE");
    let data = archive.read(&header).expect("Failed to read entry");
    assert_eq!(data, LICENSE_CONTENT);
}

#[test]
fn tbz_entry_count() {
    let file = File::open("tests/tbz/license.tar.bz2").expect("Failed to open test file");
    let reader = BufReader::new(file);
    let archive = TbzArchive::new(reader).expect("Failed to create TBZ archive");

    assert_eq!(archive.entry_count(), 1);
}

#[test]
fn test_tbz_via_unified_tar_bz2() {
    let mut archive =
        ArchiveFormat::open_path("tests/tbz/license.tar.bz2").expect("Failed to open archive");

    assert_eq!(archive.format(), ArchiveFormat::Tbz);

    let entry = archive
        .next_entry()
        .expect("Failed to get entry")
        .expect("No entry found");

    assert_eq!(entry.name(), "LICENSE");
    assert_eq!(entry.original_size(), LICENSE_CONTENT.len() as u64);

    let data = archive.read(&entry).expect("Failed to read entry");
    assert_eq!(data, LICENSE_CONTENT);
}

#[test]
fn test_tbz_via_unified_tbz2() {
    let mut archive =
        ArchiveFormat::open_path("tests/tbz/license.tbz2").expect("Failed to open archive");

    assert_eq!(archive.format(), ArchiveFormat::Tbz);

    let entry = archive
        .next_entry()
        .expect("Failed to get entry")
        .expect("No entry found");

    assert_eq!(entry.name(), "LICENSE");
    let data = archive.read(&entry).expect("Failed to read entry");
    assert_eq!(data, LICENSE_CONTENT);
}
