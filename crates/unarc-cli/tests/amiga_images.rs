use std::fs;
use std::process::Command;

const TEXT: &[u8] = include_bytes!("../../unarc-rs/tests/amiga/payload.txt");

#[test]
fn lists_and_selectively_extracts_adf_and_rdb_hdf() {
    for (extension, image, wanted) in [
        ("adf", &include_bytes!("../../unarc-rs/tests/amiga/ofs.adf")[..], "docs/readme.txt"),
        ("hdf", &include_bytes!("../../unarc-rs/tests/amiga/partitions.hdf")[..], "DH1/docs/readme.txt"),
    ] {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join(format!("disk.{extension}"));
        let output = temp.path().join("output");
        fs::write(&path, image).unwrap();
        let listing = Command::new(env!("CARGO_BIN_EXE_unarc")).args(["list", "--json"]).arg(&path).output().unwrap();
        assert!(listing.status.success(), "{listing:?}");
        let json: serde_json::Value = serde_json::from_slice(&listing.stdout).unwrap();
        let entries = json["entries"].as_array().unwrap();
        let entry = entries.iter().find(|entry| entry["name"] == wanted).unwrap();
        assert_eq!(entry["kind"], "file");
        assert_eq!(entry["size"], TEXT.len());
        assert_eq!(entry["encrypted"], false);
        let extraction = Command::new(env!("CARGO_BIN_EXE_unarc"))
            .arg("extract")
            .arg(&path)
            .arg(wanted)
            .arg("-o")
            .arg(&output)
            .output()
            .unwrap();
        assert!(extraction.status.success(), "{extraction:?}");
        assert_eq!(fs::read(output.join(wanted)).unwrap(), TEXT);
        assert!(!output.join("empty").exists());
        assert!(!output.join("DH0").exists());
        assert!(!output.join("DH1/empty").exists());
        assert!(!output.join("docs/large.bin").exists());
    }
}

#[test]
fn unsupported_amiga_filesystem_is_an_explicit_cli_error() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("disk.adf");
    let mut image = include_bytes!("../../unarc-rs/tests/amiga/ofs.adf").to_vec();
    image[3] = 7;
    fs::write(&path, image).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_unarc")).args(["list", "--json"]).arg(&path).output().unwrap();
    assert!(!result.status.success());
    assert!(result.stdout.is_empty());
    assert!(String::from_utf8_lossy(&result.stderr).contains("only DOS0 through DOS5"));
}
