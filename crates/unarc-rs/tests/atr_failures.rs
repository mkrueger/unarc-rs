//! Corrupt, unsupported and hostile ATR images.
use std::io::Cursor;

use unarc_rs::atr::AtrArchive;
use unarc_rs::error::ArchiveError;
use unarc_rs::unified::{ArchiveFormat, ArchiveOptions, UnifiedArchive};

#[path = "common/atr.rs"]
mod common;
use common::{chain, data, entry, file, header, name, vtoc, Disk, Sparta, MYDOS_DIRECTORY, SPARTA_DIRECTORY, SPARTA_FILE};

const DOS20S: &[u8] = include_bytes!("atr/dos20s.atr");

fn open(image: &[u8]) -> unarc_rs::Result<AtrArchive> {
    AtrArchive::new(Cursor::new(image))
}

/// Reads every entry; returns the first error.
fn read_all(image: &[u8]) -> unarc_rs::Result<()> {
    let mut archive = open(image)?;
    while let Some(entry) = archive.get_next_entry()? {
        archive.read(&entry)?;
    }
    Ok(())
}

fn corrupt_reason(result: unarc_rs::Result<()>) -> String {
    match result {
        Err(ArchiveError::CorruptedEntry { reason, .. }) => reason,
        other => panic!("expected a corrupt-entry error, got {other:?}"),
    }
}

/// A DOS 2 disk with a 1000-byte file in sectors 4-11, which the tests damage, and an
/// intact second file that identifies the disk as DOS 2.
fn dos2() -> Disk {
    let mut disk = Disk::new(128, 720);
    vtoc(&mut disk, 698);
    file(&mut disk, 361, 0, 4, "FILE.BIN", &data(1000, 0));
    file(&mut disk, 361, 1, 30, "OK.TXT", b"ok");
    disk
}

#[test]
fn headers_and_sector_sizes() {
    assert!(matches!(open(&[]), Err(ArchiveError::InvalidHeader { .. })));
    assert!(matches!(open(&DOS20S[..15]), Err(ArchiveError::InvalidHeader { .. })));
    let mut image = DOS20S.to_vec();
    image[0] = 0x97;
    assert!(matches!(open(&image), Err(ArchiveError::InvalidHeader { .. })));
    for size in [0u16, 512, 1024] {
        let mut image = DOS20S.to_vec();
        image[4..6].copy_from_slice(&size.to_le_bytes());
        assert!(matches!(open(&image), Err(ArchiveError::UnsupportedFormat(_))), "{size}");
    }
    // A header alone, and images too short for a directory, have no filesystem.
    for len in [16, 16 + 100, 16 + 360 * 128] {
        assert!(matches!(open(&DOS20S[..len]), Err(ArchiveError::UnsupportedFormat(_))), "{len}");
    }
}

#[test]
fn header_size_is_ignored() {
    // Many images record a wrong paragraph count; the file length decides.
    let mut image = DOS20S.to_vec();
    image[2..4].fill(0);
    image[6] = 0xFF;
    read_all(&image).unwrap();
}

#[test]
fn disks_without_a_filesystem() {
    // Boot-only disks: zeros, garbage, and a directory whose files have no valid chains.
    let blank = Disk::new(128, 720).atr();
    assert!(matches!(open(&blank), Err(ArchiveError::UnsupportedFormat(_))));
    let mut garbage = header(720 * 128, 128);
    garbage.extend(data(720 * 128, 0x5A));
    assert!(matches!(open(&garbage), Err(ArchiveError::UnsupportedFormat(_))));
    let mut disk = dos2();
    disk.sector(4)[125] = 5 << 2; // wrong file numbers
    disk.sector(30)[125] = 5 << 2;
    let mut image = disk.atr();
    assert!(matches!(open(&image), Err(ArchiveError::UnsupportedFormat(_))));
    // An empty directory without a DOS VTOC.
    image[16 + 359 * 128..16 + 368 * 128].fill(0);
    assert!(matches!(open(&image), Err(ArchiveError::UnsupportedFormat(_))));
    // Atari DOS 1.0 (VTOC code 1) stores sectors differently.
    let mut disk = dos2();
    disk.sector(360)[0] = 1;
    assert!(matches!(open(&disk.atr()), Err(ArchiveError::UnsupportedFormat(_))));
}

#[test]
fn broken_sector_chains() {
    // The size covers the readable part; reading reports the damage.
    let mut disk = dos2();
    disk.sector(5)[125..127].copy_from_slice(&[0x03, 0xFF]); // link to sector 1023
    let mut archive = open(&disk.atr()).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(entry.size, 250);
    assert!(corrupt_reason(archive.read(&entry).map(|_| ())).contains("beyond the end"));

    let mut disk = dos2();
    disk.sector(6)[126] = 5; // loops back to sector 5
    assert!(corrupt_reason(read_all(&disk.atr())).contains("loops"));

    let mut disk = dos2();
    disk.sector(6)[127] = 126; // more than 125 data bytes
    assert!(corrupt_reason(read_all(&disk.atr())).contains("126 data bytes"));

    let mut disk = dos2();
    disk.sector(7)[125] = 9 << 2; // file number mismatch (DOS error 164)
    assert!(corrupt_reason(read_all(&disk.atr())).contains("file number 9"));
    // An empty last sector is not checked; one with data is.
    let mut disk = dos2();
    disk.sector(30)[125..127].copy_from_slice(&[1 << 2, 31]);
    read_all(&disk.atr()).unwrap();
    disk.sector(31)[127] = 1;
    assert!(corrupt_reason(read_all(&disk.atr())).contains("file number 0"));

    // A file beyond a truncated image.
    let image = dos2().atr();
    let mut moved = Disk::new(128, 720);
    vtoc(&mut moved, 0);
    file(&mut moved, 361, 0, 4, "OK", b"ok");
    file(&mut moved, 361, 1, 700, "LOST", &data(1000, 1));
    let truncated = &moved.atr()[..16 + 699 * 128];
    let mut archive = open(truncated).unwrap();
    archive.get_next_entry().unwrap();
    let lost = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(lost.size, 0);
    assert!(archive.read(&lost).is_err());
    assert!(read_all(&image).is_ok());
}

#[test]
fn mydos_directory_loops_and_overlaps() {
    let mut disk = dos2();
    // A subdirectory that is the root, one inside the root's sectors, one past the end,
    // and two that overlap: none of them is read twice.
    entry(&mut disk, 361, 1, MYDOS_DIRECTORY, 8, 361, &name("SELF"));
    entry(&mut disk, 361, 2, MYDOS_DIRECTORY, 8, 365, &name("INSIDE"));
    entry(&mut disk, 361, 3, MYDOS_DIRECTORY, 8, 715, &name("PAST"));
    entry(&mut disk, 361, 4, MYDOS_DIRECTORY, 8, 400, &name("A"));
    entry(&mut disk, 361, 5, MYDOS_DIRECTORY, 8, 404, &name("B"));
    entry(&mut disk, 400, 0, MYDOS_DIRECTORY, 8, 361, &name("UP"));
    file(&mut disk, 404, 0, 500, "IN.B", b"b");
    let mut archive = open(&disk.atr()).unwrap();
    let mut names = Vec::new();
    while let Some(entry) = archive.get_next_entry().unwrap() {
        names.push(entry.name);
    }
    assert_eq!(names, ["FILE.BIN", "SELF/", "INSIDE/", "PAST/", "A/", "B/", "A/UP/"]);
}

#[test]
fn shared_chains_exhaust_the_read_budget() {
    // 64 entries that all share one 700-sector chain: listing stops instead of reading
    // the chain 64 times.
    let mut disk = Disk::new(128, 720);
    vtoc(&mut disk, 0);
    let sectors: Vec<usize> = (4..360).chain(369..713).collect();
    chain(&mut disk, &sectors, 0, &data(700 * 125, 2), false);
    for index in 0..64 {
        entry(&mut disk, 361, index, 0x42, 700, 4, &name(&format!("F{index}")));
    }
    assert!(corrupt_reason(open(&disk.atr()).map(|_| ())).contains("too many sector reads"));
}

#[test]
fn spartados_corruption() {
    let build = |size: u32, self_link: bool| {
        let mut sparta = Sparta::new(128, 720);
        let file = sparta.write(&data(1000, 3));
        if self_link {
            sparta.disk.sector(usize::from(file))[..2].copy_from_slice(&file.to_le_bytes());
        }
        let root = sparta.directory("MAIN", &[(SPARTA_FILE, file, size, "FILE.BIN", [0; 6])]);
        sparta.set_root(root);
        sparta.disk.atr()
    };
    read_all(&build(1000, false)).unwrap();
    // Size past the end of the sector map, past the disk, and a map that links to itself.
    assert!(corrupt_reason(read_all(&build(20_000, false))).contains("sector map ends"));
    assert!(corrupt_reason(read_all(&build(0xFF_FFFF, false))).contains("exceeds the disk"));
    assert!(corrupt_reason(read_all(&build(20_000, true))).contains("loops"));

    // The root directory is written to sector 4, its map to sector 5.
    let mut sparta = Sparta::new(128, 720);
    let root = sparta.directory("MAIN", &[(SPARTA_DIRECTORY, 5, 0, "LOOP", [0; 6])]);
    assert_eq!(root, 5);
    sparta.set_root(root);
    // A subdirectory that is the root itself is listed but not read again.
    let mut archive = open(&sparta.disk.atr()).unwrap();
    assert_eq!(archive.get_next_entry().unwrap().unwrap().name, "LOOP/");
    assert!(archive.get_next_entry().unwrap().is_none());
    // An impossible directory length.
    sparta.disk.sector(4)[3..6].copy_from_slice(&[0xFF, 0xFF, 0xFF]);
    assert!(matches!(open(&sparta.disk.atr()), Err(ArchiveError::CorruptedEntry { .. })));
    // A root map outside the image is not SpartaDOS.
    sparta.disk.sector(1)[9..11].copy_from_slice(&800u16.to_le_bytes());
    assert!(matches!(open(&sparta.disk.atr()), Err(ArchiveError::UnsupportedFormat(_))));
}

#[test]
fn limits_and_foreign_entries() {
    let image = dos2().atr();
    let mut archive = open(&image).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert!(matches!(
        archive.read_with_limit(&entry, Some(999)),
        Err(ArchiveError::SizeLimitExceeded { .. })
    ));
    assert_eq!(archive.read_with_limit(&entry, Some(1000)).unwrap().len(), 1000);

    let mut forged = entry.clone();
    forged.name = "OTHER".into();
    assert!(matches!(archive.read(&forged), Err(ArchiveError::IndexMismatch(_))));
    let mut other = open(DOS20S).unwrap();
    let foreign = other.get_next_entry().unwrap().unwrap();
    assert!(matches!(archive.read(&foreign), Err(ArchiveError::IndexMismatch(_))));

    let options = ArchiveOptions::new().with_max_entry_size(Some(10));
    let mut unified = UnifiedArchive::open_with_format_and_options(Cursor::new(&image), ArchiveFormat::Atr, options).unwrap();
    let entry = unified.next_entry().unwrap().unwrap();
    assert!(matches!(unified.read(&entry), Err(ArchiveError::SizeLimitExceeded { .. })));
    let options = ArchiveOptions::new().with_max_total_size(Some(1500));
    let mut disk = dos2();
    file(&mut disk, 361, 2, 20, "SECOND.BIN", &data(1000, 1));
    let image = disk.atr();
    let mut unified = UnifiedArchive::open_with_format_and_options(Cursor::new(&image), ArchiveFormat::Atr, options).unwrap();
    for _ in 0..2 {
        let entry = unified.next_entry().unwrap().unwrap();
        unified.read(&entry).unwrap();
    }
    let second = unified.next_entry().unwrap().unwrap();
    assert!(matches!(unified.read(&second), Err(ArchiveError::SizeLimitExceeded { .. })));
}

#[test]
fn truncated_and_mutated_images_do_not_panic() {
    let base = DOS20S.to_vec();
    for len in (0..base.len()).step_by(997) {
        let _ = read_all(&base[..len]);
    }
    let mut state = 0x1234_5678u32;
    for _ in 0..300 {
        let mut image = base.clone();
        for _ in 0..40 {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            // Mutate the directory, VTOC and the first file sectors.
            let offset = 16 + [359 * 128, 3 * 128][state as usize % 2] + (state as usize >> 8) % (9 * 128);
            image[offset] = (state >> 3) as u8;
        }
        let _ = read_all(&image);
    }
}
