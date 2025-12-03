use std::io::Cursor;

#[test]
fn wrong_crc() {
    let file = Cursor::new(include_bytes!("zoo/wrongcrc16.zoo"));
    let mut archive = unarc_rs::zoo::zoo_archive::ZooArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    let result = archive.read(&entry);
    assert!(result.is_err());
}
