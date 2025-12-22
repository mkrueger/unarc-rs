use std::io::Cursor;

use unarc_rs::sq::sq_archive::SqArchive;

#[test]
fn extract_sq() {
    let file = Cursor::new(include_bytes!("qqq/license.sq"));
    let mut archive = SqArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("license", entry.name);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}

#[test]
fn extract_sq2() {
    let file = Cursor::new(include_bytes!("qqq/license.sq2"));
    let mut archive = SqArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    // seems to be an error in the cmd line client with files with no extension :)
    assert_eq!(".LIC", entry.name);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}
