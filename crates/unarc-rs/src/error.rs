//! Error types for unarc-rs
//!
//! This module provides a unified error type for all archive operations.

use std::path::PathBuf;
use thiserror::Error;

/// Error type for archive operations
#[derive(Debug, Error)]
pub enum ArchiveError {
    /// IO error during archive operations
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Unsupported archive format
    #[error("Unsupported archive format: {0}")]
    UnsupportedFormat(String),

    /// Invalid archive header
    #[error("Invalid {format} archive header")]
    InvalidHeader {
        /// Archive format name (e.g., "ARC", "ARJ", "ZIP")
        format: String,
    },

    /// Corrupted archive entry
    #[error("Corrupted {format} entry '{entry_name}': {reason}")]
    CorruptedEntry {
        /// Archive format name
        format: String,
        /// Name of the corrupted entry
        entry_name: String,
        /// Reason for corruption
        reason: String,
    },

    /// Entry index mismatch
    #[error("Entry index mismatch: {0}")]
    IndexMismatch(String),

    /// CRC checksum mismatch
    #[error("CRC mismatch for '{entry}': expected {expected:#010x}, got {actual:#010x}")]
    CrcMismatch {
        /// Name of the entry with CRC mismatch
        entry: String,
        /// Expected CRC value
        expected: u32,
        /// Actual calculated CRC value
        actual: u32,
    },

    /// Decompression failed
    #[error("Decompression failed for '{entry}': {reason}")]
    DecompressionFailed {
        /// Name of the entry that failed to decompress
        entry: String,
        /// Reason for decompression failure
        reason: String,
    },

    /// Unsupported compression method
    #[error("Unsupported compression method '{method}' in {format} archive")]
    UnsupportedMethod {
        /// Archive format name
        format: String,
        /// Compression method name or identifier
        method: String,
    },

    /// Password required but not provided
    #[error("Archive '{path}' is encrypted and requires a password")]
    PasswordRequired {
        /// Path to the encrypted archive
        path: PathBuf,
    },

    /// Encryption requires password for specific entry
    #[error("Entry '{entry}' in {format} archive is encrypted and requires a password")]
    EncryptionRequired {
        /// Name of the encrypted entry
        entry: String,
        /// Archive format name
        format: String,
    },

    /// Invalid password provided
    #[error("Invalid password for '{entry}' in {format} archive")]
    InvalidPassword {
        /// Name of the entry
        entry: String,
        /// Archive format name
        format: String,
    },

    /// External library error (e.g., from zip, unrar, sevenz-rust2)
    #[error("{library} error: {message}")]
    ExternalLibrary {
        /// Name of the external library
        library: String,
        /// Error message from the library
        message: String,
    },
}

/// Result type for archive operations
pub type Result<T> = std::result::Result<T, ArchiveError>;

// From implementations for external library errors

impl<T: std::io::Read> From<delharc::decode::LhaDecodeError<T>> for ArchiveError {
    fn from(err: delharc::decode::LhaDecodeError<T>) -> Self {
        ArchiveError::ExternalLibrary {
            library: "delharc".to_string(),
            message: format!("{:?}", err),
        }
    }
}

impl From<delharc::LhaError<std::io::Error>> for ArchiveError {
    fn from(err: delharc::LhaError<std::io::Error>) -> Self {
        ArchiveError::ExternalLibrary {
            library: "delharc".to_string(),
            message: format!("{:?}", err),
        }
    }
}

// Allow converting ArchiveError to std::io::Error for compatibility
impl From<ArchiveError> for std::io::Error {
    fn from(err: ArchiveError) -> Self {
        match err {
            ArchiveError::Io(io_err) => io_err,
            other => std::io::Error::other(other.to_string()),
        }
    }
}

impl ArchiveError {
    /// Create an IO error from a string message
    pub fn io_error(message: impl Into<String>) -> Self {
        Self::Io(std::io::Error::other(message.into()))
    }

    /// Create an unsupported format error
    pub fn unsupported_format(path: impl Into<String>) -> Self {
        Self::UnsupportedFormat(path.into())
    }

    /// Create an invalid header error
    pub fn invalid_header(format: impl Into<String>) -> Self {
        Self::InvalidHeader {
            format: format.into(),
        }
    }

    /// Create a corrupted entry error
    pub fn corrupted_entry(format: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::CorruptedEntry {
            format: format.into(),
            entry_name: String::new(),
            reason: reason.into(),
        }
    }

    /// Create a corrupted entry error with entry name
    pub fn corrupted_entry_named(
        format: impl Into<String>,
        entry_name: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::CorruptedEntry {
            format: format.into(),
            entry_name: entry_name.into(),
            reason: reason.into(),
        }
    }

    /// Create a CRC mismatch error
    pub fn crc_mismatch(entry: impl Into<String>, expected: u32, actual: u32) -> Self {
        Self::CrcMismatch {
            entry: entry.into(),
            expected,
            actual,
        }
    }

    /// Create a decompression failed error
    pub fn decompression_failed(entry: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::DecompressionFailed {
            entry: entry.into(),
            reason: reason.into(),
        }
    }

    /// Create an unsupported method error
    pub fn unsupported_method(format: impl Into<String>, method: impl Into<String>) -> Self {
        Self::UnsupportedMethod {
            format: format.into(),
            method: method.into(),
        }
    }

    /// Create an external library error
    pub fn external_library(library: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ExternalLibrary {
            library: library.into(),
            message: message.into(),
        }
    }

    /// Create an encryption required error
    pub fn encryption_required(entry: impl Into<String>, format: impl Into<String>) -> Self {
        Self::EncryptionRequired {
            entry: entry.into(),
            format: format.into(),
        }
    }

    /// Create an invalid password error
    pub fn invalid_password(entry: impl Into<String>, format: impl Into<String>) -> Self {
        Self::InvalidPassword {
            entry: entry.into(),
            format: format.into(),
        }
    }
}
