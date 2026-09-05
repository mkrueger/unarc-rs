//! unarc - Universal Archive Extractor
//!
//! A command-line tool for listing and extracting files from various archive formats.

mod extraction;
mod password;

use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use unarc_rs::error::ArchiveError;
use unarc_rs::unified::{ArchiveFormat, ArchiveOptions, UnifiedArchive, VolumeProvider};

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

/// Open an archive, automatically handling multi-volume split archives
fn open_archive_auto(archive_path: &Path, format: ArchiveFormat, options: ArchiveOptions) -> Result<Box<dyn ArchiveReader>, ArchiveError> {
    // Check if this is a multi-volume split archive (ZIP .001/.z01 or 7z .001)
    if let Some(pattern) = FileVolumeProvider::detect_pattern(archive_path) {
        match pattern {
            VolumePattern::Numeric3 | VolumePattern::SevenZ => {
                // For .001/.7z.001 style, we need to use the MultiVolumeReader
                let volumes = FileVolumeProvider::find_all_volumes(archive_path);
                if volumes.len() > 1 {
                    println!("  Detected {} volumes", volumes.len());
                    match format {
                        ArchiveFormat::Zip => {
                            let archive = ArchiveFormat::open_multi_volume_zip(&volumes, options)?;
                            return Ok(Box::new(archive));
                        }
                        ArchiveFormat::SevenZ => {
                            let archive = ArchiveFormat::open_multi_volume_7z(&volumes, options)?;
                            return Ok(Box::new(archive));
                        }
                        _ => {}
                    }
                }
            }
            VolumePattern::WinZip => {
                // For .zip/.z01 style
                let volumes = FileVolumeProvider::find_all_volumes(archive_path);
                if volumes.len() > 1 {
                    println!("  Detected {} volumes", volumes.len());
                    let archive = ArchiveFormat::open_multi_volume_zip(&volumes, options)?;
                    return Ok(Box::new(archive));
                }
            }
            _ => {
                // For other patterns (ARJ, ACE), the VolumeProvider handles it
            }
        }
    }

    // Standard single-file or VolumeProvider-handled multi-volume
    let volume_provider = Arc::new(FileVolumeProvider::new(archive_path, format));
    let file = File::open(archive_path)?;
    let mut archive = UnifiedArchive::open_with_format(file, format)?;
    archive.set_options(options.with_volume_provider_arc(volume_provider));
    Ok(Box::new(archive))
}

/// Trait for abstracting archive operations
trait ArchiveReader {
    fn next_entry_box(&mut self) -> Result<Option<unarc_rs::unified::ArchiveEntry>, ArchiveError>;
    fn read_with_options_box(&mut self, entry: &unarc_rs::unified::ArchiveEntry, options: &ArchiveOptions) -> Result<Vec<u8>, ArchiveError>;
    fn skip_box(&mut self, entry: &unarc_rs::unified::ArchiveEntry) -> Result<(), ArchiveError>;
    fn set_single_file_name_box(&mut self, name: String);
}

impl<T: std::io::Read + std::io::Seek> ArchiveReader for UnifiedArchive<T> {
    fn next_entry_box(&mut self) -> Result<Option<unarc_rs::unified::ArchiveEntry>, ArchiveError> {
        self.next_entry()
    }
    fn read_with_options_box(&mut self, entry: &unarc_rs::unified::ArchiveEntry, options: &ArchiveOptions) -> Result<Vec<u8>, ArchiveError> {
        self.read_with_options(entry, options)
    }
    fn skip_box(&mut self, entry: &unarc_rs::unified::ArchiveEntry) -> Result<(), ArchiveError> {
        self.skip(entry)
    }
    fn set_single_file_name_box(&mut self, name: String) {
        self.set_single_file_name(name);
    }
}

fn cmd_list(archive_path: &Path) -> Result<(), ArchiveError> {
    let format = detect_format(archive_path)?;

    println!("Archive: {} ({})", archive_path.display(), format.name());
    println!();
    println!(
        "{:<40} {:>12} {:>12} {:>8} {:<12} Encryption",
        "Name", "Compressed", "Original", "Ratio", "Method"
    );
    println!("{}", "-".repeat(105));

    let mut archive = open_archive_auto(archive_path, format, ArchiveOptions::new())?;

    // For single-file formats, derive the filename from the archive name
    if matches!(format, ArchiveFormat::Z | ArchiveFormat::Gz | ArchiveFormat::Bz2) {
        if let Some(stem) = archive_path.file_stem() {
            archive.set_single_file_name_box(stem.to_string_lossy().to_string());
        }
    }

    let mut total_compressed = 0u64;
    let mut total_original = 0u64;
    let mut count = 0;
    let mut encrypted_count = 0;

    loop {
        let entry = match archive.next_entry_box() {
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
        if let Err(e) = archive.skip_box(&entry) {
            if !matches!(&e, ArchiveError::Io(io_err) if io_err.kind() == io::ErrorKind::UnexpectedEof) {
                return Err(e);
            }
            break;
        }
    }

    println!("{}", "-".repeat(105));
    let total_ratio = if total_original > 0 {
        format!("{:>6.1}%", (total_compressed as f64 / total_original as f64) * 100.0)
    } else {
        "   N/A".to_string()
    };

    let summary = if encrypted_count > 0 {
        format!("{} file(s), {} encrypted", count, encrypted_count)
    } else {
        format!("{} file(s)", count)
    };

    println!("{:<40} {:>12} {:>12} {:>8}", summary, total_compressed, total_original, total_ratio);

    Ok(())
}

/// Volume provider for multi-volume archives from the filesystem
struct FileVolumeProvider {
    /// Cached list of volume paths, sorted in order
    volume_paths: Vec<PathBuf>,
}

/// Pattern for multi-volume archive naming
#[derive(Debug, Clone, Copy, PartialEq)]
enum VolumePattern {
    /// .001, .002, .003 (generic split)
    Numeric3,
    /// .zip, .z01, .z02 (WinZip style)
    WinZip,
    /// .rar, .r00, .r01 (old RAR style)
    RarOld,
    /// .part1.rar, .part2.rar (new RAR5 style)
    RarNew,
    /// .7z.001, .7z.002 (7-Zip split)
    SevenZ,
    /// .ace, .c00, .c01 (ACE style)
    Ace,
    /// .arj, .a01, .a02 (ARJ style)
    Arj,
}

impl FileVolumeProvider {
    fn new(archive_path: &Path, _format: ArchiveFormat) -> Self {
        let volume_paths = Self::find_all_volumes(archive_path);
        Self { volume_paths }
    }

    /// Detect the volume pattern from a filename
    fn detect_pattern(path: &Path) -> Option<VolumePattern> {
        let name = path.file_name()?.to_str()?.to_lowercase();

        // Check for .partN.rar pattern (RAR5 new style)
        if let Some((_, number)) = name.strip_suffix(".rar").and_then(|stem| stem.rsplit_once(".part")) {
            if !number.is_empty() && number.bytes().all(|c| c.is_ascii_digit()) && number.parse::<u32>().is_ok_and(|n| n > 0) {
                return Some(VolumePattern::RarNew);
            }
        }

        // Check for .7z.NNN pattern
        if let Some(pos) = name.rfind(".7z.") {
            let suffix = &name[pos + 4..];
            if !suffix.is_empty() && suffix.bytes().all(|c| c.is_ascii_digit()) && suffix.parse::<u32>().is_ok_and(|n| n > 0) {
                return Some(VolumePattern::SevenZ);
            }
        }

        // Get the extension
        let ext = path.extension()?.to_str()?.to_lowercase();

        match ext.as_str() {
            // Numeric extensions like .001, .002
            s if s.len() == 3 && s.chars().all(|c| c.is_ascii_digit()) => Some(VolumePattern::Numeric3),
            // ZIP and WinZip split: .zip, .z01, .z02
            "zip" => Some(VolumePattern::WinZip),
            s if Self::numbered_extension(s, 'z') => Some(VolumePattern::WinZip),
            // RAR old style: .rar, .r00, .r01
            "rar" => Some(VolumePattern::RarOld),
            s if Self::numbered_extension(s, 'r') => Some(VolumePattern::RarOld),
            // ACE style: .ace, .c00, .c01
            "ace" => Some(VolumePattern::Ace),
            s if Self::numbered_extension(s, 'c') => Some(VolumePattern::Ace),
            // ARJ style: .arj, .a01, .a02
            "arj" => Some(VolumePattern::Arj),
            s if Self::numbered_extension(s, 'a') => Some(VolumePattern::Arj),
            _ => None,
        }
    }

    /// Validate the letter plus two-digit suffix used by legacy volume names.
    fn numbered_extension(ext: &str, prefix: char) -> bool {
        ext.len() == 3 && ext.starts_with(prefix) && ext.as_bytes()[1..].iter().all(u8::is_ascii_digit)
    }

    /// Get the base name without its volume suffix.
    fn get_base_for_pattern(path: &Path, pattern: VolumePattern) -> Option<PathBuf> {
        let parent = path.parent()?;
        let name = path.file_name()?.to_str()?;

        match pattern {
            VolumePattern::RarNew => {
                // Remove .partN.rar to get base
                // e.g., "archive.part1.rar" -> "archive"
                let lower = name.to_ascii_lowercase();
                if let Some(pos) = lower.rfind(".part") {
                    let base = &name[..pos];
                    return Some(parent.join(base));
                }
                None
            }
            VolumePattern::SevenZ => {
                // Remove .7z.NNN to get base
                // e.g., "archive.7z.001" -> "archive"
                let lower = name.to_ascii_lowercase();
                if let Some(pos) = lower.rfind(".7z.") {
                    let base = &name[..pos];
                    return Some(parent.join(base));
                }
                None
            }
            _ => {
                // For other patterns, just remove the extension
                let stem = path.file_stem()?.to_str()?;
                Some(parent.join(stem))
            }
        }
    }

    /// Check if a file matches the volume pattern for a given base
    fn matches_pattern(path: &Path, base: &Path, pattern: VolumePattern) -> bool {
        if Self::detect_pattern(path) != Some(pattern) {
            return false;
        }
        let Some(candidate) = Self::get_base_for_pattern(path, pattern) else {
            return false;
        };
        match (candidate.file_name().and_then(|n| n.to_str()), base.file_name().and_then(|n| n.to_str())) {
            (Some(candidate), Some(base)) => candidate.to_lowercase() == base.to_lowercase(),
            _ => false,
        }
    }

    /// Extract volume number from a path for sorting
    fn get_volume_number(path: &Path, pattern: VolumePattern) -> u32 {
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_lowercase(),
            None => return u32::MAX,
        };

        match pattern {
            VolumePattern::Numeric3 | VolumePattern::SevenZ => {
                // Extract last numeric extension
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                ext.parse().unwrap_or(u32::MAX)
            }
            VolumePattern::WinZip => {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                if ext == "zip" {
                    // The central directory is in the final .zip volume.
                    u32::MAX
                } else if let Some(stripped) = ext.strip_prefix('z') {
                    stripped.parse().unwrap_or(u32::MAX)
                } else {
                    u32::MAX
                }
            }
            VolumePattern::RarOld => {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                if ext == "rar" {
                    0
                } else if let Some(stripped) = ext.strip_prefix('r') {
                    // .r00 = volume 1, .r01 = volume 2, etc.
                    stripped.parse::<u32>().map(|n| n + 1).unwrap_or(u32::MAX)
                } else {
                    u32::MAX
                }
            }
            VolumePattern::RarNew => {
                // Extract number from .partN.rar
                if let Some(start) = name.rfind(".part") {
                    let rest = &name[start + 5..];
                    if let Some(end) = rest.find('.') {
                        return rest[..end].parse().unwrap_or(u32::MAX);
                    }
                }
                u32::MAX
            }
            VolumePattern::Ace => {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                if ext == "ace" {
                    0
                } else if let Some(stripped) = ext.strip_prefix('c') {
                    // .c00 = volume 1, .c01 = volume 2, etc.
                    stripped.parse::<u32>().map(|n| n + 1).unwrap_or(u32::MAX)
                } else {
                    u32::MAX
                }
            }
            VolumePattern::Arj => {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                if ext == "arj" {
                    0
                } else if let Some(stripped) = ext.strip_prefix('a') {
                    stripped.parse().unwrap_or(u32::MAX)
                } else {
                    u32::MAX
                }
            }
        }
    }

    /// Find all volumes for an archive
    fn find_all_volumes(archive_path: &Path) -> Vec<PathBuf> {
        let pattern = match Self::detect_pattern(archive_path) {
            Some(p) => p,
            None => {
                // No pattern detected, just return the single file
                return vec![archive_path.to_path_buf()];
            }
        };

        let base = match Self::get_base_for_pattern(archive_path, pattern) {
            Some(b) => b,
            None => return vec![archive_path.to_path_buf()],
        };

        let parent = match archive_path.parent() {
            Some(p) if p.as_os_str().is_empty() => Path::new("."),
            Some(p) => p,
            None => return vec![archive_path.to_path_buf()],
        };

        // Read directory and find matching files
        let mut volumes: Vec<PathBuf> = match std::fs::read_dir(parent) {
            Ok(entries) => entries
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.is_file() && Self::matches_pattern(p, &base, pattern))
                .collect(),
            Err(_) => return vec![archive_path.to_path_buf()],
        };

        // Sort by volume number
        volumes.sort_by_key(|p| Self::get_volume_number(p, pattern));

        if volumes.is_empty() {
            vec![archive_path.to_path_buf()]
        } else {
            volumes
        }
    }
}

impl VolumeProvider for FileVolumeProvider {
    fn open_volume(&self, volume_number: u32) -> Option<Box<dyn Read + Send>> {
        let path = self.volume_paths.get(volume_number as usize)?;

        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return None,
        };

        if volume_number > 0 {
            println!("  Opening volume: {}", path.display());
        }

        Some(Box::new(BufReader::new(file)) as Box<dyn Read + Send>)
    }
}

/// Turns an archive entry name into a safe relative path below the output directory.
/// Returns `None` for names that escape the output directory or carry no usable component.
fn sanitize_entry_path(name: &str) -> Option<PathBuf> {
    let mut result = PathBuf::new();
    for (index, part) in name.split(['/', '\\']).enumerate() {
        let part = if index == 0 {
            // Drop a DOS drive prefix such as "C:" on the first component
            match part.as_bytes() {
                [_, b':', rest @ ..] => std::str::from_utf8(rest).unwrap_or(part),
                _ => part,
            }
        } else {
            part
        };
        match part {
            "" | "." => continue,
            ".." => return None,
            _ => result.push(part),
        }
    }
    if result.as_os_str().is_empty() {
        None
    } else {
        Some(result)
    }
}

fn cmd_extract(archive_path: &Path, output_dir: &Path, force: bool, password: Option<&str>) -> Result<(), ArchiveError> {
    let format = detect_format(archive_path)?;

    println!("Extracting {} archive: {}", format.name(), archive_path.display());

    // Set up options with password
    let options = match password {
        Some(pwd) => {
            println!("Using password for decryption");
            ArchiveOptions::new().with_password(pwd)
        }
        None => ArchiveOptions::new(),
    };

    let mut archive = open_archive_auto(archive_path, format, options.clone())?;

    // For single-file formats, derive the output filename from the archive name
    if matches!(format, ArchiveFormat::Z | ArchiveFormat::Gz | ArchiveFormat::Bz2) {
        if let Some(stem) = archive_path.file_stem() {
            archive.set_single_file_name_box(stem.to_string_lossy().to_string());
        }
    }

    let output = extraction::ExtractionRoot::new(output_dir)?;

    let mut count = 0;
    let mut errors = 0;

    while let Some(entry) = archive.next_entry_box()? {
        let relative_path = match sanitize_entry_path(entry.name()) {
            Some(path) => path,
            None => {
                eprintln!("  Skipping {} (unsafe path)", entry.name());
                if let Err(e) = archive.skip_box(&entry) {
                    if !matches!(&e, ArchiveError::Io(io_err) if io_err.kind() == io::ErrorKind::UnexpectedEof) {
                        return Err(e);
                    }
                }
                errors += 1;
                continue;
            }
        };
        if entry.is_directory() {
            output.create_dir_all(&relative_path)?;
            if let Err(e) = archive.skip_box(&entry) {
                if !matches!(&e, ArchiveError::Io(io_err) if io_err.kind() == io::ErrorKind::UnexpectedEof) {
                    return Err(e);
                }
            }
            continue;
        }

        // Check if file exists
        if !force && output.exists(&relative_path)? {
            eprintln!("  Skipping {} (already exists, use -f to overwrite)", entry.name());
            // Still need to skip the entry data
            if let Err(e) = archive.skip_box(&entry) {
                if !matches!(&e, ArchiveError::Io(io_err) if io_err.kind() == io::ErrorKind::UnexpectedEof) {
                    return Err(e);
                }
            }
            continue;
        }

        print!("  {} ({} bytes)... ", entry.name(), entry.original_size());

        match archive.read_with_options_box(&entry, &options) {
            Ok(data) => {
                if output.write(&relative_path, &data, force)? {
                    println!("OK");
                    count += 1;
                } else {
                    println!("Skipped (already exists, use -f to overwrite)");
                }
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
        return Err(ArchiveError::io_error(format!("Extraction completed with {errors} error(s)")));
    } else {
        println!("Extracted {} file(s)", count);
    }

    Ok(())
}

fn cmd_formats() -> Result<(), ArchiveError> {
    println!("Supported archive formats:\n");
    println!("{:<12} {:<28} {:<33} {:<8} Aliases", "Extension", "Format", "Magic Bytes", "Offset");
    println!("{}", "-".repeat(95));

    for format in ArchiveFormat::ALL {
        let extensions = format.extensions();
        let primary = extensions.first().map(|s| format!(".{}", s)).unwrap_or_default();
        let aliases: Vec<String> = extensions.iter().skip(1).map(|s| format!(".{}", s)).collect();
        let aliases_str = if aliases.is_empty() { String::new() } else { aliases.join(", ") };
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
        let offset_str = if offset > 0 { format!("@{}", offset) } else { String::new() };
        println!("{:<12} {:<28} {:<33} {:<8} {}", primary, format.name(), magic, offset_str, aliases_str);
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
    ArchiveFormat::from_path(path).ok_or_else(|| ArchiveError::UnsupportedFormat(format!("Unsupported or unrecognized archive format: {:?}", path.extension())))
}

/// Shorten a name by Unicode scalar values, never by arbitrary UTF-8 bytes.
pub fn truncate(s: &str, max_len: usize) -> String {
    let len = s.chars().count();
    if len <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        ".".repeat(max_len)
    } else {
        let suffix: String = s.chars().skip(len - (max_len - 3)).collect();
        format!("...{suffix}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn volume_patterns_require_exact_base_names() {
        use VolumePattern::*;
        for (pattern, suffixes) in [
            (Numeric3, &["001", "002"][..]),
            (WinZip, &["zip", "z01", "z02"][..]),
            (RarOld, &["rar", "r00", "r01"][..]),
            (RarNew, &["part1.rar", "part02.rar"][..]),
            (SevenZ, &["7z.001", "7z.002"][..]),
            (Ace, &["ace", "c00", "c01"][..]),
            (Arj, &["arj", "a01", "a02"][..]),
        ] {
            for base in ["backup", "archive.part.data", "İmage", "文件"] {
                for suffix in suffixes {
                    let name = format!("{base}.{suffix}");
                    assert_eq!(FileVolumeProvider::detect_pattern(Path::new(&name)), Some(pattern));
                    assert!(FileVolumeProvider::matches_pattern(Path::new(&name), Path::new(base), pattern));
                    assert!(FileVolumeProvider::matches_pattern(
                        Path::new(&name.to_ascii_uppercase()),
                        Path::new(base),
                        pattern
                    ));
                    for extra in ["-other", ".other", "2"] {
                        assert!(!FileVolumeProvider::matches_pattern(
                            Path::new(&format!("{base}{extra}.{suffix}")),
                            Path::new(base),
                            pattern
                        ));
                    }
                }
            }
        }
    }

    #[test]
    fn rejects_malformed_volume_suffixes() {
        for name in ["backup.zxy", "backup.r0x", "backup.c_1", "backup.aé", "backup.7z.", "backup.7z.xyz"] {
            assert_eq!(FileVolumeProvider::detect_pattern(Path::new(name)), None, "{name}");
        }
        for name in ["backup.part.rar", "backup.partx.rar", "backup.part0.rar", "backup.part4294967296.rar"] {
            assert!(!FileVolumeProvider::matches_pattern(
                Path::new(name),
                Path::new("backup"),
                VolumePattern::RarNew
            ));
        }
        assert_eq!(
            FileVolumeProvider::get_volume_number(Path::new("archive.part.data.part12.rar"), VolumePattern::RarNew),
            12
        );
    }

    #[test]
    fn discovers_only_matching_volumes_in_numeric_order() {
        use VolumePattern::*;
        for (pattern, names) in [
            (Numeric3, &["backup.001", "backup.002", "backup.010"][..]),
            (WinZip, &["backup.z01", "backup.z02", "backup.zip"][..]),
            (RarOld, &["backup.rar", "backup.r00", "backup.r01"][..]),
            (RarNew, &["backup.part1.rar", "backup.part2.rar", "backup.part10.rar"][..]),
            (SevenZ, &["backup.7z.001", "backup.7z.002", "backup.7z.010"][..]),
            (Ace, &["backup.ace", "backup.c00", "backup.c01"][..]),
            (Arj, &["backup.arj", "backup.a01", "backup.a02"][..]),
        ] {
            let temp = tempfile::tempdir().unwrap();
            for name in names.iter().rev() {
                std::fs::write(temp.path().join(name), []).unwrap();
                std::fs::write(temp.path().join(name.replace("backup", "backup-other")), []).unwrap();
            }
            let actual = FileVolumeProvider::find_all_volumes(&temp.path().join(names[1]));
            let expected: Vec<_> = names.iter().map(|name| temp.path().join(name)).collect();
            assert_eq!(actual, expected, "{pattern:?}");
        }
    }

    #[test]
    fn truncate_preserves_unicode_and_ascii_suffixes() {
        assert_eq!(truncate("abcdefgh", 6), "...fgh");
        assert_eq!(truncate(&"ä".repeat(30), 40), "ä".repeat(30));
        assert_eq!(truncate(&"ä".repeat(60), 40), format!("...{}", "ä".repeat(37)));
        assert_eq!(truncate("", 0), "");
        for name in ["abcdef", "äöüßé", "文件名归档", "📁🦀🙂🙂", "e\u{301}e\u{301}"] {
            for max_len in 0..=30 {
                let result = truncate(name, max_len);
                assert!(result.chars().count() <= max_len);
                if name.chars().count() <= max_len {
                    assert_eq!(result, name);
                }
            }
        }
    }
}
