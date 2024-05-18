use std::io::Cursor;

use unarc_rs::arc::{arc_archive::ArcArchieve, local_file_header::CompressionMethod};

#[test]
fn extract_stored() {
    let file = Cursor::new(include_bytes!("arc/store.arc"));
    let mut archieve = ArcArchieve::new(file).unwrap();
    let entry = archieve.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Unpacked(0), entry.compression_method);
    let result = archieve.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}

#[test]
fn extract_packed() {
    let file = Cursor::new(include_bytes!("arc/cpm.arc"));
    let mut archieve = ArcArchieve::new(file).unwrap();
    let entry = archieve.get_next_entry().unwrap().unwrap();
    archieve.skip(&entry).unwrap();
    let entry = archieve.get_next_entry().unwrap().unwrap();
    assert_eq!(CompressionMethod::RLE, entry.compression_method);
    let result = archieve.read(&entry).unwrap();
    assert_eq!(include_bytes!("arc/READ.COM"), result.as_slice());
}

#[test]
fn extract_sqeezed() {
    let file = Cursor::new(include_bytes!("arc/cpm.arc"));
    let mut archieve = ArcArchieve::new(file).unwrap();
    let entry = archieve.get_next_entry().unwrap().unwrap();
    assert_eq!("DDTZ.COM", entry.name);
    assert_eq!(CompressionMethod::Squeezed, entry.compression_method);
    let result = archieve.read(&entry).unwrap();
    assert_eq!(include_bytes!("arc/DDTZ.COM"), result.as_slice());
}
