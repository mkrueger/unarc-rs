use std::io::Cursor;

use unarc_rs::arc::{arc_archive::ArcArchive, local_file_header::CompressionMethod};

#[test]
fn extract_stored() {
    let file = Cursor::new(include_bytes!("arc/store.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Unpacked(2), entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_packed() {
    let file = Cursor::new(include_bytes!("arc/cpm.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    archive.skip(&entry).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(CompressionMethod::RLE90, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("arc/READ.COM"), result.as_slice());
}

#[test]
fn extract_sqeezed() {
    let file = Cursor::new(include_bytes!("arc/cpm.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("DDTZ.COM", entry.name);
    assert_eq!(CompressionMethod::Squeezed, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("arc/DDTZ.COM"), result.as_slice());
}

#[test]
fn extract_squashed() {
    let file = Cursor::new(include_bytes!("arc/squashed.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(CompressionMethod::Squashed, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_crunch() {
    let file = Cursor::new(include_bytes!("arc/crunch.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(CompressionMethod::Crunched(8), entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_encrypted() {
    // First test: without password should fail with CRC or decompression error
    let file = Cursor::new(include_bytes!("arc/license_cypted.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Crunched(8), entry.compression_method);

    // Without password, should fail
    let result = archive.read(&entry);
    assert!(result.is_err(), "Expected error without password");

    // Second test: with correct password should succeed
    let file = Cursor::new(include_bytes!("arc/license_cypted.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    archive.set_password("SECRET");
    assert!(archive.has_password());

    let entry = archive.get_next_entry().unwrap().unwrap();
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

// PAK format tests (uses same ARC reader with additional compression methods)

#[test]
fn extract_pak_distilled() {
    // license.pak is compressed with method 11 (Distilled)
    let file = Cursor::new(include_bytes!("pak/license.pak"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Distilled, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_pak_crushed() {
    // license_crushed.pak is compressed with method 10 (Crushed)
    let file = Cursor::new(include_bytes!("pak/license_crushed.pak"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Crushed, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_pak_crunched() {
    // license_crunched.pak is compressed with method 8 (Crunched)
    let file = Cursor::new(include_bytes!("pak/license_crunched.pak"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Crunched(8), entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_pak_squashed() {
    // license_squashed.pak is compressed with method 9 (Squashed)
    let file = Cursor::new(include_bytes!("pak/license_squashed.pak"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Squashed, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}
