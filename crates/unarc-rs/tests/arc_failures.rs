use std::io::Cursor;

use unarc_rs::arc::arc_archive::ArcArchive;

#[test]
fn wrong_crc() {
    let file = Cursor::new(include_bytes!("arc/wrongcrc16.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    let result = archive.read(&entry);
    assert!(result.is_err());
}
