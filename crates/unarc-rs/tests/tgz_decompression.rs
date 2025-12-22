use std::fs::File;
use std::io::BufReader;

use unarc_rs::tgz::TgzArchive;
use unarc_rs::unified::ArchiveFormat;

const LICENSE_CONTENT: &[u8] = include_bytes!("../../../LICENSE");

#[test]
fn extract_tgz() {
    let file = File::open("tests/tgz/license.tar.gz").expect("Failed to open test file");
    let reader = BufReader::new(file);
    let mut archive = TgzArchive::new(reader).expect("Failed to create TGZ archive");

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
fn extract_tgz_with_tgz_extension() {
    let file = File::open("tests/tgz/license.tgz").expect("Failed to open test file");
    let reader = BufReader::new(file);
    let mut archive = TgzArchive::new(reader).expect("Failed to create TGZ archive");

    let header = archive
        .get_next_entry()
        .expect("Failed to get entry")
        .expect("No entry found");

    assert_eq!(header.name, "LICENSE");
    let data = archive.read(&header).expect("Failed to read entry");
    assert_eq!(data, LICENSE_CONTENT);
}

#[test]
fn tgz_entry_count() {
    let file = File::open("tests/tgz/license.tar.gz").expect("Failed to open test file");
    let reader = BufReader::new(file);
    let archive = TgzArchive::new(reader).expect("Failed to create TGZ archive");

    assert_eq!(archive.entry_count(), 1);
}

#[test]
fn test_tgz_via_unified_tar_gz() {
    let mut archive =
        ArchiveFormat::open_path("tests/tgz/license.tar.gz").expect("Failed to open archive");

    assert_eq!(archive.format(), ArchiveFormat::Tgz);

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
fn test_tgz_via_unified_tgz() {
    let mut archive =
        ArchiveFormat::open_path("tests/tgz/license.tgz").expect("Failed to open archive");

    assert_eq!(archive.format(), ArchiveFormat::Tgz);

    let entry = archive
        .next_entry()
        .expect("Failed to get entry")
        .expect("No entry found");

    assert_eq!(entry.name(), "LICENSE");
    let data = archive.read(&entry).expect("Failed to read entry");
    assert_eq!(data, LICENSE_CONTENT);
}
