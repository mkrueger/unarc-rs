//! `unarc` on DMS archives.
use std::fs;
use std::process::Command;

const NDOS: &[u8] = include_bytes!("../../unarc-rs/tests/dms/ndos.dms");
const MODES: &[u8] = include_bytes!("../../unarc-rs/tests/dms/modes.dms");
const LICENSE: &[u8] = include_bytes!("../../../LICENSE");

fn list(path: &std::path::Path) -> Vec<(String, String)> {
    let listing = Command::new(env!("CARGO_BIN_EXE_unarc")).args(["list", "--json"]).arg(path).output().unwrap();
    assert!(listing.status.success(), "{listing:?}");
    let json: serde_json::Value = serde_json::from_slice(&listing.stdout).unwrap();
    json["entries"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| (entry["name"].as_str().unwrap().to_string(), entry["kind"].as_str().unwrap().to_string()))
        .collect()
}

#[test]
fn lists_and_extracts_disk_files_images_and_attached_texts() {
    let temp = tempfile::tempdir().unwrap();
    let ndos = temp.path().join("game.dms");
    fs::write(&ndos, NDOS).unwrap();
    let modes = temp.path().join("modes.DMS");
    fs::write(&modes, MODES).unwrap();

    // A non-AmigaDOS disk is one image named after the archive.
    assert_eq!(list(&ndos), [("game.adf".into(), "file".into()), ("FILE_ID.DIZ".into(), "file".into())]);
    assert_eq!(list(&modes).len(), 7);

    let output = temp.path().join("output");
    for (path, wanted) in [(&ndos, "game.adf"), (&ndos, "FILE_ID.DIZ"), (&modes, "LICENSE")] {
        let extraction = Command::new(env!("CARGO_BIN_EXE_unarc"))
            .arg("extract")
            .arg(path)
            .arg(wanted)
            .arg("-o")
            .arg(&output)
            .output()
            .unwrap();
        assert!(extraction.status.success(), "{extraction:?}");
    }
    assert_eq!(fs::read(output.join("game.adf")).unwrap().len(), 80 * 11 * 1024);
    assert!(fs::read(output.join("FILE_ID.DIZ")).unwrap().starts_with(b"unarc-rs DMS fixture\n"));
    assert_eq!(fs::read(output.join("LICENSE")).unwrap(), LICENSE);
    assert!(!output.join("docs").exists());
}
