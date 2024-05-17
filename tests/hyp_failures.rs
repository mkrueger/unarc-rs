use std::io::Cursor;

use unarc_rs::hyp::hyp_archive::HypArchieve;

#[test]
fn wrong_crc() {
    let file = Cursor::new(include_bytes!("hyp/invalid.hyp"));
    let mut archieve = HypArchieve::new(file).unwrap();
    let entry = archieve.get_next_entry().unwrap().unwrap();
    let result = archieve.read(&entry);
    assert!(result.is_err());
}
