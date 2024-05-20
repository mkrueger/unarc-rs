/*
use std::io::Cursor;

use unarc_rs::uc2::uc2_archive::Uc2Archieve;

#[test]
fn extract_normal() {
    let file = Cursor::new(include_bytes!("uc2/normal.uc2"));
    let mut archieve = Uc2Archieve::new(file).unwrap();

    /*let entry = archieve.get_next_entry().unwrap().unwrap();
    assert_eq!("license", entry.name);
    assert_eq!(CompressionMethod::Stored, entry.compression_method);
    let result = archieve.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());*/
}
*/
