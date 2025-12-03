use std::fs;

use unarc_rs::arc::arc_archive::ArcArchive;

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let mut archive = ArcArchive::new(fs::File::open(path).unwrap()).unwrap();

    while let Ok(Some(header)) = archive.get_next_entry() {
        println!("Extract {}...", header.name);
        let buffer = archive.read(&header).unwrap();
        fs::write(header.name, buffer).unwrap();
    }
}
