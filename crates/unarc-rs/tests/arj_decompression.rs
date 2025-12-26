use std::io::Cursor;
use std::sync::Arc;

use unarc_rs::arj::{arj_archive::ArjArchive, local_file_header::CompressionMethod};
use unarc_rs::unified::VolumeProvider;

#[test]
fn extract_stored() {
    let file = Cursor::new(include_bytes!("arj/stored.arj"));
    let mut archive = ArjArchive::new(file).unwrap();
    assert_eq!("method1.arj", archive.get_name());
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Stored, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_method1() {
    let file = Cursor::new(include_bytes!("arj/method1.arj"));
    let mut archive = ArjArchive::new(file).unwrap();
    assert_eq!("method2.arj", archive.get_name());
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::CompressedMost, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_method2() {
    let file = Cursor::new(include_bytes!("arj/method2.arj"));
    let mut archive = ArjArchive::new(file).unwrap();
    assert_eq!("method2.arj", archive.get_name());
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Compressed, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_method3() {
    let file = Cursor::new(include_bytes!("arj/method3.arj"));
    let mut archive = ArjArchive::new(file).unwrap();
    assert_eq!("method3.arj", archive.get_name());
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(
        CompressionMethod::CompressedFaster,
        entry.compression_method
    );
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_method4() {
    let file = Cursor::new(include_bytes!("arj/method4.arj"));
    let mut archive = ArjArchive::new(file).unwrap();
    assert_eq!("method4.arj", archive.get_name());
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(
        CompressionMethod::CompressedFastest,
        entry.compression_method
    );
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

/// Volume provider for multi-volume ARJ test using embedded byte arrays
struct TestVolumeProvider {
    volume0: &'static [u8],
    volume1: &'static [u8],
    volume2: &'static [u8],
}

impl VolumeProvider for TestVolumeProvider {
    fn open_volume(&self, volume_number: u32) -> Option<Box<dyn std::io::Read + Send>> {
        match volume_number {
            0 => Some(Box::new(Cursor::new(self.volume0))),
            1 => Some(Box::new(Cursor::new(self.volume1))),
            2 => Some(Box::new(Cursor::new(self.volume2))),
            _ => None,
        }
    }
}

#[test]
fn extract_multi_volume() {
    // Include the multi-volume test files
    let volume0 = include_bytes!("arj/multi/test_file.arj");
    let volume1 = include_bytes!("arj/multi/test_file.a01");
    let volume2 = include_bytes!("arj/multi/test_file.a02");

    // First, let's debug the structure of all volumes
    println!("\n=== Volume 0 (test_file.arj) ===");
    {
        let file = Cursor::new(volume0);
        let mut archive = ArjArchive::new(file).unwrap();
        println!("Archive name: {}", archive.get_name());
        while let Some(entry) = archive.get_next_entry().unwrap() {
            println!(
                "Entry: {}, is_volume: {}, is_ext_file: {}, compressed: {}, original: {}, original_for_volumes: {}, crc: {:08X}",
                entry.name, entry.is_volume(), entry.is_ext_file(),
                entry.compressed_size, entry.original_size, entry.original_size_even_for_volumes,
                entry.original_crc32
            );
            archive.skip(&entry).unwrap();
        }
    }

    println!("\n=== Volume 1 (test_file.a01) ===");
    {
        let file = Cursor::new(volume1);
        let mut archive = ArjArchive::new(file).unwrap();
        println!("Archive name: {}", archive.get_name());
        while let Some(entry) = archive.get_next_entry().unwrap() {
            println!(
                "Entry: {}, is_volume: {}, is_ext_file: {}, compressed: {}, original: {}, original_for_volumes: {}, crc: {:08X}",
                entry.name, entry.is_volume(), entry.is_ext_file(),
                entry.compressed_size, entry.original_size, entry.original_size_even_for_volumes,
                entry.original_crc32
            );
            archive.skip(&entry).unwrap();
        }
    }

    println!("\n=== Volume 2 (test_file.a02) ===");
    {
        let file = Cursor::new(volume2);
        let mut archive = ArjArchive::new(file).unwrap();
        println!("Archive name: {}", archive.get_name());
        while let Some(entry) = archive.get_next_entry().unwrap() {
            println!(
                "Entry: {}, is_volume: {}, is_ext_file: {}, compressed: {}, original: {}, original_for_volumes: {}, crc: {:08X}",
                entry.name, entry.is_volume(), entry.is_ext_file(),
                entry.compressed_size, entry.original_size, entry.original_size_even_for_volumes,
                entry.original_crc32
            );
            archive.skip(&entry).unwrap();
        }
    }

    // Create volume provider
    let volume_provider = TestVolumeProvider {
        volume0,
        volume1,
        volume2,
    };

    // Open the first volume
    let file = Cursor::new(volume0);
    let mut archive = ArjArchive::new(file).unwrap();
    archive.set_volume_provider(Arc::new(volume_provider));

    // Get first entry
    let entry = archive.get_next_entry().unwrap().unwrap();

    // Try to read - this should work with multi-volume support
    let result = archive.read(&entry);
    assert!(
        result.is_ok(),
        "Failed to read multi-volume entry: {:?}",
        result.err()
    );
}
