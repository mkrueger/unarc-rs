use std::io::Cursor;

use unarc_rs::uc2::uc2_archive::Uc2Archive;

#[test]
fn extract_fast() {
    let file = Cursor::new(include_bytes!("uc2/fast.uc2"));
    let mut archive = Uc2Archive::new(file).unwrap();

    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    // UC2 uses method numbers, not a CompressionMethod enum
    // method 4 is typical for UC2
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_normal() {
    let file = Cursor::new(include_bytes!("uc2/normal.uc2"));
    let mut archive = Uc2Archive::new(file).unwrap();

    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    // UC2 uses method numbers, not a CompressionMethod enum
    // method 4 is typical for UC2
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_stight() {
    let file = Cursor::new(include_bytes!("uc2/stight.uc2"));
    let mut archive = Uc2Archive::new(file).unwrap();

    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    // UC2 uses method numbers, not a CompressionMethod enum
    // method 4 is typical for UC2
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_tight() {
    let file = Cursor::new(include_bytes!("uc2/tight.uc2"));
    let mut archive = Uc2Archive::new(file).unwrap();

    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    // UC2 uses method numbers, not a CompressionMethod enum
    // method 4 is typical for UC2
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}
