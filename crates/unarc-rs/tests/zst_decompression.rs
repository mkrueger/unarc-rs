use std::fs::File;
use std::io::Cursor;
use std::path::Path;

use unarc_rs::unified::ArchiveFormat;
use unarc_rs::zst::ZstArchive;

const LICENSE_CONTENT: &[u8] = include_bytes!("../../../LICENSE");

/// A skippable frame (magic 0x184D2A50) carrying eight bytes of user data
const SKIPPABLE_FRAME: &[u8] = b"\x50\x2a\x4d\x18\x08\x00\x00\x00skipped!";

#[test]
fn extract_zst() {
    let file = Cursor::new(include_bytes!("zst/LICENSE.zst"));
    let mut archive = ZstArchive::new(file).unwrap();
    let result = archive.read().unwrap();
    assert_eq!(LICENSE_CONTENT, result.as_slice());
}

#[test]
fn extract_multiple_frames_and_skippable_frames() {
    // `zstd` on each half of LICENSE, concatenated with a skippable frame in between
    let file = Cursor::new(include_bytes!("zst/multi_frame.zst"));
    let mut archive = ZstArchive::new(file).unwrap();
    let result = archive.read().unwrap();
    assert_eq!(LICENSE_CONTENT, result.as_slice());
}

#[test]
fn extract_zst_starting_with_skippable_frame() {
    let mut data = SKIPPABLE_FRAME.to_vec();
    data.extend_from_slice(include_bytes!("zst/LICENSE.zst"));
    assert_eq!(ArchiveFormat::detect_from_bytes(&data), Some(ArchiveFormat::Zst));

    let mut archive = ZstArchive::new(Cursor::new(data)).unwrap();
    assert_eq!(archive.read().unwrap(), LICENSE_CONTENT);
}

#[test]
fn test_zst_via_unified() {
    let mut archive = ArchiveFormat::open_path("tests/zst/LICENSE.zst").expect("Failed to open archive");

    assert_eq!(archive.format(), ArchiveFormat::Zst);

    let entry = archive.next_entry().expect("Failed to get entry").expect("No entry found");
    assert_eq!(entry.name(), "LICENSE");
    assert_eq!(entry.compression_method(), "Zstandard");

    let data = archive.read(&entry).expect("Failed to read entry");
    assert_eq!(data, LICENSE_CONTENT);
    assert!(archive.next_entry().unwrap().is_none());
}

#[test]
fn test_zst_detected_from_content() {
    let path = Path::new("tests/zst/LICENSE.zst");
    let mut file = File::open(path).unwrap();
    assert_eq!(ArchiveFormat::detect(&mut file, Some(path)).unwrap(), Some(ArchiveFormat::Zst));
    assert_eq!(ArchiveFormat::detect(&mut file, None).unwrap(), Some(ArchiveFormat::Zst));
}
