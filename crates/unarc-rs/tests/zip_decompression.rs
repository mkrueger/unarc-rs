//! Test for multi-volume (split) ZIP archive decompression

use std::path::PathBuf;
use unarc_rs::unified::{ArchiveFormat, ArchiveOptions};

#[test]
fn test_multi_volume_zip_extraction() {
    // Path to the multi-volume test files
    let volumes = vec![
        PathBuf::from("tests/zip/multi/unarc.001"),
        PathBuf::from("tests/zip/multi/unarc.002"),
        PathBuf::from("tests/zip/multi/unarc.003"),
    ];

    // Open the multi-volume ZIP archive
    let mut archive = ArchiveFormat::open_multi_volume_zip(&volumes, ArchiveOptions::new())
        .expect("Failed to open multi-volume ZIP archive");

    // Get the first entry
    let entry = archive
        .next_entry()
        .expect("Failed to get next entry")
        .expect("Expected at least one entry");

    println!("Entry name: {}", entry.name());
    println!("Original size: {}", entry.original_size());
    println!("Compressed size: {}", entry.compressed_size());
    println!("Compression method: {}", entry.compression_method());

    // Read the entry data
    let data = archive.read(&entry).expect("Failed to read entry");

    println!("Extracted {} bytes", data.len());

    // Verify the size matches
    assert_eq!(data.len(), entry.original_size() as usize, "Size mismatch");

    // Check that there are no more entries
    let next = archive.next_entry().expect("Failed to get next entry");
    assert!(next.is_none(), "Expected only one entry");
}

#[test]
fn test_multi_volume_zip_list() {
    // Path to the multi-volume test files
    let volumes = vec![
        PathBuf::from("tests/zip/multi/unarc.001"),
        PathBuf::from("tests/zip/multi/unarc.002"),
        PathBuf::from("tests/zip/multi/unarc.003"),
    ];

    // Open the multi-volume ZIP archive
    let mut archive = ArchiveFormat::open_multi_volume_zip(&volumes, ArchiveOptions::new())
        .expect("Failed to open multi-volume ZIP archive");

    // Count entries using iterator
    let mut count = 0;
    for entry in archive.entries_iter() {
        let entry = entry.expect("Failed to get entry");
        println!("Found: {} ({} bytes)", entry.name(), entry.original_size());
        count += 1;
    }

    assert!(count > 0, "Expected at least one entry");
}
