/*
use std::io::Cursor;

use unarc_rs::uc2::uc2_archive::Uc2Archive;

#[test]
fn extract_normal() {
    let file = Cursor::new(include_bytes!("uc2/normal.uc2"));
    let mut archive = Uc2Archive::new(file).unwrap();

    /*let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("license", entry.name);
    assert_eq!(CompressionMethod::Stored, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());*/
}
*/
