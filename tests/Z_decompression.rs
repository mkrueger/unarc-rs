use std::io::Cursor;

use unarc_rs::Z::ZArchieve;

#[test]
fn extract_z() {
    let file = Cursor::new(include_bytes!("Z/LICENSE.Z"));
    let mut archieve = ZArchieve::new(file).unwrap();
    let result = archieve.read().unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}
