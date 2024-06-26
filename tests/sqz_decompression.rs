use std::io::Cursor;

use unarc_rs::sqz::{file_header::CompressionMethod, sqz_archive::SqzArchieve};

#[test]
fn extract_stored() {
    let file = Cursor::new(include_bytes!("sqz/store.sqz"));
    let mut archieve = SqzArchieve::new(file).unwrap();
    let entry = archieve.get_next_entry().unwrap().unwrap();
    //   assert_eq!("license", entry.name);
    assert_eq!(CompressionMethod::Stored, entry.compression_method);
    let result = archieve.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}

#[test]
fn extract_default() {
    let file = Cursor::new(include_bytes!("sqz/license.sqz"));
    let mut archieve = SqzArchieve::new(file).unwrap();
    let entry = archieve.get_next_entry().unwrap().unwrap();

    let result = archieve.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}
