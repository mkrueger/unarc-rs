use std::io::Cursor;

use unarc_rs::sqz::sqz_archive::SqzArchive;

#[test]
fn wrong_crc() {
    let file = Cursor::new(include_bytes!("sqz/wrongcrc.sqz"));
    let mut archive = SqzArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    let result = archive.read(&entry);
    assert!(result.is_err());
}
