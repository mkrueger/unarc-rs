
use std::io::Cursor;

use unarc_rs::sqz::sqz_archive::SqzArchieve;

#[test]
fn wrong_crc() {
    let file = Cursor::new(include_bytes!("sqz/wrongcrc.sqz"));
    let mut archieve = SqzArchieve::new(file).unwrap();
    let entry = archieve.get_next_entry().unwrap().unwrap();
    let result = archieve.read(&entry);
    assert!(result.is_err());
}
