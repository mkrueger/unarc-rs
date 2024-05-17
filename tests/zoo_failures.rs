
use std::io::Cursor;

#[test]
fn wrong_crc() {
    let file = Cursor::new(include_bytes!("zoo/wrongcrc16.zoo"));
    let mut archieve = unarc_rs::zoo::zoo_archive::ZooArchieve::new(file).unwrap();
    let entry = archieve.get_next_entry().unwrap().unwrap();
    let result = archieve.read(&entry);
    assert!(result.is_err());
}
