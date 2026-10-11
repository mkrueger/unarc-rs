use std::io::Cursor;
use std::path::Path;

use unarc_rs::amiga::AmigaEntryKind;
use unarc_rs::dms::{DmsArchive, DmsEntry, DmsEntryKind};
use unarc_rs::error::ArchiveError;
use unarc_rs::unified::{ArchiveEntryKind, ArchiveFormat, ArchiveOptions};
use unarc_rs::EncryptionMethod;

#[path = "common/dms.rs"]
mod common;

fn entries(archive: &mut DmsArchive) -> Vec<DmsEntry> {
    std::iter::from_fn(|| archive.get_next_entry().unwrap()).collect()
}

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/dms").join(name)
}

#[test]
fn every_mode_decodes_to_the_original_disk() {
    // The disk tracks use NOCOMP, SIMPLE, QUICK, MEDIUM, DEEP, HEAVY1 and HEAVY2, with
    // and without carrying the decoder state from track to track.
    let archive = DmsArchive::open(Cursor::new(common::MODES), None).unwrap();
    assert_eq!(archive.image().len(), 80 * common::TRACK);
    assert_eq!(crc32fast::hash(archive.image()), 0xe71d_6605);
    let info = archive.info();
    assert_eq!(
        (info.creator_version, info.mode_name(), info.first_track, info.last_track),
        (111, "HEAVY2", 0, 79)
    );
    assert!(!info.is_encrypted());
    let volume = archive.volume().unwrap();
    assert_eq!((volume.name.as_str(), volume.filesystem()), ("DMSTest", "OFS"));
}

#[test]
fn lists_disk_files_and_attached_texts() {
    let mut archive = DmsArchive::open(Cursor::new(common::MODES), None).unwrap();
    let listed = entries(&mut archive);
    let names: Vec<_> = listed.iter().map(|e| (e.name.as_str(), e.kind)).collect();
    assert_eq!(
        names,
        [
            ("LICENSE", DmsEntryKind::Amiga(AmigaEntryKind::File)),
            ("FILE_ID.DIZ", DmsEntryKind::Amiga(AmigaEntryKind::File)),
            ("docs/", DmsEntryKind::Amiga(AmigaEntryKind::Directory)),
            ("docs/lines.txt", DmsEntryKind::Amiga(AmigaEntryKind::File)),
            // The disk already has a FILE_ID.DIZ, so the attached one is renamed.
            ("FILE_ID2.DIZ", DmsEntryKind::FileId),
            ("BANNER.TXT", DmsEntryKind::Banner),
            ("BANNER2.TXT", DmsEntryKind::Banner),
        ]
    );
    let expected: [&[u8]; 7] = [
        common::LICENSE,
        common::DISK_DIZ,
        b"",
        &common::lines(),
        common::ATTACHED_DIZ,
        common::BANNER,
        common::BANNER2,
    ];
    for (entry, expected) in listed.iter().zip(expected) {
        assert_eq!(archive.read(entry).unwrap(), expected, "{}", entry.name);
        assert_eq!(entry.size, expected.len() as u64, "{}", entry.name);
        assert!(!entry.encrypted);
    }
    let time = listed[0].modified_time().unwrap();
    assert_eq!(
        (time.year(), time.month(), time.day(), time.hour(), time.minute(), time.second()),
        (2024, 5, 16, 23, 8, 26)
    );
}

#[test]
fn unified_api_lists_and_reads_dms() {
    let mut archive = ArchiveFormat::open_path(fixture("modes.dms")).unwrap();
    assert_eq!(archive.format(), ArchiveFormat::Dms);
    let mut seen = Vec::new();
    while let Some(entry) = archive.next_entry().unwrap() {
        assert_eq!(entry.compression_method(), "DMS HEAVY2");
        assert_eq!(entry.encryption(), EncryptionMethod::None);
        let data = archive.read(&entry).unwrap();
        assert_eq!(data.len() as u64, entry.original_size());
        seen.push((entry.name().to_string(), entry.kind()));
    }
    assert!(seen.contains(&("docs/".to_string(), ArchiveEntryKind::Directory)));
    assert!(seen.contains(&("FILE_ID2.DIZ".to_string(), ArchiveEntryKind::File)));
    assert_eq!(seen.len(), 7);
}

#[test]
fn high_density_disk() {
    let mut archive = DmsArchive::open(Cursor::new(common::HD), None).unwrap();
    assert_eq!(archive.image().len(), 80 * 2 * common::TRACK);
    assert_eq!(crc32fast::hash(archive.image()), 0x6e66_1812);
    assert_eq!(archive.volume().unwrap().filesystem(), "FFS");
    let listed = entries(&mut archive);
    assert_eq!(listed.iter().map(|e| e.name.as_str()).collect::<Vec<_>>(), ["LICENSE", "FILE_ID.DIZ"]);
    assert_eq!(archive.read(&listed[0]).unwrap(), common::LICENSE);
    assert_eq!(archive.read(&listed[1]).unwrap(), common::DISK_DIZ);
}

#[test]
fn non_dos_disk_is_one_image_named_after_the_archive() {
    let mut archive = ArchiveFormat::open_path(fixture("ndos.dms")).unwrap();
    let image = archive.next_entry().unwrap().unwrap();
    assert_eq!(image.name(), "ndos.adf");
    assert_eq!(image.original_size(), 80 * common::TRACK as u64);
    let data = archive.read(&image).unwrap();
    assert_eq!(&data[..4], b"NDOS");
    assert_eq!(crc32fast::hash(&data), 0x9a59_c086);
    let diz = archive.next_entry().unwrap().unwrap();
    assert_eq!(diz.name(), "FILE_ID.DIZ");
    assert_eq!(archive.read(&diz).unwrap(), common::ATTACHED_DIZ);
    assert!(archive.next_entry().unwrap().is_none());

    // Without a path, the image has a generic name.
    let mut archive = DmsArchive::open(Cursor::new(common::NDOS), None).unwrap();
    assert_eq!(archive.get_next_entry().unwrap().unwrap().name, "disk.adf");
    assert!(archive.volume().is_none());
}

#[test]
fn partial_disk_concatenates_the_stored_tracks() {
    // Tracks 0, 1 and 40 are stored; like xdms, the image is not padded to a full disk.
    let mut archive = DmsArchive::open(Cursor::new(common::PARTIAL), None).unwrap();
    let listed = entries(&mut archive);
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].kind, DmsEntryKind::DiskImage);
    // Not a disk image, so not named `.adf`.
    assert_eq!(listed[0].name, "disk.tracks");
    let data = archive.read(&listed[0]).unwrap();
    assert_eq!(data.len(), 3 * common::TRACK);
    assert_eq!(crc32fast::hash(&data), 0x9319_c424);
    assert_eq!(&data[..4], b"DOS\0");
    let mut archive = ArchiveFormat::open_path(fixture("partial.dms")).unwrap();
    assert_eq!(archive.next_entry().unwrap().unwrap().name(), "partial.tracks");
}

#[test]
fn encrypted_archive_with_and_without_password() {
    // Without a password only the never-encrypted FILE_ID.DIZ can be read.
    let mut archive = ArchiveFormat::Dms.open(Cursor::new(common::ENCRYPTED)).unwrap();
    let mut locked = 0;
    while let Some(entry) = archive.next_entry().unwrap() {
        if entry.name() == "FILE_ID.DIZ" {
            assert!(!entry.is_encrypted());
            assert_eq!(archive.read(&entry).unwrap(), common::ATTACHED_DIZ);
        } else {
            assert_eq!(entry.encryption(), EncryptionMethod::Dms);
            assert!(matches!(archive.read(&entry), Err(ArchiveError::EncryptionRequired { .. })));
            locked += 1;
        }
    }
    assert_eq!(locked, 2);

    let options = ArchiveOptions::new().with_password("unarc");
    let mut archive = ArchiveFormat::Dms.open_with_options(Cursor::new(common::ENCRYPTED), options).unwrap();
    let mut names = Vec::new();
    while let Some(entry) = archive.next_entry().unwrap() {
        let data = archive.read(&entry).unwrap();
        match entry.name() {
            "LICENSE" => assert_eq!(data, common::LICENSE),
            "BANNER.TXT" => assert_eq!(data, common::BANNER),
            _ => {}
        }
        assert_eq!(entry.is_encrypted(), entry.name() != "FILE_ID2.DIZ");
        names.push(entry.name().to_string());
    }
    assert_eq!(names, ["LICENSE", "FILE_ID.DIZ", "docs/", "docs/lines.txt", "FILE_ID2.DIZ", "BANNER.TXT"]);
}

#[test]
fn wrong_password_only_garbles_the_start_of_the_key_stream() {
    // The key is updated from the encrypted bytes, so it resynchronises after a few
    // bytes: the leading banner fails, the disk behind it decodes (as with xdms).
    let mut archive = DmsArchive::open(Cursor::new(common::ENCRYPTED), Some("wrong")).unwrap();
    let listed = entries(&mut archive);
    let banner = listed.iter().find(|e| e.kind == DmsEntryKind::Banner).unwrap();
    assert!(matches!(archive.read(banner), Err(ArchiveError::InvalidPassword { .. })));
    let license = listed.iter().find(|e| e.name == "LICENSE").unwrap();
    assert_eq!(archive.read(license).unwrap(), common::LICENSE);
}

#[test]
fn detection_and_limits() {
    assert_eq!(ArchiveFormat::detect_from_bytes(common::MODES), Some(ArchiveFormat::Dms));
    assert_eq!(ArchiveFormat::from_path(Path::new("GAME.DMS")), Some(ArchiveFormat::Dms));
    assert_eq!(ArchiveFormat::detect(&mut Cursor::new(common::NDOS), None).unwrap(), Some(ArchiveFormat::Dms));
    assert_eq!(ArchiveFormat::Dms.preambles(), Some(&[b"DMS!".as_slice()][..]));

    let options = ArchiveOptions::new().with_max_entry_size(Some(1000));
    let mut archive = ArchiveFormat::Dms.open_with_options(Cursor::new(common::MODES), options).unwrap();
    let license = archive.next_entry().unwrap().unwrap();
    assert!(matches!(archive.read(&license), Err(ArchiveError::SizeLimitExceeded { .. })));

    let options = ArchiveOptions::new().with_max_total_size(Some(100_000));
    let mut archive = ArchiveFormat::Dms.open_with_options(Cursor::new(common::NDOS), options).unwrap();
    let image = archive.next_entry().unwrap().unwrap();
    assert!(matches!(archive.read(&image), Err(ArchiveError::SizeLimitExceeded { .. })));
    let diz = archive.next_entry().unwrap().unwrap();
    assert_eq!(archive.read(&diz).unwrap(), common::ATTACHED_DIZ);
}

#[test]
fn password_must_be_given_when_opening() {
    let mut archive = ArchiveFormat::Dms.open(Cursor::new(common::ENCRYPTED)).unwrap();
    let image = archive.next_entry().unwrap().unwrap();
    let options = ArchiveOptions::new().with_password("unarc");
    assert!(matches!(
        archive.read_with_options(&image, &options),
        Err(ArchiveError::UnsupportedMethod { .. })
    ));
    assert!(matches!(archive.read(&image), Err(ArchiveError::EncryptionRequired { .. })));
}

#[test]
fn unified_skip_and_iteration() {
    let options = ArchiveOptions::new();
    let mut archive = ArchiveFormat::open_path(fixture("modes.dms")).unwrap();
    let mut read = 0;
    while let Some(entry) = archive.next_entry().unwrap() {
        if entry.name() == "docs/lines.txt" || entry.name() == "BANNER2.TXT" {
            assert_eq!(archive.read(&entry).unwrap().len() as u64, entry.original_size());
            read += 1;
        } else if entry.name().ends_with(".DIZ") {
            archive.skip_with_options(&entry, &options).unwrap();
        } else {
            archive.skip(&entry).unwrap();
        }
    }
    assert_eq!(read, 2);

    let mut archive = ArchiveFormat::open_path(fixture("ndos.dms")).unwrap();
    let names: Vec<_> = archive.entries_iter().map(|entry| entry.unwrap().name().to_string()).collect();
    assert_eq!(names, ["ndos.adf", "FILE_ID.DIZ"]);
    let mut archive = ArchiveFormat::open_path(fixture("ndos.dms")).unwrap();
    assert_eq!(archive.entries().unwrap().len(), 2);
}
