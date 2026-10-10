use std::io::Cursor;

use unarc_rs::arc::{arc_archive::ArcArchive, local_file_header::CompressionMethod};

#[test]
fn extract_stored() {
    let file = Cursor::new(include_bytes!("arc/store.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Unpacked(2), entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_packed() {
    let file = Cursor::new(include_bytes!("arc/cpm.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    archive.skip(&entry).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(CompressionMethod::RLE90, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("arc/READ.COM"), result.as_slice());
}

#[test]
fn extract_sqeezed() {
    let file = Cursor::new(include_bytes!("arc/cpm.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("DDTZ.COM", entry.name);
    assert_eq!(CompressionMethod::Squeezed, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("arc/DDTZ.COM"), result.as_slice());
}

#[test]
fn extract_squashed() {
    let file = Cursor::new(include_bytes!("arc/squashed.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(CompressionMethod::Squashed, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_crunch() {
    let file = Cursor::new(include_bytes!("arc/crunch.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(CompressionMethod::Crunched(8), entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_encrypted() {
    // First test: without password should fail with CRC or decompression error
    let file = Cursor::new(include_bytes!("arc/license_cypted.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Crunched(8), entry.compression_method);

    // Without password, should fail
    let result = archive.read(&entry);
    assert!(result.is_err(), "Expected error without password");

    // Second test: with correct password should succeed
    let file = Cursor::new(include_bytes!("arc/license_cypted.arc"));
    let mut archive = ArcArchive::new(file).unwrap();
    archive.set_password("SECRET");
    assert!(archive.has_password());

    let entry = archive.get_next_entry().unwrap().unwrap();
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

// PAK format tests (uses same ARC reader with additional compression methods)

#[test]
fn extract_pak_distilled() {
    // license.pak is compressed with method 11 (Distilled)
    let file = Cursor::new(include_bytes!("pak/license.pak"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Distilled, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_pak_crushed() {
    // license_crushed.pak is compressed with method 10 (Crushed)
    let file = Cursor::new(include_bytes!("pak/license_crushed.pak"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Crushed, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_pak_crunched() {
    // license_crunched.pak is compressed with method 8 (Crunched)
    let file = Cursor::new(include_bytes!("pak/license_crunched.pak"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Crunched(8), entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

#[test]
fn extract_pak_squashed() {
    // license_squashed.pak is compressed with method 9 (Squashed)
    let file = Cursor::new(include_bytes!("pak/license_squashed.pak"));
    let mut archive = ArcArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(CompressionMethod::Squashed, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../../../LICENSE"), result.as_slice());
}

/// LICENSE, 12000 bytes of noise from a 23-letter alphabet, then LICENSE again.
/// The shift in statistics after the table fills makes ARC 5.21 reset it (CLEAR).
fn shifting_text() -> Vec<u8> {
    let license = include_bytes!("../../../LICENSE");
    let mut x: u32 = 12345;
    let noise = (0..12000).map(|_| {
        x = x.wrapping_mul(1_103_515_245).wrapping_add(12345) & 0x7fff_ffff;
        0x41 + ((x >> 16) % 23) as u8
    });
    license.iter().copied().chain(noise).chain(license.iter().copied()).collect()
}

#[test]
fn crunched_table_reset_mid_group() {
    // Made with ARC 5.21q `arc aw`. A CLEAR that does not end a group of eight
    // codes is followed by padding that the reader must skip.
    let mut archive = ArcArchive::new(Cursor::new(include_bytes!("arc/crunch_clear.arc"))).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(CompressionMethod::Crunched(8), entry.compression_method);
    assert_eq!(shifting_text(), archive.read(&entry).unwrap());
}

#[test]
fn squashed_uses_13_bit_codes() {
    // Made with ARC 5.21q `arc awq`. Squashing grows codes to 13 bits and fills
    // the table up to code 8191.
    let mut archive = ArcArchive::new(Cursor::new(include_bytes!("arc/squash_clear.arc"))).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    assert_eq!(CompressionMethod::Squashed, entry.compression_method);
    assert_eq!(shifting_text(), archive.read(&entry).unwrap());
}
