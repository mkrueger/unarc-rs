use std::io::Cursor;
use unarc_rs::uc2::uc2_archive::Uc2Archive;

fn main() {
    let file = Cursor::new(include_bytes!("../tests/uc2/normal.uc2"));
    let mut archive = Uc2Archive::new(file).unwrap();

    let entry = archive.get_next_entry().unwrap().unwrap();
    println!(
        "Entry: {}, method: {}, size: {}",
        entry.name, entry.compress_info.method, entry.original_size
    );

    let result = archive.read(&entry).unwrap();
    println!("Decompressed {} bytes", result.len());
    println!("First 100 bytes: {:?}", &result[0..100.min(result.len())]);

    // Check if all zeros
    let all_zeros = result.iter().all(|&b| b == 0);
    println!("All zeros: {}", all_zeros);

    // Count non-zero bytes
    let non_zero_count = result.iter().filter(|&&b| b != 0).count();
    println!("Non-zero bytes: {}", non_zero_count);
}
