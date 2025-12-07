//! Example: List contents of any supported archive format using the unified API
//!
//! Usage: cargo run --example unified_list <archive_file>

use std::env;
use std::fs::File;
use std::path::Path;
use unarc_rs::error::ArchiveError;
use unarc_rs::unified::{ArchiveFormat, UnifiedArchive};

fn main() -> Result<(), ArchiveError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <archive_file>", args[0]);
        eprintln!();
        eprintln!("Supported formats:");
        for format in ArchiveFormat::ALL {
            eprintln!("  .{} - {}", format.extension(), format.name());
        }
        std::process::exit(1);
    }

    let archive_path = Path::new(&args[1]);

    // Detect format from extension
    let format = ArchiveFormat::from_path(archive_path).ok_or_else(|| {
        ArchiveError::UnsupportedFormat(format!(
            "Unsupported archive format: {:?}",
            archive_path.extension()
        ))
    })?;

    println!("Archive: {} ({})", archive_path.display(), format.name());
    println!();
    println!(
        "{:<30} {:>12} {:>12} {:>8} {}",
        "Name", "Compressed", "Original", "Ratio", "Method"
    );
    println!("{}", "-".repeat(80));

    let file = File::open(archive_path)?;
    let mut archive = UnifiedArchive::open_with_format(file, format)?;

    // For .Z files, derive the filename from the archive name
    if format == ArchiveFormat::Z {
        if let Some(stem) = archive_path.file_stem() {
            archive.set_z_filename(stem.to_string_lossy().to_string());
        }
    }

    let mut total_compressed = 0u64;
    let mut total_original = 0u64;
    let mut count = 0;

    loop {
        let entry = match archive.next_entry() {
            Ok(Some(e)) => e,
            Ok(None) => break,
            Err(ArchiveError::Io(ref e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        };

        let ratio = if entry.original_size() > 0 {
            format!("{:>6.1}%", entry.compression_ratio() * 100.0)
        } else {
            "   N/A".to_string()
        };

        println!(
            "{:<30} {:>12} {:>12} {:>8} {}",
            truncate(entry.name(), 30),
            entry.compressed_size(),
            entry.original_size(),
            ratio,
            entry.compression_method()
        );

        total_compressed += entry.compressed_size();
        total_original += entry.original_size();
        count += 1;

        // Skip to next entry (don't decompress)
        if let Err(e) = archive.skip(&entry) {
            if !matches!(&e, ArchiveError::Io(io_err) if io_err.kind() == std::io::ErrorKind::UnexpectedEof)
            {
                return Err(e);
            }
            break;
        }
    }

    println!("{}", "-".repeat(80));
    let total_ratio = if total_original > 0 {
        format!(
            "{:>6.1}%",
            (total_compressed as f64 / total_original as f64) * 100.0
        )
    } else {
        "   N/A".to_string()
    };
    println!(
        "{:<30} {:>12} {:>12} {:>8}",
        format!("{} file(s)", count),
        total_compressed,
        total_original,
        total_ratio
    );

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("...{}", &s[s.len() - max_len + 3..])
    }
}
