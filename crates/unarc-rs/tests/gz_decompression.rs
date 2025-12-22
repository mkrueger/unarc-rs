use std::io::Cursor;

use unarc_rs::gz::GzArchive;

#[test]
fn extract_gz() {
    let file = Cursor::new(include_bytes!("gz/LICENSE.gz"));
    let mut archive = GzArchive::new(file).unwrap();
    let result = archive.read().unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}
