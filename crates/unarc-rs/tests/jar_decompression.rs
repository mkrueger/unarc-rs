//! Golden tests against JAR 1.02, including the two reversible binary transforms.
use std::{collections::BTreeMap, io::Cursor, path::Path};
use unarc_rs::{
    jar::JarArchive,
    unified::{ArchiveFormat, UnifiedArchive},
};

fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/jar").join(name)).unwrap()
}

const LICENSE: &[u8] = include_bytes!("../../../LICENSE");

#[test]
fn all_license_presets_extract_exactly() {
    for method in 1..=4 {
        let bytes = fixture(&format!("license_m{method}.j"));
        assert!(JarArchive::<Cursor<&[u8]>>::probe(&bytes));
        let mut archive = JarArchive::new(Cursor::new(bytes)).unwrap();
        assert_eq!(archive.header().version_needed, 27);
        let entries = archive.extract_all().unwrap();
        assert_eq!(entries.len(), 1);
        let (entry, data) = &entries[0];
        assert_eq!(entry.name, "LICENSE");
        assert_eq!(entry.original_size, 11357);
        assert_eq!(entry.modification_date, ((2025 - 1980) << 9) | (12 << 5) | 16);
        assert_eq!(entry.modification_time, (16 << 11) | (18 << 5) | 29);
        assert_eq!(entry.crc32, 0xc7e2d0b8);
        assert_eq!(data, LICENSE, "preset {method}");
        assert_eq!(archive.read_entry(entry).unwrap(), LICENSE);
        assert_eq!(archive.extract_all().unwrap()[0].1, LICENSE);
    }
}

#[test]
fn small_solid_archive_is_not_stored_data() {
    let mut archive = JarArchive::new(Cursor::new(fixture("test.j"))).unwrap();
    let files = archive.extract_all().unwrap();
    let expected: [(&str, &[u8]); 4] = [("A", b"a\r\n"), ("AA", b"aa\r\n"), ("B", b"b\r\n"), ("BB", b"bb\r\n")];
    assert_eq!(files.len(), expected.len());
    for ((entry, data), (name, bytes)) in files.iter().zip(expected) {
        assert_eq!(entry.name, name);
        assert_eq!(data, bytes);
    }
}

fn expected_multi() -> BTreeMap<String, Vec<u8>> {
    let delta: Vec<u8> = (0..4000u16)
        .flat_map(|i| (i * 2).to_le_bytes().into_iter().chain((i * 3).to_le_bytes()))
        .collect();
    let wide: Vec<u8> = "The quick brown fox jumps over the lazy dog.\r\n"
        .repeat(60)
        .encode_utf16()
        .flat_map(u16::to_le_bytes)
        .collect();
    [
        ("SUB", Vec::new()),
        ("EMPTY", Vec::new()),
        ("HELLO.TXT", b"Hello, world!\r\nThis is a test.\r\n".repeat(20)),
        ("LARGE.TXT", LICENSE.repeat(24)),
        ("SUB/NESTED.TXT", b"Nested file\n".repeat(80)),
        ("BINARY.DAT", (0..=255u8).cycle().take(1024).collect()),
        ("RANDOM.BIN", include_bytes!("jar/random.bin").to_vec()),
        ("DELTA.BIN", delta),
        ("WIDE.TXT", wide),
    ]
    .into_iter()
    .map(|(n, b)| (n.to_string(), b))
    .collect()
}

#[test]
fn all_presets_multi_file_transforms_and_multiple_huffman_blocks() {
    for method in 1..=4 {
        let mut expected = expected_multi();
        let mut archive = JarArchive::new(Cursor::new(fixture(&format!("multi_m{method}.j")))).unwrap();
        let files = archive.extract_all().unwrap();
        assert_eq!(files.len(), 9);
        for (entry, data) in files {
            assert_eq!(entry.is_directory, entry.name == "SUB");
            assert_eq!(entry.original_size as usize, data.len());
            assert_eq!(data, expected.remove(&entry.name).unwrap(), "preset {method}: {}", entry.name);
        }
        assert!(expected.is_empty());
    }
}

#[test]
fn unified_api_lists_reads_and_skips_solid_entries() {
    assert_eq!(ArchiveFormat::from_path(Path::new("archive.j")), Some(ArchiveFormat::Jar));
    let bytes = fixture("multi_m4.j");
    assert_eq!(ArchiveFormat::detect_from_bytes(&bytes), Some(ArchiveFormat::Jar));
    let mut archive = UnifiedArchive::open_with_format(Cursor::new(bytes), ArchiveFormat::Jar).unwrap();
    let mut expected = expected_multi();
    while let Some(entry) = archive.next_entry().unwrap() {
        assert_eq!(entry.is_directory(), entry.name() == "SUB");
        assert!(!entry.is_stored());
        let bytes = expected.remove(entry.name()).unwrap();
        assert_eq!(archive.read(&entry).unwrap(), bytes);
        archive.skip(&entry).unwrap();
    }
    assert!(expected.is_empty());
    assert!(archive.next_entry().unwrap().is_none());
}

#[test]
fn rejects_truncation_header_corruption_and_output_bombs() {
    let bytes = fixture("license_m1.j");
    for len in 0..bytes.len() {
        assert!(JarArchive::new(Cursor::new(&bytes[..len])).is_err(), "length {len}");
    }
    for pos in 0..64 {
        let mut corrupted = bytes.clone();
        corrupted[pos] ^= 1;
        assert!(JarArchive::new(Cursor::new(corrupted)).is_err(), "header offset {pos}");
    }
    let mut archive = JarArchive::new(Cursor::new(bytes)).unwrap();
    archive.set_output_limit(10000);
    assert!(archive.extract_all().is_err());
}

#[test]
fn rejects_corrupt_packets_huffman_data_and_directory_crc() {
    let bytes = fixture("license_m1.j");
    for pos in [64, 70, 72, 74, 77, 83, 100, 432, 433, 1000, 3000, 3344, 3450, 3623] {
        let mut corrupted = bytes.clone();
        corrupted[pos] ^= 0x80;
        let mut archive = JarArchive::new(Cursor::new(corrupted)).unwrap();
        assert!(archive.extract_all().is_err(), "corrupt byte {pos}");
    }
}
