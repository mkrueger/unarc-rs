use std::io::Cursor;

use unarc_rs::tar::TarArchive;

#[test]
fn extract_tar() {
    let file = Cursor::new(include_bytes!("tar/license.tar"));
    let mut archive = TarArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}

#[test]
fn tar_entry_count() {
    let file = Cursor::new(include_bytes!("tar/license.tar"));
    let archive = TarArchive::new(file).unwrap();
    assert_eq!(1, archive.entry_count());
}
