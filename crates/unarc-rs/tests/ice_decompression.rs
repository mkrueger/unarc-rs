use std::io::Cursor;

use unarc_rs::ice::IceArchive;

#[test]
fn extract_ice() {
    let file = Cursor::new(include_bytes!("ice/license_lha.ice"));
    let mut archive = IceArchive::new(file).unwrap();
    let result: Vec<u8> = archive.read().unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}
