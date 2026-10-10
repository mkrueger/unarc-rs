use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use unarc_rs::txz::TxzArchive;
use unarc_rs::unified::ArchiveFormat;
use unarc_rs::ArchiveEntryKind;

const LICENSE_CONTENT: &[u8] = include_bytes!("../../../LICENSE");

#[test]
fn extract_txz() {
    let file = File::open("tests/txz/license.tar.xz").expect("Failed to open test file");
    let reader = BufReader::new(file);
    let mut archive = TxzArchive::new(reader).expect("Failed to create TXZ archive");

    assert_eq!(archive.entry_count(), 3);

    let header = archive.get_next_entry().expect("Failed to get entry").expect("No entry found");
    assert_eq!(header.name, "LICENSE");
    assert_eq!(header.size, LICENSE_CONTENT.len() as u64);
    let data = archive.read(&header).expect("Failed to read entry");
    assert_eq!(data, LICENSE_CONTENT);

    let header = archive.get_next_entry().expect("Failed to get entry").expect("No entry found");
    assert_eq!(header.name, "sub/");
    archive.skip(&header).expect("Failed to skip entry");

    let header = archive.get_next_entry().expect("Failed to get entry").expect("No entry found");
    assert_eq!(header.name, "sub/LICENSE");
    let data = archive.read(&header).expect("Failed to read entry");
    assert_eq!(data, LICENSE_CONTENT);

    assert!(archive.get_next_entry().expect("Failed to get entry").is_none());
}

#[test]
fn test_txz_via_unified() {
    for path in ["tests/txz/license.tar.xz", "tests/txz/license.txz"] {
        let mut archive = ArchiveFormat::open_path(path).expect("Failed to open archive");
        assert_eq!(archive.format(), ArchiveFormat::Txz);

        let mut files = Vec::new();
        while let Some(entry) = archive.next_entry().expect("Failed to get entry") {
            assert_eq!(entry.compression_method(), "XZ + Stored");
            if entry.kind() == ArchiveEntryKind::File {
                assert_eq!(archive.read(&entry).expect("Failed to read entry"), LICENSE_CONTENT);
                files.push(entry.name().to_string());
            } else {
                assert!(entry.is_directory());
                archive.skip(&entry).expect("Failed to skip entry");
            }
        }
        assert_eq!(files, ["LICENSE", "sub/LICENSE"], "{path}");
    }
}

#[test]
fn test_txz_detected_by_name() {
    for path in ["tests/txz/license.tar.xz", "tests/txz/license.txz"] {
        let path = Path::new(path);
        let mut file = File::open(path).unwrap();
        assert_eq!(ArchiveFormat::detect(&mut file, Some(path)).unwrap(), Some(ArchiveFormat::Txz));
        // Without a name only the xz layer can be recognised
        assert_eq!(ArchiveFormat::detect(&mut file, None).unwrap(), Some(ArchiveFormat::Xz));
    }
}
