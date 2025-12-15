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

    let result = archive.read(&entry).unwrap();
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
fn extract_39b0aefdSQZ() {
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
fn extract_f5f09v09SQZ() {
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
fn extract_TPLZH025SQZ() {
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