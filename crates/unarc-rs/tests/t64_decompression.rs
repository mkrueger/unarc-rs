//! T64 extraction, checked against VICE c1541 and cbmconvert (see tests/t64/README.md).
mod c64_common;

use std::io::Cursor;
use std::path::Path;

use unarc_rs::cbm::CbmFileType;
use unarc_rs::t64::T64Archive;
use unarc_rs::unified::ArchiveFormat;

fn expected() -> Vec<(&'static str, CbmFileType, Vec<u8>)> {
    use c64_common::*;
    vec![
        ("HELLO.prg", CbmFileType::Prg, hello()),
        ("LONG.prg", CbmFileType::Prg, long()),
        ("DATA.seq", CbmFileType::Seq, data()),
    ]
}

fn check(image: &[u8], description: &str) {
    let mut archive = T64Archive::new(Cursor::new(image)).unwrap();
    assert_eq!(archive.description(), description);
    for (name, file_type, bytes) in expected() {
        let entry = archive.get_next_entry().unwrap().unwrap();
        assert_eq!(entry.name, name);
        assert_eq!(entry.file_type, file_type);
        assert_eq!(entry.start_address, u16::from_le_bytes([bytes[0], bytes[1]]));
        assert_eq!(entry.size(), bytes.len() as u64, "{name}");
        assert_eq!(archive.read(&entry).unwrap(), bytes, "{name}");
    }
    assert!(archive.get_next_entry().unwrap().is_none());
}

#[test]
fn extracts_with_load_address() {
    check(include_bytes!("t64/test.t64"), "UNARC TEST TAPE");
}

#[test]
fn ignores_wrong_end_addresses() {
    // Every end address is $C3C6 and the data is stored in a different order than the directory.
    let image = include_bytes!("t64/c3c6.t64");
    check(image, "BAD END ADDRESSES");
    let mut archive = T64Archive::new(Cursor::new(&image[..])).unwrap();
    assert_eq!(archive.get_next_entry().unwrap().unwrap().end_address, 0xC3C6);
}

#[test]
fn unified_api() {
    let image = include_bytes!("t64/test.t64");
    let mut reader = Cursor::new(&image[..]);
    assert_eq!(ArchiveFormat::detect(&mut reader, None).unwrap(), Some(ArchiveFormat::T64));
    assert_eq!(ArchiveFormat::from_path(Path::new("GAME.T64")), Some(ArchiveFormat::T64));
    let mut archive = ArchiveFormat::T64.open(reader).unwrap();
    let mut expected = expected().into_iter();
    for entry in archive.entries().unwrap() {
        let (name, _, bytes) = expected.next().unwrap();
        assert_eq!(entry.name(), name);
        assert_eq!(entry.original_size(), bytes.len() as u64);
        assert!(entry.modified_time().is_none());
    }
    assert!(expected.next().is_none());
}
