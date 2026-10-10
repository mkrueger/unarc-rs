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

#[test]
fn track_header_damage_and_truncation_are_rejected() {
    let pos = common::track(common::MODES, 40);
    let mut data = common::MODES.to_vec();
    data[pos + 11] ^= 1;
    assert!(matches!(open(&data), Err(ArchiveError::CrcMismatch { .. })));

    let mut data = common::MODES.to_vec();
    common::put(&mut data, pos + 10, 32_001);
    let value = common::crc(&data[pos..pos + 18]);
    common::put(&mut data, pos + 18, value);
    assert!(matches!(open(&data), Err(ArchiveError::CorruptedEntry { .. })));

    for cut in [pos + 10, pos + 30] {
        assert!(matches!(open(&common::MODES[..cut]), Err(ArchiveError::CorruptedEntry { .. })), "cut={cut}");
    }
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
