//! Reader for Lynx containers, the C64 "linked files" format by Will Corley.
//!
//! A Lynx file usually starts with a small BASIC program ("USE LYNX TO
//! DISSOLVE THIS FILE"), followed by a text directory and the files, each
//! padded to a multiple of 254 bytes (a 1541 sector without its link bytes).
//! Numbers in the directory are decimal text terminated by carriage returns.
//!
//! See `doc/lynx.md` for the layout and the supported variants.

use std::io::{Read, Seek, SeekFrom};

use crate::cbm::{entry_name, petscii_to_string, CbmFileType};
use crate::error::{ArchiveError, Result};

/// Data bytes in a 1541 sector; Lynx aligns the directory and every file to this.
const BLOCK_SIZE: u64 = 254;
/// The BASIC loader in front of the directory ends within this many bytes.
const MAX_BASIC_LENGTH: usize = 1024;
/// Upper bound for the directory that is read into memory (more than 4000 blocks).
const MAX_DIRECTORY_SIZE: u64 = 1024 * 1024;
/// Longest header or number line that is accepted.
const MAX_LINE_LENGTH: usize = 80;
/// Longest Commodore file name.
const MAX_NAME_LENGTH: usize = 16;
const CR: u8 = 0x0D;

/// The directory header: block count, signature and number of entries.
struct Header {
    /// Offset just behind the header, where the first directory entry starts.
    end: usize,
    /// Directory size in 254-byte blocks, including the BASIC loader.
    blocks: u32,
    /// Number of directory entries.
    entries: u32,
    signature: String,
}

/// Returns the line starting at `pos` (without its carriage return) and the position behind it.
fn line(buf: &[u8], pos: usize, max_len: usize) -> Option<(&[u8], usize)> {
    let rest = buf.get(pos..)?;
    let len = rest.iter().take(max_len + 1).position(|&b| b == CR)?;
    Some((&rest[..len], pos + len + 1))
}

/// Parses a decimal number with optional leading spaces; returns it and the text behind it.
fn number(line: &[u8]) -> Option<(u32, &[u8])> {
    let start = line.iter().position(|&b| b != b' ')?;
    let digits = line[start..].iter().take_while(|b| b.is_ascii_digit()).count();
    if digits == 0 || digits > 9 {
        return None;
    }
    let value = line[start..start + digits].iter().fold(0u32, |acc, &d| acc * 10 + u32::from(d - b'0'));
    Some((value, &line[start + digits..]))
}

/// Parses the directory header at `pos`.
fn header_at(buf: &[u8], pos: usize) -> Option<Header> {
    let (first, pos) = line(buf, pos, MAX_LINE_LENGTH)?;
    let (blocks, signature) = number(first)?;
    let (second, end) = line(buf, pos, MAX_LINE_LENGTH)?;
    let (entries, _) = number(second)?;
    if blocks == 0 || entries == 0 {
        return None;
    }
    let signature = petscii_to_string(signature.trim_ascii());
    Some(Header {
        end,
        blocks,
        entries,
        signature,
    })
}

/// Finds the directory header, either at the start of the file or behind the BASIC loader.
///
/// The loader ends with the end-of-line and end-of-program markers (three zero
/// bytes), usually followed by a carriage return.
fn find_header(buf: &[u8]) -> Option<Header> {
    header_at(buf, 0).or_else(|| {
        let search = &buf[..buf.len().min(MAX_BASIC_LENGTH)];
        let mut pos = search.windows(3).position(|w| w == [0, 0, 0])?;
        while buf.get(pos) == Some(&0) {
            pos += 1;
        }
        if buf.get(pos) == Some(&CR) {
            pos += 1;
        }
        header_at(buf, pos)
    })
}

/// Returns true if `data` (the start of a file) holds a Lynx header whose signature mentions "LYNX".
pub(crate) fn has_signature(data: &[u8]) -> bool {
    find_header(data).is_some_and(|header| header.signature.to_ascii_uppercase().contains("LYNX"))
}

/// A file in a Lynx container.
#[derive(Debug, Clone)]
pub struct LynxEntry {
    /// Entry name: the converted PETSCII name plus an extension for the file type, e.g. `GAME.prg`.
    pub name: String,
    /// The raw PETSCII name from the directory (up to 16 bytes, usually padded with `0xA0`).
    pub petscii_name: Vec<u8>,
    /// CBM file type.
    pub file_type: CbmFileType,
    /// Size in 254-byte blocks, including the side sectors of a REL file.
    pub blocks: u32,
    /// Number of used bytes in the last block plus one, if recorded.
    pub last_block: Option<u8>,
    /// Record length of a REL file, otherwise 0.
    pub record_length: u8,
    /// Offset of the file data in the container.
    pub offset: u64,
    /// File size in bytes.
    pub size: u64,
}

/// Lynx container reader.
pub struct LynxArchive<T: Read + Seek> {
    reader: T,
    start: u64,
    len: u64,
    signature: String,
    /// The directory blocks, from the start of the file.
    directory: Vec<u8>,
    /// Parse position of the next directory entry.
    pos: usize,
    entries: u32,
    index: u32,
    /// Offset of the next file's data.
    data_pos: u64,
}

impl<T: Read + Seek> LynxArchive<T> {
    /// Opens a Lynx container and reads its directory header.
    pub fn new(mut reader: T) -> Result<Self> {
        let start = reader.stream_position()?;
        let len = reader.seek(SeekFrom::End(0))?.saturating_sub(start);
        reader.seek(SeekFrom::Start(start))?;

        let mut prefix = Vec::new();
        (&mut reader).take((MAX_BASIC_LENGTH + 2 * MAX_LINE_LENGTH) as u64).read_to_end(&mut prefix)?;
        let header = find_header(&prefix).ok_or_else(|| ArchiveError::invalid_header("Lynx"))?;
        let data_pos = u64::from(header.blocks) * BLOCK_SIZE;
        if data_pos < header.end as u64 {
            return Err(ArchiveError::invalid_header("Lynx"));
        }

        // The entries live between the header and the first file.
        reader.seek(SeekFrom::Start(start))?;
        let mut directory = Vec::new();
        (&mut reader).take(data_pos.min(MAX_DIRECTORY_SIZE)).read_to_end(&mut directory)?;

        Ok(Self {
            reader,
            start,
            len,
            signature: header.signature,
            directory,
            pos: header.end,
            entries: header.entries,
            index: 0,
            data_pos,
        })
    }

    /// The signature text of the header, e.g. `*LYNX XV  BY WILL CORLEY`.
    pub fn signature(&self) -> &str {
        &self.signature
    }

    /// Number of entries announced by the header.
    pub fn entry_count(&self) -> u32 {
        self.entries
    }

    fn directory_error(&self, reason: &str) -> ArchiveError {
        ArchiveError::corrupted_entry("Lynx", format!("directory entry {}: {reason}", self.index + 1))
    }

    fn number_line(&mut self) -> Option<u32> {
        let (text, next) = line(&self.directory, self.pos, MAX_LINE_LENGTH)?;
        let (value, _) = number(text)?;
        self.pos = next;
        Some(value)
    }

    /// Returns the next directory entry, or `None` after the last one.
    ///
    /// After a malformed entry the rest of the directory cannot be located, so
    /// the error is returned once and the iteration ends.
    pub fn get_next_entry(&mut self) -> Result<Option<LynxEntry>> {
        if self.index >= self.entries {
            return Ok(None);
        }
        let result = self.parse_next_entry();
        if result.is_err() {
            self.index = self.entries;
        }
        result.map(Some)
    }

    fn parse_next_entry(&mut self) -> Result<LynxEntry> {
        let is_last = self.index + 1 == self.entries;

        let (petscii_name, next) = line(&self.directory, self.pos, MAX_NAME_LENGTH).ok_or_else(|| self.directory_error("bad file name"))?;
        let petscii_name = petscii_name.to_vec();
        self.pos = next;
        let blocks = self.number_line().ok_or_else(|| self.directory_error("bad block count"))?;
        let (type_line, next) = line(&self.directory, self.pos, MAX_LINE_LENGTH).ok_or_else(|| self.directory_error("bad file type"))?;
        let type_char = type_line.trim_ascii().first().copied().ok_or_else(|| self.directory_error("bad file type"))?;
        self.pos = next;
        let file_type = match type_char {
            b'D' => CbmFileType::Del,
            b'S' => CbmFileType::Seq,
            b'U' => CbmFileType::Usr,
            b'R' => CbmFileType::Rel,
            _ => CbmFileType::Prg,
        };

        let mut record_length = 0;
        let mut data_blocks = blocks;
        let mut offset = self.data_pos;
        if file_type == CbmFileType::Rel {
            let length = self.number_line().ok_or_else(|| self.directory_error("bad record length"))?;
            record_length = u8::try_from(length).map_err(|_| self.directory_error("bad record length"))?;
            // The side sectors (one per 120 data blocks) are stored in front of the data.
            let side_sectors = blocks.saturating_add(119) / 121;
            if side_sectors == 0 || blocks < 121 * side_sectors - 119 {
                return Err(self.directory_error("bad REL block count"));
            }
            data_blocks = blocks - side_sectors;
            offset = offset.saturating_add(u64::from(side_sectors) * BLOCK_SIZE);
        }

        // Some archivers omit the last block size of the last file.
        let last_block = match self.number_line() {
            Some(value) => Some(u8::try_from(value).map_err(|_| self.directory_error("bad last block size"))?),
            None if is_last => None,
            None => return Err(self.directory_error("missing last block size")),
        };

        let size = match (data_blocks, last_block) {
            (0, _) => 0,
            // The last block holds `last - 1` bytes, so values below 2 are invalid.
            (_, Some(0 | 1)) => return Err(self.directory_error("bad last block size")),
            (blocks, Some(last)) => u64::from(blocks - 1) * BLOCK_SIZE + u64::from(last) - 1,
            (blocks, None) => (u64::from(blocks) * BLOCK_SIZE).min(self.len.saturating_sub(offset)),
        };
        // Some archivers (Lynx XVI, Star Lynx) write the container a few bytes short of the
        // last file's recorded size. c1541 and cbmconvert keep what is there; so do we, but only
        // when the container ends inside that file's last block, so a real truncation still fails.
        let available = self.len.saturating_sub(offset);
        let size = if is_last && size > available && size - available < BLOCK_SIZE {
            available
        } else {
            size
        };
        self.data_pos = self.data_pos.saturating_add(u64::from(blocks) * BLOCK_SIZE);
        self.index += 1;

        Ok(LynxEntry {
            name: entry_name(&petscii_name, file_type),
            petscii_name,
            file_type,
            blocks,
            last_block,
            record_length,
            offset,
            size,
        })
    }

    /// Reads a file. PRG files include their two-byte load address.
    pub fn read(&mut self, entry: &LynxEntry) -> Result<Vec<u8>> {
        if entry.offset.saturating_add(entry.size) > self.len {
            return Err(ArchiveError::corrupted_entry_named(
                "Lynx",
                &entry.name,
                "data extends past the end of the container",
            ));
        }
        self.reader.seek(SeekFrom::Start(self.start + entry.offset))?;
        Ok(crate::limits::read_exact_vec(&mut self.reader, entry.size)?)
    }

    /// Skips an entry. Entries are located by offset, so there is nothing to do.
    pub fn skip(&mut self, _entry: &LynxEntry) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        assert_eq!(number(b" 12 "), Some((12, &b" "[..])));
        assert_eq!(number(b"7"), Some((7, &b""[..])));
        assert_eq!(number(b" 1  *LYNX"), Some((1, &b"  *LYNX"[..])));
        assert_eq!(number(b" P"), None);
        assert_eq!(number(b"   "), None);
        assert_eq!(number(b"1234567890"), None);
    }

    #[test]
    fn header_with_and_without_basic_loader() {
        let bare = b" 1  *LYNX XV  BY WILL CORLEY\r 3 \r";
        let header = find_header(bare).unwrap();
        assert_eq!((header.blocks, header.entries, header.end), (1, 3, bare.len()));
        assert_eq!(header.signature, "*LYNX XV  BY WILL CORLEY");
        assert!(has_signature(bare));

        let mut with_basic = vec![0x01, 0x08, 0x0B, 0x08, 0x0A, 0x00, 0x99, 0x22, 0x48, 0x22, 0x00, 0x00, 0x00, CR];
        with_basic.extend_from_slice(bare);
        assert!(has_signature(&with_basic));
        assert_eq!(find_header(&with_basic).unwrap().end, with_basic.len());
    }

    #[test]
    fn rejects_non_lynx_headers() {
        assert!(!has_signature(b" 1  SOMETHING ELSE\r 3 \r"));
        assert!(!has_signature(b" 0  *LYNX\r 3 \r"));
        assert!(!has_signature(b" 1  *LYNX\r 0 \r"));
        assert!(!has_signature(b"PK\x03\x04 1  *LYNX\r 3 \r"));
        assert!(!has_signature(b""));
    }
}
