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
    // Volume discovery is reported on stderr, keeping stdout for output
    let stderr = String::from_utf8(result.stderr).unwrap();
    if let Some(count) = expected_volumes {
        assert!(stderr.contains(&format!("Detected {count} volumes")), "{stderr}");
    } else {
        assert!(!stderr.contains("Detected"), "{stderr}");
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

#[test]
fn split_zip_json_listing_is_not_mixed_with_progress() {
    let bytes = zip_bytes();
    let split = bytes.len() / 2;
    let temp = tempfile::tempdir().unwrap();
    fs::write(temp.path().join("backup.001"), &bytes[..split]).unwrap();
    fs::write(temp.path().join("backup.002"), &bytes[split..]).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_unarc"))
        .current_dir(temp.path())
        .args(["list", "--json", "backup.001"])
        .output()
        .unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    let listing: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(listing["entries"][0]["name"], "file.txt");
}
