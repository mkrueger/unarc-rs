use std::{
    fs,
    path::Path,
    process::{Command, Output},
};

fn make_tar(path: &Path, name: &str, data: &[u8]) {
    let mut builder = tar::Builder::new(fs::File::create(path).unwrap());
    let mut header = tar::Header::new_ustar();
    header.set_path(name).unwrap();
    header.set_mode(0o644);
    header.set_size(data.len() as u64);
    header.set_cksum();
    builder.append(&header, data).unwrap();
    builder.finish().unwrap();
}

fn extract(archive: &Path, output: &Path, force: bool) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_unarc"));
    cmd.arg("extract").arg(archive).arg("-o").arg(output);
    if force {
        cmd.arg("--force");
    }
    cmd.output().unwrap()
}

#[test]
fn extracts_directory_type_without_trailing_slash_and_its_child() {
    let temp = tempfile::tempdir().unwrap();
    let archive = temp.path().join("directory.tar");
    let output = temp.path().join("output");
    let mut builder = tar::Builder::new(fs::File::create(&archive).unwrap());
    for (name, kind, data) in [
        ("dir", tar::EntryType::Directory, &b""[..]),
        ("dir/file.txt", tar::EntryType::Regular, &b"child data"[..]),
    ] {
        let mut header = tar::Header::new_ustar();
        header.set_path(name).unwrap();
        header.set_entry_type(kind);
        header.set_size(data.len() as u64);
        header.set_mode(0o755);
        header.set_cksum();
        builder.append(&header, data).unwrap();
    }
    builder.finish().unwrap();
    drop(builder);
    let result = extract(&archive, &output, false);
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    assert!(output.join("dir").is_dir());
    assert_eq!(fs::read(output.join("dir/file.txt")).unwrap(), b"child data");
}

#[test]
fn list_accepts_multibyte_filenames() {
    let temp = tempfile::tempdir().unwrap();
    let archive = temp.path().join("unicode.tar");
    make_tar(&archive, &"ä".repeat(30), b"content");
    let result = Command::new(env!("CARGO_BIN_EXE_unarc")).arg("list").arg(&archive).output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    assert!(String::from_utf8(result.stdout).unwrap().contains(&"ä".repeat(30)));
}

#[test]
fn nested_extraction_preserves_files_unless_forced() {
    let temp = tempfile::tempdir().unwrap();
    let archive = temp.path().join("input.tar");
    let output = temp.path().join("output");
    make_tar(&archive, "nested/file.txt", b"new");
    assert!(extract(&archive, &output, false).status.success());
    let file = output.join("nested/file.txt");
    assert_eq!(fs::read(&file).unwrap(), b"new");
    fs::write(&file, b"old").unwrap();
    assert!(extract(&archive, &output, false).status.success());
    assert_eq!(fs::read(&file).unwrap(), b"old");
    assert!(extract(&archive, &output, true).status.success());
    assert_eq!(fs::read(&file).unwrap(), b"new");
}

#[cfg(unix)]
#[test]
fn parent_symlink_cannot_escape_output() {
    for target_is_absolute in [false, true] {
        let temp = tempfile::tempdir().unwrap();
        let archive = temp.path().join("input.tar");
        let output = temp.path().join("output");
        let outside = temp.path().join("outside");
        fs::create_dir(&output).unwrap();
        fs::create_dir(&outside).unwrap();
        let target = if target_is_absolute { outside.clone() } else { "../outside".into() };
        std::os::unix::fs::symlink(target, output.join("redirect")).unwrap();
        make_tar(&archive, "redirect/child/file.txt", b"must not escape");
        for force in [false, true] {
            assert!(!extract(&archive, &output, force).status.success());
            assert!(!outside.join("child").exists());
        }
    }
}

#[cfg(unix)]
#[test]
fn force_replaces_symlink_without_touching_target() {
    let temp = tempfile::tempdir().unwrap();
    let archive = temp.path().join("input.tar");
    let output = temp.path().join("output");
    let target = temp.path().join("target.txt");
    fs::create_dir(&output).unwrap();
    fs::write(&target, b"original").unwrap();
    std::os::unix::fs::symlink(&target, output.join("file.txt")).unwrap();
    make_tar(&archive, "file.txt", b"new");
    assert!(extract(&archive, &output, false).status.success());
    assert_eq!(fs::read(&target).unwrap(), b"original");
    assert!(extract(&archive, &output, true).status.success());
    assert_eq!(fs::read(&target).unwrap(), b"original");
    assert_eq!(fs::read(output.join("file.txt")).unwrap(), b"new");
    assert!(!fs::symlink_metadata(output.join("file.txt")).unwrap().file_type().is_symlink());
}

#[cfg(unix)]
#[test]
fn dangling_symlink_is_not_followed_without_force() {
    let temp = tempfile::tempdir().unwrap();
    let archive = temp.path().join("input.tar");
    let output = temp.path().join("output");
    let target = temp.path().join("absent.txt");
    fs::create_dir(&output).unwrap();
    std::os::unix::fs::symlink(&target, output.join("file.txt")).unwrap();
    make_tar(&archive, "file.txt", b"new");
    assert!(extract(&archive, &output, false).status.success());
    assert!(!target.exists());
}

#[test]
fn force_replaces_hard_link_without_touching_target() {
    let temp = tempfile::tempdir().unwrap();
    let archive = temp.path().join("input.tar");
    let output = temp.path().join("output");
    let target = temp.path().join("target.txt");
    fs::create_dir(&output).unwrap();
    fs::write(&target, b"original").unwrap();
    fs::hard_link(&target, output.join("file.txt")).unwrap();
    make_tar(&archive, "file.txt", b"new");
    assert!(extract(&archive, &output, true).status.success());
    assert_eq!(fs::read(&target).unwrap(), b"original");
    assert_eq!(fs::read(output.join("file.txt")).unwrap(), b"new");
}
