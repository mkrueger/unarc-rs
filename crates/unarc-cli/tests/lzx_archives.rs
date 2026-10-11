use std::fs;
use std::process::Command;

const MULTI: &[u8] = include_bytes!("../../unarc-rs/tests/lzx/multi.lzx");

#[test]
fn lists_and_selectively_extracts_merged_lzx_entries() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("multi.lzx");
    let output = temp.path().join("output");
    fs::write(&path, MULTI).unwrap();

    let listing = Command::new(env!("CARGO_BIN_EXE_unarc")).args(["list", "--json"]).arg(&path).output().unwrap();
    assert!(listing.status.success(), "{listing:?}");
    let json: serde_json::Value = serde_json::from_slice(&listing.stdout).unwrap();
    let entries = json["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 8);
    let entry = |name: &str| entries.iter().find(|entry| entry["name"] == name).unwrap();
    assert_eq!(entry("docs/")["kind"], "directory");
    assert_eq!(entry("docs/binary.dat")["method"], "LZX (merged)");
    assert_eq!(entry("docs/binary.dat")["compressed_size"], 0);
    assert_eq!(entry("café.txt")["method"], "LZX");

    // Both requested files follow skipped entries; the first sits behind two members of its merged group.
    let extraction = Command::new(env!("CARGO_BIN_EXE_unarc"))
        .arg("extract")
        .arg(&path)
        .args(["docs/binary.dat", "café.txt", "-o"])
        .arg(&output)
        .output()
        .unwrap();
    assert!(extraction.status.success(), "{extraction:?}");
    assert_eq!(fs::read(output.join("docs/binary.dat")).unwrap(), (0..=255u8).collect::<Vec<_>>().repeat(160));
    assert_eq!(fs::read(output.join("café.txt")).unwrap(), b"Latin-1 name\n".repeat(20));
    assert!(!output.join("README.TXT").exists());
    assert!(!output.join("docs/LICENSE").exists());
    assert!(!output.join("docs/words.txt").exists());
}
