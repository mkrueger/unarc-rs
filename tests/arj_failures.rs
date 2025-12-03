use std::io::Cursor;

use unarc_rs::arj::arj_archive::ArjArchive;

#[test]
fn wrong_crc32() {
    let file = Cursor::new(include_bytes!("arj/wrongcrc32.arj"));
    let mut archive = ArjArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    let result = archive.read(&entry);
    assert!(result.is_err());
}
