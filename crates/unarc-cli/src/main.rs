//! unarc - Universal Archive Extractor
//!
//! A command-line tool for listing and extracting files from various archive formats.

mod password;

use clap::{Parser, Subcommand};
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use unarc_rs::error::ArchiveError;
use unarc_rs::unified::{ArchiveFormat, ArchiveOptions, UnifiedArchive};

#[derive(Parser)]
#[command(name = "unarc")]
#[command(author, version, about = "Universal Archive Extractor", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List contents of an archive
    #[command(alias = "l")]
    List {
        /// Archive file to list
        archive: PathBuf,
    },

    /// Extract files from an archive
    #[command(alias = "x")]
    Extract {
        /// Archive file to extract
        archive: PathBuf,

        /// Output directory (default: current directory)
        #[arg(short, long, default_value = ".")]
        output: PathBuf,

        /// Overwrite existing files
        #[arg(short = 'f', long)]
        force: bool,

        /// Password for encrypted archives
        #[arg(short, long)]
        password: Option<String>,
    },

    /// Try passwords from a file, directory, or stdin to find the correct one
    #[command(alias = "tp")]
    TryPasswords {
        /// Archive file to test
        archive: PathBuf,

        /// File containing passwords (one per line), use "-" for stdin
        #[arg(short = 'f', long, conflicts_with = "password_dir")]
        password_file: Option<PathBuf>,

        /// Directory containing password files (searches recursively for *.txt)
        #[arg(short = 'd', long, conflicts_with = "password_file")]
        password_dir: Option<PathBuf>,

        /// Read passwords from stdin
        #[arg(long, conflicts_with_all = ["password_file", "password_dir"])]
        stdin: bool,

        /// Show progress every N passwords
        #[arg(short = 'v', long, default_value = "1000")]
        verbose_interval: usize,

        /// Specific entry name to test against (useful for selecting a small file)
        #[arg(short = 'e', long)]
        entry: Option<String>,
    },

    /// Show supported archive formats
    Formats,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::List { archive } => cmd_list(&archive),
        Commands::Extract {
            archive,
            output,
            force,
            password,
        } => cmd_extract(&archive, &output, force, password.as_deref()),
        Commands::TryPasswords {
            archive,
            password_file,
            password_dir,
            stdin,
            verbose_interval,
            entry,
        } => password::cmd_try_passwords(
            &archive,
            password_file.as_deref(),
            password_dir.as_deref(),
            stdin,
            verbose_interval,
            entry.as_deref(),
        ),
        Commands::Formats => cmd_formats(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn cmd_list(archive_path: &Path) -> Result<(), ArchiveError> {
    let format = detect_format(archive_path)?;

    println!("Archive: {} ({})", archive_path.display(), format.name());
    println!();
    println!(
        "{:<40} {:>12} {:>12} {:>8} {:<12} {}",
        "Name", "Compressed", "Original", "Ratio", "Method", "Encryption"
    );
    println!("{}", "-".repeat(105));

    let file = File::open(archive_path)?;
    let mut archive = UnifiedArchive::open_with_format(file, format)?;

    // For single-file formats, derive the filename from the archive name
    if matches!(
        format,
        ArchiveFormat::Z | ArchiveFormat::Gz | ArchiveFormat::Bz2
    ) {
        if let Some(stem) = archive_path.file_stem() {
            archive.set_single_file_name(stem.to_string_lossy().to_string());
        }
    }

    let mut total_compressed = 0u64;
    let mut total_original = 0u64;
    let mut count = 0;
    let mut encrypted_count = 0;

    loop {
        let entry = match archive.next_entry() {
            Ok(Some(e)) => e,
            Ok(None) => break,
            Err(ArchiveError::Io(ref e)) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        };

        let ratio = if entry.original_size() > 0 {
            format!("{:>6.1}%", entry.compression_ratio() * 100.0)
        } else {
            "   N/A".to_string()
        };

        let encryption = if entry.is_encrypted() {
            encrypted_count += 1;
            entry.encryption().to_string()
        } else {
            String::new()
        };

        println!(
            "{:<40} {:>12} {:>12} {:>8} {:<12} {}",
            truncate(entry.name(), 40),
            entry.compressed_size(),
            entry.original_size(),
            ratio,
            entry.compression_method(),
            encryption
        );

        total_compressed += entry.compressed_size();
        total_original += entry.original_size();
        count += 1;

        // Skip to next entry (don't decompress)
        if let Err(e) = archive.skip(&entry) {
            if !matches!(&e, ArchiveError::Io(io_err) if io_err.kind() == io::ErrorKind::UnexpectedEof)
            {
                return Err(e);
            }
            break;
        }
    }

    println!("{}", "-".repeat(105));
    let total_ratio = if total_original > 0 {
        format!(
            "{:>6.1}%",
            (total_compressed as f64 / total_original as f64) * 100.0
        )
    } else {
        "   N/A".to_string()
    };

    let summary = if encrypted_count > 0 {
        format!("{} file(s), {} encrypted", count, encrypted_count)
    } else {
        format!("{} file(s)", count)
    };

    println!(
        "{:<40} {:>12} {:>12} {:>8}",
        summary, total_compressed, total_original, total_ratio
    );

    Ok(())
}

fn cmd_extract(
    archive_path: &Path,
    output_dir: &Path,
    force: bool,
    password: Option<&str>,
) -> Result<(), ArchiveError> {
    let format = detect_format(archive_path)?;

    println!(
        "Extracting {} archive: {}",
        format.name(),
        archive_path.display()
    );

    let file = File::open(archive_path)?;
    let mut archive = UnifiedArchive::open_with_format(file, format)?;

    // Set up options with password if provided
    let options = match password {
        Some(pwd) => {
            println!("Using password for decryption");
            ArchiveOptions::new().with_password(pwd)
        }
        None => ArchiveOptions::new(),
    };

    // For single-file formats, derive the output filename from the archive name
    if matches!(
        format,
        ArchiveFormat::Z | ArchiveFormat::Gz | ArchiveFormat::Bz2
    ) {
        if let Some(stem) = archive_path.file_stem() {
            archive.set_single_file_name(stem.to_string_lossy().to_string());
        }
    }

    // Create output directory if needed
    fs::create_dir_all(output_dir)?;

    let mut count = 0;
    let mut errors = 0;

    while let Some(entry) = archive.next_entry()? {
        let output_path = output_dir.join(entry.file_name());

        // Check if file exists
        if output_path.exists() && !force {
            eprintln!(
                "  Skipping {} (already exists, use -f to overwrite)",
                entry.name()
            );
            // Still need to skip the entry data
            if let Err(e) = archive.skip(&entry) {
                if !matches!(&e, ArchiveError::Io(io_err) if io_err.kind() == io::ErrorKind::UnexpectedEof)
                {
                    return Err(e);
                }
            }
            continue;
        }

        // Create parent directories if needed
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        print!("  {} ({} bytes)... ", entry.name(), entry.original_size());

        match archive.read_with_options(&entry, &options) {
            Ok(data) => {
                fs::write(&output_path, &data)?;
                println!("OK");
                count += 1;
            }
            Err(e) => {
                println!("ERROR: {}", e);
                errors += 1;
            }
        }
    }

    println!();
    if errors > 0 {
        println!("Extracted {} file(s), {} error(s)", count, errors);
    } else {
        println!("Extracted {} file(s)", count);
    }

    Ok(())
}

fn cmd_formats() -> Result<(), ArchiveError> {
    println!("Supported archive formats:\n");
    println!(
        "{:<12} {:<25} {:<18} {}",
        "Extension", "Format", "Magic Bytes", "Aliases"
    );
    println!("{}", "-".repeat(80));

    for format in ArchiveFormat::ALL {
        let extensions = format.extensions();
        let primary = extensions
            .first()
            .map(|s| format!(".{}", s))
            .unwrap_or_default();
        let aliases: Vec<String> = extensions
            .iter()
            .skip(1)
            .map(|s| format!(".{}", s))
            .collect();
        let aliases_str = if aliases.is_empty() {
            String::new()
        } else {
            aliases.join(", ")
        };
        let magic = format
            .preambles()
            .map(|preambles| {
                preambles
                    .iter()
                    .map(|bytes| {
                        bytes
                            .iter()
                            .take(8)
                            .map(|b| {
                                if b.is_ascii_graphic() || *b == b' ' {
                                    format!("{}", *b as char)
                                } else {
                                    format!("\\x{:02X}", b)
                                }
                            })
                            .collect::<String>()
                    })
                    .collect::<Vec<_>>()
                    .join(" | ")
            })
            .unwrap_or_else(|| "-".to_string());
        let offset = format.preamble_offset();
        let offset_str = if offset > 0 {
            format!("@{}", offset)
        } else {
            String::new()
        };
        println!(
            "{:<12} {:<25} {:<20}{} {}",
            primary,
            format.name(),
            magic,
            offset_str,
            aliases_str
        );
    }

    Ok(())
}

fn detect_format(path: &Path) -> Result<ArchiveFormat, ArchiveError> {
    // Try content-based detection first
    let mut file = File::open(path)?;
    if let Ok(Some(format)) = ArchiveFormat::detect(&mut file, Some(path)) {
        return Ok(format);
    }

    // Fall back to extension only
    ArchiveFormat::from_path(path).ok_or_else(|| {
        ArchiveError::UnsupportedFormat(format!(
            "Unsupported or unrecognized archive format: {:?}",
            path.extension()
        ))
    })
}

pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("...{}", &s[s.len() - max_len + 3..])
    }
}
