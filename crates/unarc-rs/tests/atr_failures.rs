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

/// Writes a SpartaDOS sector map at `map` that lists `sectors`.
fn sparta_map(sparta: &mut Sparta, map: usize, sectors: &[usize]) {
    let raw = sparta.disk.sector(map);
    raw[..4].fill(0);
    for (slot, &sector) in sectors.iter().enumerate() {
        raw[4 + 2 * slot..6 + 2 * slot].copy_from_slice(&(sector as u16).to_le_bytes());
    }
}

#[test]
fn spartados_directories_sharing_sectors() {
    // One directory of 100 subdirectory entries, reached through 100 different sector
    // maps that all list the same data sectors. Read through every map, it would list
    // 100 x 100 entries; each directory's sectors may be read for one directory only.
    const DIRECTORIES: usize = 100;
    let mut sparta = Sparta::new(128, 720);
    let maps: Vec<usize> = (0..DIRECTORIES).map(|_| sparta.allocate()).collect();
    let mut directory = vec![0; 23];
    directory[0] = SPARTA_DIRECTORY;
    directory[3..6].copy_from_slice(&(((DIRECTORIES + 1) * 23) as u32).to_le_bytes()[..3]);
    for (i, &map) in maps.iter().enumerate() {
        directory.push(SPARTA_DIRECTORY);
        directory.extend((map as u16).to_le_bytes());
        directory.extend([0; 3]);
        directory.extend(name(&format!("D{i}")));
        directory.extend([0; 6]);
    }
    let sectors: Vec<usize> = directory
        .chunks(128)
        .map(|chunk| {
            let sector = sparta.allocate();
            sparta.disk.sector(sector)[..chunk.len()].copy_from_slice(chunk);
            sector
        })
        .collect();
    for &map in &maps {
        sparta_map(&mut sparta, map, &sectors);
    }
    sparta.set_root(maps[0] as u16);
    let mut archive = open(&sparta.disk.atr()).unwrap();
    let mut entries = Vec::new();
    while let Some(entry) = archive.get_next_entry().unwrap() {
        assert!(entry.is_directory && !entry.name[..entry.name.len() - 1].contains('/'), "{}", entry.name);
        entries.push(entry);
    }
    assert_eq!(entries.len(), DIRECTORIES);

    // A subdirectory that reuses the root's data sector, and one that lists a sector twice.
    let mut sparta = Sparta::new(128, 720);
    let file = sparta.write(b"x");
    let (root_map, root_data, shared, twice, twice_data) = (sparta.allocate(), sparta.allocate(), sparta.allocate(), sparta.allocate(), sparta.allocate());
    let entry = |status: u8, map: usize, size: u8, entry_name: &str| {
        let mut raw = vec![status];
        raw.extend((map as u16).to_le_bytes());
        raw.extend([size, 0, 0]);
        raw.extend(name(entry_name));
        raw.extend([0; 6]);
        raw
    };
    let mut root = entry(SPARTA_DIRECTORY, 0, 69, "MAIN");
    root.extend(entry(SPARTA_DIRECTORY, shared, 0, "SHARED"));
    root.extend(entry(SPARTA_DIRECTORY, twice, 0, "TWICE"));
    sparta.disk.sector(root_data)[..root.len()].copy_from_slice(&root);
    // 150 bytes take two sectors.
    let mut directory = entry(SPARTA_DIRECTORY, 0, 150, "TWICE");
    directory.extend(entry(SPARTA_FILE, usize::from(file), 1, "X"));
    sparta.disk.sector(twice_data)[..directory.len()].copy_from_slice(&directory);
    sparta_map(&mut sparta, root_map, &[root_data]);
    sparta_map(&mut sparta, shared, &[root_data]);
    sparta_map(&mut sparta, twice, &[twice_data, twice_data]);
    sparta.set_root(root_map as u16);
    let mut archive = open(&sparta.disk.atr()).unwrap();
    let mut names = Vec::new();
    while let Some(entry) = archive.get_next_entry().unwrap() {
        names.push(entry.name);
    }
    assert_eq!(names, ["SHARED/", "TWICE/"]);
    // Listed once, TWICE's directory is read.
    sparta_map(&mut sparta, twice, &[twice_data]);
    sparta.disk.sector(twice_data)[3] = 46;
    let mut archive = open(&sparta.disk.atr()).unwrap();
    let mut names = Vec::new();
    while let Some(entry) = archive.get_next_entry().unwrap() {
        names.push(entry.name);
    }
    assert_eq!(names, ["SHARED/", "TWICE/", "TWICE/X"]);
}

#[test]
fn spartados_failed_directory_read_releases_sectors() {
    let mut sparta = Sparta::new(256, 720);
    let file = sparta.write(b"x");
    let shared = sparta.allocate();
    sparta.disk.sector(shared)[0] = SPARTA_DIRECTORY;
    sparta.disk.sector(shared)[3..6].copy_from_slice(&300u32.to_le_bytes()[..3]);
    let bad = sparta.allocate();
    // A boot sector holds only 128 bytes and cannot be directory data on this disk.
    sparta_map(&mut sparta, bad, &[shared, 1]);

    let good = sparta.directory("GOOD", &[(SPARTA_FILE, file, 1, "X", [0; 6])]);
    let map = sparta.disk.sector(usize::from(good));
    let good_data = usize::from(u16::from_le_bytes([map[4], map[5]]));
    sparta.disk.sector(good_data)[3..6].copy_from_slice(&300u32.to_le_bytes()[..3]);
    sparta_map(&mut sparta, usize::from(good), &[good_data, shared]);
    let root = sparta.directory(
        "MAIN",
        &[(SPARTA_DIRECTORY, bad as u16, 0, "BAD", [0; 6]), (SPARTA_DIRECTORY, good, 0, "GOOD", [0; 6])],
    );
    sparta.set_root(root);

    let mut archive = open(&sparta.disk.atr()).unwrap();
    let entries: Vec<_> = std::iter::from_fn(|| archive.get_next_entry().unwrap()).collect();
    assert_eq!(entries.iter().map(|entry| entry.name.as_str()).collect::<Vec<_>>(), ["BAD/", "GOOD/", "GOOD/X"]);
    assert_eq!(archive.read(&entries[2]).unwrap(), b"x");
}

#[test]
fn spartados_unreadable_subdirectories_are_empty() {
    // Subdirectories with a map past the end, a length of 0, a length past the disk and a
    // sparse sector are listed empty; the files around them stay readable.
    let mut sparta = Sparta::new(128, 720);
    let before = sparta.write(&data(300, 5));
    let after = sparta.write(&data(200, 6));
    let zero_length = sparta.directory("ZERO", &[]);
    let zero_sector = sparta.write(&[0; 23]);
    let huge = sparta.directory("HUGE", &[(SPARTA_FILE, after, 200, "HIDDEN", [0; 6])]);
    let mut header = vec![SPARTA_DIRECTORY, 0, 0, 200, 0, 0];
    header.resize(128, 0);
    let sparse = sparta.write_sectors(&[Some(&header), None]);
    let root = sparta.directory(
        "MAIN",
        &[
            (SPARTA_FILE, before, 300, "BEFORE", [0; 6]),
            (SPARTA_DIRECTORY, 800, 0, "PAST", [0; 6]),
            (SPARTA_DIRECTORY, zero_sector, 0, "ZERO", [0; 6]),
            (SPARTA_DIRECTORY, huge, 0, "HUGE", [0; 6]),
            (SPARTA_DIRECTORY, sparse, 0, "SPARSE", [0; 6]),
            (SPARTA_FILE, after, 200, "AFTER", [0; 6]),
            (SPARTA_DIRECTORY, zero_length, 0, "EMPTY", [0; 6]),
        ],
    );
    // HUGE's directory claims 0xFFFFFF bytes.
    let huge_data = usize::from(huge) - 1;
    sparta.disk.sector(huge_data)[3..6].fill(0xFF);
    sparta.set_root(root);
    let mut archive = open(&sparta.disk.atr()).unwrap();
    let mut names = Vec::new();
    while let Some(entry) = archive.get_next_entry().unwrap() {
        if entry.name == "BEFORE" || entry.name == "AFTER" {
            assert_eq!(archive.read(&entry).unwrap().len(), entry.size as usize);
        }
        names.push(entry.name);
    }
    assert_eq!(names, ["BEFORE", "PAST/", "ZERO/", "HUGE/", "SPARSE/", "AFTER", "EMPTY/"]);
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

/// A MyDOS disk with a subdirectory and a file with 16-bit links.
fn mydos() -> Vec<u8> {
    let mut disk = Disk::new(256, 1100);
    vtoc(&mut disk, 0);
    file(&mut disk, 361, 0, 4, "OK.TXT", &data(600, 7));
    entry(&mut disk, 361, 1, MYDOS_DIRECTORY, 8, 370, &name("SUB"));
    file(&mut disk, 370, 0, 10, "IN.SUB", &data(400, 8));
    let sectors: Vec<usize> = (1050..1053).chain(20..23).collect();
    chain(&mut disk, &sectors, 0, &data(1500, 9), true);
    entry(&mut disk, 370, 1, common::MYDOS_LONG_LINKS, 6, 1050, &name("LONG"));
    disk.atr()
}

/// A SpartaDOS disk with a subdirectory and a sparse file, all in sectors 4-30.
fn spartados() -> Vec<u8> {
    let mut sparta = Sparta::new(128, 720);
    let file = sparta.write(&data(700, 10));
    let sparse = sparta.write_sectors(&[Some(&[1; 128]), None, Some(&[2; 5])]);
    let sub = sparta.directory("SUB", &[(SPARTA_FILE, sparse, 261, "SPARSE", [0; 6])]);
    let root = sparta.directory(
        "MAIN",
        &[(SPARTA_FILE, file, 700, "FILE", [1, 2, 90, 0, 0, 0]), (SPARTA_DIRECTORY, sub, 0, "SUB", [0; 6])],
    );
    sparta.set_root(root);
    sparta.disk.atr()
}

#[test]
fn truncated_and_mutated_images_do_not_panic() {
    // Each image with the byte ranges that hold its directories, maps and first sectors.
    let images = [
        (DOS20S.to_vec(), vec![16 + 359 * 128..16 + 368 * 128, 16 + 3 * 128..16 + 12 * 128]),
        (mydos(), vec![16 + 384 + 356 * 256..16 + 384 + 375 * 256, 16 + 384..16 + 384 + 20 * 256]),
        (spartados(), vec![16..16 + 3 * 128, 16 + 3 * 128..16 + 30 * 128]),
    ];
    assert!(images.iter().all(|(image, _)| read_all(image).is_ok()));
    let mut state = 0x1234_5678u32;
    for (base, ranges) in &images {
        for len in (0..base.len()).step_by(997) {
            let _ = read_all(&base[..len]);
        }
        for _ in 0..300 {
            let mut image = base.clone();
            for _ in 0..40 {
                state ^= state << 13;
                state ^= state >> 17;
                state ^= state << 5;
                let range = &ranges[state as usize % ranges.len()];
                let offset = range.start + (state as usize >> 8) % range.len();
                image[offset] = (state >> 3) as u8;
            }
            let _ = read_all(&image);
        }
    }
}
