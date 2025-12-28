//! Example: Extract files from any supported archive format using the unified API
//!
//! Usage: cargo run --example unified_extract <archive_file> [output_dir]

use std::env;
use std::fs::{self, File};
use std::path::Path;
use unarc_rs::error::ArchiveError;
use unarc_rs::unified::{ArchiveFormat, UnifiedArchive};

fn main() -> Result<(), ArchiveError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <archive_file> [output_dir]", args[0]);
        eprintln!();
        eprintln!("Supported formats:");
        for format in ArchiveFormat::ALL {
            eprintln!("  {} - extensions: {:?}", format.name(), format.extensions());
        }
        std::process::exit(1);
    }

    let archive_path = Path::new(&args[1]);
    let output_dir = if args.len() > 2 {
        Path::new(&args[2]).to_path_buf()
    } else {
        Path::new(".").to_path_buf()
    };

    // Detect format from extension
    let format = ArchiveFormat::from_path(archive_path)
        .ok_or_else(|| ArchiveError::UnsupportedFormat(format!("Unsupported archive format: {:?}", archive_path.extension())))?;

    println!("Opening {} archive: {}", format.name(), archive_path.display());

    let file = File::open(archive_path)?;
    let mut archive = UnifiedArchive::open_with_format(file, format)?;

    // For single-file formats (.Z, .gz, .bz2), derive the output filename from the archive name
    if matches!(format, ArchiveFormat::Z | ArchiveFormat::Gz | ArchiveFormat::Bz2) {
        if let Some(stem) = archive_path.file_stem() {
            archive.set_single_file_name(stem.to_string_lossy().to_string());
        }
    }

    // Create output directory if needed
    fs::create_dir_all(&output_dir)?;

    // Process all entries
    let mut count = 0;
    while let Some(entry) = archive.next_entry()? {
        println!(
            "  {} ({} -> {} bytes, {})",
            entry.name(),
            entry.compressed_size(),
            entry.original_size(),
            entry.compression_method()
        );

        // Read and write the file
        match archive.read(&entry) {
            Ok(data) => {
                let output_path = output_dir.join(entry.file_name());
                fs::write(&output_path, &data)?;
                println!("    -> Extracted to {}", output_path.display());
                count += 1;
            }
            Err(e) => {
                eprintln!("    -> Error: {}", e);
            }
        }
    }

    println!();
    println!("Extracted {} file(s)", count);

    Ok(())
}
