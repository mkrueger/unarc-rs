//! Extract legacy JAR archives into a supplied output directory.
use std::{
    fs,
    io::Cursor,
    path::{Component, Path},
};
use unarc_rs::jar::JarArchive;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let data = fs::read(&args[1])?;
    let mut archive = JarArchive::new(Cursor::new(data))?;
    for (entry, data) in archive.extract_all()? {
        println!("{} {} {:08x}", entry.name, data.len(), entry.crc32);
        if let Some(root) = args.get(2) {
            let relative = Path::new(&entry.name);
            if relative.components().any(|c| !matches!(c, Component::Normal(_))) {
                return Err("Unsafe archive path".into());
            }
            let path = Path::new(root).join(relative);
            if entry.is_directory {
                fs::create_dir_all(path)?;
            } else {
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(path, data)?;
            }
        }
    }
    Ok(())
}
