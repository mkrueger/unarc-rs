//! The machine-readable form of `unarc list`.
//!
//! Field names are part of the CLI's interface: add fields freely, but don't
//! rename or remove one without a major version bump.
use serde::Serialize;
use unarc_rs::unified::{ArchiveEntry, ArchiveEntryKind};

#[derive(Serialize)]
pub struct Listing {
    /// The archive path as given on the command line
    pub archive: String,
    /// Detected format, as `unarc formats` names it
    pub format: &'static str,
    pub entries: Vec<Entry>,
}

#[derive(Serialize)]
pub struct Entry {
    /// Full name as stored, including any path; not sanitized
    pub name: String,
    pub kind: &'static str,
    /// Original (uncompressed) size in bytes
    pub size: u64,
    pub compressed_size: u64,
    pub method: String,
    /// As recorded, without a time zone: `YYYY-MM-DDTHH:MM:SS`
    pub modified: Option<String>,
    pub crc: u64,
    pub encrypted: bool,
    pub encryption: Option<String>,
    pub link_target: Option<String>,
}

impl Listing {
    pub fn to_json(&self) -> String {
        // Only owned strings and integers: serialization cannot fail
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
}

impl From<&ArchiveEntry> for Entry {
    fn from(entry: &ArchiveEntry) -> Self {
        Self {
            name: entry.name().to_string(),
            kind: kind_name(entry.kind()),
            size: entry.original_size(),
            compressed_size: entry.compressed_size(),
            method: entry.compression_method().to_string(),
            modified: entry.modified_time().map(|t| {
                format!(
                    "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
                    t.year(),
                    t.month(),
                    t.day(),
                    t.hour(),
                    t.minute(),
                    t.second()
                )
            }),
            crc: entry.crc(),
            encrypted: entry.is_encrypted(),
            encryption: entry.is_encrypted().then(|| entry.encryption().to_string()),
            link_target: entry.link_target().map(str::to_string),
        }
    }
}

const fn kind_name(kind: ArchiveEntryKind) -> &'static str {
    match kind {
        ArchiveEntryKind::File => "file",
        ArchiveEntryKind::Directory => "directory",
        ArchiveEntryKind::SymbolicLink => "symlink",
        ArchiveEntryKind::HardLink => "hardlink",
        ArchiveEntryKind::Special => "special",
        ArchiveEntryKind::Unknown => "unknown",
    }
}
