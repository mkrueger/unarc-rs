//! Extraction tests for Microsoft Cabinet archives (see tests/cab/README.md)
use std::io::Cursor;
use std::path::Path;

use unarc_rs::cab::{CabArchive, CompressionMethod};
use unarc_rs::unified::{ArchiveEntryKind, ArchiveFormat, UnifiedArchive};

const LICENSE: &[u8] = include_bytes!("../../../LICENSE");

fn big() -> Vec<u8> {
    LICENSE.repeat(8)
}

fn binary() -> Vec<u8> {
    (0..=255u8).collect::<Vec<_>>().repeat(160)
}

fn extract_all(data: &[u8]) -> Vec<(String, Vec<u8>)> {
    let mut archive = UnifiedArchive::open_with_format(Cursor::new(data), ArchiveFormat::Cab).unwrap();
    let mut files = Vec::new();
    while let Some(entry) = archive.next_entry().unwrap() {
        let data = archive.read(&entry).unwrap();
        assert_eq!(data.len() as u64, entry.original_size());
        files.push((entry.name().to_string(), data));
    }
    files
}

fn assert_single_license(data: &[u8], method: CompressionMethod, method_name: &str, hour: u8) {
    let mut archive = CabArchive::new(Cursor::new(data)).unwrap();
    assert_eq!(archive.header().version, (1, 3));
    assert_eq!(archive.folders().len(), 1);
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(entry.name, "LICENSE");
    assert_eq!(entry.original_size, 11357);
    assert_eq!(entry.compression_method, method);
    assert_eq!((entry.date_time.year(), entry.date_time.month(), entry.date_time.day()), (2026, 10, 9));
    assert_eq!((entry.date_time.hour(), entry.date_time.minute(), entry.date_time.second()), (hour, 34, 56));
    assert_eq!(archive.read(&entry).unwrap(), LICENSE);
    // A second read of the same entry restarts the folder.
    assert_eq!(archive.read(&entry).unwrap(), LICENSE);
    assert!(archive.get_next_entry().unwrap().is_none());

    let mut archive = UnifiedArchive::open_with_format(Cursor::new(data), ArchiveFormat::Cab).unwrap();
    let entry = archive.next_entry().unwrap().unwrap();
    assert_eq!(entry.compression_method(), method_name);
    assert_eq!(entry.kind(), ArchiveEntryKind::File);
    assert_eq!(entry.is_stored(), method == CompressionMethod::None);
    assert_eq!(archive.read(&entry).unwrap(), LICENSE);
    assert!(archive.next_entry().unwrap().is_none());
}

#[test]
fn extract_stored() {
    assert_single_license(include_bytes!("cab/license_none.cab"), CompressionMethod::None, "Stored", 18);
}

#[test]
fn extract_mszip() {
    assert_single_license(include_bytes!("cab/license_mszip.cab"), CompressionMethod::MsZip, "MSZIP", 18);
}

#[test]
fn extract_mszip_multiple_blocks() {
    let files = extract_all(include_bytes!("cab/big_mszip.cab"));
    assert_eq!(files, vec![("BIG.TXT".to_string(), big())]);
}

#[test]
fn extract_lzx_window_sizes() {
    for (bits, data) in [
        (15, &include_bytes!("cab/license_lzx15.cab")[..]),
        (18, &include_bytes!("cab/license_lzx18.cab")[..]),
        (21, &include_bytes!("cab/license_lzx21.cab")[..]),
    ] {
        assert_single_license(data, CompressionMethod::Lzx { window_bits: bits }, &format!("LZX:{bits}"), 12);
    }
}

fn expected_multi() -> Vec<(&'static str, Vec<u8>, &'static str, u16)> {
    vec![
        ("README.TXT", b"Self-authored CAB test fixture for unarc-rs.\r\n".to_vec(), "Stored", 0x20),
        ("EMPTY.TXT", Vec::new(), "Stored", 0x20),
        ("docs/LICENSE", LICENSE.to_vec(), "MSZIP", 0x20),
        ("docs/BIG.TXT", big(), "MSZIP", 0x21),
        ("data/BINARY.DAT", binary(), "LZX:16", 0x20),
        ("données/naïve-ü.txt", "UTF-8 name: données/naïve-ü\n".as_bytes().to_vec(), "LZX:16", 0xA0),
        ("café.txt", b"Latin-1 name without the UTF-8 flag\n".to_vec(), "LZX:16", 0x20),
        ("data/BIG.TXT", big(), "LZX:16", 0x20),
    ]
}

#[test]
fn extract_multiple_folders() {
    let data = include_bytes!("cab/multi.cab");
    let mut archive = UnifiedArchive::open_with_format(Cursor::new(&data[..]), ArchiveFormat::Cab).unwrap();
    let mut seen = Vec::new();
    while let Some(entry) = archive.next_entry().unwrap() {
        assert_eq!(entry.kind(), ArchiveEntryKind::File);
        seen.push((entry.name().to_string(), archive.read(&entry).unwrap(), entry.compression_method().to_string()));
    }
    let expected: Vec<_> = expected_multi().into_iter().map(|(n, d, m, _)| (n.to_string(), d, m.to_string())).collect();
    assert_eq!(seen, expected);

    let mut archive = CabArchive::new(Cursor::new(&data[..])).unwrap();
    assert_eq!(archive.header().set_id, 0x1234);
    let methods: Vec<_> = archive.folders().iter().map(|f| f.compression_method).collect();
    assert_eq!(
        methods,
        [CompressionMethod::None, CompressionMethod::MsZip, CompressionMethod::Lzx { window_bits: 16 }]
    );
    let mut attributes = Vec::new();
    while let Some(entry) = archive.get_next_entry().unwrap() {
        attributes.push(entry.attributes);
    }
    assert_eq!(attributes, expected_multi().iter().map(|e| e.3).collect::<Vec<_>>());
}

#[test]
fn read_entries_out_of_order() {
    let mut archive = CabArchive::new(Cursor::new(&include_bytes!("cab/multi.cab")[..])).unwrap();
    let mut entries = Vec::new();
    while let Some(entry) = archive.get_next_entry().unwrap() {
        entries.push(entry);
    }
    let expected = expected_multi();
    // Backwards: every read within a folder restarts it.
    for (entry, (name, data, _, _)) in entries.iter().zip(&expected).rev() {
        assert_eq!(entry.name, *name);
        assert_eq!(&archive.read(entry).unwrap(), data, "{name}");
    }
    // Skipping ahead decodes and discards the preceding files.
    assert_eq!(archive.read(&entries[7]).unwrap(), big());
    assert_eq!(archive.read(&entries[3]).unwrap(), big());
}

#[test]
fn unified_iteration_skips_without_reading() {
    let mut archive = UnifiedArchive::open_with_format(Cursor::new(&include_bytes!("cab/multi.cab")[..]), ArchiveFormat::Cab).unwrap();
    let names: Vec<_> = archive.entries_iter().map(|e| e.unwrap().name().to_string()).collect();
    assert_eq!(names, expected_multi().iter().map(|e| e.0).collect::<Vec<_>>());
}

#[test]
fn detect_cab() {
    let data = include_bytes!("cab/license_mszip.cab");
    assert_eq!(ArchiveFormat::detect_from_bytes(data), Some(ArchiveFormat::Cab));
    assert_eq!(
        ArchiveFormat::detect(&mut Cursor::new(&data[..]), Some(Path::new("SETUP.CAB"))).unwrap(),
        Some(ArchiveFormat::Cab)
    );
    assert_eq!(ArchiveFormat::from_path(Path::new("driver.cab")), Some(ArchiveFormat::Cab));
    assert!(unarc_rs::unified::supported_extensions().contains(&"cab"));
}
