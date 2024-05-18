use std::io::Cursor;

use unarc_rs::arj::arj_archive::ArjArchieve;

#[test]
fn wrong_crc32() {
    let file = Cursor::new(include_bytes!("arj/wrongcrc32.arj"));
    let mut archieve = ArjArchieve::new(file).unwrap();
    let entry = archieve.get_next_entry().unwrap().unwrap();
    let result = archieve.read(&entry);
    assert!(result.is_err());
}
