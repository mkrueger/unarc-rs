//! `list --json` and extracting named entries: the interface other programs drive.
use std::{
    fs,
    io::Write,
    path::Path,
    process::{Command, Output},
};

#[path = "../../unarc-rs/tests/common/ace.rs"]
mod ace;

fn unarc(args: &[&std::ffi::OsStr]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_unarc")).args(args).output().unwrap()
}

fn make_tar(path: &Path, members: &[(&str, tar::EntryType, &[u8])]) {
    let mut builder = tar::Builder::new(fs::File::create(path).unwrap());
    for (name, kind, data) in members {
        let mut header = tar::Header::new_gnu();
        header.set_entry_type(*kind);
        header.set_size(data.len() as u64);
        header.set_mode(0o644);
        if *kind == tar::EntryType::Symlink {
            builder.append_link(&mut header, name, "target.txt").unwrap();
        } else if name.len() <= 100 {
            // Write raw TAR names rather than host-dependent filesystem paths.
            header.as_mut_bytes()[..name.len()].copy_from_slice(name.as_bytes());
            header.set_cksum();
            builder.append(&header, *data).unwrap();
        } else {
            builder.append_data(&mut header, name, *data).unwrap();
        }
    }
    builder.finish().unwrap();
}

fn list_json(archive: &Path) -> serde_json::Value {
    let result = unarc(&["list".as_ref(), "--json".as_ref(), archive.as_os_str()]);
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    serde_json::from_slice(&result.stdout).expect("stdout is a single JSON document")
}

#[test]
fn json_listing_keeps_full_names_and_entry_kinds() {
    let temp = tempfile::tempdir().unwrap();
    let archive = temp.path().join("listing.tar");
    let long_name = format!("dir/{}.txt", "ä".repeat(60));
    let awkward = "dir/a \"quoted\" back\\slash.txt";
    make_tar(
        &archive,
        &[
            ("dir/", tar::EntryType::Directory, b""),
            (&long_name, tar::EntryType::Regular, b"long"),
            (awkward, tar::EntryType::Regular, b"awkward data"),
            ("dir/link", tar::EntryType::Symlink, b""),
        ],
    );

    let listing = list_json(&archive);
    assert_eq!(listing["format"], "TAR");
    let entries = listing["entries"].as_array().unwrap();
    let names: Vec<_> = entries.iter().map(|e| e["name"].as_str().unwrap()).collect();
    assert_eq!(names, ["dir/", long_name.as_str(), awkward, "dir/link"]);

    let kinds: Vec<_> = entries.iter().map(|e| e["kind"].as_str().unwrap()).collect();
    assert_eq!(kinds, ["directory", "file", "file", "symlink"]);

    assert_eq!(entries[2]["size"], 12);
    assert_eq!(entries[2]["encrypted"], false);
    assert!(entries[2]["encryption"].is_null());
    assert_eq!(entries[3]["link_target"], "target.txt");
}

#[test]
fn json_listing_reports_modified_time() {
    let temp = tempfile::tempdir().unwrap();
    let archive = temp.path().join("dated.zip");
    let mut writer = zip::ZipWriter::new(fs::File::create(&archive).unwrap());
    let modified = zip::DateTime::from_date_and_time(1994, 3, 12, 10, 22, 30).unwrap();
    let options = zip::write::SimpleFileOptions::default().last_modified_time(modified);
    writer.start_file("FILE_ID.DIZ", options).unwrap();
    writer.write_all(b"a fine release").unwrap();
    writer.finish().unwrap();

    let listing = list_json(&archive);
    let entry = &listing["entries"][0];
    assert_eq!(entry["name"], "FILE_ID.DIZ");
    assert_eq!(entry["modified"], "1994-03-12T10:22:30");
    assert_eq!(entry["size"], 14);
}

#[test]
fn extracts_only_the_named_entries() {
    let temp = tempfile::tempdir().unwrap();
    let archive = temp.path().join("release.tar");
    let output = temp.path().join("output");
    make_tar(
        &archive,
        &[
            ("a.txt", tar::EntryType::Regular, b"first"),
            ("sub/FILE_ID.DIZ", tar::EntryType::Regular, b"description"),
            ("b.txt", tar::EntryType::Regular, b"second"),
            ("c.txt", tar::EntryType::Regular, b"third"),
        ],
    );

    let result = unarc(&[
        "extract".as_ref(),
        archive.as_os_str(),
        "sub/FILE_ID.DIZ".as_ref(),
        "b.txt".as_ref(),
        "-o".as_ref(),
        output.as_os_str(),
    ]);
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    assert_eq!(fs::read(output.join("sub/FILE_ID.DIZ")).unwrap(), b"description");
    assert_eq!(fs::read(output.join("b.txt")).unwrap(), b"second");
    assert!(!output.join("a.txt").exists());
    assert!(!output.join("c.txt").exists());
}

#[test]
fn selective_extraction_decodes_solid_ace_predecessors() {
    for stored_prefix in [false, true] {
        for existing_prefix in [false, true] {
            let temp = tempfile::tempdir().unwrap();
            let archive = temp.path().join("solid.ace");
            let output = temp.path().join("output");
            fs::write(&archive, ace::synthetic_ace(true, stored_prefix)).unwrap();
            let mut args = vec!["extract".as_ref(), archive.as_os_str()];
            if existing_prefix {
                fs::create_dir(&output).unwrap();
                fs::write(output.join("first.txt"), b"keep").unwrap();
            } else {
                args.push("second.txt".as_ref());
            }
            args.extend(["-o".as_ref(), output.as_os_str()]);
            let result = unarc(&args);
            assert!(result.status.success(), "{result:?}");
            assert_eq!(fs::read(output.join("second.txt")).unwrap(), b"AA");
            if existing_prefix {
                assert_eq!(fs::read(output.join("first.txt")).unwrap(), b"keep");
            } else {
                assert!(!output.join("first.txt").exists());
            }
        }
    }
}

#[test]
fn selective_ace_extraction_uses_password_only_when_history_requires_it() {
    for solid in [false, true] {
        for password in [None, Some("wrong"), Some("test")] {
            let temp = tempfile::tempdir().unwrap();
            let archive = temp.path().join("encrypted.ace");
            let output = temp.path().join("output");
            fs::write(&archive, ace::synthetic_encrypted_ace(solid)).unwrap();
            assert_eq!(list_json(&archive)["entries"].as_array().unwrap().len(), 2);
            let mut args = vec!["extract".as_ref(), archive.as_os_str(), "second.txt".as_ref()];
            if let Some(password) = password {
                args.push("-p".as_ref());
                args.push(password.as_ref());
            }
            args.extend(["-o".as_ref(), output.as_os_str()]);
            let result = unarc(&args);
            if solid && password != Some("test") {
                assert!(!result.status.success(), "{result:?}");
                assert!(!output.join("second.txt").exists());
                assert!(!result.stderr.is_empty());
            } else {
                assert!(result.status.success(), "{result:?}");
                assert_eq!(fs::read(output.join("second.txt")).unwrap(), if solid { &b"AA"[..] } else { &b"B"[..] });
            }
            assert!(!output.join("first.txt").exists());
        }
    }
}

#[test]
fn repeated_requested_names_are_extracted_once() {
    let temp = tempfile::tempdir().unwrap();
    let archive = temp.path().join("release.tar");
    let output = temp.path().join("output");
    make_tar(&archive, &[("a.txt", tar::EntryType::Regular, b"first")]);
    let result = unarc(&[
        "extract".as_ref(),
        archive.as_os_str(),
        "a.txt".as_ref(),
        "a.txt".as_ref(),
        "-o".as_ref(),
        output.as_os_str(),
    ]);
    assert!(result.status.success(), "{result:?}");
    assert_eq!(fs::read(output.join("a.txt")).unwrap(), b"first");
    assert!(String::from_utf8_lossy(&result.stdout).contains("Extracted 1 file(s)"));
}

#[test]
fn single_stream_listings_preserve_archive_derived_names() {
    for (extension, data) in [
        ("xz", &include_bytes!("../../unarc-rs/tests/xz/LICENSE.xz")[..]),
        ("zst", &include_bytes!("../../unarc-rs/tests/zst/LICENSE.zst")[..]),
    ] {
        let temp = tempfile::tempdir().unwrap();
        let archive = temp.path().join(format!("renamed.txt.{extension}"));
        fs::write(&archive, data).unwrap();
        assert_eq!(list_json(&archive)["entries"][0]["name"], "renamed.txt");
        let result = unarc(&["list".as_ref(), archive.as_os_str()]);
        assert!(result.status.success(), "{result:?}");
        assert!(String::from_utf8_lossy(&result.stdout).contains("renamed.txt"));
    }
}

#[test]
fn a_missing_name_fails_but_the_rest_are_extracted() {
    let temp = tempfile::tempdir().unwrap();
    let archive = temp.path().join("release.tar");
    let output = temp.path().join("output");
    make_tar(&archive, &[("a.txt", tar::EntryType::Regular, b"first")]);

    let result = unarc(&[
        "extract".as_ref(),
        archive.as_os_str(),
        "a.txt".as_ref(),
        "README.1ST".as_ref(),
        "README.1ST".as_ref(),
        "-o".as_ref(),
        output.as_os_str(),
    ]);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("README.1ST not found"));
    assert_eq!(String::from_utf8_lossy(&result.stderr).matches("README.1ST not found").count(), 1);
    assert_eq!(fs::read(output.join("a.txt")).unwrap(), b"first");
}
