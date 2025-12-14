use std::io::Cursor;

use unarc_rs::hyp::hyp_archive::HypArchive;

#[test]
fn wrong_crc() {
    let file = Cursor::new(include_bytes!("hyp/invalid.hyp"));
    let mut archive = HypArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    let result = archive.read(&entry);
    assert!(result.is_err());
}
