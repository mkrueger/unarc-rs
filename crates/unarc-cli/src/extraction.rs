//! Filesystem access confined to the user-selected extraction directory.
//!
//! Archive paths must never be rejoined to an ambient filesystem path after
//! sanitizing. cap-std resolves paths relative to a directory handle and rejects
//! symlink escapes, including changes made between inspection and opening.
use cap_std::{
    ambient_authority,
    fs::{Dir, OpenOptions},
};
use std::{
    io::{self, Write},
    path::Path,
};

pub struct ExtractionRoot {
    dir: Dir,
}

impl ExtractionRoot {
    pub fn new(path: &Path) -> io::Result<Self> {
        // Only this user-supplied path may use ambient authority.
        std::fs::create_dir_all(path)?;
        Ok(Self {
            dir: Dir::open_ambient_dir(path, ambient_authority())?,
        })
    }

    pub fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        self.dir.create_dir_all(path)
    }

    pub fn exists(&self, path: &Path) -> io::Result<bool> {
        // Count dangling symlinks as existing too. This check is an optimization;
        // create_new below provides the actual no-overwrite guarantee.
        match self.dir.symlink_metadata(path) {
            Ok(_) => Ok(true),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Returns false if an existing entry was preserved without --force.
    pub fn write(&self, path: &Path, data: &[u8], force: bool) -> io::Result<bool> {
        let parent = path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
        self.dir.create_dir_all(parent)?;
        let parent = self.dir.open_dir(parent)?;
        let name = path
            .file_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing output filename"))?;
        if force {
            // Replace the directory entry, not the contents of its old inode.
            // This protects both symlink and hard-link targets and leaves the
            // previous file intact if writing the temporary file fails.
            let mut file = cap_tempfile::TempFile::new(&parent)?;
            file.write_all(data)?;
            file.replace(name)?;
        } else {
            let mut options = OpenOptions::new();
            options.write(true).create_new(true);
            let mut file = match parent.open_with(name, &options) {
                Ok(file) => file,
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => return Ok(false),
                Err(e) => return Err(e),
            };
            file.write_all(data)?;
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_file_created_after_the_exists_check_is_not_overwritten() {
        let temp = tempfile::tempdir().unwrap();
        let root = ExtractionRoot::new(temp.path()).unwrap();
        let path = Path::new("file.txt");
        assert!(!root.exists(path).unwrap());
        std::fs::write(temp.path().join(path), b"concurrent writer").unwrap();
        assert!(!root.write(path, b"archive data", false).unwrap());
        assert_eq!(std::fs::read(temp.path().join(path)).unwrap(), b"concurrent writer");
    }

    #[cfg(unix)]
    #[test]
    fn directory_creation_cannot_follow_an_escaping_symlink() {
        let temp = tempfile::tempdir().unwrap();
        let root_path = temp.path().join("out");
        let root = ExtractionRoot::new(&root_path).unwrap();
        std::os::unix::fs::symlink("..", root_path.join("escape")).unwrap();
        assert!(root.create_dir_all(Path::new("escape/new-directory")).is_err());
        assert!(!temp.path().join("new-directory").exists());
    }
}
