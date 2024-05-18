use std::io::Cursor;

use unarc_rs::hyp::{header::CompressionMethod, hyp_archive::HypArchieve};

#[test]
fn extract_stored() {
    let file = Cursor::new(include_bytes!("hyp/stored.hyp"));
    let mut archieve = HypArchieve::new(file).unwrap();
    let entry = archieve.get_next_entry().unwrap().unwrap();
    assert_eq!("A", entry.name);
    assert_eq!(CompressionMethod::Stored, entry.compression_method);
    let result = archieve.read(&entry).unwrap();
    assert_eq!(b"a\r\n", &result[..]);

    let entry = archieve.get_next_entry().unwrap().unwrap();
    assert_eq!("B", entry.name);
    assert_eq!(CompressionMethod::Compressed, entry.compression_method);
    archieve.skip(&entry).unwrap();

    let entry = archieve.get_next_entry().unwrap().unwrap();
    assert_eq!("C", entry.name);
    assert_eq!(CompressionMethod::Stored, entry.compression_method);
    let result = archieve.read(&entry).unwrap();
    assert_eq!(b"aa\r\n", &result[..]);
}

/*

#[test]
fn extract_compressed() {
    let file = Cursor::new(include_bytes!("hyp/license.hyp"));
    let mut archieve = HypArchieve::new(file).unwrap();
    let entry = archieve.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Compressed, entry.compression_method);
    let result = archieve.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}
*/
