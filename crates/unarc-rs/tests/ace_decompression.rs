//! ACE archive decompression tests

use std::fs::File;
use unarc_rs::ace::AceArchive;

#[test]
fn test_ace1_archive() {
    let file = File::open("tests/ace/license1.ace").expect("Failed to open test file");
    let mut archive = AceArchive::new(file).expect("Failed to open ACE archive");

    let mut found_files = Vec::new();

    while let Ok(Some(entry)) = archive.get_next_entry() {
        println!(
            "Entry: {} ({} -> {} bytes)",
            entry.filename, entry.packed_size, entry.original_size
        );
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
