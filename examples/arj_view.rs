use std::fs;

use unarc_rs::arj::arj_archive::ArjArchieve;

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let mut archieve = ArjArchieve::new(fs::File::open(path).unwrap()).unwrap();
    println!(
        "Arj file: {} created at {}-{}-{} {}:{}:{}",
        archieve.get_name(),
        archieve.get_creation_date_time().year(),
        archieve.get_creation_date_time().month(),
        archieve.get_creation_date_time().day(),
        archieve.get_creation_date_time().hour(),
        archieve.get_creation_date_time().minute(),
        archieve.get_creation_date_time().second(),
    );
    println!("Name            Size            Compression   DateTime modified");
    println!("---------------------------------------------------");
    while let Ok(Some(header)) = archieve.get_next_entry() {
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
        archieve.skip(&header).unwrap();
    }
}
