use std::io::Cursor;

use unarc_rs::amiga::AmigaArchive;
use unarc_rs::error::ArchiveError;

#[path = "common/amiga.rs"]
mod common;

fn read_file(data: Vec<u8>) -> unarc_rs::Result<Vec<u8>> {
    let mut archive = AmigaArchive::open_hdf(Cursor::new(data))?;
    archive.get_next_entry()?.unwrap();
    let entry = archive.get_next_entry()?.unwrap();
    archive.read(&entry)
}

#[test]
fn corrupt_root_directory_and_names_are_rejected() {
    for (block, word, value) in [
        (8, 3, 500),
        (8, 6, 999),
        (8, 6, 8),
        (3, 6, 999),
        (4, 124, 4),
        (4, 125, 8),
        (4, 1, 7),
        (4, 127, 999),
        (4, 106, 1440),
        (4, 107, 3000),
    ] {
        let mut data = common::image(1, 512, 16);
        let header = &mut data[block * 512..(block + 1) * 512];
        common::put(header, word, value);
        common::fix_checksum(header, 5);
        assert!(AmigaArchive::open_hdf(Cursor::new(data)).is_err(), "block={block}, word={word}");
    }
    for (offset, value) in [(432, 31), (433, b'/'), (433, 0), (328, 80)] {
        let mut data = common::image(0, 512, 16);
        let header = &mut data[4 * 512..5 * 512];
        header[offset] = value;
        common::fix_checksum(header, 5);
        assert!(AmigaArchive::open_hdf(Cursor::new(data)).is_err());
    }
    let mut data = common::image(0, 512, 16);
    data[8 * 512 + 50] ^= 1;
    assert!(AmigaArchive::open_hdf(Cursor::new(data)).is_err());
}

#[test]
fn bad_file_sizes_and_pointers_are_rejected() {
    for mode in [0, 1] {
        for (word, value) in [(2, 0), (2, 73), (81, u32::MAX), (77, 0), (77, 999), (77, 3), (77, 8), (126, 4)] {
            let mut data = common::image(mode, 512, 16);
            let header = &mut data[4 * 512..5 * 512];
            common::put(header, word, value);
            common::fix_checksum(header, 5);
            assert!(read_file(data).is_err(), "mode={mode}, word={word}");
        }
    }
}

#[test]
fn ofs_validates_every_data_header_field_and_checksum() {
    for (word, value) in [(0, 7), (1, 3), (2, 2), (3, 0), (3, 489), (4, 5)] {
        let mut data = common::image(0, 512, 16);
        let block = &mut data[5 * 512..6 * 512];
        common::put(block, word, value);
        common::fix_checksum(block, 5);
        assert!(read_file(data).is_err(), "word={word}");
    }
    let mut data = common::image(0, 512, 16);
    data[5 * 512 + 24] ^= 1;
    assert!(read_file(data).is_err());
}

#[test]
fn extension_cycles_wrong_parents_and_checksums_are_rejected() {
    for fixture in [&include_bytes!("amiga/ofs.adf")[..], &include_bytes!("amiga/ffs.adf")[..]] {
        let header_offset = fixture
            .as_chunks::<512>()
            .0
            .iter()
            .position(|block| common::get(&block[..], 127) as i32 == -3 && block[432] == 9 && &block[433..442] == b"large.bin")
            .unwrap()
            * 512;
        let extension = common::get(&fixture[header_offset..header_offset + 512], 126) as usize;
        assert_ne!(extension, 0);
        for (word, value) in [(0, 2), (125, 999), (126, extension as u32), (2, 100)] {
            let mut data = fixture.to_vec();
            let block = &mut data[extension * 512..(extension + 1) * 512];
            common::put(block, word, value);
            common::fix_checksum(block, 5);
            let mut archive = AmigaArchive::open_adf(Cursor::new(data)).unwrap();
            while let Some(entry) = archive.get_next_entry().unwrap() {
                if entry.name == "docs/large.bin" {
                    assert!(archive.read(&entry).is_err(), "word={word}");
                }
            }
        }
    }
}

#[test]
fn rdb_checksums_geometry_lists_and_unsupported_filesystems() {
    let fixture = include_bytes!("amiga/partitions.hdf");
    let partition = common::get(&fixture[..512], 7) as usize;
    for (block, word, value) in [
        (0, 1, 500),
        (0, 6, 1),
        (0, 7, 999),
        (0, 33, u32::MAX),
        (partition, 4, partition as u32),
        (partition, 32, 10),
        (partition, 33, 0),
        (partition, 35, u32::MAX),
        (partition, 36, 0),
        (partition, 41, 0),
        (partition, 42, u32::MAX),
    ] {
        let mut data = fixture.to_vec();
        let header = &mut data[block * 512..(block + 1) * 512];
        common::put(header, word, value);
        common::fix_checksum(&mut header[..256], 2);
        assert!(AmigaArchive::open_hdf(Cursor::new(data)).is_err(), "block={block}, word={word}");
    }
    let mut data = fixture.to_vec();
    data[512 + 40] ^= 1;
    assert!(AmigaArchive::open_hdf(Cursor::new(data)).is_err());
    let mut data = fixture.to_vec();
    let start = common::get(&data[partition * 512..(partition + 1) * 512], 41) as usize * 22 * 512;
    data[start..start + 4].copy_from_slice(b"PFS\x03");
    assert!(matches!(AmigaArchive::open_hdf(Cursor::new(data)), Err(ArchiveError::UnsupportedFormat(_))));
}

#[test]
fn unformatted_and_truncated_images_fail_explicitly() {
    assert!(AmigaArchive::open_adf(Cursor::new(vec![0; 901120])).is_err());
    for signature in [b"DOS\x06", b"DOS\x07", b"PFS\x03", b"SFS\x00"] {
        let mut data = common::image(1, 512, 16);
        data[..4].copy_from_slice(signature);
        assert!(AmigaArchive::open_hdf(Cursor::new(data)).is_err());
    }
    let data = common::image(1, 512, 16);
    for len in [0, 4, 511, 1024, data.len() - 1, 7 * 512] {
        assert!(AmigaArchive::open_hdf(Cursor::new(&data[..len])).is_err(), "{len}");
    }
}

#[test]
fn direct_limits_and_modified_public_metadata_cannot_bypass_checks() {
    let mut archive = AmigaArchive::open_hdf(Cursor::new(common::image(1, 512, 16))).unwrap();
    archive.get_next_entry().unwrap();
    let mut entry = archive.get_next_entry().unwrap().unwrap();
    assert!(matches!(archive.read_with_limit(&entry, Some(1)), Err(ArchiveError::SizeLimitExceeded { .. })));
    entry.size = 0;
    assert!(matches!(archive.read_with_limit(&entry, Some(1)), Err(ArchiveError::SizeLimitExceeded { .. })));
    assert_eq!(archive.read_with_limit(&entry, Some(common::PAYLOAD.len() as u64)).unwrap(), common::PAYLOAD);
    entry.name = "foreign".to_owned();
    assert!(matches!(archive.read(&entry), Err(ArchiveError::IndexMismatch(_))));
}

#[test]
fn random_mutations_never_panic() {
    let mut state = 0x1234_5678_u64;
    for mode in 0..=5 {
        let base = common::image(mode, 512, 16);
        for _ in 0..100 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let mut data = base.clone();
            let index = state as usize % data.len();
            data[index] ^= (state >> 32) as u8;
            if let Ok(mut archive) = AmigaArchive::open_hdf(Cursor::new(data)) {
                while let Ok(Some(entry)) = archive.get_next_entry() {
                    let _ = archive.read(&entry);
                }
            }
        }
    }
}
