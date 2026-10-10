use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::Path;

use unarc_rs::amiga::{AmigaArchive, AmigaEntryKind};
use unarc_rs::error::ArchiveError;
use unarc_rs::unified::{ArchiveEntryKind, ArchiveFormat, ArchiveOptions, UnifiedArchive};

#[path = "common/amiga.rs"]
mod common;

const OFS: &[u8] = include_bytes!("amiga/ofs.adf");
const FFS: &[u8] = include_bytes!("amiga/ffs.adf");
const FLAT: &[u8] = include_bytes!("amiga/flat.hdf");
const RDB: &[u8] = include_bytes!("amiga/partitions.hdf");
const BLOCKS: &[u8] = include_bytes!("amiga/blocks.hdf");
const TEXT: &[u8] = include_bytes!("amiga/payload.txt");

#[test]
fn extracts_independent_adf_fixtures_and_extension_chains() {
    for (data, method) in [(OFS, "OFS"), (FFS, "FFS")] {
        let mut archive = AmigaArchive::open_adf(Cursor::new(data)).unwrap();
        assert_eq!(archive.volumes()[0].name, "Retro");
        assert_eq!(archive.volumes()[0].filesystem(), method);
        let mut entries = Vec::new();
        while let Some(entry) = archive.get_next_entry().unwrap() {
            entries.push(entry);
        }
        assert_eq!(entries.len(), 4);
        for entry in entries.iter().rev() {
            match entry.name.as_str() {
                "docs/" => {
                    assert_eq!(entry.kind, AmigaEntryKind::Directory);
                    assert!(archive.read(entry).unwrap().is_empty());
                }
                "empty" => assert!(archive.read(entry).unwrap().is_empty()),
                "docs/readme.txt" => {
                    assert_eq!(archive.read(entry).unwrap(), TEXT);
                    if method == "OFS" {
                        let time = entry.modified_time.unwrap();
                        assert_eq!(
                            (time.year(), time.month(), time.day(), time.hour(), time.minute(), time.second()),
                            (2024, 5, 16, 23, 8, 26)
                        );
                    }
                }
                "docs/large.bin" => assert_eq!(archive.read(entry).unwrap(), vec![0; 102400]),
                other => panic!("unexpected {other}"),
            }
            archive.skip(entry).unwrap();
        }
    }
}

#[test]
fn extracts_flat_hdf_and_rdb_partition_namespaces() {
    for (data, partitions) in [(FLAT, false), (RDB, true)] {
        let mut archive = AmigaArchive::open_hdf(Cursor::new(data)).unwrap();
        assert_eq!(archive.volumes().len(), if partitions { 2 } else { 1 });
        if partitions {
            assert_eq!(archive.volumes()[0].partition_name.as_deref(), Some("DH0"));
            assert_eq!(archive.volumes()[1].partition_name.as_deref(), Some("DH1"));
        }
        let mut files = 0;
        while let Some(entry) = archive.get_next_entry().unwrap() {
            let data = archive.read(&entry).unwrap();
            if entry.name.ends_with("readme.txt") {
                files += 1;
                assert_eq!(data, TEXT);
            } else if entry.name.ends_with("large.bin") {
                assert_eq!(data, vec![0; 102400]);
            }
        }
        assert_eq!(files, if partitions { 2 } else { 1 });
    }
}

#[test]
fn all_dos_variants_and_larger_filesystem_blocks() {
    for dos_type in 0..=5 {
        for size in [512, 1024, 4096, 65536] {
            let data = common::image(dos_type, size, 16);
            let mut archive = AmigaArchive::open_hdf(Cursor::new(data)).unwrap();
            assert_eq!(archive.volumes()[0].block_size, size);
            let directory = archive.get_next_entry().unwrap().unwrap();
            assert_eq!(directory.name, "docs/");
            let file = archive.get_next_entry().unwrap().unwrap();
            assert_eq!(archive.read(&file).unwrap(), common::PAYLOAD);
        }
    }
}

#[test]
fn rdb_larger_blocks_use_sizeblock_times_sectors_per_block() {
    let mut archive = AmigaArchive::open_hdf(Cursor::new(BLOCKS)).unwrap();
    assert_eq!(archive.volumes()[0].block_size, 1024);
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(entry.name, "DH0/");
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(entry.name, "DH0/readme.txt");
    assert_eq!(archive.read(&entry).unwrap(), TEXT);
}

#[test]
fn hd_adf_uses_midpoint_not_bootblocks_historical_880_pointer() {
    let mut data = common::image(1, 512, 3520);
    common::put(&mut data[..512], 2, 880);
    assert_eq!(common::get(&data[..512], 2), 880);
    let mut archive = AmigaArchive::open_adf(Cursor::new(data)).unwrap();
    archive.get_next_entry().unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(archive.read(&entry).unwrap(), common::PAYLOAD);
}

#[test]
fn unified_entries_and_limits() {
    for (data, format) in [(OFS, ArchiveFormat::Adf), (FFS, ArchiveFormat::Adf), (RDB, ArchiveFormat::Hdf)] {
        let mut archive = UnifiedArchive::open_with_format(Cursor::new(data), format).unwrap();
        let mut entries = Vec::new();
        while let Some(entry) = archive.next_entry().unwrap() {
            entries.push(entry);
        }
        for entry in &entries {
            assert!(entry.is_stored());
            assert!(!entry.is_encrypted());
            if entry.name().ends_with('/') {
                assert_eq!(entry.kind(), ArchiveEntryKind::Directory);
            } else if entry.name().ends_with("readme.txt") {
                let options = ArchiveOptions::new().with_max_entry_size(Some(TEXT.len() as u64 - 1));
                assert!(matches!(
                    archive.read_with_options(entry, &options),
                    Err(ArchiveError::SizeLimitExceeded { .. })
                ));
                assert_eq!(archive.read(entry).unwrap(), TEXT);
            }
        }
        let entry = entries.iter().find(|e| e.name().ends_with("readme.txt")).unwrap();
        let options = ArchiveOptions::new().with_max_total_size(Some(archive.read(entry).unwrap().len() as u64));
        assert!(matches!(
            archive.read_with_options(entry, &options),
            Err(ArchiveError::SizeLimitExceeded { .. })
        ));
    }
}

#[test]
fn detection_preserves_position_and_distinguishes_images() {
    for (data, format) in [
        (OFS, ArchiveFormat::Adf),
        (FFS, ArchiveFormat::Adf),
        (FLAT, ArchiveFormat::Hdf),
        (RDB, ArchiveFormat::Hdf),
    ] {
        assert_eq!(ArchiveFormat::detect_from_bytes(data), Some(format));
        let mut prefixed = b"prefix".to_vec();
        prefixed.extend_from_slice(data);
        let mut reader = Cursor::new(prefixed);
        reader.set_position(6);
        assert_eq!(ArchiveFormat::detect_from_reader(&mut reader).unwrap(), Some(format));
        assert_eq!(reader.position(), 6);
        if format == ArchiveFormat::Hdf {
            let mut archive = AmigaArchive::open_hdf(reader).unwrap();
            assert!(archive.get_next_entry().unwrap().is_some());
        } else {
            assert_eq!(
                ArchiveFormat::detect(&mut reader, Some(Path::new("image.hdf"))).unwrap(),
                Some(ArchiveFormat::Hdf)
            );
        }
    }
    assert_eq!(ArchiveFormat::from_path(Path::new("DISK.ADF")), Some(ArchiveFormat::Adf));
    assert_eq!(ArchiveFormat::from_path(Path::new("DISK.HDF")), Some(ArchiveFormat::Hdf));
    assert!(unarc_rs::unified::supported_extensions().contains(&"adf"));
    assert!(unarc_rs::unified::supported_extensions().contains(&"hdf"));
}

struct ShortReads(Cursor<Vec<u8>>);
impl Read for ShortReads {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = buf.len().min(1);
        self.0.read(&mut buf[..size])
    }
}
impl Seek for ShortReads {
    fn seek(&mut self, position: SeekFrom) -> std::io::Result<u64> {
        self.0.seek(position)
    }
}

#[test]
fn short_reads_and_nonzero_rdb_location() {
    let mut data = RDB.to_vec();
    data.copy_within(0..512, 512 * 15);
    data[..512].fill(0);
    let mut reader = ShortReads(Cursor::new(data));
    assert_eq!(ArchiveFormat::detect_from_reader(&mut reader).unwrap(), Some(ArchiveFormat::Hdf));
    assert_eq!(reader.0.position(), 0);
    let mut archive = AmigaArchive::open_hdf(reader).unwrap();
    assert!(archive.get_next_entry().unwrap().is_some());
}

#[test]
fn links_are_metadata_not_recursive_directory_walks() {
    let size = 512;
    let mut data = common::image(1, size, 16);
    for (block, subtype, name) in [(6, 3, &b"soft"[..]), (7, -4, &b"hard"[..]), (9, 4, &b"dirlink"[..])] {
        let mut header = common::named_header(size, block, 3, subtype, name);
        if subtype == 3 {
            header[24..36].copy_from_slice(b"Test:readme\0");
        } else {
            common::put(&mut header, size / 4 - 11, if subtype == -4 { 4 } else { 3 });
        }
        common::fix_checksum(&mut header, 5);
        data[block as usize * size..(block as usize + 1) * size].copy_from_slice(&header);
        common::put(&mut data[3 * size..4 * size], block as usize + 1, block);
    }
    common::fix_checksum(&mut data[3 * size..4 * size], 5);
    let mut archive = UnifiedArchive::open_with_format(Cursor::new(data), ArchiveFormat::Hdf).unwrap();
    let mut links = 0;
    while let Some(entry) = archive.next_entry().unwrap() {
        match entry.name() {
            "docs/soft" => {
                links += 1;
                assert_eq!(entry.kind(), ArchiveEntryKind::SymbolicLink);
                assert_eq!(entry.link_target(), Some("Test:readme"));
                assert_eq!(archive.read(&entry).unwrap(), b"Test:readme");
            }
            "docs/hard" => {
                links += 1;
                assert_eq!(entry.kind(), ArchiveEntryKind::HardLink);
                assert_eq!(entry.link_target(), Some("docs/readme"));
                assert_eq!(archive.read(&entry).unwrap(), common::PAYLOAD);
            }
            "docs/dirlink" => {
                links += 1;
                assert_eq!(entry.kind(), ArchiveEntryKind::HardLink);
                assert_eq!(entry.link_target(), Some("docs/"));
                assert!(matches!(archive.read(&entry), Err(ArchiveError::UnsupportedMethod { .. })));
            }
            _ => {}
        }
    }
    assert_eq!(links, 3);
}

struct SparseImage {
    len: u64,
    position: u64,
    segments: Vec<(u64, Vec<u8>)>,
    bytes_read: std::rc::Rc<std::cell::Cell<usize>>,
}

impl Read for SparseImage {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let count = self.len.saturating_sub(self.position).min(buffer.len() as u64) as usize;
        buffer[..count].fill(0);
        for (offset, data) in &self.segments {
            let begin = self.position.max(*offset);
            let end = (self.position + count as u64).min(*offset + data.len() as u64);
            if begin < end {
                buffer[(begin - self.position) as usize..(end - self.position) as usize]
                    .copy_from_slice(&data[(begin - offset) as usize..(end - offset) as usize]);
            }
        }
        self.position += count as u64;
        self.bytes_read.set(self.bytes_read.get() + count);
        Ok(count)
    }
}

impl Seek for SparseImage {
    fn seek(&mut self, from: SeekFrom) -> std::io::Result<u64> {
        let position = match from {
            SeekFrom::Start(n) => i128::from(n),
            SeekFrom::End(n) => i128::from(self.len) + i128::from(n),
            SeekFrom::Current(n) => i128::from(self.position) + i128::from(n),
        };
        self.position = u64::try_from(position).map_err(|_| std::io::ErrorKind::InvalidInput)?;
        Ok(self.position)
    }
}

#[test]
fn opening_a_four_gib_hdf_reads_only_directory_metadata() {
    let len = 4 * 1024 * 1024 * 1024_u64;
    let root_index = len / 512 / 2;
    let mut data = common::image(1, 512, 16);
    common::put(&mut data[3 * 512..4 * 512], 125, root_index as u32);
    common::fix_checksum(&mut data[3 * 512..4 * 512], 5);
    let read_count = std::rc::Rc::new(std::cell::Cell::new(0));
    let reader = SparseImage {
        len,
        position: 0,
        segments: vec![(0, data[..7 * 512].to_vec()), (root_index * 512, data[8 * 512..9 * 512].to_vec())],
        bytes_read: read_count.clone(),
    };
    let mut archive = AmigaArchive::open_hdf(reader).unwrap();
    assert!(read_count.get() < 8192, "opening read {} bytes", read_count.get());
    archive.get_next_entry().unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(archive.read(&entry).unwrap(), common::PAYLOAD);
}
