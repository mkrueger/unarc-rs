use std::fs;

use unarc_rs::zoo::zoo_archive::ZooArchieve;

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let mut archieve = ZooArchieve::new(fs::File::open(path).unwrap()).unwrap();

    while let Ok(Some(header)) = archieve.get_next_entry() {
        println!("Extract {}...", header.name);
        let buffer = archieve.read(&header).unwrap();
        fs::write(header.name, buffer).unwrap();
    }
}
