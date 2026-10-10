//! Reader for T64 tape images, the container format of the C64S emulator.
//!
//! A T64 file has a 64-byte header, a directory of 32-byte records and the
//! file data. Each record holds the load (start) and end address and the
//! offset of the data, which is stored without the two-byte load address.
//!
//! See `doc/t64.md` for the layout.

use std::io::{Read, Seek, SeekFrom};

use crate::cbm::{entry_name, petscii_to_string, CbmFileType};
use crate::error::{ArchiveError, Result};

const HEADER_SIZE: u64 = 64;
const RECORD_SIZE: u64 = 32;
/// A C64 program cannot be larger than the 64 KiB address space.
const ADDRESS_SPACE: u64 = 0x1_0000;

/// Returns true if `header` starts with a T64 signature.
///
/// Signatures vary between tools ("C64S tape file", "C64 tape image file",
/// "C64S tape image file"), so this accepts "C64 " or "C64S" followed by
/// "tape" within the 32-byte signature field. Raw tape images (TAP) start with
/// "C64-TAPE-RAW" and are rejected.
pub(crate) fn has_signature(header: &[u8]) -> bool {
    header.len() >= 32 && (header.starts_with(b"C64 ") || header.starts_with(b"C64S")) && header[..32].windows(4).any(|w| w.eq_ignore_ascii_case(b"tape"))
}

/// A file in a T64 tape image.
#[derive(Debug, Clone)]
pub struct T64Entry {
    /// Entry name: the converted PETSCII name plus an extension for the file type, e.g. `GAME.prg`.
    pub name: String,
    /// The raw PETSCII name from the directory, padded with spaces.
    pub petscii_name: [u8; 16],
    /// CBM file type.
    pub file_type: CbmFileType,
    /// C64S entry type (1 = normal tape file).
    pub entry_type: u8,
    /// The raw 1541 file type byte.
    pub type_byte: u8,
    /// Load address, prepended to the extracted data.
    pub start_address: u16,
    /// End address as recorded in the directory (often wrong, see `data_size`).
    pub end_address: u16,
    /// Offset of the file data in the image.
    pub offset: u32,
    /// Stored data bytes, determined from the data offsets and the image size.
    pub data_size: u64,
}

impl T64Entry {
    /// Size of the extracted file: the load address plus the stored data.
    pub fn size(&self) -> u64 {
        self.data_size + 2
    }
}

/// T64 tape image reader.
pub struct T64Archive<T: Read + Seek> {
    reader: T,
    start: u64,
    len: u64,
    description: String,
    entries: Vec<T64Entry>,
    next: usize,
}

impl<T: Read + Seek> T64Archive<T> {
    /// Opens a T64 image and reads its directory.
    pub fn new(mut reader: T) -> Result<Self> {
        let start = reader.stream_position()?;
        let len = reader.seek(SeekFrom::End(0))?.saturating_sub(start);
        reader.seek(SeekFrom::Start(start))?;

        let mut header = [0; HEADER_SIZE as usize];
        reader.read_exact(&mut header).map_err(|_| ArchiveError::invalid_header("T64"))?;
        if !has_signature(&header) {
            return Err(ArchiveError::invalid_header("T64"));
        }
        let max_entries = u16::from_le_bytes([header[34], header[35]]);
        let used_entries = u16::from_le_bytes([header[36], header[37]]);
        let description = petscii_to_string(trim_padding(&header[40..64]));

        // Many images have wrong entry counts, so read every record up to the
        // largest count, but never past the end of the file or into file data.
        let records = u64::from(max_entries.max(used_entries).max(1));
        let mut entries = Vec::new();
        let mut data_start = len;
        for index in 0..records {
            let position = HEADER_SIZE + index * RECORD_SIZE;
            if position + RECORD_SIZE > data_start {
                break;
            }
            let mut record = [0; RECORD_SIZE as usize];
            reader.read_exact(&mut record)?;
            if record[0] == 0 {
                // Free entry.
                continue;
            }
            let offset = u32::from_le_bytes([record[8], record[9], record[10], record[11]]);
            if offset_is_valid(offset, len) {
                data_start = data_start.min(u64::from(offset));
            }
            let type_byte = record[1];
            let file_type = match type_byte & 0x8F {
                0x80 => CbmFileType::Del,
                0x81 => CbmFileType::Seq,
                0x83 => CbmFileType::Usr,
                _ => CbmFileType::Prg,
            };
            let mut petscii_name = [0; 16];
            petscii_name.copy_from_slice(&record[16..32]);
            entries.push(T64Entry {
                name: entry_name(trim_padding(&petscii_name), file_type),
                petscii_name,
                file_type,
                entry_type: record[0],
                type_byte,
                start_address: u16::from_le_bytes([record[2], record[3]]),
                end_address: u16::from_le_bytes([record[4], record[5]]),
                offset,
                data_size: 0,
            });
        }
        compute_data_sizes(&mut entries, len);

        Ok(Self {
            reader,
            start,
            len,
            description,
            entries,
            next: 0,
        })
    }

    /// The tape name from the header.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the next directory entry, or `None` after the last one.
    pub fn get_next_entry(&mut self) -> Result<Option<T64Entry>> {
        let entry = self.entries.get(self.next).cloned();
        if entry.is_some() {
            self.next += 1;
        }
        Ok(entry)
    }

    /// Reads a file as a standard `.prg`: the load address followed by the stored data.
    pub fn read(&mut self, entry: &T64Entry) -> Result<Vec<u8>> {
        if !offset_is_valid(entry.offset, self.len) || u64::from(entry.offset) + entry.data_size > self.len {
            return Err(ArchiveError::corrupted_entry_named(
                "T64",
                &entry.name,
                format!("data offset {} is outside the image", entry.offset),
            ));
        }
        self.reader.seek(SeekFrom::Start(self.start + u64::from(entry.offset)))?;
        let mut data = Vec::with_capacity(crate::limits::capacity_hint(entry.size()));
        data.extend_from_slice(&entry.start_address.to_le_bytes());
        data.extend(crate::limits::read_exact_vec(&mut self.reader, entry.data_size)?);
        Ok(data)
    }

    /// Skips an entry. Entries are located by offset, so there is nothing to do.
    pub fn skip(&mut self, _entry: &T64Entry) -> Result<()> {
        Ok(())
    }
}

/// File data must follow the header and the first directory record and start within the image.
fn offset_is_valid(offset: u32, len: u64) -> bool {
    (HEADER_SIZE + RECORD_SIZE..=len).contains(&u64::from(offset))
}

/// Removes the space, NUL or shifted-space padding at the end of a T64 name.
fn trim_padding(name: &[u8]) -> &[u8] {
    let end = name.iter().rposition(|&b| !matches!(b, b' ' | 0 | 0xA0)).map_or(0, |i| i + 1);
    &name[..end]
}

/// Determines how much data each entry stores.
///
/// Many images have a wrong end address (CONV64 wrote `$C3C6` for every file),
/// so, like VICE, the size of a file is the distance to the next file's data.
/// The last file ends at the end of the image, or earlier if its address range
/// says so (some images have trailing garbage).
fn compute_data_sizes(entries: &mut [T64Entry], len: u64) {
    let mut offsets: Vec<u64> = entries.iter().filter(|e| offset_is_valid(e.offset, len)).map(|e| u64::from(e.offset)).collect();
    offsets.sort_unstable();
    offsets.dedup();
    for entry in entries {
        let offset = u64::from(entry.offset);
        let next = offsets.iter().copied().find(|&o| o > offset);
        let gap = next.unwrap_or(len).saturating_sub(offset);
        let start = u64::from(entry.start_address);
        let end = match entry.end_address {
            0 => ADDRESS_SPACE,
            end => u64::from(end),
        };
        let recorded = end.checked_sub(start).filter(|&size| size > 0);
        let size = match (next, recorded) {
            (None, Some(recorded)) => recorded.min(gap),
            _ => gap,
        };
        entry.data_size = size.min(ADDRESS_SPACE - start);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signatures() {
        let pad = |s: &[u8]| {
            let mut v = s.to_vec();
            v.resize(32, 0);
            v
        };
        assert!(has_signature(&pad(b"C64S tape image file")));
        assert!(has_signature(&pad(b"C64S tape file")));
        assert!(has_signature(&pad(b"C64 tape image file")));
        assert!(!has_signature(&pad(b"C64-TAPE-RAW")));
        assert!(!has_signature(&pad(b"C64 cartridge")));
        assert!(!has_signature(b"C64S tape"));
    }

    #[test]
    fn padding_is_trimmed() {
        assert_eq!(trim_padding(b"HELLO           "), b"HELLO");
        assert_eq!(trim_padding(b"A B \0\0"), b"A B");
        assert_eq!(trim_padding(b"    "), b"");
    }
}
