use std::io::Cursor;

use unarc_rs::arj::arj_archive::ArjArchive;

#[test]
fn wrong_crc32() {
    let file = Cursor::new(include_bytes!("arj/wrongcrc32.arj"));
    let mut archive = ArjArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    let result = archive.read(&entry);
    assert!(result.is_err());
}

#[test]
fn invalid_initial_method4_match_returns_error_without_panicking() {
    let mut cursor = Cursor::new(include_bytes!("arj/method4.arj").to_vec());
    {
        let mut archive = ArjArchive::new(&mut cursor).unwrap();
        archive.get_next_entry().unwrap().unwrap();
    }
    // Keep both header checksums valid; damage only the compressed payload.
    let offset = cursor.position() as usize;
    cursor.get_mut()[offset..offset + 2].copy_from_slice(&[0x80, 0]);
    cursor.set_position(0);
    let mut archive = ArjArchive::new(cursor).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert!(archive.read(&entry).is_err());
}
