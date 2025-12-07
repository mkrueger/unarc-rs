use std::io::Cursor;

use unarc_rs::hyp::{header::CompressionMethod, hyp_archive::HypArchive};

#[test]
fn extract_stored() {
    let file = Cursor::new(include_bytes!("hyp/stored.hyp"));
    let mut archive = HypArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("A", entry.name);
    assert_eq!(CompressionMethod::Stored, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(b"a\r\n", &result[..]);

    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("B", entry.name);
    assert_eq!(CompressionMethod::Compressed, entry.compression_method);
    archive.skip(&entry).unwrap();

    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("C", entry.name);
    assert_eq!(CompressionMethod::Stored, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(b"aa\r\n", &result[..]);
}

#[ignore = "infinite loop in decompression (error in the legacy code - dos infinite loops too"]
#[test]
fn extract_compressed() {
    let file = Cursor::new(include_bytes!("hyp/license.hyp"));
    let mut archive = HypArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Compressed, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}

#[ignore = "infinite loop in decompression (error in the legacy code - dos infinite loops too"]
#[test]
fn extract_minimal() {
    let file = Cursor::new(include_bytes!("hyp/atest.hyp"));
    let mut archive = HypArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("ATEST.TXT", entry.name);
    assert_eq!(CompressionMethod::Compressed, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("hyp/atest.txt"), result.as_slice());
}
