//! `unarc` on FAT12 floppy images.
use std::fs;
use std::process::Command;

#[path = "../../unarc-rs/tests/common/fat.rs"]
mod common;

const TEXT: &[u8] = include_bytes!("../../unarc-rs/tests/fat/payload.txt");

#[test]
fn st_extension_selects_atari_names_with_pc_boot_code() {
    use unarc_rs::unified::{ArchiveFormat, ArchiveOptions};

    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("disk.ST");
    let mut image = common::blank(common::ATARI720, false);
    let name = [0x9E, 0xB0, 0xC2, 0xFE, b' ', b' ', b' ', b' ', b'T', b'X', b'T'];
    let mut entry = common::entry(&name, 0x20, 2, 1);
    entry[12] = 0x18;
    common::put_entries(&mut image, common::ATARI720.root_offset(), &[entry]);
    common::chain(&mut image, common::ATARI720, &[2]);
    image[common::ATARI720.cluster_offset(2)] = 42;
    fs::write(&path, image).unwrap();
    for mut archive in [
        ArchiveFormat::open_path(&path).unwrap(),
        ArchiveFormat::open_path_with_options(&path, ArchiveOptions::new()).unwrap(),
    ] {
        assert_eq!(archive.next_entry().unwrap().unwrap().name(), "ßãא³.TXT");
    }
    let mut pc = ArchiveFormat::open_path_with_options(&path, ArchiveOptions::new().with_fat_atari_names(false)).unwrap();
    assert_eq!(pc.next_entry().unwrap().unwrap().name(), "₧░┬■.txt");
    let listing = Command::new(env!("CARGO_BIN_EXE_unarc")).args(["list", "--json"]).arg(&path).output().unwrap();
    assert!(listing.status.success(), "{listing:?}");
    let json: serde_json::Value = serde_json::from_slice(&listing.stdout).unwrap();
    assert_eq!(json["entries"][0]["name"], "ßãא³.TXT");
    let output = temp.path().join("output");
    let extraction = Command::new(env!("CARGO_BIN_EXE_unarc"))
        .arg("extract")
        .arg(&path)
        .arg("ßãא³.TXT")
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(extraction.status.success(), "{extraction:?}");
    assert_eq!(fs::read(output.join("ßãא³.TXT")).unwrap(), [42]);
}

#[test]
fn lists_and_selectively_extracts_pc_and_atari_images() {
    for (file, image, wanted) in [
        (
            "disk.img",
            &include_bytes!("../../unarc-rs/tests/fat/pc720.img")[..],
            "DOCS/NESTED/A long file name.txt",
        ),
        ("disk.st", &include_bytes!("../../unarc-rs/tests/fat/atari.st")[..], "AUTO/README.TXT"),
    ] {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join(file);
        let output = temp.path().join("output");
        fs::write(&path, image).unwrap();
        let listing = Command::new(env!("CARGO_BIN_EXE_unarc")).args(["list", "--json"]).arg(&path).output().unwrap();
        assert!(listing.status.success(), "{listing:?}");
        let json: serde_json::Value = serde_json::from_slice(&listing.stdout).unwrap();
        assert_eq!(json["format"], "FAT12 (PC/Atari ST floppy image)");
        let entries = json["entries"].as_array().unwrap();
        let entry = entries.iter().find(|entry| entry["name"] == wanted).unwrap();
        assert_eq!(entry["kind"], "file");
        assert_eq!(entry["size"], TEXT.len());
        assert_eq!(entry["modified"], "1994-03-12T10:22:30");
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
        assert!(!output.join("BIG.BIN").exists());
    }
}

#[test]
fn fat16_images_are_an_explicit_cli_error() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("hard.img");
    let mut image = include_bytes!("../../unarc-rs/tests/fat/pc360.img").to_vec();
    // 40000 single-sector clusters: FAT16 territory.
    image[13] = 1;
    image[19..21].copy_from_slice(&40_000u16.to_le_bytes());
    fs::write(&path, image).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_unarc")).args(["list", "--json"]).arg(&path).output().unwrap();
    assert!(!result.status.success());
    assert!(result.stdout.is_empty());
    assert!(String::from_utf8_lossy(&result.stderr).contains("FAT16/FAT32"), "{result:?}");
}
