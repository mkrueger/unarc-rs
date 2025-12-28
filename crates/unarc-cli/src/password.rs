//! Password testing functionality for archives.
//!
//! This module provides commands and utilities for testing passwords
//! against encrypted archives.

#![allow(clippy::while_let_loop)]

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use rayon::prelude::*;
use unarc_rs::ace::AceArchive;
use unarc_rs::ace::AcePasswordVerifier;
use unarc_rs::arc::arc_archive::ArcArchive;
use unarc_rs::arc::password_verifier::ArcPasswordVerifier;
use unarc_rs::arj::arj_archive::ArjArchive;
use unarc_rs::arj::password_verifier::ArjPasswordVerifier;
use unarc_rs::error::ArchiveError;
use unarc_rs::rar::rar_archive::RarArchive;
use unarc_rs::rar::RarPasswordVerifier;
use unarc_rs::sevenz::sevenz_archive::SevenZArchive;
use unarc_rs::sevenz::SevenZPasswordVerifier;
use unarc_rs::unified::ArchiveFormat;
use unarc_rs::zip::zip_archive::ZipArchive;
use unarc_rs::zip::ZipPasswordVerifier;

use crate::truncate;

/// Unified password verifier that wraps format-specific verifiers
enum UnifiedPasswordVerifier {
    Arc(ArcPasswordVerifier),
    Arj(ArjPasswordVerifier),
    Ace(AcePasswordVerifier),
    Zip(ZipPasswordVerifier),
    Rar(RarPasswordVerifier),
    SevenZ(SevenZPasswordVerifier),
}

impl UnifiedPasswordVerifier {
    fn verify(&self, password: &str) -> bool {
        match self {
            Self::Arc(v) => v.verify(password),
            Self::Arj(v) => v.verify(password),
            Self::Ace(v) => v.verify(password),
            Self::Zip(v) => v.verify(password),
            Self::Rar(v) => v.verify(password),
            Self::SevenZ(v) => v.verify(password),
        }
    }

    fn entry_name(&self) -> &str {
        match self {
            Self::Arc(v) => v.entry_name(),
            Self::Arj(v) => v.entry_name(),
            Self::Ace(v) => v.entry_name(),
            Self::Zip(v) => v.entry_name(),
            Self::Rar(v) => v.entry_name(),
            Self::SevenZ(v) => v.entry_name(),
        }
    }

    fn original_size(&self) -> u64 {
        match self {
            Self::Arc(v) => v.original_size() as u64,
            Self::Arj(v) => v.original_size() as u64,
            Self::Ace(v) => v.original_size(),
            Self::Zip(v) => v.original_size(),
            Self::Rar(v) => v.original_size(),
            Self::SevenZ(v) => v.original_size(),
        }
    }

    fn extra_info(&self) -> Option<String> {
        match self {
            Self::Arc(_) => None,
            Self::Arj(v) => v.encryption_type().map(|e| format!("{:?}", e)),
            Self::Ace(_) => None,
            Self::Zip(_) => None,
            Self::Rar(_) => None,
            Self::SevenZ(_) => None,
        }
    }
}

// Required for rayon parallel processing
unsafe impl Send for UnifiedPasswordVerifier {}
unsafe impl Sync for UnifiedPasswordVerifier {}

/// Execute the try-passwords command
pub fn cmd_try_passwords(
    archive_path: &Path,
    password_file: Option<&Path>,
    password_dir: Option<&Path>,
    use_stdin: bool,
    verbose_interval: usize,
    entry_filter: Option<&str>,
) -> Result<(), ArchiveError> {
    let format = detect_format(archive_path)?;

    let (verifier, format_name) = create_verifier(archive_path, format, entry_filter)?;

    println!("Testing passwords for {} archive: {}", format_name, archive_path.display());

    let mut info = format!("Testing against entry: {} ({} bytes original", verifier.entry_name(), verifier.original_size());

    if let Some(extra) = verifier.extra_info() {
        info.push_str(&format!(", {}", extra));
    }
    info.push(')');

    println!("{}", info);
    println!();

    run_password_test(&verifier, password_file, password_dir, use_stdin, verbose_interval)
}

/// Create a password verifier for the given archive
fn create_verifier(archive_path: &Path, format: ArchiveFormat, entry_filter: Option<&str>) -> Result<(UnifiedPasswordVerifier, &'static str), ArchiveError> {
    match format {
        ArchiveFormat::Arc => {
            let file = File::open(archive_path)?;
            let mut archive = ArcArchive::new(file)?;

            // ARC entries are always encrypted if archive is encrypted
            // Collect all entries to find the one requested
            let mut encrypted_entries = Vec::new();
            while let Some(header) = archive.get_next_entry()? {
                encrypted_entries.push((header.name.clone(), header.original_size as u64));
            }

            if encrypted_entries.is_empty() {
                return Err(ArchiveError::io_error("Archive is empty"));
            }

            // Re-open archive and find the requested entry
            let file = File::open(archive_path)?;
            let mut archive = ArcArchive::new(file)?;

            if let Some(filter) = entry_filter {
                while let Some(header) = archive.get_next_entry()? {
                    if header.name == filter {
                        let verifier = archive.create_password_verifier(&header)?;
                        return Ok((UnifiedPasswordVerifier::Arc(verifier), "ARC"));
                    }
                }
                return Err(ArchiveError::io_error(format!(
                    "Entry '{}' not found. Available entries:\n{}",
                    filter,
                    format_entry_list(&encrypted_entries)
                )));
            }

            // No filter - take first entry
            let header = archive.get_next_entry()?.ok_or_else(|| ArchiveError::io_error("Archive is empty"))?;

            let verifier = archive.create_password_verifier(&header)?;
            Ok((UnifiedPasswordVerifier::Arc(verifier), "ARC"))
        }
        ArchiveFormat::Arj => {
            let file = File::open(archive_path)?;
            let mut archive = ArjArchive::new(file)?;

            // Collect encrypted entries
            let mut encrypted_entries = Vec::new();
            loop {
                let header = match archive.get_next_entry()? {
                    Some(h) => h,
                    None => break,
                };
                if header.is_garbled() {
                    encrypted_entries.push((header.name.clone(), header.original_size as u64));
                }
                archive.skip(&header)?;
            }

            if encrypted_entries.is_empty() {
                return Err(ArchiveError::io_error("No encrypted entries found in archive"));
            }

            // Re-open and find requested entry
            let file = File::open(archive_path)?;
            let mut archive = ArjArchive::new(file)?;

            if let Some(filter) = entry_filter {
                loop {
                    let header = match archive.get_next_entry()? {
                        Some(h) => h,
                        None => {
                            return Err(ArchiveError::io_error(format!(
                                "Entry '{}' not found or not encrypted. Encrypted entries:\n{}",
                                filter,
                                format_entry_list(&encrypted_entries)
                            )));
                        }
                    };
                    if header.is_garbled() && header.name == filter {
                        if let Some(verifier) = archive.create_password_verifier(&header)? {
                            return Ok((UnifiedPasswordVerifier::Arj(verifier), "ARJ"));
                        }
                    }
                    archive.skip(&header)?;
                }
            }

            // No filter - take first encrypted entry
            loop {
                let header = match archive.get_next_entry()? {
                    Some(h) => h,
                    None => {
                        return Err(ArchiveError::io_error("No encrypted entries found in archive"));
                    }
                };

                if header.is_garbled() {
                    if let Some(verifier) = archive.create_password_verifier(&header)? {
                        return Ok((UnifiedPasswordVerifier::Arj(verifier), "ARJ"));
                    }
                } else {
                    archive.skip(&header)?;
                }
            }
        }
        ArchiveFormat::Ace => {
            let file = File::open(archive_path)?;
            let mut archive = AceArchive::new(file)?;

            if archive.is_solid() {
                return Err(ArchiveError::unsupported_method(
                    "ACE",
                    "solid archives not supported - LZ77 decoder state cannot be parallelized",
                ));
            }

            // Collect encrypted entries
            let mut encrypted_entries = Vec::new();
            loop {
                let header = match archive.get_next_entry()? {
                    Some(h) => h,
                    None => break,
                };
                if header.is_encrypted() {
                    encrypted_entries.push((header.filename.clone(), header.original_size));
                }
                archive.skip(&header)?;
            }

            if encrypted_entries.is_empty() {
                return Err(ArchiveError::io_error("No encrypted entries found in archive"));
            }

            // Re-open and find requested entry
            let file = File::open(archive_path)?;
            let mut archive = AceArchive::new(file)?;

            if let Some(filter) = entry_filter {
                loop {
                    let header = match archive.get_next_entry()? {
                        Some(h) => h,
                        None => {
                            return Err(ArchiveError::io_error(format!(
                                "Entry '{}' not found or not encrypted. Encrypted entries:\n{}",
                                filter,
                                format_entry_list(&encrypted_entries)
                            )));
                        }
                    };
                    if header.is_encrypted() && header.filename == filter {
                        let verifier = archive.create_password_verifier(&header)?;
                        return Ok((UnifiedPasswordVerifier::Ace(verifier), "ACE"));
                    }
                    archive.skip(&header)?;
                }
            }

            // No filter - take first encrypted entry
            loop {
                let header = match archive.get_next_entry()? {
                    Some(h) => h,
                    None => {
                        return Err(ArchiveError::io_error("No encrypted entries found in archive"));
                    }
                };

                if header.is_encrypted() {
                    let verifier = archive.create_password_verifier(&header)?;
                    return Ok((UnifiedPasswordVerifier::Ace(verifier), "ACE"));
                } else {
                    archive.skip(&header)?;
                }
            }
        }
        ArchiveFormat::Zip => {
            // Read archive data first
            let mut file = File::open(archive_path)?;
            let mut archive_data = Vec::new();
            file.read_to_end(&mut archive_data)?;

            let cursor = std::io::Cursor::new(archive_data.clone());
            let mut archive = ZipArchive::new(cursor)?;

            // Collect encrypted entries
            let mut encrypted_entries = Vec::new();
            loop {
                let header = match archive.get_next_entry()? {
                    Some(h) => h,
                    None => break,
                };
                if header.is_encrypted && !header.is_directory {
                    encrypted_entries.push((header.name.clone(), header.original_size));
                }
            }

            if encrypted_entries.is_empty() {
                return Err(ArchiveError::io_error("No encrypted entries found in archive"));
            }

            // Re-open and find requested entry
            let cursor = std::io::Cursor::new(archive_data.clone());
            let mut archive = ZipArchive::new(cursor)?;

            if let Some(filter) = entry_filter {
                loop {
                    let header = match archive.get_next_entry()? {
                        Some(h) => h,
                        None => {
                            return Err(ArchiveError::io_error(format!(
                                "Entry '{}' not found or not encrypted. Encrypted entries:\n{}",
                                filter,
                                format_entry_list(&encrypted_entries)
                            )));
                        }
                    };
                    if header.is_encrypted && !header.is_directory && header.name == filter {
                        let verifier = archive.create_password_verifier(&header, archive_data)?;
                        return Ok((UnifiedPasswordVerifier::Zip(verifier), "ZIP"));
                    }
                }
            }

            // No filter - take first encrypted entry
            loop {
                let header = match archive.get_next_entry()? {
                    Some(h) => h,
                    None => {
                        return Err(ArchiveError::io_error("No encrypted entries found in archive"));
                    }
                };

                if header.is_encrypted && !header.is_directory {
                    let verifier = archive.create_password_verifier(&header, archive_data)?;
                    return Ok((UnifiedPasswordVerifier::Zip(verifier), "ZIP"));
                }
            }
        }
        ArchiveFormat::Rar => {
            let file = File::open(archive_path)?;
            let mut archive = RarArchive::new(file)?;

            // Collect encrypted entries
            let mut encrypted_entries = Vec::new();
            loop {
                let header = match archive.get_next_entry()? {
                    Some(h) => h,
                    None => break,
                };
                if header.is_encrypted && !header.is_directory {
                    encrypted_entries.push((header.name.clone(), header.original_size));
                }
                archive.skip(&header)?;
            }

            if encrypted_entries.is_empty() {
                return Err(ArchiveError::io_error("No encrypted entries found in archive"));
            }

            // Re-open and find requested entry
            let file = File::open(archive_path)?;
            let mut archive = RarArchive::new(file)?;

            if let Some(filter) = entry_filter {
                loop {
                    let header = match archive.get_next_entry()? {
                        Some(h) => h,
                        None => {
                            return Err(ArchiveError::io_error(format!(
                                "Entry '{}' not found or not encrypted. Encrypted entries:\n{}",
                                filter,
                                format_entry_list(&encrypted_entries)
                            )));
                        }
                    };
                    if header.is_encrypted && !header.is_directory && header.name == filter {
                        let verifier = archive.create_password_verifier(&header)?;
                        return Ok((UnifiedPasswordVerifier::Rar(verifier), "RAR"));
                    }
                    archive.skip(&header)?;
                }
            }

            // No filter - take first encrypted entry
            loop {
                let header = match archive.get_next_entry()? {
                    Some(h) => h,
                    None => {
                        return Err(ArchiveError::io_error("No encrypted entries found in archive"));
                    }
                };

                if header.is_encrypted && !header.is_directory {
                    let verifier = archive.create_password_verifier(&header)?;
                    return Ok((UnifiedPasswordVerifier::Rar(verifier), "RAR"));
                } else {
                    archive.skip(&header)?;
                }
            }
        }
        ArchiveFormat::SevenZ => {
            let mut file = File::open(archive_path)?;

            // Read archive data for the verifier
            let mut archive_data = Vec::new();
            file.read_to_end(&mut archive_data)?;
            file.seek(SeekFrom::Start(0))?;
            let mut archive = SevenZArchive::new(file)?;

            // Collect encrypted entries
            let mut encrypted_entries = Vec::new();
            loop {
                let header = match archive.get_next_entry()? {
                    Some(h) => h,
                    None => break,
                };
                if header.is_encrypted && !header.is_directory {
                    encrypted_entries.push((header.name.clone(), header.original_size));
                }
                archive.skip(&header)?;
            }

            if encrypted_entries.is_empty() {
                return Err(ArchiveError::io_error("No encrypted entries found in archive"));
            }

            // Re-open and find requested entry
            let mut file = File::open(archive_path)?;
            file.seek(SeekFrom::Start(0))?;
            let mut archive = SevenZArchive::new(file)?;

            if let Some(filter) = entry_filter {
                loop {
                    let header = match archive.get_next_entry()? {
                        Some(h) => h,
                        None => {
                            return Err(ArchiveError::io_error(format!(
                                "Entry '{}' not found or not encrypted. Encrypted entries:\n{}",
                                filter,
                                format_entry_list(&encrypted_entries)
                            )));
                        }
                    };
                    if header.is_encrypted && !header.is_directory && header.name == filter {
                        let verifier = archive.create_password_verifier(&header)?;
                        return Ok((UnifiedPasswordVerifier::SevenZ(verifier), "7z"));
                    }
                    archive.skip(&header)?;
                }
            }

            // No filter - take first encrypted entry
            loop {
                let header = match archive.get_next_entry()? {
                    Some(h) => h,
                    None => {
                        return Err(ArchiveError::io_error("No encrypted entries found in archive"));
                    }
                };

                if header.is_encrypted && !header.is_directory {
                    let verifier = archive.create_password_verifier(&header)?;
                    return Ok((UnifiedPasswordVerifier::SevenZ(verifier), "7z"));
                } else {
                    archive.skip(&header)?;
                }
            }
        }
        _ => Err(ArchiveError::UnsupportedFormat(format!(
            "Password testing currently only supported for ARC, ARJ, ACE, ZIP, RAR, and 7z archives, not {}",
            format.name()
        ))),
    }
}

/// Format a list of encrypted entries for display
fn format_entry_list(entries: &[(String, u64)]) -> String {
    entries
        .iter()
        .map(|(name, size)| format!("  - {} ({} bytes)", name, size))
        .collect::<Vec<_>>()
        .join("\n")
}

fn run_password_test(
    verifier: &UnifiedPasswordVerifier,
    password_file: Option<&Path>,
    password_dir: Option<&Path>,
    use_stdin: bool,
    verbose_interval: usize,
) -> Result<(), ArchiveError> {
    // Collect password sources
    let password_files: Vec<PathBuf> = if let Some(dir) = password_dir {
        collect_password_files(dir)?
    } else if let Some(file) = password_file {
        if file.to_string_lossy() == "-" {
            vec![] // Will use stdin
        } else {
            vec![file.to_path_buf()]
        }
    } else if use_stdin {
        vec![]
    } else {
        return Err(ArchiveError::Io(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No password source specified. Use --password-file, --password-dir, or --stdin",
        )));
    };

    let use_stdin = use_stdin || password_file.map(|p| p.to_string_lossy() == "-").unwrap_or(false);

    let tested = AtomicUsize::new(0);
    let found = AtomicBool::new(false);
    let total_files = password_files.len();
    let start_time = Instant::now();

    if use_stdin {
        println!("Reading passwords from stdin...");
        let result = try_passwords_from_reader(verifier, io::stdin().lock(), &tested, &found, verbose_interval, None);
        let tested_count = tested.load(Ordering::Relaxed);
        let elapsed = start_time.elapsed();
        if let Some(password) = result {
            eprintln!();
            println!("✓ Found valid password: {}", password);
            println!();
            println!(
                "Tested {} passwords in {:.2}s ({:.0} passwords/sec).",
                tested_count,
                elapsed.as_secs_f64(),
                tested_count as f64 / elapsed.as_secs_f64()
            );
            return Ok(());
        }
    } else {
        println!("Scanning {} password file(s)...", total_files);
        println!();

        for (file_idx, pwd_file_path) in password_files.iter().enumerate() {
            if found.load(Ordering::Relaxed) {
                break;
            }

            let file_name = pwd_file_path
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| pwd_file_path.display().to_string());

            eprint!("\r[{}/{}] {}...", file_idx + 1, total_files, truncate(&file_name, 40));

            let pwd_file = match File::open(pwd_file_path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("\rSkipping {}: {}", pwd_file_path.display(), e);
                    continue;
                }
            };
            let reader = BufReader::new(pwd_file);

            let result = try_passwords_from_reader(verifier, reader, &tested, &found, verbose_interval, Some(&file_name));

            if let Some(password) = result {
                eprintln!(); // Clear progress line
                let tested_count = tested.load(Ordering::Relaxed);
                let elapsed = start_time.elapsed();
                println!();
                println!("✓ Found valid password: {}", password);
                println!("  (in file: {})", pwd_file_path.display());
                println!();
                println!(
                    "Tested {} passwords in {:.2}s ({:.0} passwords/sec) from {} file(s).",
                    tested_count,
                    elapsed.as_secs_f64(),
                    tested_count as f64 / elapsed.as_secs_f64(),
                    file_idx + 1
                );
                return Ok(());
            }
        }
    }

    eprintln!(); // Clear progress line
    let tested_count = tested.load(Ordering::Relaxed);
    let elapsed = start_time.elapsed();
    println!();
    println!(
        "Tested {} passwords in {:.2}s ({:.0} passwords/sec) from {} source(s).",
        tested_count,
        elapsed.as_secs_f64(),
        if elapsed.as_secs_f64() > 0.0 {
            tested_count as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        },
        if use_stdin { 1 } else { total_files }
    );
    println!("No valid password found.");

    Ok(())
}

/// Collect all .txt files recursively from a directory
fn collect_password_files(dir: &Path) -> Result<Vec<PathBuf>, ArchiveError> {
    let mut files = Vec::new();
    collect_password_files_recursive(dir, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_password_files_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), ArchiveError> {
    if !dir.is_dir() {
        return Err(ArchiveError::Io(io::Error::new(
            io::ErrorKind::NotFound,
            format!("'{}' is not a directory", dir.display()),
        )));
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_password_files_recursive(&path, files)?;
        } else if path.is_file() {
            // Accept .txt files and common wordlist extensions
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                if ext == "txt" || ext == "lst" || ext == "dict" || ext == "passwords" {
                    files.push(path);
                }
            }
        }
    }

    Ok(())
}

/// Try passwords from a reader using the efficient password verifier (parallel)
fn try_passwords_from_reader<R: BufRead>(
    verifier: &UnifiedPasswordVerifier,
    reader: R,
    tested: &AtomicUsize,
    found: &AtomicBool,
    verbose_interval: usize,
    current_file: Option<&str>,
) -> Option<String> {
    // Batch size for parallel processing - balance between parallelism and memory
    const BATCH_SIZE: usize = 10000;

    let result: Mutex<Option<String>> = Mutex::new(None);
    let mut batch = Vec::with_capacity(BATCH_SIZE);

    for line in reader.lines() {
        // Early exit if found
        if found.load(Ordering::Relaxed) {
            break;
        }

        let password = match line {
            Ok(p) if !p.is_empty() => p,
            _ => continue,
        };

        batch.push(password);

        // Process batch when full
        if batch.len() >= BATCH_SIZE {
            if let Some(pwd) = process_batch(&batch, verifier, tested, found, verbose_interval, current_file, &result) {
                return Some(pwd);
            }
            batch.clear();
        }
    }

    // Process remaining passwords
    if !batch.is_empty() && !found.load(Ordering::Relaxed) {
        if let Some(pwd) = process_batch(&batch, verifier, tested, found, verbose_interval, current_file, &result) {
            return Some(pwd);
        }
    }

    result.into_inner().unwrap()
}

/// Process a batch of passwords in parallel
fn process_batch(
    batch: &[String],
    verifier: &UnifiedPasswordVerifier,
    tested: &AtomicUsize,
    found: &AtomicBool,
    verbose_interval: usize,
    current_file: Option<&str>,
    result: &Mutex<Option<String>>,
) -> Option<String> {
    batch
        .par_iter()
        .find_any(|password| {
            // Early exit if already found
            if found.load(Ordering::Relaxed) {
                return false;
            }

            let count = tested.fetch_add(1, Ordering::Relaxed) + 1;

            if count.is_multiple_of(verbose_interval) {
                if let Some(file_name) = current_file {
                    eprint!("\r[{}] Tested {} passwords...", truncate(file_name, 30), count);
                } else {
                    eprint!("\rTested {} passwords...", count);
                }
            }

            if verifier.verify(password) {
                found.store(true, Ordering::Relaxed);
                *result.lock().unwrap() = Some(password.to_string());
                true
            } else {
                false
            }
        })
        .cloned()
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
