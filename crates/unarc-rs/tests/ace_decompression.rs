//! ACE archive decompression tests

use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use unarc_rs::ace::AceArchive;
use unarc_rs::unified::VolumeProvider;

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
