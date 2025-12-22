use std::io::Cursor;

use unarc_rs::arc::{arc_archive::ArcArchive, local_file_header::CompressionMethod};

#[test]
fn extract_stored() {
    let file = Cursor::new(include_bytes!("arc/store.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Unpacked(2), entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}

#[test]
fn extract_packed() {
    let file = Cursor::new(include_bytes!("arc/cpm.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    archive.skip(&entry).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(CompressionMethod::RLE90, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("arc/READ.COM"), result.as_slice());
}

#[test]
fn extract_sqeezed() {
    let file = Cursor::new(include_bytes!("arc/cpm.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("DDTZ.COM", entry.name);
    assert_eq!(CompressionMethod::Squeezed, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("arc/DDTZ.COM"), result.as_slice());
}

#[test]
fn extract_squashed() {
    let file = Cursor::new(include_bytes!("arc/squashed.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(CompressionMethod::Squashed, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}

#[test]
fn extract_crunch() {
    let file = Cursor::new(include_bytes!("arc/crunch.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(CompressionMethod::Crunched(8), entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}
