//! Damaged and unusual T64 images.
use std::io::Cursor;

use unarc_rs::t64::{T64Archive, T64Entry};
use unarc_rs::unified::ArchiveFormat;
use unarc_rs::ArchiveError;

const IMAGE: &[u8] = include_bytes!("t64/test.t64");

fn open(image: &[u8]) -> (T64Archive<Cursor<&[u8]>>, Vec<T64Entry>) {
    let mut archive = T64Archive::new(Cursor::new(image)).unwrap();
    let entries = std::iter::from_fn(|| archive.get_next_entry().unwrap()).collect();
    (archive, entries)
}

#[test]
fn rejects_bad_signatures_and_short_headers() {
    assert!(T64Archive::new(Cursor::new(&IMAGE[..40])).is_err());
    let mut tap = IMAGE.to_vec();
    tap[..12].copy_from_slice(b"C64-TAPE-RAW");
    assert!(matches!(T64Archive::new(Cursor::new(&tap)), Err(ArchiveError::InvalidHeader { .. })));
    assert_ne!(ArchiveFormat::detect_from_bytes(&tap), Some(ArchiveFormat::T64));
}

#[test]
fn truncated_image_shortens_the_last_file() {
    // Like VICE, the last file ends at the end of the image.
    let image = &IMAGE[..IMAGE.len() - 100];
    let (mut archive, entries) = open(image);
    assert_eq!(entries.len(), 3);
    let last = &entries[2];
    assert_eq!(last.size(), 690 - 100);
    assert_eq!(archive.read(last).unwrap().len(), 590);
}

#[test]
fn data_offset_outside_the_image_is_an_error() {
    let mut image = IMAGE.to_vec();
    // Second record: offset field at 0x48 + 32
    image[0x68..0x6C].copy_from_slice(&0x0010_0000u32.to_le_bytes());
    let (mut archive, entries) = open(&image);
    assert_eq!(entries.len(), 3);
    assert!(matches!(archive.read(&entries[1]), Err(ArchiveError::CorruptedEntry { .. })));
    // The other entries stay readable (the first one now extends to the next valid offset).
    assert_eq!(archive.read(&entries[0]).unwrap().len(), 22 + 1500);
    assert_eq!(archive.read(&entries[2]).unwrap().len(), 690);

    // An offset pointing into the header
    image[0x68..0x6C].copy_from_slice(&8u32.to_le_bytes());
    let (mut archive, entries) = open(&image);
    assert!(archive.read(&entries[1]).is_err());
}

#[test]
fn wrong_entry_counts() {
    // Used entries 0: the directory records are still read.
    let mut image = IMAGE.to_vec();
    image[36] = 0;
    image[37] = 0;
    assert_eq!(open(&image).1.len(), 3);

    // Maximum entries far larger than the image: reading stops at the file data.
    image[34] = 0xFF;
    image[35] = 0xFF;
    let (mut archive, entries) = open(&image);
    assert_eq!(entries.len(), 3);
    for entry in &entries {
        archive.read(entry).unwrap();
    }

    // Zero maximum entries
    image[34] = 0;
    image[35] = 0;
    assert_eq!(open(&image).1.len(), 1);
}

#[test]
fn file_sizes_are_capped_to_the_address_space() {
    let mut image = IMAGE.to_vec();
    // Third record (DATA) loads at $FFF0, so at most 16 bytes fit into memory.
    image[0x82..0x84].copy_from_slice(&0xFFF0u16.to_le_bytes());
    let (mut archive, entries) = open(&image);
    assert_eq!(entries[2].data_size, 16);
    assert_eq!(archive.read(&entries[2]).unwrap().len(), 18);
}
