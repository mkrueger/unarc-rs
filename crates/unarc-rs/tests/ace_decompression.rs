//! ACE archive decompression tests

use std::fs::File;
use std::io::{Cursor, Read};
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

struct AceBits(Vec<bool>);

impl AceBits {
    fn push(&mut self, value: u32, width: u32) {
        for bit in (0..width).rev() {
            self.0.push(value & (1 << bit) != 0);
        }
    }

    fn single_symbol_tree(&mut self, symbol: u32) {
        self.push(symbol, 9);
        self.push(0, 4);
        self.push(2, 4);
        for width in [1, 1, 0] {
            self.push(width, 3);
        }
        // Delta widths are zero until the selected symbol; this width tree encodes 0 as 1.
        for index in 0..=symbol {
            self.push(u32::from(index != symbol), 1);
        }
    }

    fn finish(self) -> Vec<u8> {
        let mut bytes = vec![0u8; self.0.len().div_ceil(32) * 4];
        for (index, bit) in self.0.into_iter().enumerate() {
            if bit {
                let word = index / 32 * 4;
                let value = u32::from_le_bytes(bytes[word..word + 4].try_into().unwrap());
                bytes[word..word + 4].copy_from_slice(&(value | (1 << (31 - index % 32))).to_le_bytes());
            }
        }
        bytes
    }
}

fn ace_lz77_payload(symbol: u32, symbols_in_block: u32) -> Vec<u8> {
    let mut bits = AceBits(Vec::new());
    bits.single_symbol_tree(symbol);
    bits.single_symbol_tree(0);
    bits.push(symbols_in_block, 15);
    bits.push(0, 1);
    if symbol == 260 {
        // Explicit distance zero means a distance-one copy; length symbol zero means two bytes.
        bits.push(0, 1);
    }
    bits.finish()
}

fn ace_header(data: &[u8]) -> Vec<u8> {
    let mut bytes = ((!crc32fast::hash(data)) as u16).to_le_bytes().to_vec();
    bytes.extend_from_slice(&(data.len() as u16).to_le_bytes());
    bytes.extend_from_slice(data);
    bytes
}

fn ace_main_header(flags: u16) -> Vec<u8> {
    let mut data = vec![0];
    data.extend_from_slice(&flags.to_le_bytes());
    data.extend_from_slice(b"**ACE**");
    data.extend_from_slice(&[10, 10, 0, 0]);
    data.extend_from_slice(&[0; 12]);
    ace_header(&data)
}

fn ace_member(name: &str, method: u8, packed: &[u8], output: &[u8]) -> Vec<u8> {
    let mut data = vec![1];
    data.extend_from_slice(&1u16.to_le_bytes());
    data.extend_from_slice(&(packed.len() as u32).to_le_bytes());
    data.extend_from_slice(&(output.len() as u32).to_le_bytes());
    data.extend_from_slice(&[0; 8]);
    data.extend_from_slice(&(!crc32fast::hash(output)).to_le_bytes());
    data.extend_from_slice(&[method, 0]);
    data.extend_from_slice(&[0; 4]);
    data.extend_from_slice(&(name.len() as u16).to_le_bytes());
    data.extend_from_slice(name.as_bytes());
    let mut bytes = ace_header(&data);
    bytes.extend_from_slice(packed);
    bytes
}

fn synthetic_ace(solid: bool, stored_prefix: bool) -> Vec<u8> {
    let mut bytes = ace_main_header(if solid { 0x8000 } else { 0 });
    let first = if stored_prefix { b"A".to_vec() } else { ace_lz77_payload(65, 2) };
    bytes.extend(ace_member("first.txt", u8::from(!stored_prefix), &first, b"A"));
    let second = ace_lz77_payload(if solid { 260 } else { 66 }, 1);
    bytes.extend(ace_member("second.txt", 1, &second, if solid { b"AA" } else { b"B" }));
    bytes
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
