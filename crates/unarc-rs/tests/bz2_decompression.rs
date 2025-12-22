use std::io::Cursor;

use unarc_rs::bz2::Bz2Archive;

#[test]
fn extract_bz2() {
    let file = Cursor::new(include_bytes!("bz2/LICENSE.bz2"));
    let mut archive = Bz2Archive::new(file).unwrap();
    let result = archive.read().unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}
