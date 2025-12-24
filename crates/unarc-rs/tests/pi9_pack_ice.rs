use sha1::{Digest, Sha1};

use unarc_rs::packice::PackIceArchive;

fn sha1_hex(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let digest = hasher.finalize();
    format!("{:x}", digest)
}

#[test]
fn decompress_pi9_koerper() {
    let mut archive = PackIceArchive::new(include_bytes!("pi9/KOERPER.PI9")).unwrap();
    let result = archive.read().unwrap();
    assert_eq!(result.len(), 0x13000);
    assert_eq!(
        sha1_hex(&result),
        "4a55b24a482eaa653e30bd593b16d743bfdcb3c9"
    );
}

#[test]
fn decompress_pi9_maggie() {
    let mut archive = PackIceArchive::new(include_bytes!("pi9/MAGGIE.PI9")).unwrap();
    let result = archive.read().unwrap();
    assert_eq!(result.len(), 0x13000);
    assert_eq!(
        sha1_hex(&result),
        "762f5a69fce8925546d2f55dbc20920231eeb133"
    );
}

#[test]
fn decompress_pi9_rebate() {
    let mut archive = PackIceArchive::new(include_bytes!("pi9/REBATE.PI9")).unwrap();
    let result = archive.read().unwrap();
    assert_eq!(result.len(), 0x13000);
    assert_eq!(
        sha1_hex(&result),
        "9505ccb5120ff0c13155785abf6e7eef823642ae"
    );
}
