use std::io::{Cursor, Write};

use unarc_rs::unified::{ArchiveFormat, ArchiveOptions, UnifiedArchive};
use unarc_rs::ArchiveEntryKind;

fn open(data: Vec<u8>, format: ArchiveFormat) -> UnifiedArchive<Cursor<Vec<u8>>> {
    UnifiedArchive::open_with_format(Cursor::new(data), format).unwrap()
}

#[test]
fn existing_regular_entries_keep_their_type_and_payload() {
    for (format, path) in [
        (ArchiveFormat::Ace, "tests/ace/license1.ace"),
        (ArchiveFormat::Arc, "tests/arc/store.arc"),
        (ArchiveFormat::Arj, "tests/arj/stored.arj"),
        (ArchiveFormat::Zoo, "tests/zoo/store.zoo"),
        (ArchiveFormat::Sq, "tests/qqq/license.sq"),
        (ArchiveFormat::Sqz, "tests/sqz/store.sqz"),
        (ArchiveFormat::Ha, "tests/ha/copy.ha"),
        (ArchiveFormat::Hyp, "tests/hyp/stored.hyp"),
        (ArchiveFormat::Jar, "tests/jar/license_m1.j"),
        (ArchiveFormat::Uc2, "tests/uc2/fast.uc2"),
        (ArchiveFormat::Ice, "tests/ice/license_lha.ice"),
        (ArchiveFormat::PackIce, "tests/pi9/REBATE.PI9"),
        (ArchiveFormat::Z, "tests/Z/LICENSE.Z"),
        (ArchiveFormat::Gz, "tests/gz/LICENSE.gz"),
        (ArchiveFormat::Bz2, "tests/bz2/LICENSE.bz2"),
        (ArchiveFormat::Xz, "tests/xz/LICENSE.xz"),
        (ArchiveFormat::Zst, "tests/zst/LICENSE.zst"),
        (ArchiveFormat::TarZ, "tests/tarz/license.tar.Z"),
        (ArchiveFormat::Txz, "tests/txz/license.tar.xz"),
        (ArchiveFormat::Tzst, "tests/tzst/license.tar.zst"),
    ] {
        let bytes = std::fs::read(path).unwrap();
        let mut archive = open(bytes, format);
        let entry = archive.next_entry().unwrap().unwrap();
        assert_eq!(entry.kind(), ArchiveEntryKind::File, "{path}: {entry:?}");
        assert_eq!(entry.link_target(), None, "{path}");
        assert!(!entry.is_directory(), "{path}");
        assert!(!archive.read(&entry).unwrap().is_empty(), "{path}");
    }
}

#[test]
fn arj_uses_its_own_unix_type_bits_not_posix_modes() {
    let original = std::fs::read("tests/arj/stored.arj").unwrap();
    let local = original
        .windows(2)
        .enumerate()
        .filter_map(|(index, magic)| (magic == b"\x60\xea").then_some(index))
        .nth(1)
        .unwrap();
    let start = local + 4;
    let length = u16::from_le_bytes(original[local + 2..local + 4].try_into().unwrap()) as usize;
    let end = start + length;
    for (mode, kind) in [
        (0x1000u16, ArchiveEntryKind::File),
        (0x2000, ArchiveEntryKind::Directory),
        (0x4000, ArchiveEntryKind::Special),
        (0x8000, ArchiveEntryKind::Unknown),
    ] {
        let mut bytes = original.clone();
        bytes[start + 26..start + 28].copy_from_slice(&(mode | 0o644).to_le_bytes());
        let checksum = crc32fast::hash(&bytes[start..end]);
        bytes[end..end + 4].copy_from_slice(&checksum.to_le_bytes());
        let mut archive = open(bytes, ArchiveFormat::Arj);
        let entry = archive.next_entry().unwrap().unwrap();
        assert_eq!(entry.kind(), kind);
        assert_eq!(entry.is_directory(), kind == ArchiveEntryKind::Directory);
        assert_eq!(entry.link_target(), None);
    }
}

fn tar_entries() -> Vec<u8> {
    let mut builder = tar::Builder::new(Vec::new());
    for (name, entry_type, target, data) in [
        ("file", tar::EntryType::Regular, None, &b"hello"[..]),
        ("dir/", tar::EntryType::Directory, None, &b""[..]),
        ("symlink", tar::EntryType::Symlink, Some("file"), &b""[..]),
        ("hardlink", tar::EntryType::Link, Some("file"), &b""[..]),
        ("fifo", tar::EntryType::Fifo, None, &b""[..]),
        ("char", tar::EntryType::Char, None, &b""[..]),
        ("block", tar::EntryType::Block, None, &b""[..]),
        ("unknown", tar::EntryType::new(b'?'), None, &b""[..]),
    ] {
        let mut header = tar::Header::new_gnu();
        header.set_entry_type(entry_type);
        header.set_mode(0o644);
        header.set_size(data.len() as u64);
        if let Some(target) = target {
            header.set_link_name(target).unwrap();
        }
        header.set_cksum();
        builder.append_data(&mut header, name, data).unwrap();
    }
    let mut header = tar::Header::new_gnu();
    header.set_entry_type(tar::EntryType::Symlink);
    header.set_mode(0o777);
    header.set_size(0);
    builder.append_link(&mut header, "long-link", "a/".repeat(100)).unwrap();
    builder.into_inner().unwrap()
}

#[test]
fn tar_types_and_extended_link_names() {
    let tar = tar_entries();
    let mut gzip = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    gzip.write_all(&tar).unwrap();
    let mut bzip = bzip2::write::BzEncoder::new(Vec::new(), bzip2::Compression::default());
    bzip.write_all(&tar).unwrap();
    for (format, bytes) in [
        (ArchiveFormat::Tar, tar),
        (ArchiveFormat::Tgz, gzip.finish().unwrap()),
        (ArchiveFormat::Tbz, bzip.finish().unwrap()),
    ] {
        let mut archive = open(bytes, format);
        for (name, kind, target) in [
            ("file", ArchiveEntryKind::File, None),
            ("dir/", ArchiveEntryKind::Directory, None),
            ("symlink", ArchiveEntryKind::SymbolicLink, Some("file")),
            ("hardlink", ArchiveEntryKind::HardLink, Some("file")),
            ("fifo", ArchiveEntryKind::Special, None),
            ("char", ArchiveEntryKind::Special, None),
            ("block", ArchiveEntryKind::Special, None),
            ("unknown", ArchiveEntryKind::Unknown, None),
        ] {
            let entry = archive.next_entry().unwrap().unwrap();
            assert_eq!(entry.name(), name);
            assert_eq!(entry.kind(), kind);
            assert_eq!(entry.is_directory(), kind == ArchiveEntryKind::Directory);
            assert_eq!(entry.link_target(), target);
            if kind == ArchiveEntryKind::File {
                assert_eq!(archive.read(&entry).unwrap(), b"hello");
            } else {
                archive.skip(&entry).unwrap();
            }
        }
        let entry = archive.next_entry().unwrap().unwrap();
        assert_eq!(entry.name(), "long-link");
        assert_eq!(entry.kind(), ArchiveEntryKind::SymbolicLink);
        assert_eq!(entry.link_target(), Some("a/".repeat(100).as_str()));
        archive.skip(&entry).unwrap();
        assert!(archive.next_entry().unwrap().is_none());
    }
}

fn zip_entries() -> Vec<u8> {
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let options = zip::write::SimpleFileOptions::default();
    writer.start_file("file", options).unwrap();
    writer.write_all(b"hello").unwrap();
    writer.add_directory("dir/", options).unwrap();
    writer.add_symlink("symlink", "file", options).unwrap();
    writer.start_file("fifo", options).unwrap();
    let mut bytes = writer.finish().unwrap().into_inner();
    let central_headers: Vec<usize> = bytes
        .windows(4)
        .enumerate()
        .filter_map(|(index, signature)| (signature == b"PK\x01\x02").then_some(index))
        .collect();
    let fifo = central_headers[3];
    bytes[fifo + 38..fifo + 42].copy_from_slice(&((0o010644u32) << 16).to_le_bytes());
    bytes
}

#[test]
fn zip_unix_links_special_and_regular_entries() {
    let mut archive = open(zip_entries(), ArchiveFormat::Zip);
    for (name, kind, target) in [
        ("file", ArchiveEntryKind::File, None),
        ("dir/", ArchiveEntryKind::Directory, None),
        ("symlink", ArchiveEntryKind::SymbolicLink, Some("file")),
        ("fifo", ArchiveEntryKind::Special, None),
    ] {
        let entry = archive.next_entry().unwrap().unwrap();
        assert_eq!(entry.name(), name);
        assert_eq!(entry.kind(), kind);
        assert_eq!(entry.is_directory(), kind == ArchiveEntryKind::Directory);
        assert_eq!(entry.link_target(), target);
        if kind == ArchiveEntryKind::File {
            assert_eq!(archive.read(&entry).unwrap(), b"hello");
        } else if kind == ArchiveEntryKind::SymbolicLink {
            assert_eq!(archive.read(&entry).unwrap(), b"file");
        }
    }
}

#[test]
fn zip_target_limit_does_not_hide_link_kind() {
    let mut archive = UnifiedArchive::open_with_format_and_options(
        Cursor::new(zip_entries()),
        ArchiveFormat::Zip,
        ArchiveOptions::new().with_max_entry_size(Some(3)),
    )
    .unwrap();
    archive.next_entry().unwrap();
    archive.next_entry().unwrap();
    let link = archive.next_entry().unwrap().unwrap();
    assert_eq!(link.kind(), ArchiveEntryKind::SymbolicLink);
    assert_eq!(link.link_target(), None);
    assert!(matches!(archive.read(&link), Err(unarc_rs::ArchiveError::SizeLimitExceeded { limit: 3, .. })));
}

fn sevenz_entries() -> Vec<u8> {
    let mut writer = sevenz_rust2::ArchiveWriter::new(Cursor::new(Vec::new())).unwrap();
    for (name, mode, data) in [
        ("file", 0o100644, &b"hello"[..]),
        ("symlink", 0o120777, &b"file"[..]),
        ("fifo", 0o010644, &b""[..]),
        ("dir", 0o040755, &b""[..]),
        ("reparse", 0, &b""[..]),
    ] {
        let entry = sevenz_rust2::ArchiveEntry {
            name: name.to_string(),
            is_directory: name == "dir",
            has_windows_attributes: true,
            windows_attributes: if name == "reparse" { 0x400 } else { (mode << 16) | 0x8000 },
            ..Default::default()
        };
        writer.push_archive_entry(entry, Some(data)).unwrap();
    }
    writer.finish().unwrap().into_inner()
}

#[test]
fn sevenz_unix_types_and_targets() {
    let mut archive = open(sevenz_entries(), ArchiveFormat::SevenZ);
    for (name, kind, target) in [
        ("file", ArchiveEntryKind::File, None),
        ("symlink", ArchiveEntryKind::SymbolicLink, Some("file")),
        ("fifo", ArchiveEntryKind::Special, None),
        ("dir", ArchiveEntryKind::Directory, None),
        ("reparse", ArchiveEntryKind::Unknown, None),
    ] {
        let entry = archive.next_entry().unwrap().unwrap();
        assert_eq!(entry.name(), name);
        assert_eq!(entry.kind(), kind);
        assert_eq!(entry.link_target(), target);
        assert_eq!(entry.is_directory(), kind == ArchiveEntryKind::Directory);
        if kind == ArchiveEntryKind::File {
            assert_eq!(archive.read(&entry).unwrap(), b"hello");
        }
    }
}

#[test]
fn sevenz_target_limit_does_not_hide_link_kind() {
    let mut archive = UnifiedArchive::open_with_format_and_options(
        Cursor::new(sevenz_entries()),
        ArchiveFormat::SevenZ,
        ArchiveOptions::new().with_max_entry_size(Some(3)),
    )
    .unwrap();
    archive.next_entry().unwrap();
    let link = archive.next_entry().unwrap().unwrap();
    assert_eq!(link.kind(), ArchiveEntryKind::SymbolicLink);
    assert_eq!(link.link_target(), None);
}

#[test]
fn sevenz_solid_links_duplicate_names_and_per_read_limits() {
    let mut writer = sevenz_rust2::ArchiveWriter::new(Cursor::new(Vec::new())).unwrap();
    let entries = [("same", 0o100644), ("same", 0o100644), ("link", 0o120777)]
        .into_iter()
        .map(|(name, mode)| sevenz_rust2::ArchiveEntry {
            name: name.to_owned(),
            has_stream: true,
            has_windows_attributes: true,
            windows_attributes: (mode << 16) | 0x8000,
            ..Default::default()
        })
        .collect();
    let sources = [&b"first"[..], &b"second"[..], &b"same"[..]]
        .into_iter()
        .map(sevenz_rust2::SourceReader::new)
        .collect();
    writer.push_archive_entries(entries, sources).unwrap();
    let mut archive = open(writer.finish().unwrap().into_inner(), ArchiveFormat::SevenZ);
    let first = archive.next_entry().unwrap().unwrap();
    let second = archive.next_entry().unwrap().unwrap();
    let link = archive.next_entry().unwrap().unwrap();
    assert_eq!(link.kind(), ArchiveEntryKind::SymbolicLink);
    assert_eq!(link.link_target(), Some("same"));
    assert_eq!(archive.read(&second).unwrap(), b"second");
    assert_eq!(archive.read(&first).unwrap(), b"first");
    let options = ArchiveOptions::new().with_max_entry_size(Some(3));
    assert!(matches!(
        archive.read_with_options(&second, &options),
        Err(unarc_rs::ArchiveError::SizeLimitExceeded { limit: 3, .. })
    ));
}

fn vint(mut value: u64) -> Vec<u8> {
    let mut bytes = Vec::new();
    loop {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        bytes.push(if value == 0 { byte } else { byte | 0x80 });
        if value == 0 {
            return bytes;
        }
    }
}

fn rar_block(body: &[u8]) -> Vec<u8> {
    let mut header = vint(body.len() as u64);
    header.extend_from_slice(body);
    let mut bytes = crc32fast::hash(&header).to_le_bytes().to_vec();
    bytes.extend(header);
    bytes
}

fn rar_entries() -> Vec<u8> {
    let mut bytes = b"Rar!\x1a\x07\x01\x00".to_vec();
    bytes.extend(rar_block(&[1, 0, 0]));
    for (name, mode, redirection, directory) in [
        ("file", 0o100644u64, None, false),
        ("symlink", 0o120777, Some(1), false),
        ("win-link", 0, Some(2), false),
        ("junction", 0, Some(3), true),
        ("hardlink", 0o100644, Some(4), false),
        ("copy", 0o100644, Some(5), false),
        ("fifo", 0o010644, None, false),
        ("dir", 0o040755, None, true),
        ("unknown", 0, Some(99), false),
    ] {
        let mut extra = Vec::new();
        if let Some(redirection) = redirection {
            let record = [5, redirection, 0, 4, b'f', b'i', b'l', b'e'];
            extra.extend(vint(record.len() as u64));
            extra.extend(record);
        }
        let mut body = vec![2, u8::from(!extra.is_empty())];
        if !extra.is_empty() {
            body.extend(vint(extra.len() as u64));
        }
        body.push(u8::from(directory));
        body.push(0);
        body.extend(vint(mode));
        body.extend([0, 1]);
        body.extend(vint(name.len() as u64));
        body.extend_from_slice(name.as_bytes());
        body.extend(extra);
        bytes.extend(rar_block(&body));
    }
    bytes.extend(rar_block(&[5, 0, 0]));
    bytes
}

#[test]
fn rar_redirections_unix_types_and_file_copies() {
    let mut archive = open(rar_entries(), ArchiveFormat::Rar);
    for (name, kind, target) in [
        ("file", ArchiveEntryKind::File, None),
        ("symlink", ArchiveEntryKind::SymbolicLink, Some("file")),
        ("win-link", ArchiveEntryKind::SymbolicLink, Some("file")),
        ("junction", ArchiveEntryKind::SymbolicLink, Some("file")),
        ("hardlink", ArchiveEntryKind::HardLink, Some("file")),
        ("copy", ArchiveEntryKind::File, None),
        ("fifo", ArchiveEntryKind::Special, None),
        ("dir", ArchiveEntryKind::Directory, None),
        ("unknown", ArchiveEntryKind::Unknown, None),
    ] {
        let entry = archive.next_entry().unwrap().unwrap();
        assert_eq!(entry.name(), name);
        assert_eq!(entry.kind(), kind);
        assert_eq!(entry.link_target(), target);
        assert_eq!(entry.is_directory(), kind == ArchiveEntryKind::Directory);
        archive.skip(&entry).unwrap();
    }
}

#[test]
fn ha_explicit_directory_and_special_entries() {
    let mut bytes = b"HA\x03\x00".to_vec();
    for (method, name) in [(0, "file"), (14, "dir"), (15, "special")] {
        bytes.push(method);
        bytes.extend([0; 16]);
        bytes.push(0);
        bytes.extend_from_slice(name.as_bytes());
        bytes.extend([0, 0]);
    }
    let mut archive = open(bytes, ArchiveFormat::Ha);
    for kind in [ArchiveEntryKind::File, ArchiveEntryKind::Directory, ArchiveEntryKind::Special] {
        let entry = archive.next_entry().unwrap().unwrap();
        assert_eq!(entry.kind(), kind);
        assert_eq!(entry.link_target(), None);
        archive.skip(&entry).unwrap();
    }
}

#[test]
fn lha_unix_symlink_and_directory_without_suffix() {
    for (mode, name, expected_name, kind, target) in [
        (0o120777u16, "link|file", "link|file", ArchiveEntryKind::SymbolicLink, Some("file")),
        (0o120777, "link|/../file", "link|/file", ArchiveEntryKind::SymbolicLink, Some("/../file")),
        (0o040755, "dir", "dir", ArchiveEntryKind::Directory, None),
        (0o010644, "fifo", "fifo", ArchiveEntryKind::Special, None),
        (0o100644, "file", "file", ArchiveEntryKind::File, None),
        (0o100644, "nested/file", "nested/file", ArchiveEntryKind::File, None),
        (0o100644, "nested\\file", "nested/file", ArchiveEntryKind::File, None),
        (0o040755, "nested\\dir\\", "nested/dir/", ArchiveEntryKind::Directory, None),
    ] {
        let mut bytes = vec![0, 0];
        bytes.extend_from_slice(if kind == ArchiveEntryKind::File { b"-lh0-" } else { b"-lhd-" });
        bytes.extend([0; 12]);
        bytes.extend([0x20, 0, name.len() as u8]);
        bytes.extend_from_slice(name.as_bytes());
        bytes.extend([0; 2]);
        let mut unix = [0; 12];
        unix[0] = b'U';
        unix[6..8].copy_from_slice(&mode.to_le_bytes());
        bytes.extend(unix);
        bytes[0] = (bytes.len() - 2) as u8;
        bytes[1] = bytes[2..].iter().fold(0u8, |sum, byte| sum.wrapping_add(*byte));
        bytes.push(0);
        let mut archive = open(bytes, ArchiveFormat::Lha);
        let entry = archive.next_entry().unwrap().unwrap();
        assert_eq!(entry.name(), expected_name);
        assert_eq!(entry.kind(), kind);
        assert_eq!(entry.link_target(), target);
        assert_eq!(entry.is_directory(), kind == ArchiveEntryKind::Directory);
    }
}
