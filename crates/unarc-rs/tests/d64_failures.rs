//! Damaged D64 images: bad sizes, broken and looping sector chains.
use std::io::Cursor;
use std::path::Path;

use unarc_rs::d64::{D64Archive, D64Entry};
use unarc_rs::unified::ArchiveFormat;
use unarc_rs::ArchiveError;

const IMAGE: &[u8] = include_bytes!("d64/test.d64");

/// Byte offset of a sector in a 35-track image.
fn offset(track: usize, sector: usize) -> usize {
    let spt = |t: usize| match t {
        1..=17 => 21,
        18..=24 => 19,
        25..=30 => 18,
        _ => 17,
    };
    ((1..track).map(spt).sum::<usize>() + sector) * 256
}

fn entries(image: &[u8]) -> Vec<D64Entry> {
    let mut archive = D64Archive::new(Cursor::new(image)).unwrap();
    std::iter::from_fn(|| archive.get_next_entry().unwrap()).collect()
}

fn entry(image: &[u8], name: &str) -> D64Entry {
    entries(image).into_iter().find(|e| e.name == name).unwrap()
}

fn read(image: &[u8], name: &str) -> unarc_rs::Result<Vec<u8>> {
    let archive = D64Archive::new(Cursor::new(image)).unwrap();
    archive.read(&entry(image, name))
}

#[test]
fn rejects_truncated_and_oversized_images() {
    for len in [0, 256, IMAGE.len() - 1, IMAGE.len() + 1] {
        let mut image = IMAGE.to_vec();
        image.resize(len, 0);
        assert!(matches!(D64Archive::new(Cursor::new(&image)), Err(ArchiveError::InvalidHeader { .. })), "{len}");
        assert!(!D64Archive::probe(&mut Cursor::new(&image)).unwrap());
    }
}

#[test]
fn garbage_of_d64_size_is_not_detected() {
    let image: Vec<u8> = (0..IMAGE.len()).map(|i| (i * 31 % 251) as u8).collect();
    assert!(!D64Archive::probe(&mut Cursor::new(&image)).unwrap());
    assert_eq!(ArchiveFormat::detect(&mut Cursor::new(&image), None).unwrap(), None);
    // The extension still selects D64, and opening it does not fail.
    assert_eq!(
        ArchiveFormat::detect(&mut Cursor::new(&image), Some(Path::new("x.d64"))).unwrap(),
        Some(ArchiveFormat::D64)
    );
    let mut archive = D64Archive::new(Cursor::new(&image)).unwrap();
    while let Some(entry) = archive.get_next_entry().unwrap() {
        let _ = archive.read(&entry);
    }
}

#[test]
fn file_chain_loop_is_an_error() {
    let long = entry(IMAGE, "LONG.prg");
    let mut image = IMAGE.to_vec();
    // Point the first sector of LONG back at itself.
    let first = offset(long.track.into(), long.sector.into());
    image[first] = long.track;
    image[first + 1] = long.sector;
    let err = read(&image, "LONG.prg").unwrap_err();
    assert!(matches!(&err, ArchiveError::CorruptedEntry { reason, .. } if reason.contains("loops")), "{err}");
    // The other files are unaffected.
    assert_eq!(read(&image, "DATA.seq").unwrap().len(), 690);
}

#[test]
fn bad_track_sector_links_are_errors() {
    let data = entry(IMAGE, "DATA.seq");
    for (track, sector) in [(36, 0), (99, 0), (18, 19), (1, 21), (31, 17)] {
        let mut image = IMAGE.to_vec();
        image[offset(data.track.into(), data.sector.into())] = track;
        image[offset(data.track.into(), data.sector.into()) + 1] = sector;
        let err = read(&image, "DATA.seq").unwrap_err();
        assert!(
            matches!(&err, ArchiveError::CorruptedEntry { reason, .. } if reason.contains("invalid")),
            "{err}"
        );
    }

    // A directory entry pointing off the disk
    let mut image = IMAGE.to_vec();
    image[offset(18, 1) + 3] = 40;
    assert!(read(&image, "HELLO.prg").is_err());
}

#[test]
fn directory_loops_terminate() {
    // The second directory sector links back to the first one.
    let first = offset(18, 1);
    let next = (usize::from(IMAGE[first]), usize::from(IMAGE[first + 1]));
    assert_ne!(next.0, 0, "fixture directory spans two sectors");
    let mut image = IMAGE.to_vec();
    image[offset(next.0, next.1)] = 18;
    image[offset(next.0, next.1) + 1] = 1;
    assert_eq!(entries(&image).len(), 10);

    // The first directory sector links to itself.
    let mut image = IMAGE.to_vec();
    image[first] = 18;
    image[first + 1] = 1;
    assert_eq!(entries(&image).len(), 8);
}

#[test]
fn bad_directory_link_ends_the_listing() {
    let mut image = IMAGE.to_vec();
    image[offset(18, 1)] = 0xFF;
    assert_eq!(entries(&image).len(), 8);
}

#[test]
fn unified_read_respects_size_limit() {
    let options = unarc_rs::ArchiveOptions::new().with_max_entry_size(Some(100));
    let mut archive = ArchiveFormat::D64.open_with_options(Cursor::new(IMAGE), options).unwrap();
    let hello = archive.next_entry().unwrap().unwrap();
    assert_eq!(archive.read(&hello).unwrap().len(), 22);
    let data = archive.next_entry().unwrap().unwrap();
    assert!(matches!(archive.read(&data), Err(ArchiveError::SizeLimitExceeded { .. })));
}

#[test]
fn directory_art_pointing_at_the_bam_is_empty() {
    // Scene disks list separator lines as DEL entries whose first sector is 18/0
    let mut image = IMAGE.to_vec();
    let directory = offset(18, 1);
    // Turn the first entry into one
    let slot = directory;
    image[slot + 2] = 0x80; // closed DEL
    image[slot + 3] = 18;
    image[slot + 4] = 0;
    image[slot + 5..slot + 21].copy_from_slice(b"----------------");
    let separator = entry(&image, "----------------.del");
    assert_eq!(separator.size, 0);
    assert_eq!(read(&image, "----------------.del").unwrap(), b"");
}
