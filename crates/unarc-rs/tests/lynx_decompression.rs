//! Lynx extraction, checked against VICE c1541 and cbmconvert (see tests/lynx/README.md).
mod c64_common;

use std::io::Cursor;
use std::path::Path;

use unarc_rs::cbm::CbmFileType;
use unarc_rs::lynx::LynxArchive;
use unarc_rs::unified::ArchiveFormat;

fn check(image: &[u8], signature: &str, expected: &[(&str, CbmFileType, Vec<u8>)]) {
    let mut archive = LynxArchive::new(Cursor::new(image)).unwrap();
    assert_eq!(archive.signature(), signature);
    assert_eq!(archive.entry_count() as usize, expected.len());
    for (name, file_type, bytes) in expected {
        let entry = archive.get_next_entry().unwrap().unwrap();
        assert_eq!(entry.name, *name);
        assert_eq!(entry.file_type, *file_type);
        assert_eq!(entry.size, bytes.len() as u64, "{name}");
        assert_eq!(archive.read(&entry).unwrap(), *bytes, "{name}");
        assert_eq!(entry.offset % 254, 0, "{name}");
    }
    assert!(archive.get_next_entry().unwrap().is_none());
}

#[test]
fn with_basic_loader() {
    use c64_common::*;
    let expected = [
        ("HELLO.prg", CbmFileType::Prg, hello()),
        ("DATA.seq", CbmFileType::Seq, data()),
        ("EXACT.prg", CbmFileType::Prg, exact()),
        ("LONG.prg", CbmFileType::Prg, long()),
        ("NOTE.usr", CbmFileType::Usr, note()),
    ];
    check(include_bytes!("lynx/basic.lnx"), "*LYNX XV  BY UNARC TESTS", &expected);
}

#[test]
fn without_basic_loader_with_rel_and_del() {
    use c64_common::*;
    let expected = [
        ("----------------.del", CbmFileType::Del, Vec::new()),
        ("HELLO.prg", CbmFileType::Prg, hello()),
        ("Shift.prg", CbmFileType::Prg, hello()),
        ("RECS.rel", CbmFileType::Rel, records()),
        ("LONG.prg", CbmFileType::Prg, long()),
        ("NOTE.usr", CbmFileType::Usr, note()),
    ];
    check(include_bytes!("lynx/bare.lnx"), "*LYNX XV  BY UNARC TESTS", &expected);

    let mut archive = LynxArchive::new(Cursor::new(&include_bytes!("lynx/bare.lnx")[..])).unwrap();
    let rel = std::iter::from_fn(|| archive.get_next_entry().unwrap())
        .find(|e| e.file_type == CbmFileType::Rel)
        .unwrap();
    assert_eq!(rel.record_length, 32);
    // 26 data blocks and one side sector
    assert_eq!(rel.blocks, 27);
}

#[test]
fn last_block_size_may_be_missing_for_the_last_file() {
    let mut image = b" 1  *LYNX TEST\r 2 \rFIRST\r 1 \rP\r 4 \rLAST\r 1 \rS\r".to_vec();
    image.resize(254, 0);
    image.extend_from_slice(&[0x01, 0x08, 0xAA, 0xEA]);
    image.resize(254 * 2, 0xEA);
    image.extend_from_slice(b"ABC");
    let mut archive = LynxArchive::new(Cursor::new(&image)).unwrap();
    let first = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(archive.read(&first).unwrap(), [0x01, 0x08, 0xAA]);
    let last = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(last.last_block, None);
    assert_eq!(archive.read(&last).unwrap(), b"ABC");
}

#[test]
fn unified_api() {
    let image = include_bytes!("lynx/basic.lnx");
    let mut reader = Cursor::new(&image[..]);
    assert_eq!(ArchiveFormat::detect(&mut reader, None).unwrap(), Some(ArchiveFormat::Lynx));
    assert_eq!(ArchiveFormat::detect_from_bytes(include_bytes!("lynx/bare.lnx")), Some(ArchiveFormat::Lynx));
    assert_eq!(ArchiveFormat::from_path(Path::new("DEMO.LNX")), Some(ArchiveFormat::Lynx));
    let mut archive = ArchiveFormat::Lynx.open(reader).unwrap();
    let names: Vec<String> = archive.entries_iter().map(|e| e.unwrap().name().to_string()).collect();
    assert_eq!(names, ["HELLO.prg", "DATA.seq", "EXACT.prg", "LONG.prg", "NOTE.usr"]);
}
