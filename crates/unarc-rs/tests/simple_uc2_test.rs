use std::io::Cursor;

#[test]
fn test_include_bytes_loads_file() {
    let data = include_bytes!("uc2/normal.uc2");
    eprintln!("📦 include_bytes!() loaded {} bytes", data.len());
    eprintln!("📦 First 16 bytes: {:02x?}", &data[..16.min(data.len())]);
    assert!(!data.is_empty(), "include_bytes! should load the file");
    assert_eq!(&data[0..4], b"UC2\x1a", "Should start with UC2 signature");
}

#[test]
fn test_cursor_reads_all_bytes() {
    let data = include_bytes!("uc2/normal.uc2");
    let cursor = Cursor::new(data);
    eprintln!("📦 Cursor position: {}", cursor.position());
    eprintln!("📦 Cursor len: {}", cursor.get_ref().len());
}

#[test]
fn test_archive_new() {
    use unarc_rs::uc2::uc2_archive::Uc2Archive;

    let data = include_bytes!("uc2/normal.uc2");
    eprintln!("📦 Creating archive from {} bytes", data.len());
    let cursor = Cursor::new(data);
    let result = Uc2Archive::new(cursor);
    eprintln!("📦 Uc2Archive::new() result: {:?}", result.is_ok());
    let _archive = result.unwrap();
    eprintln!("✅ Archive created successfully");
}

#[test]
fn test_get_next_entry() {
    use unarc_rs::uc2::uc2_archive::Uc2Archive;

    let data = include_bytes!("uc2/normal.uc2");
    let cursor = Cursor::new(data);
    let mut archive = Uc2Archive::new(cursor).unwrap();

    eprintln!("📦 Calling get_next_entry()");
    let entry_result = archive.get_next_entry();
    eprintln!("📦 get_next_entry() result: {:?}", entry_result.is_ok());

    if let Ok(Some(entry)) = entry_result {
        eprintln!(
            "✅ Got entry: name={}, original_size={}, compressed_length={}",
            entry.name, entry.original_size, entry.compress_info.compressed_length
        );
    } else {
        eprintln!("❌ get_next_entry() returned: {:?}", entry_result);
    }
}

#[test]
fn test_extract_file() {
    use unarc_rs::uc2::uc2_archive::Uc2Archive;

    let data = include_bytes!("uc2/normal.uc2");
    let cursor = Cursor::new(data);
    let mut archive = Uc2Archive::new(cursor).unwrap();

    let entry = archive.get_next_entry().unwrap().unwrap();
    eprintln!("📦 Entry: name={}", entry.name);

    eprintln!("📦 Calling archive.read()");
    let read_result = archive.read(&entry);
    eprintln!("📦 archive.read() result: {}", read_result.is_ok());

    if let Ok(data) = read_result {
        eprintln!("✅ Successfully decompressed {} bytes", data.len());
        let expected_size = include_bytes!("../../../LICENSE").len();
        eprintln!("✅ Expected size: {} bytes", expected_size);
        if data.len() == expected_size {
            eprintln!("✅ Size matches!");
            if data == include_bytes!("../../../LICENSE") {
                eprintln!("✅✅✅ CONTENT MATCHES!!!!");
            } else {
                eprintln!("❌ Content does not match");
            }
        } else {
            eprintln!("❌ Size mismatch: got {} expected {}", data.len(), expected_size);
        }
    } else {
        eprintln!("❌ archive.read() failed: {:?}", read_result);
    }
}
