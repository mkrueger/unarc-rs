use std::io::Cursor;

use unarc_rs::error::ArchiveError;
use unarc_rs::uc2::uc2_archive::Uc2Archive;

#[test]
fn ue2_is_detected_as_uc2_but_unsupported() {
    let file = Cursor::new(include_bytes!("ue2/license.ue2"));
    match Uc2Archive::new(file) {
        Ok(_) => panic!("expected UE2 to be rejected as unsupported"),
        Err(ArchiveError::UnsupportedMethod { format, method }) => {
            assert_eq!(format, "UC2");
            assert!(method.contains("UltraCrypt"));
            assert!(method.contains("UE2"));
        }
        Err(other) => panic!("unexpected error: {other:?}"),
    }
}
