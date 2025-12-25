use std::fs::File;
use std::path::Path;
use unarc_rs::unified::{
    is_supported_archive, supported_extensions, ArchiveFormat, ArchiveOptions, UnifiedArchive,
};

#[test]
fn test_arc_via_unified() {
    let file = File::open("tests/arc/store.arc").unwrap();
    let mut archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Arc).unwrap();

    assert_eq!(archive.format(), ArchiveFormat::Arc);

    let entry = archive.next_entry().unwrap().unwrap();
    // ARC file contains LICENSE
    assert!(!entry.file_name().is_empty());

    let data = archive.read(&entry).unwrap();
    assert!(!data.is_empty());
}

#[test]
fn test_arc_encrypted_via_unified_options() {
    // ARC reads consume the underlying reader; use a fresh archive for each attempt.
    {
        let file = File::open("tests/arc/license_cypted.arc").unwrap();
        let mut archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Arc).unwrap();
        let entry = archive.next_entry().unwrap().unwrap();

        // ARC encryption can't be reliably detected; without password, CRC typically fails.
        assert!(archive.read(&entry).is_err());
    }

    let file = File::open("tests/arc/license_cypted.arc").unwrap();
    let mut archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Arc).unwrap();
    let entry = archive.next_entry().unwrap().unwrap();

    let options = ArchiveOptions::new().with_password("SECRET");
    let data = archive.read_with_options(&entry, &options).unwrap();
    assert!(!data.is_empty());
}

#[test]
fn test_arj_via_unified() {
    let file = File::open("tests/arj/stored.arj").unwrap();
    let mut archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Arj).unwrap();

    assert_eq!(archive.format(), ArchiveFormat::Arj);

    let entry = archive.next_entry().unwrap().unwrap();
    // Case may vary
    assert!(entry.file_name().to_lowercase().contains("license"));

    let data = archive.read(&entry).unwrap();
    assert!(!data.is_empty());
    // Check for MIT license text (case-insensitive check)
    let text = String::from_utf8_lossy(&data);
    assert!(text.contains("MIT") || text.contains("License"));
}

#[test]
fn test_zoo_via_unified() {
    let file = File::open("tests/zoo/store.zoo").unwrap();
    let mut archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Zoo).unwrap();

    assert_eq!(archive.format(), ArchiveFormat::Zoo);

    let entry = archive.next_entry().unwrap().unwrap();
    // Case may vary between systems
    assert!(entry.file_name().to_lowercase().contains("license"));

    let data = archive.read(&entry).unwrap();
    assert!(!data.is_empty());
}

#[test]
fn test_ice_via_unified() {
    let file = File::open("tests/ice/license_lha.ice").unwrap();
    let mut archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Ice).unwrap();

    assert_eq!(archive.format(), ArchiveFormat::Ice);

    let entry = archive.next_entry().unwrap().unwrap();
    // ICE is a single-file format, the name is derived from filename or default
    assert!(!entry.file_name().is_empty());
    assert_eq!(entry.compression_method(), "LH1");
    assert_eq!(entry.original_size(), 11357); // Known size of license file

    let data = archive.read(&entry).unwrap();
    assert!(!data.is_empty());
    assert_eq!(data.len(), 11357);

    // Check for MIT license text
    let text = String::from_utf8_lossy(&data);
    assert!(text.contains("MIT") || text.contains("License"));

    // Second call should return None (single-file format)
    assert!(archive.next_entry().unwrap().is_none());
}

#[test]
fn test_hyp_via_unified() {
    let file = File::open("tests/hyp/stored.hyp").unwrap();
    let mut archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Hyp).unwrap();

    assert_eq!(archive.format(), ArchiveFormat::Hyp);

    let entry = archive.next_entry().unwrap().unwrap();
    // HYP might have short names
    assert!(!entry.file_name().is_empty());

    let data = archive.read(&entry).unwrap();
    assert!(!data.is_empty());
}

#[test]
fn test_ha_via_unified() {
    let file = File::open("tests/ha/copy.ha").unwrap();
    let mut archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Ha).unwrap();

    assert_eq!(archive.format(), ArchiveFormat::Ha);

    let entry = archive.next_entry().unwrap().unwrap();
    assert_eq!(entry.name(), "license");

    let data = archive.read(&entry).unwrap();
    assert!(!data.is_empty());
    // Check for license text
    let text = String::from_utf8_lossy(&data);
    assert!(text.contains("MIT") || text.contains("License") || text.contains("license"));
}

#[test]
fn test_tar_via_unified() {
    let file = File::open("tests/tar/license.tar").unwrap();
    let mut archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Tar).unwrap();

    assert_eq!(archive.format(), ArchiveFormat::Tar);

    let entry = archive.next_entry().unwrap().unwrap();
    assert_eq!(entry.name(), "LICENSE");

    let data = archive.read(&entry).unwrap();
    assert!(!data.is_empty());
    // Check for license text
    let text = String::from_utf8_lossy(&data);
    assert!(text.contains("MIT") || text.contains("License"));
}

#[test]
fn test_tgz_via_unified() {
    let file = File::open("tests/tgz/license.tar.gz").unwrap();
    let mut archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Tgz).unwrap();

    assert_eq!(archive.format(), ArchiveFormat::Tgz);

    let entry = archive.next_entry().unwrap().unwrap();
    assert_eq!(entry.name(), "LICENSE");

    let data = archive.read(&entry).unwrap();
    assert!(!data.is_empty());
    // Check for license text
    let text = String::from_utf8_lossy(&data);
    assert!(text.contains("MIT") || text.contains("License"));
}

#[test]
fn test_tbz_via_unified() {
    let file = File::open("tests/tbz/license.tar.bz2").unwrap();
    let mut archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Tbz).unwrap();

    assert_eq!(archive.format(), ArchiveFormat::Tbz);

    let entry = archive.next_entry().unwrap().unwrap();
    assert_eq!(entry.name(), "LICENSE");

    let data = archive.read(&entry).unwrap();
    assert!(!data.is_empty());
    // Check for license text
    let text = String::from_utf8_lossy(&data);
    assert!(text.contains("MIT") || text.contains("License"));
}

#[test]
fn test_sq_via_unified() {
    let file = File::open("tests/qqq/license.sq").unwrap();
    let mut archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Sq).unwrap();

    assert_eq!(archive.format(), ArchiveFormat::Sq);

    let entry = archive.next_entry().unwrap().unwrap();
    // SQ original filename is in the header
    assert!(!entry.name().is_empty());

    let data = archive.read(&entry).unwrap();
    assert!(!data.is_empty());
}

#[test]
fn test_sqz_stored_via_unified() {
    let file = File::open("tests/sqz/store.sqz").unwrap();
    let mut archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Sqz).unwrap();

    assert_eq!(archive.format(), ArchiveFormat::Sqz);

    let entry = archive.next_entry().unwrap().unwrap();
    assert!(entry.file_name().to_lowercase().contains("license"));

    let data = archive.read(&entry).unwrap();
    assert!(!data.is_empty());
}

#[test]
fn test_z_via_unified() {
    let file = File::open("tests/Z/LICENSE.Z").unwrap();
    let mut archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Z).unwrap();
    archive.set_single_file_name("LICENSE".to_string());

    assert_eq!(archive.format(), ArchiveFormat::Z);

    let entry = archive.next_entry().unwrap().unwrap();
    assert_eq!(entry.name(), "LICENSE");

    let data = archive.read(&entry).unwrap();
    assert!(!data.is_empty());
    // Check for license text
    let text = String::from_utf8_lossy(&data);
    assert!(text.contains("MIT") || text.contains("License") || text.contains("license"));
}

#[test]
fn test_entries_iterator() {
    let file = File::open("tests/arj/stored.arj").unwrap();
    let mut archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Arj).unwrap();

    let mut count = 0;
    while let Ok(Some(entry)) = archive.next_entry() {
        assert!(!entry.name().is_empty());
        println!("Entry: {} ({} bytes)", entry.name(), entry.original_size());
        archive.skip(&entry).unwrap();
        count += 1;
    }
    assert!(count > 0);
}

#[test]
fn test_format_detection() {
    assert_eq!(
        ArchiveFormat::from_path(Path::new("test.arj")),
        Some(ArchiveFormat::Arj)
    );
    assert_eq!(
        ArchiveFormat::from_path(Path::new("test.zoo")),
        Some(ArchiveFormat::Zoo)
    );
    assert_eq!(
        ArchiveFormat::from_path(Path::new("test.arc")),
        Some(ArchiveFormat::Arc)
    );
    assert_eq!(
        ArchiveFormat::from_path(Path::new("test.hyp")),
        Some(ArchiveFormat::Hyp)
    );
    assert_eq!(
        ArchiveFormat::from_path(Path::new("test.sqz")),
        Some(ArchiveFormat::Sqz)
    );
    assert_eq!(
        ArchiveFormat::from_path(Path::new("test.sq")),
        Some(ArchiveFormat::Sq)
    );
    assert_eq!(
        ArchiveFormat::from_path(Path::new("test.Z")),
        Some(ArchiveFormat::Z)
    );
    assert_eq!(
        ArchiveFormat::from_path(Path::new("test.ha")),
        Some(ArchiveFormat::Ha)
    );
    assert_eq!(
        ArchiveFormat::from_path(Path::new("test.lha")),
        Some(ArchiveFormat::Lha)
    );
    assert_eq!(
        ArchiveFormat::from_path(Path::new("test.lzh")),
        Some(ArchiveFormat::Lha)
    );
    assert_eq!(
        ArchiveFormat::from_path(Path::new("test.zip")),
        Some(ArchiveFormat::Zip)
    );
    assert_eq!(
        ArchiveFormat::from_path(Path::new("test.rar")),
        Some(ArchiveFormat::Rar)
    );
    assert_eq!(ArchiveFormat::from_path(Path::new("test.txt")), None);
}

#[test]
fn test_is_supported() {
    assert!(is_supported_archive(Path::new("file.arj")));
    assert!(is_supported_archive(Path::new("file.zoo")));
    assert!(is_supported_archive(Path::new("FILE.ARC")));
    assert!(is_supported_archive(Path::new("file.ha")));
    assert!(is_supported_archive(Path::new("file.lha")));
    assert!(is_supported_archive(Path::new("file.lzh")));
    assert!(is_supported_archive(Path::new("file.zip")));
    assert!(is_supported_archive(Path::new("file.rar")));
    assert!(is_supported_archive(Path::new("file.tar")));
    assert!(is_supported_archive(Path::new("file.tgz")));
    assert!(is_supported_archive(Path::new("file.tar.gz")));
    assert!(is_supported_archive(Path::new("file.tbz2")));
    assert!(is_supported_archive(Path::new("file.tar.bz2")));
    assert!(is_supported_archive(Path::new("file.ice")));
    assert!(!is_supported_archive(Path::new("file.txt")));
}

#[test]
fn test_supported_extensions_list() {
    let exts = supported_extensions();
    assert!(exts.contains(&"arj"));
    assert!(exts.contains(&"zoo"));
    assert!(exts.contains(&"arc"));
    assert!(exts.contains(&"hyp"));
    assert!(exts.contains(&"ha"));
    assert!(exts.contains(&"sqz"));
    assert!(exts.contains(&"sq"));
    assert!(exts.contains(&"Z"));
    assert!(exts.contains(&"lha"));
    assert!(exts.contains(&"lzh"));
    assert!(exts.contains(&"zip"));
    assert!(exts.contains(&"rar"));
    assert!(exts.contains(&"tar"));
    assert!(exts.contains(&"tgz"));
    assert!(exts.contains(&"tar.gz"));
    assert!(exts.contains(&"tbz"));
    assert!(exts.contains(&"tbz2"));
    assert!(exts.contains(&"tar.bz2"));
    assert!(exts.contains(&"ice"));
}

#[test]
fn test_entry_metadata() {
    let file = File::open("tests/arj/stored.arj").unwrap();
    let mut archive = UnifiedArchive::open_with_format(file, ArchiveFormat::Arj).unwrap();

    let entry = archive.next_entry().unwrap().unwrap();

    // Check that metadata is accessible
    assert!(!entry.name().is_empty());
    assert!(entry.original_size() > 0);
    assert!(entry.compressed_size() > 0);
    assert!(!entry.compression_method().is_empty());
    assert!(entry.modified_time().is_some());

    // For stored files, compression ratio should be close to 1.0
    let ratio = entry.compression_ratio();
    println!("Compression ratio: {}", ratio);
}
