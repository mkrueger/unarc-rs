use std::fs::File;
use std::io::Cursor;
use std::path::Path;

use unarc_rs::unified::ArchiveFormat;
use unarc_rs::xz::XzArchive;

const LICENSE_CONTENT: &[u8] = include_bytes!("../../../LICENSE");

#[test]
fn extract_xz() {
    let file = Cursor::new(include_bytes!("xz/LICENSE.xz"));
    let mut archive = XzArchive::new(file).unwrap();
    let result = archive.read().unwrap();
    assert_eq!(LICENSE_CONTENT, result.as_slice());
}

#[test]
fn extract_concatenated_xz_streams() {
    // `xz` on each half of LICENSE, concatenated: `xz -d` decodes both streams
    let file = Cursor::new(include_bytes!("xz/multi_stream.xz"));
    let mut archive = XzArchive::new(file).unwrap();
    let result = archive.read().unwrap();
    assert_eq!(LICENSE_CONTENT, result.as_slice());
}

#[test]
fn test_xz_via_unified() {
    let mut archive = ArchiveFormat::open_path("tests/xz/LICENSE.xz").expect("Failed to open archive");

    assert_eq!(archive.format(), ArchiveFormat::Xz);

    let entry = archive.next_entry().expect("Failed to get entry").expect("No entry found");
    assert_eq!(entry.name(), "LICENSE");
    assert_eq!(entry.compression_method(), "LZMA2");

    let data = archive.read(&entry).expect("Failed to read entry");
    assert_eq!(data, LICENSE_CONTENT);
    assert!(archive.next_entry().unwrap().is_none());
}

#[test]
fn test_xz_detected_from_content() {
    let path = Path::new("tests/xz/LICENSE.xz");
    let mut file = File::open(path).unwrap();
    assert_eq!(ArchiveFormat::detect(&mut file, Some(path)).unwrap(), Some(ArchiveFormat::Xz));
    assert_eq!(ArchiveFormat::detect(&mut file, None).unwrap(), Some(ArchiveFormat::Xz));
}
