use std::{
    fs,
    io::{Cursor, Write},
    path::Path,
    process::Command,
};

fn zip_bytes() -> Vec<u8> {
    let mut archive = zip::ZipWriter::new(Cursor::new(Vec::new()));
    archive
        .start_file(
            "file.txt",
            zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored),
        )
        .unwrap();
    archive.write_all(b"archive contents").unwrap();
    archive.finish().unwrap().into_inner()
}

fn check_archive(dir: &Path, name: &str, expected_volumes: Option<usize>) {
    // A bare relative archive name also exercises read_dir(".") discovery.
    let result = Command::new(env!("CARGO_BIN_EXE_unarc"))
        .current_dir(dir)
        .arg("extract")
        .arg(name)
        .args(["-o", "output"])
        .output()
        .unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    let stdout = String::from_utf8(result.stdout).unwrap();
    if let Some(count) = expected_volumes {
        assert!(stdout.contains(&format!("Detected {count} volumes")), "{stdout}");
    } else {
        assert!(!stdout.contains("Detected"), "{stdout}");
    }
    assert_eq!(fs::read(dir.join("output/file.txt")).unwrap(), b"archive contents");
}

#[test]
fn independent_zip_with_same_prefix_is_not_a_volume() {
    let temp = tempfile::tempdir().unwrap();
    let bytes = zip_bytes();
    fs::write(temp.path().join("backup.zip"), &bytes).unwrap();
    fs::write(temp.path().join("backup-other.zip"), &bytes).unwrap();
    check_archive(temp.path(), "backup.zip", None);
}

#[test]
fn split_zip_discovery_uses_exact_names_and_correct_order() {
    let bytes = zip_bytes();
    let split = bytes.len() / 2;
    for names in [["backup.001", "backup.002"], ["backup.z01", "backup.zip"]] {
        let temp = tempfile::tempdir().unwrap();
        fs::write(temp.path().join(names[0]), &bytes[..split]).unwrap();
        fs::write(temp.path().join(names[1]), &bytes[split..]).unwrap();
        fs::write(temp.path().join(names[0].replace("backup", "backup-other")), b"unrelated").unwrap();
        check_archive(temp.path(), names[0], Some(2));
    }
}
