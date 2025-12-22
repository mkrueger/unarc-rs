use std::fs;

use unarc_rs::arj::arj_archive::ArjArchive;

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let mut archive = ArjArchive::new(fs::File::open(path).unwrap()).unwrap();
    println!(
        "Arj file: {} created at {}-{}-{} {}:{}:{}",
        archive.get_name(),
        archive.get_creation_date_time().year(),
        archive.get_creation_date_time().month(),
        archive.get_creation_date_time().day(),
        archive.get_creation_date_time().hour(),
        archive.get_creation_date_time().minute(),
        archive.get_creation_date_time().second(),
    );
    println!("Name            Size            Compression   DateTime modified");
    println!("---------------------------------------------------");
    while let Ok(Some(header)) = archive.get_next_entry() {
        println!(
            "{:<15}\t{:<7}\t\t{:?}  {}-{}-{} {}:{}:{}",
            header.name,
            header.original_size,
            header.compression_method,
            header.date_time_modified.year(),
            header.date_time_modified.month(),
            header.date_time_modified.day(),
            header.date_time_modified.hour(),
            header.date_time_modified.minute(),
            header.date_time_modified.second(),
        );
        archive.skip(&header).unwrap();
    }
}
