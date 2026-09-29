//! Helpers that keep memory use proportional to the actual input instead of
//! trusting sizes stored in archive headers.

use std::io::{self, Read};

use crate::error::{ArchiveError, Result};

/// Default per-entry size limit applied by the unified API (1 GiB).
pub const DEFAULT_MAX_ENTRY_SIZE: u64 = 1024 * 1024 * 1024;

/// Upper bound for buffer pre-allocation based on header sizes.
const MAX_PREALLOCATION: u64 = 16 * 1024 * 1024;

/// Initial capacity for a buffer whose final size comes from an untrusted header.
pub(crate) fn capacity_hint(size: u64) -> usize {
    usize::try_from(size.min(MAX_PREALLOCATION)).unwrap_or(0)
}

/// Reads exactly `len` bytes.
///
/// The buffer grows as data arrives, so a header claiming more data than the
/// file contains cannot force a huge allocation.
pub(crate) fn read_exact_vec<R: Read + ?Sized>(reader: &mut R, len: u64) -> io::Result<Vec<u8>> {
    let mut data = Vec::with_capacity(capacity_hint(len));
    Read::take(reader, len).read_to_end(&mut data)?;
    if (data.len() as u64) < len {
        return Err(io::ErrorKind::UnexpectedEof.into());
    }
    Ok(data)
}

/// Reads a (decompressing) stream to its end, failing once it produces more than `limit` bytes.
pub(crate) fn read_to_end_limited<R: Read>(reader: R, limit: Option<u64>, entry: &str) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    let Some(limit) = limit else {
        let mut reader = reader;
        reader.read_to_end(&mut data)?;
        return Ok(data);
    };
    reader.take(limit.saturating_add(1)).read_to_end(&mut data)?;
    if data.len() as u64 > limit {
        return Err(ArchiveError::size_limit_exceeded(entry, limit));
    }
    Ok(data)
}

/// Fails if a size announced by a header exceeds `limit`.
pub(crate) fn check_size(size: u64, limit: Option<u64>, entry: &str) -> Result<()> {
    match limit {
        Some(limit) if size > limit => Err(ArchiveError::size_limit_exceeded(entry, limit)),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_exact_vec_does_not_trust_length() {
        let mut reader: &[u8] = b"abc";
        assert_eq!(read_exact_vec(&mut reader, u64::MAX).unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
        let mut reader: &[u8] = b"abcdef";
        assert_eq!(read_exact_vec(&mut reader, 3).unwrap(), b"abc");
        assert_eq!(reader, b"def");
    }

    #[test]
    fn read_to_end_limited_enforces_limit() {
        assert_eq!(read_to_end_limited(&b"abc"[..], Some(3), "x").unwrap(), b"abc");
        assert!(matches!(
            read_to_end_limited(&b"abcd"[..], Some(3), "x"),
            Err(ArchiveError::SizeLimitExceeded { limit: 3, .. })
        ));
        assert_eq!(read_to_end_limited(&b"abcd"[..], None, "x").unwrap(), b"abcd");
    }
}
