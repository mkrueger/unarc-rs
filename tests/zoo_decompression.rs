use std::io::Cursor;

use unarc_rs::zoo::{dirent::CompressionMethod, zoo_archive::ZooArchive};

#[test]
fn extract_stored() {
    let file = Cursor::new(include_bytes!("zoo/store.zoo"));
    let mut archive = ZooArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("license", entry.name);
    assert_eq!(CompressionMethod::Stored, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}

#[test]
fn extract_default() {
    let file = Cursor::new(include_bytes!("zoo/default.zoo"));
    let mut archive = ZooArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("license", entry.name);
    assert_eq!(CompressionMethod::Compressed, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}

#[test]
fn extract_high_per() {
    let file = Cursor::new(include_bytes!("zoo/high_per.zoo"));
    let mut archive = ZooArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("license", entry.name);
    assert_eq!(CompressionMethod::CompressedLh5, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}
