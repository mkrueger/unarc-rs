use std::io::Cursor;

use unarc_rs::arj::{arj_archive::ArjArchive, local_file_header::CompressionMethod};

#[test]
fn extract_stored() {
    let file = Cursor::new(include_bytes!("arj/stored.arj"));
    let mut archive = ArjArchive::new(file).unwrap();
    assert_eq!("method1.arj", archive.get_name());
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Stored, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_method1() {
    let file = Cursor::new(include_bytes!("arj/method1.arj"));
    let mut archive = ArjArchive::new(file).unwrap();
    assert_eq!("method2.arj", archive.get_name());
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::CompressedMost, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_method2() {
    let file = Cursor::new(include_bytes!("arj/method2.arj"));
    let mut archive = ArjArchive::new(file).unwrap();
    assert_eq!("method2.arj", archive.get_name());
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Compressed, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_method3() {
    let file = Cursor::new(include_bytes!("arj/method3.arj"));
    let mut archive = ArjArchive::new(file).unwrap();
    assert_eq!("method3.arj", archive.get_name());
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(
        CompressionMethod::CompressedFaster,
        entry.compression_method
    );
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_method4() {
    let file = Cursor::new(include_bytes!("arj/method4.arj"));
    let mut archive = ArjArchive::new(file).unwrap();
    assert_eq!("method4.arj", archive.get_name());
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(
        CompressionMethod::CompressedFastest,
        entry.compression_method
    );
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}
