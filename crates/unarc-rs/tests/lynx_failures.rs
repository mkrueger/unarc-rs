//! Damaged and hostile Lynx containers.
use std::io::Cursor;

use unarc_rs::lynx::{LynxArchive, LynxEntry};
use unarc_rs::unified::{ArchiveFormat, ArchiveOptions};
use unarc_rs::ArchiveError;

const BASIC: &[u8] = include_bytes!("lynx/basic.lnx");

fn list(image: &[u8]) -> (Vec<LynxEntry>, Option<ArchiveError>) {
    let mut archive = LynxArchive::new(Cursor::new(image)).unwrap();
    let mut entries = Vec::new();
    loop {
        match archive.get_next_entry() {
            Ok(Some(entry)) => entries.push(entry),
            Ok(None) => return (entries, None),
            Err(err) => {
                // The iteration ends after an error.
                assert!(archive.get_next_entry().unwrap().is_none());
                return (entries, Some(err));
            }
        }
    }
}

/// A container with one directory block and the given directory text.
fn container(directory: &[u8], data: &[u8]) -> Vec<u8> {
    let mut image = directory.to_vec();
    image.resize(254, 0);
    image.extend_from_slice(data);
    image
}

#[test]
fn not_a_lynx_container() {
    for data in [&b""[..], b"hello world", b" 1  *LYNX\r", b" 0  *LYNX\r 1 \r", &[0u8; 2000]] {
        assert!(matches!(LynxArchive::new(Cursor::new(data)), Err(ArchiveError::InvalidHeader { .. })));
    }
    // The directory must not end before the header does.
    let mut image = vec![0x01, 0x08];
    image.resize(300, b'A');
    image.extend_from_slice(b"\0\0\0\r 1  *LYNX\r 1 \rNAME\r 1 \rP\r 3 \r");
    assert!(matches!(LynxArchive::new(Cursor::new(&image)), Err(ArchiveError::InvalidHeader { .. })));
    let pos = image.windows(4).position(|w| w == b" 1  ").unwrap();
    image[pos + 1] = b'2';
    assert!(LynxArchive::new(Cursor::new(&image)).is_ok());
}

#[test]
fn truncated_container() {
    let image = &BASIC[..BASIC.len() - 600];
    let mut archive = LynxArchive::new(Cursor::new(image)).unwrap();
    let mut results = Vec::new();
    while let Some(entry) = archive.get_next_entry().unwrap() {
        results.push((entry.name.clone(), archive.read(&entry).is_ok()));
    }
    // LONG and NOTE lie (partly) behind the cut.
    assert_eq!(
        results,
        [
            ("HELLO.prg".into(), true),
            ("DATA.seq".into(), true),
            ("EXACT.prg".into(), true),
            ("LONG.prg".into(), false),
            ("NOTE.usr".into(), false)
        ]
    );
}

#[test]
fn more_entries_announced_than_present() {
    let image = container(b" 1  *LYNX\r 9 \rONLY\r 1 \rP\r 3 \r", &[1, 8]);
    let (entries, err) = list(&image);
    assert_eq!(entries.len(), 1);
    assert!(matches!(err, Some(ArchiveError::CorruptedEntry { .. })));
}

#[test]
fn malformed_entries() {
    for directory in [
        &b" 1  *LYNX\r 1 \rNAME IS FAR TOO LONG\r 1 \rP\r 3 \r"[..],
        b" 1  *LYNX\r 1 \rNAME\rX\rP\r 3 \r",
        b" 1  *LYNX\r 1 \rNAME\r 1 \r\r 3 \r",
        b" 1  *LYNX\r 2 \rNAME\r 1 \rP\rX\rNEXT\r 1 \rP\r 3 \r",
        b" 1  *LYNX\r 1 \rNAME\r 1 \rP\r 1 \r",
        b" 1  *LYNX\r 1 \rNAME\r 1 \rP\r 256 \r",
        b" 1  *LYNX\r 1 \rNAME\r 0 \rR\r 32 \r 0 \r",
        b" 1  *LYNX\r 1 \rNAME\r 2 \rR\r 300 \r 2 \r",
    ] {
        let (entries, err) = list(&container(directory, &[0; 600]));
        assert!(entries.is_empty(), "{}", String::from_utf8_lossy(directory));
        assert!(
            matches!(err, Some(ArchiveError::CorruptedEntry { .. })),
            "{}",
            String::from_utf8_lossy(directory)
        );
    }
}

#[test]
fn rel_side_sectors() {
    // 121 data blocks need two side sectors, stored in front of the data.
    let mut data = vec![0x55; 2 * 254];
    data.extend((0..121 * 254).map(|i| (i % 253) as u8));
    let image = container(b" 1  *LYNX\r 1 \rREL\r 123 \rR\r 10 \r 255 \r", &data);
    let (entries, err) = list(&image);
    assert!(err.is_none());
    assert_eq!(entries[0].offset, 3 * 254);
    assert_eq!(entries[0].size, 121 * 254);
    let mut archive = LynxArchive::new(Cursor::new(&image)).unwrap();
    assert_eq!(archive.read(&entries[0]).unwrap(), data[2 * 254..]);
}

#[test]
fn forged_sizes_do_not_allocate() {
    let image = container(b" 1  *LYNX\r 1 \rHUGE\r 999999999 \rP\r 255 \r", &[1, 8, 0, 0]);
    let (entries, _) = list(&image);
    assert_eq!(entries[0].size, 999_999_999 * 254);
    let mut archive = LynxArchive::new(Cursor::new(&image)).unwrap();
    assert!(matches!(archive.read(&entries[0]), Err(ArchiveError::CorruptedEntry { .. })));

    let mut archive = ArchiveFormat::Lynx.open_with_options(Cursor::new(&image), ArchiveOptions::new()).unwrap();
    let entry = archive.next_entry().unwrap().unwrap();
    assert!(matches!(archive.read(&entry), Err(ArchiveError::SizeLimitExceeded { .. })));
}

#[test]
fn huge_directory_and_entry_counts() {
    // The directory claims 999999999 blocks; only what is present is read.
    let image = container(b" 999999999  *LYNX\r 999999999 \rA\r 1 \rP\r 3 \r", &[1, 8]);
    let (entries, err) = list(&image);
    assert_eq!(entries.len(), 1);
    assert!(err.is_some());
}

#[test]
fn last_file_a_few_bytes_short_keeps_what_is_there() {
    // Lynx XVI and Star Lynx can write the container short of the last file's recorded size
    let mut archive = LynxArchive::new(Cursor::new(BASIC)).unwrap();
    let last = std::iter::from_fn(|| archive.get_next_entry().unwrap()).last().unwrap();
    let full = archive.read(&last).unwrap();
    let image = &BASIC[..usize::try_from(last.offset + last.size).unwrap() - 1];

    let mut archive = LynxArchive::new(Cursor::new(image)).unwrap();
    let short = std::iter::from_fn(|| archive.get_next_entry().unwrap()).last().unwrap();
    assert_eq!(short.size, last.size - 1);
    assert_eq!(archive.read(&short).unwrap(), &full[..full.len() - 1]);
}
