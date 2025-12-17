use std::io::Cursor;

use unarc_rs::sqz::{file_header::CompressionMethod, sqz_archive::SqzArchive};

#[test]
fn extract_stored() {
    let file = Cursor::new(include_bytes!("sqz/store.sqz"));
    let mut archive = SqzArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();
    //   assert_eq!("license", entry.name);
    assert_eq!(CompressionMethod::Stored, entry.compression_method);
    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}

#[test]
fn extract_m1() {
    let file = Cursor::new(include_bytes!("sqz/license_m1.sqz"));
    let mut archive = SqzArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();

    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}

#[test]
fn extract_m2() {
    let file = Cursor::new(include_bytes!("sqz/license_m2.sqz"));
    let mut archive = SqzArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();

    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}

#[test]
fn extract_m3() {
    let file = Cursor::new(include_bytes!("sqz/license_m3.sqz"));
    let mut archive = SqzArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();

    eprintln!(
        "license_m3: original_size={}, compressed_size={}, method={}",
        entry.original_size, entry.compressed_size, entry.method
    );
    let result = archive.read(&entry).unwrap();
    eprintln!(
        "Decompressed {} bytes, expected {}",
        result.len(),
        entry.original_size
    );
    eprintln!(
        "Expected LICENSE size: {}",
        include_bytes!("../LICENSE").len()
    );
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}

#[test]
fn extract_m4() {
    let file = Cursor::new(include_bytes!("sqz/license_m4.sqz"));
    let mut archive = SqzArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();

    let result = archive.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}

#[test]
fn extract_m3_bug() {
    let file = Cursor::new(include_bytes!("sqz/T3.SQZ"));
    let mut archive = SqzArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();

    eprintln!(
        "T3.SQZ: original_size={}, compressed_size={}, method={}",
        entry.original_size, entry.compressed_size, entry.method
    );
    archive.read(&entry).unwrap();
}

#[test]
fn extract_m4_bug() {
    let file = Cursor::new(include_bytes!("sqz/T4.SQZ"));
    let mut archive = SqzArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();

    eprintln!(
        "T3.SQZ: original_size={}, compressed_size={}, method={}",
        entry.original_size, entry.compressed_size, entry.method
    );
    archive.read(&entry).unwrap();
}

/*

#[test]
fn extract_bug() {
    let file = Cursor::new(include_bytes!("sqz/T3.SQZ"));
    let mut archive = SqzArchive::new(file).unwrap();
    let entry = archive.get_next_entry().unwrap().unwrap();

    archive.read(&entry).unwrap();
}
*/

#[test]
fn extract_39b0aefd_sqz() {
    let file = Cursor::new(include_bytes!("sqz/39b0aefd.SQZ"));
    let mut archive = SqzArchive::new(file).unwrap();

    while let Ok(entry) = archive.get_next_entry() {
        if let Some(entry) = entry {
            archive.read(&entry).unwrap();
        } else {
            break;
        }
    }
}

#[test]
fn extract_f5f09v09_sqz() {
    let file = Cursor::new(include_bytes!("sqz/f5f09v09.SQZ"));
    let mut archive = SqzArchive::new(file).unwrap();

    while let Ok(entry) = archive.get_next_entry() {
        if let Some(entry) = entry {
            archive.read(&entry).unwrap();
        } else {
            break;
        }
    }
}

#[test]
fn extract_tplzh025_sqz() {
    let file = Cursor::new(include_bytes!("sqz/TPLZH025.SQZ"));
    let mut archive = SqzArchive::new(file).unwrap();

    while let Ok(entry) = archive.get_next_entry() {
        if let Some(entry) = entry {
            archive.read(&entry).unwrap();
        } else {
            break;
        }
    }
}
