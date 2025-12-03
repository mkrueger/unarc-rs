use std::fs;

use unarc_rs::arj::arj_archive::ArjArchive;

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let mut archive = ArjArchive::new(fs::File::open(path).unwrap()).unwrap();

    while let Ok(Some(header)) = archive.get_next_entry() {
        println!("Extract {}...", header.name);
        let buffer = archive.read(&header).unwrap();
        fs::write(header.name, buffer).unwrap();
    }
}
