//! TAR.Z decompression tests

use std::io::Cursor;

/// Read a test archive file
fn read_test_file(name: &str) -> Vec<u8> {
    let path = format!("tests/tarz/{}", name);
    std::fs::read(&path).unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e))
}

#[test]
fn test_tarz_basic() {
    use unarc_rs::tarz::TarZArchive;

    let data = read_test_file("license.tar.Z");
    let cursor = Cursor::new(data);
    let mut archive = TarZArchive::new(cursor).expect("Failed to open TAR.Z archive");

    // Get the first entry
    let entry = archive
        .get_next_entry()
        .expect("Failed to read entry")
        .expect("Expected at least one entry");

    assert_eq!(entry.name, "LICENSE");
    assert!(entry.size > 0);
}

#[test]
fn test_tarz_read_content() {
    use unarc_rs::tarz::TarZArchive;

    let data = read_test_file("license.tar.Z");
    let cursor = Cursor::new(data);
    let mut archive = TarZArchive::new(cursor).expect("Failed to open TAR.Z archive");

    let entry = archive
        .get_next_entry()
        .expect("Failed to read entry")
        .expect("Expected at least one entry");

    let content = archive.read(&entry).expect("Failed to read content");
    assert_eq!(include_bytes!("../../../LICENSE"), content.as_slice());
}

#[test]
fn test_tarz_unified_api() {
    use unarc_rs::unified::ArchiveFormat;

    let data = read_test_file("license.tar.Z");
    let cursor = Cursor::new(data);
    let mut archive = ArchiveFormat::TarZ
        .open(cursor)
        .expect("Failed to open TAR.Z archive");

    let entry = archive
        .next_entry()
        .expect("Failed to read entry")
        .expect("Expected at least one entry");

    assert_eq!(entry.name(), "LICENSE");

    let content = archive.read(&entry).expect("Failed to read content");
    assert_eq!(include_bytes!("../../../LICENSE"), content.as_slice());
}

#[test]
fn test_tarz_format_detection() {
    use std::path::Path;
    use unarc_rs::unified::ArchiveFormat;

    // Test .tar.Z detection
    assert_eq!(
        ArchiveFormat::from_path(Path::new("archive.tar.Z")),
        Some(ArchiveFormat::TarZ)
    );
    assert_eq!(
        ArchiveFormat::from_path(Path::new("archive.tar.z")),
        Some(ArchiveFormat::TarZ)
    );
    assert_eq!(
        ArchiveFormat::from_path(Path::new("/path/to/file.tar.Z")),
        Some(ArchiveFormat::TarZ)
    );

    // Make sure .Z alone is still detected as plain Z format
    assert_eq!(
        ArchiveFormat::from_path(Path::new("file.Z")),
        Some(ArchiveFormat::Z)
    );
}

#[test]
fn test_tarz_format_info() {
    use unarc_rs::unified::ArchiveFormat;

    assert_eq!(ArchiveFormat::TarZ.extension(), "tar.Z");
    assert_eq!(ArchiveFormat::TarZ.name(), "TAR.Z (tar + Unix compress)");
    assert_eq!(ArchiveFormat::TarZ.extensions(), &["tar.Z"]);
}
