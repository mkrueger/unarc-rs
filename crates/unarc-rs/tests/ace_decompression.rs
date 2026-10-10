//! ACE archive decompression tests

use std::fs::File;
use std::io::{Cursor, Read};
use std::sync::Arc;
use unarc_rs::ace::AceArchive;
use unarc_rs::unified::VolumeProvider;

#[path = "common/ace.rs"]
mod ace;
use ace::{ace_main_header, synthetic_ace, synthetic_encrypted_ace};

#[test]
fn test_ace1_archive() {
    let file = File::open("tests/ace/license1.ace").expect("Failed to open test file");
    let mut archive = AceArchive::new(file).expect("Failed to open ACE archive");

    let mut found_files = Vec::new();

    while let Ok(Some(entry)) = archive.get_next_entry() {
        println!("Entry: {} ({} -> {} bytes)", entry.filename, entry.packed_size, entry.original_size);
        found_files.push(entry.filename.clone());

        if !entry.is_directory() {
            let data = archive.read(&entry).expect("Failed to decompress");
            assert_eq!(data.len(), entry.original_size as usize);
        }
    }

    assert!(!found_files.is_empty(), "No files found in archive");
}

#[test]
fn test_ace2_archive() {
    let file = File::open("tests/ace/license2.ace").expect("Failed to open test file");
    let mut archive = AceArchive::new(file).expect("Failed to open ACE archive");

    let mut found_files = Vec::new();

    while let Ok(Some(entry)) = archive.get_next_entry() {
        println!(
            "Entry: {} ({} -> {} bytes, {:?})",
            entry.filename, entry.packed_size, entry.original_size, entry.compression_type
        );
        found_files.push(entry.filename.clone());

        if !entry.is_directory() {
            let data = archive.read(&entry).expect("Failed to decompress");
            assert_eq!(data.len(), entry.original_size as usize);
        }
    }

    assert!(!found_files.is_empty(), "No files found in archive");
}

/// Volume provider for ACE multi-volume test archives
struct AceTestVolumeProvider;

impl VolumeProvider for AceTestVolumeProvider {
    fn open_volume(&self, volume_number: u32) -> Option<Box<dyn Read + Send>> {
        let path = if volume_number == 0 {
            "tests/ace/multi/unarc.ace".to_string()
        } else {
            format!("tests/ace/multi/unarc.c{:02}", volume_number - 1)
        };
        File::open(&path).ok().map(|f| Box::new(f) as Box<dyn Read + Send>)
    }
}

#[test]
fn test_ace_multivolume() {
    let file = File::open("tests/ace/multi/unarc.ace").expect("Failed to open test file");
    let mut archive = AceArchive::new(file).expect("Failed to open ACE archive");

    // Set up volume provider for multi-volume support
    archive.set_volume_provider(Arc::new(AceTestVolumeProvider));

    assert!(archive.is_multivolume(), "Archive should be multi-volume");

    let mut found_files = Vec::new();
    let mut total_size = 0usize;

    while let Ok(Some(entry)) = archive.get_next_entry() {
        found_files.push(entry.filename.clone());

        if !entry.is_directory() {
            let data = archive.read(&entry).expect("Failed to decompress");
            assert_eq!(data.len(), entry.original_size as usize, "Size mismatch for {}", entry.filename);
            total_size += data.len();
        }
    }

    assert!(!found_files.is_empty(), "No files found in archive");
    assert!(total_size > 0, "Expected non-zero total size");
}

#[test]
fn stored_prefix_populates_solid_dictionary() {
    let mut archive = AceArchive::new(Cursor::new(synthetic_ace(true, true))).unwrap();
    let first = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(archive.read(&first).unwrap(), b"A");
    let second = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(archive.read(&second).unwrap(), b"AA");
}

#[test]
fn canonical_solid_flag_and_per_member_tree_reset() {
    let mut archive = AceArchive::new(Cursor::new(synthetic_ace(true, false))).unwrap();
    assert!(archive.is_solid());
    let first = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(archive.read(&first).unwrap(), b"A");
    let second = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(archive.read(&second).unwrap(), b"AA");
}

#[test]
fn non_solid_members_reinitialize_their_trees() {
    let mut archive = AceArchive::new(Cursor::new(synthetic_ace(false, false))).unwrap();
    assert!(!archive.is_solid());
    let first = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(archive.read(&first).unwrap(), b"A");
    let second = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(archive.read(&second).unwrap(), b"B");
}

#[test]
fn unrelated_low_main_flag_is_not_solid() {
    let archive = AceArchive::new(Cursor::new(ace_main_header(0x0010))).unwrap();
    assert!(!archive.is_solid());
}

#[test]
fn extraction_skip_preserves_solid_history_and_enforces_limits() {
    use unarc_rs::error::ArchiveError;
    use unarc_rs::unified::{ArchiveFormat, ArchiveOptions, UnifiedArchive};

    for limit in [7, 8, 10] {
        let mut archive = UnifiedArchive::open_with_format(Cursor::new(synthetic_encrypted_ace(true)), ArchiveFormat::Ace).unwrap();
        let options = ArchiveOptions::new().with_password("test").with_max_total_size(Some(limit));
        let first = archive.next_entry().unwrap().unwrap();
        let skipped = archive.skip_with_options(&first, &options);
        if limit < 8 {
            assert!(matches!(skipped, Err(ArchiveError::SizeLimitExceeded { .. })));
            continue;
        }
        skipped.unwrap();
        let second = archive.next_entry().unwrap().unwrap();
        let result = archive.read_with_options(&second, &options);
        if limit < 10 {
            assert!(matches!(result, Err(ArchiveError::SizeLimitExceeded { .. })));
        } else {
            assert_eq!(result.unwrap(), b"AA");
        }
    }
}
