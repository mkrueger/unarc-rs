use std::io::Cursor;

use unarc_rs::arc::arc_archive::ArcArchieve;

#[test]
fn wrong_crc() {
    let file = Cursor::new(include_bytes!("arc/wrongcrc16.arc"));
    let mut archieve = ArcArchieve::new(file).unwrap();
    let entry = archieve.get_next_entry().unwrap().unwrap();
    let result = archieve.read(&entry);
    assert!(result.is_err());
}
