use std::io::Cursor;

use unarc_rs::ha::asc::decompress_asc;
use unarc_rs::ha::header::{ArchiveHeader, FileHeader};
use unarc_rs::ha::{ha_archive::HaArchive, header::CompressionMethod};

#[test]
fn extract_stored() {
    let file = Cursor::new(include_bytes!("ha/copy.ha"));
    let mut archive = HaArchive::new(file).unwrap();

    // Get the first entry
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(CompressionMethod::Cpy, entry.method);
    assert_eq!("license", entry.name);

    // Read and verify contents
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());

    // Verify no more entries
    let next = archive.get_next_entry().unwrap();
    assert!(next.is_none());
}

#[test]
fn extract_asc() {
    let file = Cursor::new(include_bytes!("ha/asc.ha"));
    let mut archive = HaArchive::new(file).unwrap();

    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(CompressionMethod::Asc, entry.method);
    assert_eq!("license", entry.name);

    // ASC is now supported
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_hsc() {
    let file = Cursor::new(include_bytes!("ha/hsc.ha"));
    let mut archive = HaArchive::new(file).unwrap();

    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(CompressionMethod::Hsc, entry.method);
    assert_eq!("license", entry.name);

    // HSC is now supported
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_asc_alt() {
    use std::io::Read;

    let data = include_bytes!("ha/asc.ha");
    let mut cursor = Cursor::new(&data[..]);

    let _ = ArchiveHeader::load_from(&mut cursor).expect("archive header");
    let file_header = FileHeader::load_from(&mut cursor).expect("file header");

    let mut compressed = vec![0u8; file_header.compressed_size as usize];
    cursor.read_exact(&mut compressed).expect("read compressed");

    // Use alternative implementation
    let result = decompress_asc(Cursor::new(&compressed)).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}
