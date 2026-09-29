//! Tests for RAR archive decompression

use std::io::Cursor;

use unarc_rs::error::ArchiveError;
use unarc_rs::rar::rar_archive::RarArchive;
use unarc_rs::unified::{ArchiveFormat, ArchiveOptions, UnifiedArchive};

fn build(version: rars::ArchiveVersion, configure: impl FnOnce(rars::Builder) -> rars::Builder) -> Vec<u8> {
    let mut builder = configure(rars::Builder::new(version));
    builder.add_bytes(b"dir/hello.txt".to_vec(), b"Hello, RAR!".to_vec(), None, None).unwrap();
    builder.add_bytes(b"second.txt".to_vec(), b"second entry ".repeat(100), None, None).unwrap();
    builder.add_bytes(b"third.txt".to_vec(), b"third".to_vec(), None, None).unwrap();
    builder.to_bytes().unwrap()
}

fn open(data: Vec<u8>, password: Option<&str>) -> UnifiedArchive<Cursor<Vec<u8>>> {
    let mut options = ArchiveOptions::new();
    if let Some(password) = password {
        options = options.with_password(password);
    }
    UnifiedArchive::open_with_format_and_options(Cursor::new(data), ArchiveFormat::Rar, options).expect("Failed to open RAR archive")
}

fn read_all(archive: &mut UnifiedArchive<Cursor<Vec<u8>>>) -> Vec<(String, Vec<u8>)> {
    let mut result = Vec::new();
    while let Some(entry) = archive.next_entry().expect("Failed to read entry header") {
        let data = archive.read(&entry).expect("Failed to read entry data");
        assert_eq!(data.len() as u64, entry.original_size());
        result.push((entry.name().to_string(), data));
    }
    result
}

fn expected() -> Vec<(String, Vec<u8>)> {
    vec![
        ("dir/hello.txt".to_string(), b"Hello, RAR!".to_vec()),
        ("second.txt".to_string(), b"second entry ".repeat(100)),
        ("third.txt".to_string(), b"third".to_vec()),
    ]
}

#[test]
fn test_rar_versions_and_solid() {
    for version in [
        rars::ArchiveVersion::Rar15,
        rars::ArchiveVersion::Rar29,
        rars::ArchiveVersion::Rar50,
        rars::ArchiveVersion::Rar70,
    ] {
        for solid in [false, true] {
            let data = build(version, |b| b.solid(solid));
            let mut archive = open(data, None);
            assert_eq!(read_all(&mut archive), expected(), "{version:?} solid={solid}");
        }
    }
}

#[test]
fn test_rar_skip_entries() {
    for solid in [false, true] {
        let data = build(rars::ArchiveVersion::Rar50, |b| b.solid(solid));
        let mut archive = open(data, None);
        let first = archive.next_entry().unwrap().unwrap();
        archive.skip(&first).unwrap();
        let second = archive.next_entry().unwrap().unwrap();
        archive.skip(&second).unwrap();
        let third = archive.next_entry().unwrap().unwrap();
        assert_eq!(archive.read(&third).unwrap(), b"third", "solid={solid}");
        assert!(archive.next_entry().unwrap().is_none());
    }
}

#[test]
fn test_rar_encrypted_entries() {
    for version in [rars::ArchiveVersion::Rar29, rars::ArchiveVersion::Rar50] {
        let data = build(version, |b| b.password(Some(b"secret".to_vec())));

        let mut archive = open(data.clone(), None);
        let entry = archive.next_entry().unwrap().unwrap();
        assert!(entry.is_encrypted());
        assert!(matches!(archive.read(&entry), Err(ArchiveError::EncryptionRequired { .. })));

        let mut archive = open(data.clone(), Some("wrong"));
        let entry = archive.next_entry().unwrap().unwrap();
        assert!(archive.read(&entry).is_err(), "{version:?} accepted a wrong password");

        let mut archive = open(data, Some("secret"));
        assert_eq!(read_all(&mut archive), expected(), "{version:?}");
    }
}

#[test]
fn test_rar_encrypted_headers() {
    let data = build(rars::ArchiveVersion::Rar50, |b| b.password(Some(b"secret".to_vec())).header_encryption(true));

    let mut archive = open(data.clone(), None);
    assert!(matches!(archive.next_entry(), Err(ArchiveError::EncryptionRequired { .. })));

    let mut archive = open(data.clone(), Some("wrong"));
    assert!(matches!(archive.next_entry(), Err(ArchiveError::InvalidPassword { .. })));

    let mut archive = open(data, Some("secret"));
    assert_eq!(read_all(&mut archive), expected());
}

#[test]
fn test_rar_password_verifier() {
    for (version, header_encryption) in [
        (rars::ArchiveVersion::Rar29, false),
        (rars::ArchiveVersion::Rar50, false),
        (rars::ArchiveVersion::Rar50, true),
    ] {
        for solid in [false, true] {
            let data = build(version, |b| {
                b.password(Some(b"secret".to_vec())).header_encryption(header_encryption).solid(solid)
            });
            let mut archive = RarArchive::new(Cursor::new(data)).unwrap();
            archive.set_password("secret");
            let header = archive.get_next_entry().unwrap().unwrap();
            let verifier = archive.create_password_verifier(&header).unwrap();
            assert!(verifier.verify("secret"), "{version:?} hp={header_encryption} solid={solid}");
            assert!(!verifier.verify("wrong"), "{version:?} hp={header_encryption} solid={solid}");
        }
    }
}

#[test]
fn test_rar5_winrar_archive() {
    let mut archive = ArchiveFormat::open_path("tests/acid-70a.rar").expect("Failed to open acid-70a.rar");
    let mut count = 0;
    let mut saw_directory = false;
    while let Some(entry) = archive.next_entry().unwrap() {
        saw_directory |= entry.is_directory();
        assert!(!entry.name().contains('\\'), "unexpected path separator in {}", entry.name());
        assert!(entry.modified_time().is_some());
        // Reading verifies the stored checksums
        let data = archive.read(&entry).unwrap_or_else(|e| panic!("Failed to read {}: {e}", entry.name()));
        assert_eq!(data.len() as u64, entry.original_size());
        if entry.name() == "file_id.diz" {
            assert_eq!(data.len(), 375);
        }
        count += 1;
    }
    assert_eq!(count, 28);
    assert!(saw_directory);
}
