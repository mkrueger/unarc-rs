use std::io::Cursor;

use unarc_rs::tar::TarArchive;

#[test]
fn extract_tar() {
    let file = Cursor::new(include_bytes!("tar/license.tar"));
    let mut archive = TarArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn tar_entry_count() {
    let file = Cursor::new(include_bytes!("tar/license.tar"));
    let archive = TarArchive::new(file).unwrap();
    assert_eq!(1, archive.entry_count());
}

fn duplicate_names_tar() -> Vec<u8> {
    let mut builder = tar::Builder::new(Vec::new());
    // Same name and size: metadata alone cannot identify an entry.
    for data in [b"first", b"other", b"third"] {
        let mut header = tar::Header::new_ustar();
        header.set_path("same.txt").unwrap();
        header.set_size(data.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        builder.append(&header, &data[..]).unwrap();
    }
    builder.into_inner().unwrap()
}

#[test]
fn duplicate_names_return_the_correct_data() {
    let mut archive = TarArchive::new(Cursor::new(duplicate_names_tar())).unwrap();
    for (index, data) in [b"first", b"other", b"third"].iter().enumerate() {
        let header = archive.get_next_entry().unwrap().unwrap();
        assert_eq!(header.index, index);
        assert_eq!(archive.read(&header).unwrap(), *data);
        // Reading/skipping an earlier entry must not consume the next one.
        assert_eq!(archive.read(&header).unwrap(), *data);
        archive.skip(&header).unwrap();
    }
    assert!(archive.get_next_entry().unwrap().is_none());
}

#[test]
fn indexed_reads_do_not_depend_on_iteration_position() {
    let mut archive = TarArchive::new(Cursor::new(duplicate_names_tar())).unwrap();
    let first = archive.get_next_entry().unwrap().unwrap();
    archive.skip(&first).unwrap();
    let second = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(archive.read(&first).unwrap(), b"first");
    assert_eq!(archive.get_next_entry().unwrap().unwrap().index, second.index);
    assert_eq!(archive.read(&second).unwrap(), b"other");
    let mut invalid = second;
    invalid.index = usize::MAX;
    assert!(archive.read(&invalid).is_err());
    assert!(archive.skip(&invalid).is_err());
    assert_eq!(archive.get_next_entry().unwrap().unwrap().index, 2);
}

#[test]
fn duplicate_names_work_through_unified_tar_and_compressed_wrappers() {
    use std::io::Write;
    use unarc_rs::unified::{ArchiveFormat, UnifiedArchive};
    let tar = duplicate_names_tar();
    let mut gzip = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    gzip.write_all(&tar).unwrap();
    let mut bzip = bzip2::write::BzEncoder::new(Vec::new(), bzip2::Compression::default());
    bzip.write_all(&tar).unwrap();
    for (format, bytes) in [
        (ArchiveFormat::Tar, tar),
        (ArchiveFormat::Tgz, gzip.finish().unwrap()),
        (ArchiveFormat::Tbz, bzip.finish().unwrap()),
    ] {
        let mut archive = UnifiedArchive::open_with_format(Cursor::new(bytes), format).unwrap();
        let entries = archive.entries().unwrap();
        for (i, data) in [(2, b"third"), (0, b"first"), (1, b"other")] {
            assert_eq!(archive.read(&entries[i]).unwrap(), data);
        }
    }
}
