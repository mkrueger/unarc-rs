//! Unsupported, corrupt and hostile Amiga LZX archives
use std::io::Cursor;

use unarc_rs::error::ArchiveError;
use unarc_rs::lzx::{LzxArchive, LzxEntry, PackMode};
use unarc_rs::unified::{ArchiveFormat, ArchiveOptions, UnifiedArchive};

const LICENSE: &[u8] = include_bytes!("../../../LICENSE");
const SINGLE: &[u8] = include_bytes!("lzx/license.lzx");
const MULTI: &[u8] = include_bytes!("lzx/multi.lzx");

// license.lzx: info header, one 31-byte entry header named "LICENSE", 4014 packed bytes
const HEADER: usize = 10;
const DATA: usize = HEADER + 31 + 7;
// multi.lzx: README.TXT (stored, with comment) and the header of docs/binary.dat, the last of the merged group
const README_HEADER: usize = 10;
const README_DATA: usize = README_HEADER + 31 + 10 + 27;
const BINARY_HEADER: usize = 288;

/// Recomputes the header CRC of the entry header at `offset`
fn fix_header_crc(data: &mut [u8], offset: usize) {
    let end = offset + 31 + usize::from(data[offset + 30]) + usize::from(data[offset + 14]);
    data[offset + 26..offset + 30].fill(0);
    let crc = crc32fast::hash(&data[offset..end]);
    data[offset + 26..offset + 30].copy_from_slice(&crc.to_le_bytes());
}

fn patched(archive: &[u8], header: usize, field: usize, bytes: &[u8]) -> Vec<u8> {
    let mut data = archive.to_vec();
    data[header + field..header + field + bytes.len()].copy_from_slice(bytes);
    fix_header_crc(&mut data, header);
    data
}

fn entries(archive: &mut LzxArchive<Cursor<Vec<u8>>>) -> (Vec<LzxEntry>, Option<ArchiveError>) {
    let mut entries = Vec::new();
    loop {
        match archive.get_next_entry() {
            Ok(Some(entry)) => entries.push(entry),
            Ok(None) => return (entries, None),
            Err(error) => return (entries, Some(error)),
        }
    }
}

fn read_first(data: Vec<u8>) -> unarc_rs::Result<Vec<u8>> {
    let mut archive = LzxArchive::new(Cursor::new(data))?;
    let entry = archive.get_next_entry()?.unwrap();
    archive.read(&entry)
}

#[test]
fn not_an_lzx_archive() {
    for data in [&b""[..], b"LZ", b"LZY\0\0\0\0\0\0\0", b"LZX\0"] {
        assert!(matches!(LzxArchive::new(Cursor::new(data.to_vec())), Err(ArchiveError::InvalidHeader { .. })));
    }
    // An info header alone is an empty archive.
    let mut archive = LzxArchive::new(Cursor::new(SINGLE[..HEADER].to_vec())).unwrap();
    assert!(archive.get_next_entry().unwrap().is_none());
}

#[test]
fn header_crc_mismatch() {
    let mut data = SINGLE.to_vec();
    data[HEADER + 2] ^= 1;
    assert!(matches!(LzxArchive::new(Cursor::new(data)), Err(ArchiveError::CorruptedEntry { .. })));

    // A damaged later header still lists the entries before it.
    let mut data = MULTI.to_vec();
    data[BINARY_HEADER + 40] ^= 1;
    let mut archive = LzxArchive::new(Cursor::new(data)).unwrap();
    let (list, error) = entries(&mut archive);
    assert_eq!(list.len(), 5);
    assert!(matches!(error, Some(ArchiveError::CorruptedEntry { reason, .. }) if reason == "header CRC mismatch"));
    assert_eq!(archive.read(&list[0]).unwrap(), b"Self-authored LZX test fixture for unarc-rs.\n");
    // The merged group never got its data.
    assert!(matches!(archive.read(&list[2]), Err(ArchiveError::CorruptedEntry { .. })));
    assert!(archive.read(&list[4]).unwrap().is_empty());
}

#[test]
fn data_crc_mismatch() {
    let mut data = MULTI.to_vec();
    data[README_DATA] ^= 0x20;
    assert!(matches!(read_first(data), Err(ArchiveError::CrcMismatch { .. })));

    let data = patched(SINGLE, HEADER, 22, &0x1234_5678u32.to_le_bytes());
    assert!(matches!(
        read_first(data),
        Err(ArchiveError::CrcMismatch {
            expected: 0x1234_5678,
            actual: 0x7B5D_04BC,
            ..
        })
    ));
}

#[test]
fn corrupt_compressed_data_never_panics() {
    // Every byte of the block header and trees, then a sample of the token data
    for position in (DATA..SINGLE.len()).filter(|p| p - DATA < 400 || p % 13 == 0) {
        for mask in [0x01, 0xFF] {
            let mut data = SINGLE.to_vec();
            data[position] ^= mask;
            match read_first(data) {
                Ok(output) => assert_eq!(output, LICENSE, "byte {position} ^ {mask:#x}"),
                Err(ArchiveError::CrcMismatch { .. } | ArchiveError::CorruptedEntry { .. }) => {}
                Err(error) => panic!("byte {position} ^ {mask:#x}: {error}"),
            }
        }
    }
}

#[test]
fn truncated_archives_never_panic() {
    for (archive, step) in [(SINGLE, 1), (MULTI, 61)] {
        for cut in 0..archive.len() {
            if cut % step != 0 && cut > 400 {
                continue;
            }
            let Ok(mut lzx) = LzxArchive::new(Cursor::new(archive[..cut].to_vec())) else {
                continue;
            };
            let (list, _) = entries(&mut lzx);
            for entry in list {
                let _ = lzx.read(&entry);
            }
        }
    }
    // Cut inside the packed data: listed, but not readable.
    let mut archive = LzxArchive::new(Cursor::new(SINGLE[..SINGLE.len() - 1].to_vec())).unwrap();
    let (list, error) = entries(&mut archive);
    assert_eq!(list.len(), 1);
    assert!(matches!(error, Some(ArchiveError::CorruptedEntry { reason, .. }) if reason == "entry data extends past the end of the archive"));
    assert!(matches!(archive.read(&list[0]), Err(ArchiveError::CorruptedEntry { reason, .. }) if reason == "the archive ends before the entry's data"));
}

#[test]
fn trailing_bytes() {
    // Too short for a header (e.g. XMODEM padding): ignored
    let mut data = SINGLE.to_vec();
    data.extend_from_slice(&[0x1A; 30]);
    let mut archive = LzxArchive::new(Cursor::new(data)).unwrap();
    let (list, error) = entries(&mut archive);
    assert_eq!(list.len(), 1);
    assert!(error.is_none());
    assert_eq!(archive.read(&list[0]).unwrap(), LICENSE);

    // Long enough for a header, but not one
    let mut data = SINGLE.to_vec();
    data.extend_from_slice(&[0x1A; 128]);
    let mut archive = LzxArchive::new(Cursor::new(data)).unwrap();
    let (list, error) = entries(&mut archive);
    assert_eq!(list.len(), 1);
    assert!(error.is_some());
    assert_eq!(archive.read(&list[0]).unwrap(), LICENSE);
}

#[test]
fn unknown_pack_mode() {
    let data = patched(SINGLE, HEADER, 11, &[1]);
    let mut archive = UnifiedArchive::open_with_format(Cursor::new(data), ArchiveFormat::Lzx).unwrap();
    let entry = archive.next_entry().unwrap().unwrap();
    assert_eq!(entry.compression_method(), "Unknown(1)");
    assert!(matches!(
        archive.read(&entry),
        Err(ArchiveError::UnsupportedMethod { method, .. }) if method == "pack mode 1"
    ));

    let mut archive = LzxArchive::new(Cursor::new(patched(SINGLE, HEADER, 11, &[3]))).unwrap();
    assert_eq!(archive.get_next_entry().unwrap().unwrap().pack_mode, PackMode::Unknown(3));
}

#[test]
fn stored_size_larger_than_its_data() {
    // Original size 46 with 45 stored bytes
    let data = patched(MULTI, README_HEADER, 2, &46u32.to_le_bytes());
    assert!(matches!(read_first(data), Err(ArchiveError::CorruptedEntry { reason, .. }) if reason == "stored data is shorter than the entry"));
}

#[test]
fn size_limits() {
    let options = ArchiveOptions::new().with_max_entry_size(Some(11356));
    let mut archive = UnifiedArchive::open_with_format_and_options(Cursor::new(SINGLE), ArchiveFormat::Lzx, options).unwrap();
    let entry = archive.next_entry().unwrap().unwrap();
    assert!(matches!(archive.read(&entry), Err(ArchiveError::SizeLimitExceeded { limit: 11356, .. })));

    // A forged size cannot make the decoder produce more than its input allows.
    let data = patched(SINGLE, HEADER, 2, &u32::MAX.to_le_bytes());
    let options = ArchiveOptions::new().with_max_entry_size(None);
    let mut archive = UnifiedArchive::open_with_format_and_options(Cursor::new(data), ArchiveFormat::Lzx, options).unwrap();
    let entry = archive.next_entry().unwrap().unwrap();
    assert!(matches!(archive.read(&entry), Err(ArchiveError::CorruptedEntry { .. })));
}

#[test]
fn entries_of_another_archive_are_rejected() {
    let mut multi = LzxArchive::new(Cursor::new(MULTI.to_vec())).unwrap();
    let (list, _) = entries(&mut multi);
    let mut single = LzxArchive::new(Cursor::new(SINGLE.to_vec())).unwrap();
    assert!(matches!(single.read(&list[5]), Err(ArchiveError::IndexMismatch(_))));
    // Same group index, but beyond the end of the group
    let (other, _) = entries(&mut single);
    let mut multi = LzxArchive::new(Cursor::new(MULTI.to_vec())).unwrap();
    assert!(matches!(multi.read(&other[0]), Err(ArchiveError::IndexMismatch(_))));
}
