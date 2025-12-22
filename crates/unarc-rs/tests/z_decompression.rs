use std::io::Cursor;

use unarc_rs::z::ZArchive;

#[test]
fn extract_z() {
    let file = Cursor::new(include_bytes!("Z/LICENSE.Z"));
    let mut archive = ZArchive::new(file).unwrap();
    let result = archive.read().unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}
