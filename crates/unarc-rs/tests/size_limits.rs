//! Tests for the decompressed size limits of the unified API

use std::io::{Cursor, Write};

use unarc_rs::error::ArchiveError;
use unarc_rs::unified::{ArchiveFormat, ArchiveOptions, UnifiedArchive};

const MIB: u64 = 1024 * 1024;

fn gzip(data: &[u8]) -> Vec<u8> {
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

fn tar(name: &str, data: &[u8]) -> Vec<u8> {
    let mut builder = tar::Builder::new(Vec::new());
    let mut header = tar::Header::new_gnu();
    header.set_size(data.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    builder.append_data(&mut header, name, data).unwrap();
    builder.into_inner().unwrap()
}

fn zip(entries: &[(&str, usize)]) -> Vec<u8> {
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for (name, size) in entries {
        writer.start_file(*name, options).unwrap();
        writer.write_all(&vec![b'x'; *size]).unwrap();
    }
    writer.finish().unwrap().into_inner()
}

fn open(data: Vec<u8>, format: ArchiveFormat, options: ArchiveOptions) -> unarc_rs::Result<UnifiedArchive<Cursor<Vec<u8>>>> {
    UnifiedArchive::open_with_format_and_options(Cursor::new(data), format, options)
}

#[test]
fn test_default_entry_limit() {
    assert_eq!(ArchiveOptions::new().max_entry_size(), Some(unarc_rs::DEFAULT_MAX_ENTRY_SIZE));
    assert_eq!(ArchiveOptions::new().max_total_size(), None);
}

#[test]
fn test_gz_bomb_is_stopped() {
    let data = gzip(&vec![0u8; 4 * MIB as usize]);
    assert!(data.len() < 64 * 1024);

    let options = ArchiveOptions::new().with_max_entry_size(Some(MIB));
    let mut archive = open(data.clone(), ArchiveFormat::Gz, options).unwrap();
    archive.set_single_file_name("bomb".to_string());
    let entry = archive.next_entry().unwrap().unwrap();
    match archive.read(&entry) {
        Err(ArchiveError::SizeLimitExceeded { entry, limit }) => {
            assert_eq!(entry, "bomb");
            assert_eq!(limit, MIB);
        }
        other => panic!("expected size limit error, got {other:?}"),
    }

    let mut archive = open(data, ArchiveFormat::Gz, ArchiveOptions::new()).unwrap();
    let entry = archive.next_entry().unwrap().unwrap();
    assert_eq!(archive.read(&entry).unwrap().len() as u64, 4 * MIB);
}

#[test]
fn test_tgz_limit_applies_when_opening() {
    let data = gzip(&tar("big.bin", &vec![0u8; 2 * MIB as usize]));

    let options = ArchiveOptions::new().with_max_entry_size(Some(MIB));
    assert!(matches!(
        open(data.clone(), ArchiveFormat::Tgz, options),
        Err(ArchiveError::SizeLimitExceeded { .. })
    ));

    let options = ArchiveOptions::new().with_max_entry_size(Some(MIB)).with_max_total_size(Some(4 * MIB));
    let mut archive = open(data, ArchiveFormat::Tgz, options).unwrap();
    let entry = archive.next_entry().unwrap().unwrap();
    assert!(matches!(archive.read(&entry), Err(ArchiveError::SizeLimitExceeded { limit, .. }) if limit == MIB));
}

#[test]
fn test_entry_limit_uses_recorded_size() {
    let data = zip(&[("small.txt", 1000), ("large.txt", 3000)]);
    let options = ArchiveOptions::new().with_max_entry_size(Some(2000));
    let mut archive = open(data, ArchiveFormat::Zip, options).unwrap();

    let small = archive.next_entry().unwrap().unwrap();
    assert_eq!(archive.read(&small).unwrap().len(), 1000);
    let large = archive.next_entry().unwrap().unwrap();
    assert!(matches!(archive.read(&large), Err(ArchiveError::SizeLimitExceeded { .. })));

    // Per-read options override the archive's limits
    let unlimited = ArchiveOptions::new().with_max_entry_size(None);
    assert_eq!(archive.read_with_options(&large, &unlimited).unwrap().len(), 3000);
}

#[test]
fn test_total_limit() {
    let data = zip(&[("a", 400), ("b", 400), ("c", 400)]);
    let options = ArchiveOptions::new().with_max_total_size(Some(1000));
    let mut archive = open(data, ArchiveFormat::Zip, options).unwrap();

    let mut results = Vec::new();
    while let Some(entry) = archive.next_entry().unwrap() {
        results.push(archive.read(&entry).map(|d| d.len()));
    }
    assert!(matches!(
        results[..],
        [Ok(400), Ok(400), Err(ArchiveError::SizeLimitExceeded { limit: 200, .. })]
    ));
}

#[test]
fn test_forged_size_is_rejected_before_allocation() {
    // ICE: 4 byte little endian original size followed by LH1 data; claim 4 GiB
    let mut data = u32::MAX.to_le_bytes().to_vec();
    data.extend_from_slice(&[0u8; 16]);
    assert!(matches!(
        open(data, ArchiveFormat::Ice, ArchiveOptions::new()),
        Err(ArchiveError::SizeLimitExceeded { .. })
    ));
}
