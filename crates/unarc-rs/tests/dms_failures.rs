use std::io::Cursor;

use unarc_rs::dms::{DmsArchive, DmsEntryKind};
use unarc_rs::error::ArchiveError;

#[path = "common/dms.rs"]
mod common;

fn open(data: &[u8]) -> unarc_rs::Result<DmsArchive> {
    DmsArchive::open(Cursor::new(data), None)
}

/// Reads every entry; returns the image/filesystem result and whether the attached texts were readable.
fn read_all(archive: &mut DmsArchive) -> (Vec<unarc_rs::Result<Vec<u8>>>, bool) {
    let mut disk = Vec::new();
    let mut texts = true;
    while let Some(entry) = archive.get_next_entry().unwrap() {
        let result = archive.read(&entry);
        match entry.kind {
            DmsEntryKind::FileId | DmsEntryKind::Banner => texts &= result.is_ok(),
            _ => disk.push(result),
        }
    }
    (disk, texts)
}

#[test]
fn header_damage_is_rejected() {
    assert!(matches!(open(b"DMS!"), Err(ArchiveError::InvalidHeader { .. })));
    assert!(matches!(open(&common::NDOS[..55]), Err(ArchiveError::InvalidHeader { .. })));
    let mut data = common::NDOS.to_vec();
    data[0] = b'X';
    assert!(matches!(open(&data), Err(ArchiveError::InvalidHeader { .. })));
    let mut data = common::NDOS.to_vec();
    data[20] ^= 1;
    assert!(matches!(open(&data), Err(ArchiveError::CrcMismatch { .. })));
    // Disk type 7 is an FMS file archive, not a disk.
    let mut data = common::NDOS.to_vec();
    common::put(&mut data, 50, 7);
    common::fix_header_crc(&mut data);
    assert!(matches!(open(&data), Err(ArchiveError::UnsupportedFormat(_))));
}

/// Asserts that the archive opens, the disk image (not its files) is listed and
/// fails to read with a named error, and returns the readable attached texts.
fn damaged_disk(data: &[u8]) -> Vec<String> {
    let mut archive = open(data).unwrap();
    assert!(archive.volume().is_none());
    let mut texts = Vec::new();
    while let Some(entry) = archive.get_next_entry().unwrap() {
        match (entry.kind, archive.read(&entry)) {
            (DmsEntryKind::DiskImage, Err(ArchiveError::CorruptedEntry { entry_name, .. })) => assert_eq!(entry_name, entry.name),
            (DmsEntryKind::DiskImage, result) => panic!("image read: {result:?}"),
            (_, Ok(_)) => texts.push(entry.name),
            (_, Err(error)) => panic!("{}: {error}", entry.name),
        }
    }
    texts
}

#[test]
fn damaged_track_headers_keep_the_rest_of_the_archive() {
    let all_texts = ["FILE_ID.DIZ", "BANNER.TXT", "BANNER2.TXT"];
    let pos = common::track(common::MODES, 40);
    // Header CRC mismatch: parsing resumes at the next track header.
    let mut data = common::MODES.to_vec();
    data[pos + 11] ^= 1;
    assert_eq!(damaged_disk(&data), all_texts);
    // A damaged track magic is not taken as the end of the archive.
    let mut data = common::MODES.to_vec();
    data[common::track(common::MODES, 60)] = b'X';
    assert_eq!(damaged_disk(&data), all_texts);
    // An oversized track.
    let mut data = common::MODES.to_vec();
    common::put(&mut data, pos + 10, 32_001);
    let value = common::crc(&data[pos..pos + 18]);
    common::put(&mut data, pos + 18, value);
    assert_eq!(damaged_disk(&data), all_texts);
}

#[test]
fn truncated_archives_keep_what_was_decoded() {
    // Cut inside the last banner: the disk is complete, only that banner fails.
    let data = &common::MODES[..common::MODES.len() - 3];
    let mut archive = open(data).unwrap();
    assert!(archive.volume().is_some());
    let (disk, _) = read_all(&mut archive);
    assert!(disk.iter().all(Result::is_ok));
    let mut archive = open(data).unwrap();
    let banner = std::iter::from_fn(|| archive.get_next_entry().unwrap())
        .find(|e| e.name == "BANNER2.TXT")
        .unwrap();
    assert!(matches!(archive.read(&banner), Err(ArchiveError::CorruptedEntry { entry_name, .. }) if entry_name == "BANNER2.TXT"));

    // Cut inside a disk track or its header: the image fails, the banner before it does not.
    let pos = common::track(common::MODES, 40);
    for cut in [pos + 10, pos + 30] {
        assert_eq!(damaged_disk(&common::MODES[..cut]), ["BANNER.TXT"], "cut={cut}");
    }
}

#[test]
fn attached_texts_are_bounded_and_never_padded() {
    // 50 banners claiming 32000 bytes each, backed by no data.
    let mut data = common::NDOS.to_vec();
    for _ in 0..50 {
        data.extend_from_slice(&common::track_record(0xffff, 0, 32_000, 0, &[]));
    }
    let mut archive = open(&data).unwrap();
    let entries: Vec<_> = std::iter::from_fn(|| archive.get_next_entry().unwrap()).collect();
    let banners: Vec<_> = entries.iter().filter(|e| e.kind == DmsEntryKind::Banner).collect();
    assert!(banners.len() <= 16 && banners.iter().map(|e| e.size).sum::<u64>() <= 256 * 1024);
    for banner in banners {
        assert!(matches!(archive.read(banner), Err(ArchiveError::DecompressionFailed { .. })));
    }

    // A short NOCOMP disk track is an error, not zero-filled.
    let pos = common::track(common::NDOS, 79);
    let mut data = common::NDOS[..pos].to_vec();
    data.extend_from_slice(&common::track_record(79, 0, 11 * 1024, 0, &[]));
    data.extend_from_slice(&common::NDOS[common::track(common::NDOS, 80)..]);
    let (disk, texts) = read_all(&mut open(&data).unwrap());
    assert!(texts);
    assert!(matches!(disk[..], [Err(ArchiveError::DecompressionFailed { .. })]));
}

#[test]
fn trailing_data_after_the_last_track_is_ignored() {
    let expected = read_all(&mut open(common::MODES).unwrap()).0.len();
    for trailer in [&b"\0"[..], b"Uploaded to a BBS\n", &[0u8; 64]] {
        let mut data = common::MODES.to_vec();
        data.extend_from_slice(trailer);
        let (disk, texts) = read_all(&mut open(&data).unwrap());
        assert_eq!(disk.len(), expected);
        assert!(disk.iter().all(Result::is_ok) && texts);
    }
}

#[test]
fn damaged_disk_track_keeps_attached_texts_readable() {
    let pos = common::track(common::MODES, 42);
    let mut data = common::MODES.to_vec();
    data[pos + 25] ^= 0x40;
    let mut archive = open(&data).unwrap();
    assert!(archive.volume().is_none());
    assert!(archive.image().is_empty());
    let (disk, texts) = read_all(&mut archive);
    assert!(texts);
    assert!(matches!(disk[..], [Err(ArchiveError::CrcMismatch { .. })]));

    // With matching CRCs, the damage surfaces as a checksum or decoding error.
    for mode in 0..=6u16 {
        let number = 40 + mode;
        let pos = common::track(common::MODES, number);
        let mut data = common::MODES.to_vec();
        let len = usize::from(common::word(&data, pos + 6));
        data[pos + 20 + len / 2] ^= 0x5a;
        common::fix_track_crc(&mut data, pos);
        let (disk, texts) = read_all(&mut open(&data).unwrap());
        assert!(texts);
        assert!(
            matches!(disk[..], [Err(ArchiveError::CorruptedEntry { .. } | ArchiveError::DecompressionFailed { .. })]),
            "mode={mode}: {:?}",
            disk[0]
        );
    }
}

#[test]
fn encrypted_banner_crc_error_keeps_disk_readable() {
    let expected = DmsArchive::open(Cursor::new(common::ENCRYPTED), Some("unarc")).unwrap();
    let mut data = common::ENCRYPTED.to_vec();
    let pos = common::track(&data, 0xffff);
    data[pos + 16] ^= 1;
    let header_crc = common::crc(&data[pos..pos + 18]);
    common::put(&mut data, pos + 18, header_crc);

    let mut archive = DmsArchive::open(Cursor::new(data), Some("unarc")).unwrap();
    assert!(archive.image() == expected.image(), "a bad banner CRC changed the decoded disk");
    assert!(archive.volume().is_some());
    while let Some(entry) = archive.get_next_entry().unwrap() {
        if entry.kind == DmsEntryKind::Banner {
            assert!(matches!(
                archive.read(&entry),
                Err(ArchiveError::CrcMismatch { entry, .. }) if entry == "BANNER.TXT (track 65535)"
            ));
        } else {
            assert!(archive.read(&entry).is_ok(), "{}", entry.name);
        }
    }
}

#[test]
fn heavy_tracks_cannot_reuse_unsent_tables() {
    for mode in [5, 6] {
        let mut data = common::NDOS[..56].to_vec();
        common::put(&mut data, 10, 0);
        common::put(&mut data, 16, 0);
        common::put(&mut data, 18, 0);
        common::fix_header_crc(&mut data);
        let mut record = common::track_record(0, mode, common::TRACK as u16, 0, &[]);
        common::put(&mut record, 8, common::TRACK as u16);
        common::fix_track_crc(&mut record, 0);
        data.extend_from_slice(&record);

        let mut archive = open(&data).unwrap();
        assert!(archive.image().is_empty());
        let entry = archive.get_next_entry().unwrap().unwrap();
        assert!(matches!(
            archive.read(&entry),
            Err(ArchiveError::DecompressionFailed { reason, .. }) if reason.contains("tables that were never sent")
        ));
    }
}

#[test]
fn unknown_mode_and_too_many_tracks() {
    let pos = common::track(common::NDOS, 3);
    let mut data = common::NDOS.to_vec();
    data[pos + 13] = 7;
    common::fix_track_crc(&mut data, pos);
    let (disk, texts) = read_all(&mut open(&data).unwrap());
    assert!(texts);
    assert!(matches!(disk[..], [Err(ArchiveError::UnsupportedMethod { .. })]));

    // Track 3 stored a second time: 81 disk tracks.
    let end = common::track(common::NDOS, 80);
    let next = common::track(common::NDOS, 4);
    let mut data = common::NDOS[..end].to_vec();
    data.extend_from_slice(&common::NDOS[pos..next]);
    data.extend_from_slice(&common::NDOS[end..]);
    let (disk, _) = read_all(&mut open(&data).unwrap());
    assert!(matches!(disk[..], [Err(ArchiveError::CorruptedEntry { .. })]));
}

#[test]
fn a_flood_of_disk_tracks_stays_bounded() {
    // 2000 disk track headers claiming 2049 bytes each, backed by no data: neither the
    // listed image size nor the reader's bookkeeping may grow with them.
    let mut data = common::NDOS[..common::track(common::NDOS, 80)].to_vec();
    for _ in 0..2000 {
        data.extend_from_slice(&common::track_record(0, 0, 2049, 0, &[]));
    }
    let mut archive = open(&data).unwrap();
    let entries: Vec<_> = std::iter::from_fn(|| archive.get_next_entry().unwrap()).collect();
    let image = entries.iter().find(|e| e.kind == DmsEntryKind::DiskImage).expect("the disk as an image");
    assert!(image.size <= 81 * 32_000, "listed {} bytes", image.size);
    assert!(matches!(archive.read(image), Err(ArchiveError::CorruptedEntry { .. })));
}

#[test]
fn corrupted_compressed_data_never_panics() {
    // Damage packed data and recompute the CRCs, so the decoders see the garbage.
    for fixture in common::FIXTURES {
        let tracks = common::tracks(fixture);
        for (i, &pos) in tracks.iter().enumerate() {
            let len = usize::from(common::word(fixture, pos + 6));
            for (step, offset) in [0, len / 3, len / 2, len.saturating_sub(1)].into_iter().enumerate() {
                if offset >= len {
                    continue;
                }
                let mut data = fixture.to_vec();
                data[pos + 20 + offset] ^= [0xff, 0x01, 0x80, 0x5a][step];
                common::fix_track_crc(&mut data, pos);
                if let Ok(mut archive) = DmsArchive::open(Cursor::new(&data), Some("unarc")) {
                    let _ = read_all(&mut archive);
                }
                // Header fields that steer the decoders.
                let mut data = fixture.to_vec();
                data[pos + 12] = [0xff, 0x02, 0x05, 0x00][step];
                data[pos + 13] = [6, 5, 4, 2][(step + i) % 4];
                common::put(&mut data, pos + 8, [0, 1, 32_000, 11_264][step]);
                common::fix_track_crc(&mut data, pos);
                if let Ok(mut archive) = DmsArchive::open(Cursor::new(&data), Some("unarc")) {
                    let _ = read_all(&mut archive);
                }
            }
            if i > 20 {
                break;
            }
        }
    }
}
