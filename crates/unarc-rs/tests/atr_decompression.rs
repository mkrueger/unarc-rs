//! Atari 8-bit ATR disk images: fixtures made with the `atr` tool, plus synthetic
//! MyDOS and SpartaDOS layouts.
use std::io::Cursor;
use std::path::Path;

use unarc_rs::atr::{AtrArchive, AtrEntry, AtrFilesystem};
use unarc_rs::unified::{ArchiveEntryKind, ArchiveFormat, UnifiedArchive};

#[path = "common/atr.rs"]
mod common;
use common::{chain, data, entry, file, name, vtoc, Disk, Sparta, MYDOS_DIRECTORY, MYDOS_LONG_LINKS, SPARTA_DIRECTORY, SPARTA_FILE};

const DOS20S: &[u8] = include_bytes!("atr/dos20s.atr");
const DOS25: &[u8] = include_bytes!("atr/dos25.atr");
const DOS20D: &[u8] = include_bytes!("atr/dos20d.atr");
const LICENSE: &[u8] = include_bytes!("../../../LICENSE");
const FILE_ID: &[u8] = include_bytes!("atr/FILE_ID.DIZ");

fn open(image: &[u8]) -> AtrArchive {
    AtrArchive::new(Cursor::new(image)).unwrap()
}

fn entries(archive: &mut AtrArchive) -> Vec<AtrEntry> {
    let mut entries = Vec::new();
    while let Some(entry) = archive.get_next_entry().unwrap() {
        entries.push(entry);
    }
    entries
}

fn names(entries: &[AtrEntry]) -> Vec<&str> {
    entries.iter().map(|e| e.name.as_str()).collect()
}

/// Every fixture holds the repository LICENSE and a FILE_ID.DIZ.
fn check_fixture(image: &[u8], filesystem: AtrFilesystem, sector_size: usize, sectors: usize, expected: &[&str]) {
    let mut archive = open(image);
    assert_eq!(
        (archive.filesystem(), archive.sector_size(), archive.sectors()),
        (filesystem, sector_size, sectors)
    );
    let entries = entries(&mut archive);
    assert_eq!(names(&entries), expected);
    for entry in &entries {
        let data = archive.read(entry).unwrap();
        assert_eq!(data.len() as u64, entry.size, "{}", entry.name);
        assert!(!entry.is_directory && !entry.locked && entry.modified_time.is_none());
        assert_eq!(data, if entry.name == "FILE_ID.DIZ" { FILE_ID } else { LICENSE }, "{}", entry.name);
    }
}

#[test]
fn dos20s_fixture() {
    // GONE.TXT was deleted and is not listed.
    check_fixture(DOS20S, AtrFilesystem::AtariDos2, 128, 720, &["LICENSE", "FILE_ID.DIZ"]);
}

#[test]
fn dos25_fixture_with_files_above_sector_719() {
    check_fixture(
        DOS25,
        AtrFilesystem::AtariDos2,
        128,
        1040,
        &[
            "PAD1.TXT",
            "PAD2.TXT",
            "PAD3.TXT",
            "PAD4.TXT",
            "PAD5.TXT",
            "PAD6.TXT",
            "PAD7.TXT",
            "LICENSE",
            "FILE_ID.DIZ",
        ],
    );
    // DOS 2.5 flags files that use the extended area $03, so DOS 2.0 does not see them.
    assert_eq!(DOS25[16 + 360 * 128 + 7 * 16], 0x03);
    let mut archive = open(DOS25);
    let file_id = entries(&mut archive).pop().unwrap();
    assert_eq!(file_id.start_sector, 742);
}

#[test]
fn dos20d_fixture_and_boot_sector_layouts() {
    let expected = ["LICENSE", "FILE_ID.DIZ"];
    check_fixture(DOS20D, AtrFilesystem::AtariDos2, 256, 720, &expected);
    // The same disk with its boot sectors in 256-byte slots, padded or packed first.
    let body = &DOS20D[16..];
    let mut padded = common::header(720 * 256, 256);
    let mut packed = padded.clone();
    for boot in body[..384].chunks(128) {
        padded.extend(boot);
        padded.resize(padded.len() + 128, 0);
    }
    packed.extend(&body[..384]);
    packed.resize(16 + 768, 0);
    for image in [&mut padded, &mut packed] {
        image.extend(&body[384..]);
        check_fixture(image, AtrFilesystem::AtariDos2, 256, 720, &expected);
    }
    // 128-byte boot sectors followed by trailing data, which makes the length a multiple of 256.
    let mut trailing = DOS20D.to_vec();
    trailing.extend([0xAA; 384]);
    check_fixture(&trailing, AtrFilesystem::AtariDos2, 256, 721, &expected);
}

#[test]
fn unified_api() {
    assert_eq!(ArchiveFormat::from_path(Path::new("GAMES.ATR")), Some(ArchiveFormat::Atr));
    assert_eq!(ArchiveFormat::Atr.extension(), "atr");
    assert_eq!(ArchiveFormat::detect_from_bytes(DOS20S), Some(ArchiveFormat::Atr));
    assert_eq!(ArchiveFormat::detect(&mut Cursor::new(DOS20D), None).unwrap(), Some(ArchiveFormat::Atr));
    assert!(ArchiveFormat::ALL.contains(&ArchiveFormat::Atr));
    // The magic alone is not enough: the header must name a sector size.
    assert_eq!(ArchiveFormat::detect_from_bytes(&common::header(1000, 300)), None);
    assert_eq!(ArchiveFormat::detect_from_bytes(&DOS20S[..15]), None);

    let mut archive = UnifiedArchive::open_with_format(Cursor::new(DOS20S), ArchiveFormat::Atr).unwrap();
    let mut names = Vec::new();
    while let Some(entry) = archive.next_entry().unwrap() {
        assert_eq!(entry.kind(), ArchiveEntryKind::File);
        assert_eq!(entry.compression_method(), "Stored");
        let data = archive.read(&entry).unwrap();
        assert_eq!(data.len() as u64, entry.original_size());
        names.push(entry.name().to_string());
    }
    assert_eq!(names, ["LICENSE", "FILE_ID.DIZ"]);
}

/// Atari DOS details: empty files, directory art, locked and unclosed files, sorted
/// directories, and the never-written last sector of files that fill their sectors.
#[test]
fn dos2_directory_details() {
    let mut disk = Disk::new(128, 720);
    vtoc(&mut disk, 600);
    let text = data(300, 1);
    file(&mut disk, 361, 0, 4, "TEXT.TXT", &text);
    // DOS writes an empty file as one sector holding no data.
    file(&mut disk, 361, 1, 10, "EMPTY", b"");
    // Directory art: a title line without data, and a name of spaces only.
    entry(&mut disk, 361, 2, 0x42, 0, 0, b"--MY DISK--");
    entry(&mut disk, 361, 3, 0x42, 0, 0, b"           ");
    // Locked file whose name contains ATASCII graphics, inverse video and a slash.
    file(&mut disk, 361, 4, 20, "LOCKED", b"locked");
    entry(
        &mut disk,
        361,
        4,
        0x62,
        1,
        20,
        &[0x11, 0x12, b'/', 0xC1, b' ', b' ', b' ', b' ', b'B', b'A', b'K'],
    );
    // Never closed after writing ($43): listed, unlike DOS. Deleted ($80): hidden.
    file(&mut disk, 361, 5, 30, "OPEN", b"open");
    disk.sector(361)[5 * 16] = 0x43;
    file(&mut disk, 361, 6, 40, "GONE", b"gone");
    disk.sector(361)[6 * 16] = 0x80;
    // 250 bytes fill two sectors; the link to the third points at an unwritten sector.
    let full = data(250, 2);
    file(&mut disk, 361, 7, 50, "FULL", &full);
    disk.sector(51)[125..127].copy_from_slice(&[7 << 2, 52]);
    // A file from a sorted directory: its sectors carry another file number.
    let moved = data(200, 3);
    chain(&mut disk, &[60, 61], 20, &moved, false);
    entry(&mut disk, 361, 8, 0x42, 2, 60, &name("MOVED"));
    // Unknown flags without the in-use bit are skipped; the directory ends at the first $00.
    entry(&mut disk, 361, 9, 0x01, 1, 4, &name("ODD"));
    entry(&mut disk, 361, 11, 0x42, 1, 4, &name("AFTEREND"));

    let mut archive = open(&disk.atr());
    let entries = entries(&mut archive);
    assert_eq!(names(&entries), ["TEXT.TXT", "EMPTY", "--MY DIS.K--", "_", "┏━_A.BAK", "OPEN", "FULL", "MOVED"]);
    let read = |name: &str| archive.read(entries.iter().find(|e| e.name == name).unwrap()).unwrap();
    assert_eq!(read("TEXT.TXT"), text);
    assert!(read("EMPTY").is_empty() && read("--MY DIS.K--").is_empty() && read("_").is_empty());
    assert_eq!(read("┏━_A.BAK"), b"locked");
    assert!(entries[4].locked && !entries[0].locked);
    assert_eq!(read("FULL"), full);
    assert_eq!(read("MOVED"), moved);
    assert_eq!(entries[2].start_sector, 0);
}

/// A directory with only art is accepted when the VTOC looks like DOS; an empty one too.
#[test]
fn dos2_disks_without_files() {
    let mut disk = Disk::new(128, 720);
    vtoc(&mut disk, 707);
    let mut archive = open(&disk.atr());
    assert!(archive.get_next_entry().unwrap().is_none());
    entry(&mut disk, 361, 0, 0x60, 0, 0, b"* TITLE  * ");
    let mut archive = open(&disk.atr());
    assert_eq!(names(&entries(&mut archive)), ["* TITLE. *"]);
}

/// MyDOS: subdirectories hold their own 64 entries with their own file numbers; files on
/// disks above 1023 sectors use 16-bit links.
#[test]
fn mydos_subdirectories_and_long_links() {
    let mut disk = Disk::new(256, 2000);
    vtoc(&mut disk, 0);
    file(&mut disk, 361, 0, 4, "FILE_ID.DIZ", FILE_ID);
    entry(&mut disk, 361, 1, MYDOS_DIRECTORY, 8, 400, &name("DOCS"));
    file(&mut disk, 400, 0, 500, "LICENSE", LICENSE);
    entry(&mut disk, 400, 1, MYDOS_DIRECTORY | 0x20, 8, 410, &name("NESTED"));
    // 16-bit links reach sectors above 1023; the chain jumps from 1900 down to 1200.
    let big = data(3000, 4);
    let sectors: Vec<usize> = (1900..1906).chain(1200..1210).collect();
    chain(&mut disk, &sectors, 0, &big, true);
    entry(&mut disk, 410, 0, MYDOS_LONG_LINKS, 12, 1900, &name("BIG.BIN"));

    let mut archive = open(&disk.atr());
    assert_eq!(archive.filesystem(), AtrFilesystem::MyDos);
    let entries = entries(&mut archive);
    assert_eq!(names(&entries), ["FILE_ID.DIZ", "DOCS/", "DOCS/LICENSE", "DOCS/NESTED/", "DOCS/NESTED/BIG.BIN"]);
    assert!(entries[1].is_directory && entries[3].is_directory && entries[3].locked);
    assert_eq!(entries[4].size, 3000);
    assert_eq!(archive.read(&entries[0]).unwrap(), FILE_ID);
    assert_eq!(archive.read(&entries[2]).unwrap(), LICENSE);
    assert!(archive.read(&entries[3]).unwrap().is_empty());
    assert_eq!(archive.read(&entries[4]).unwrap(), big);
}

fn sparta(sector_size: usize) -> Vec<u8> {
    let mut sparta = Sparta::new(sector_size, 720);
    let license = sparta.write(LICENSE);
    let file_id = sparta.write(FILE_ID);
    // A sparse file: the middle sector is unallocated and reads as zeros.
    let ss = sector_size;
    let (head, tail) = (vec![1; ss], vec![2; 10]);
    let sparse = sparta.write_sectors(&[Some(&head), None, Some(&tail)]);
    let nested = sparta.directory("NESTED", &[(SPARTA_FILE, sparse, (2 * ss + 10) as u32, "SPARSE.BIN", [29, 2, 96, 23, 59, 59])]);
    let docs = sparta.directory(
        "DOCS",
        &[
            (SPARTA_FILE | 0x01, license, LICENSE.len() as u32, "LICENSE", [12, 3, 94, 10, 22, 30]),
            (SPARTA_DIRECTORY, nested, 0, "NESTED", [1, 1, 5, 0, 0, 0]),
        ],
    );
    let root = sparta.directory(
        "MAIN",
        &[
            (SPARTA_FILE, file_id, FILE_ID.len() as u32, "FILE_ID.DIZ", [7, 10, 85, 15, 52, 44]),
            (0x18, license, 1, "DELETED", [0; 6]),
            (SPARTA_DIRECTORY, docs, 0, "DOCS", [0; 6]),
        ],
    );
    sparta.set_root(root);
    sparta.disk.atr()
}

#[test]
fn spartados_directories_time_stamps_and_sparse_files() {
    for sector_size in [128, 256] {
        let mut archive = open(&sparta(sector_size));
        assert_eq!(archive.filesystem(), AtrFilesystem::SpartaDos);
        let entries = entries(&mut archive);
        assert_eq!(
            names(&entries),
            ["FILE_ID.DIZ", "DOCS/", "DOCS/LICENSE", "DOCS/NESTED/", "DOCS/NESTED/SPARSE.BIN"]
        );
        let stamp = |entry: &AtrEntry| entry.modified_time.map(|t| (t.year(), t.month(), t.day(), t.hour(), t.minute(), t.second()));
        assert_eq!(stamp(&entries[0]), Some((1985, 10, 7, 15, 52, 44)));
        assert_eq!(stamp(&entries[2]), Some((1994, 3, 12, 10, 22, 30)));
        assert_eq!(stamp(&entries[3]), Some((2005, 1, 1, 0, 0, 0)));
        assert_eq!(stamp(&entries[1]), None);
        assert!(entries[2].locked && !entries[0].locked);
        assert_eq!(archive.read(&entries[0]).unwrap(), FILE_ID);
        assert_eq!(archive.read(&entries[2]).unwrap(), LICENSE);
        let mut sparse = vec![1; sector_size];
        sparse.resize(2 * sector_size, 0);
        sparse.resize(2 * sector_size + 10, 2);
        assert_eq!(archive.read(&entries[4]).unwrap(), sparse);
    }
}
