use std::fs;

use unarc_rs::zoo::zoo_archive::ZooArchive;

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let mut archive = ZooArchive::new(fs::File::open(path).unwrap()).unwrap();

    println!("Name            Size            Compression  DateTime modified");
    println!("---------------------------------------------------");
    while let Ok(Some(header)) = archive.get_next_entry() {
        println!(
            "{:<15}\t{:<7}\t\t{:?} {}-{}-{} {}:{}:{}",
            header.name,
            header.org_size,
            header.compression_method,
            header.date_time.year(),
            header.date_time.month(),
            header.date_time.day(),
            header.date_time.hour(),
            header.date_time.minute(),
            header.date_time.second(),
        );
        archive.skip(&header).unwrap();
    }
}
