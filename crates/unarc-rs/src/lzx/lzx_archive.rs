//! Amiga LZX archive reader

use std::fmt;
use std::io::{BufReader, Read, Seek, SeekFrom};

use crate::date_time::DosDateTime;
use crate::error::{ArchiveError, Result};

use super::decoder::{DecodeError, Decoder};

const INFO_HEADER_SIZE: u64 = 10;
const ENTRY_HEADER_SIZE: usize = 31;
/// Entry header flag set on every entry of a merged group
const MERGED_FLAG: u8 = 1;

/// How an entry's data is stored
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackMode {
    /// Uncompressed (pack mode 0)
    Stored,
    /// LZX compressed (pack mode 2)
    Normal,
    /// Any other pack mode; such entries are listed but cannot be read
    Unknown(u8),
}

impl From<u8> for PackMode {
    fn from(mode: u8) -> Self {
        match mode {
            0 => Self::Stored,
            2 => Self::Normal,
            mode => Self::Unknown(mode),
        }
    }
}

impl fmt::Display for PackMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stored => write!(f, "Stored"),
            Self::Normal => write!(f, "LZX"),
            Self::Unknown(mode) => write!(f, "Unknown({mode})"),
        }
    }
}

/// A file in an Amiga LZX archive
#[derive(Debug, Clone, PartialEq)]
pub struct LzxEntry {
    /// Path, with `/` separators. Bytes are mapped as Latin-1.
    pub name: String,
    /// File comment (Amiga file note), mapped as Latin-1
    pub comment: String,
    /// Amiga protection bits as stored: bit 0 read, 1 write, 2 delete, 3 execute,
    /// 4 archived, 5 hidden, 6 script, 7 pure. See [`LzxEntry::protection`].
    pub attributes: u8,
    /// Uncompressed size
    pub original_size: u32,
    /// Compressed size from the header. In a merged group this is 0 except on the
    /// group's last entry, where it covers the whole group.
    pub packed_size: u32,
    /// Pack mode of the entry header; a merged group is decoded with the mode of its last entry
    pub pack_mode: PackMode,
    /// True if the entry shares its compressed stream with other entries (a merged group)
    pub merged: bool,
    /// Modification time, if representable as a DOS date (1980 or later)
    pub modified_time: Option<DosDateTime>,
    /// CRC-32 of the uncompressed data
    pub crc32: u32,
    /// Index of the entry's group (compressed stream)
    group: usize,
    /// Offset of the entry's data in its group's uncompressed stream
    group_offset: u64,
}

impl LzxEntry {
    /// Formats the protection bits like the Amiga `list` command: `hspa`, then `rwed`
    pub fn protection(&self) -> String {
        [(5, 'h'), (6, 's'), (7, 'p'), (4, 'a'), (0, 'r'), (1, 'w'), (3, 'e'), (2, 'd')]
            .iter()
            .map(|&(bit, flag)| if self.attributes & (1 << bit) != 0 { flag } else { '-' })
            .collect()
    }
}

/// One compressed stream: a single file, or several files merged into one stream
#[derive(Debug)]
struct Group {
    /// Archive offset and size of the packed data; `None` if the archive ends before the group's data
    data: Option<(u64, u64)>,
    pack_mode: PackMode,
    /// Total uncompressed size of the group's files
    size: u64,
}

/// Amiga LZX archive reader
///
/// LZX can compress several files as one stream ("merged" groups). Reading a file
/// of such a group decodes the group up to the end of that file; reading the files
/// of a group in order continues where the previous read stopped, while reading an
/// earlier file restarts the group.
pub struct LzxArchive<R: Read + Seek> {
    reader: R,
    entries: Vec<LzxEntry>,
    groups: Vec<Group>,
    /// Error that ended the header scan, returned after the entries before it
    error: Option<ArchiveError>,
    next_entry: usize,
    cursor: Option<GroupCursor>,
}

struct GroupCursor {
    group: usize,
    decoder: Box<Decoder>,
}

fn corrupt(name: &str, reason: impl Into<String>) -> ArchiveError {
    ArchiveError::corrupted_entry_named("LZX", name, reason)
}

fn latin1(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| char::from(b)).collect()
}

/// Converts a packed LZX date stamp (day, month, year, hour, minute, second, from the
/// most significant bits). Invalid dates and years before 1980 are treated as missing.
fn modified_time(stamp: u32) -> Option<DosDateTime> {
    let day = stamp >> 27;
    let month = (stamp >> 23 & 0xF) + 1;
    let hour = stamp >> 12 & 0x1F;
    let minute = stamp >> 6 & 0x3F;
    let second = stamp & 0x3F;
    // The 6-bit year field wraps around; see doc/lzx.md.
    let year = match stamp >> 17 & 0x3F {
        field @ 0..=7 => field + 2034,
        field @ 8..=29 => field + 1970,
        field @ 30..=57 => field + 1976,
        field => field + 1942,
    };
    if !(1..=31).contains(&day) || month > 12 || hour > 23 || minute > 59 || second > 59 || year < 1980 {
        return None;
    }
    Some(DosDateTime::new(
        (year - 1980) << 25 | month << 21 | day << 16 | hour << 11 | minute << 5 | (second / 2),
    ))
}

impl<R: Read + Seek> LzxArchive<R> {
    /// Opens an archive and reads all entry headers
    ///
    /// If a damaged header follows valid ones, the entries before it are still
    /// returned, then [`get_next_entry`](Self::get_next_entry) reports the error.
    pub fn new(mut reader: R) -> Result<Self> {
        let start = reader.stream_position()?;
        let end = reader.seek(SeekFrom::End(0))?;
        reader.seek(SeekFrom::Start(start))?;

        let (entries, groups, error) = {
            let mut buffered = BufReader::new(&mut reader);
            let mut info = [0u8; INFO_HEADER_SIZE as usize];
            if buffered.read_exact(&mut info).is_err() || &info[..3] != b"LZX" {
                return Err(ArchiveError::invalid_header("LZX"));
            }
            let mut scan = HeaderScan {
                position: start + INFO_HEADER_SIZE,
                end,
                entries: Vec::new(),
                groups: Vec::new(),
                open_group: Vec::new(),
                open_size: 0,
            };
            let error = scan.run(&mut buffered).err();
            scan.close_unterminated_group();
            if scan.entries.is_empty() {
                if let Some(error) = error {
                    return Err(error);
                }
            }
            (scan.entries, scan.groups, error)
        };

        Ok(Self {
            reader,
            entries,
            groups,
            error,
            next_entry: 0,
            cursor: None,
        })
    }

    /// Returns the next entry, or `None` after the last one
    pub fn get_next_entry(&mut self) -> Result<Option<LzxEntry>> {
        match self.entries.get(self.next_entry) {
            Some(entry) => {
                self.next_entry += 1;
                Ok(Some(entry.clone()))
            }
            None => self.error.take().map_or(Ok(None), Err),
        }
    }

    /// Skips an entry; data is only decoded when an entry is read
    pub const fn skip(&mut self, _header: &LzxEntry) -> Result<()> {
        Ok(())
    }

    /// Reads, decompresses and CRC-checks an entry
    pub fn read(&mut self, header: &LzxEntry) -> Result<Vec<u8>> {
        self.read_with_limit(header, None)
    }

    /// Reads, decompresses and CRC-checks an entry, failing if it is larger than `limit` bytes
    ///
    /// The output never exceeds the size recorded for the entry, which is checked
    /// against `limit` before anything is decoded.
    pub fn read_with_limit(&mut self, header: &LzxEntry, limit: Option<u64>) -> Result<Vec<u8>> {
        let size = u64::from(header.original_size);
        crate::limits::check_size(size, limit, &header.name)?;
        let group = self
            .groups
            .get(header.group)
            .filter(|group| header.group_offset + size <= group.size)
            .ok_or_else(|| ArchiveError::IndexMismatch(format!("LZX entry '{}' does not belong to this archive", header.name)))?;

        let data = if size == 0 {
            Vec::new()
        } else {
            let Some((offset, packed)) = group.data else {
                return Err(corrupt(&header.name, "the archive ends before the entry's data"));
            };
            match group.pack_mode {
                PackMode::Stored => {
                    if header.group_offset + size > packed {
                        return Err(corrupt(&header.name, "stored data is shorter than the entry"));
                    }
                    self.reader.seek(SeekFrom::Start(offset + header.group_offset))?;
                    crate::limits::read_exact_vec(&mut self.reader, size)?
                }
                PackMode::Normal => self.decode(header, offset, packed)?,
                PackMode::Unknown(mode) => return Err(ArchiveError::unsupported_method("LZX", format!("pack mode {mode}"))),
            }
        };

        let crc = crc32fast::hash(&data);
        if crc != header.crc32 {
            return Err(ArchiveError::crc_mismatch(&header.name, header.crc32, crc));
        }
        Ok(data)
    }

    fn decode(&mut self, header: &LzxEntry, offset: u64, packed: u64) -> Result<Vec<u8>> {
        let start = header.group_offset;
        let cursor = match self.cursor.take() {
            Some(cursor) if cursor.group == header.group && cursor.decoder.position() <= start => cursor,
            _ => GroupCursor {
                group: header.group,
                decoder: Box::new(Decoder::new(offset, packed)),
            },
        };
        let cursor = self.cursor.insert(cursor);
        let mut data = Vec::with_capacity(crate::limits::capacity_hint(u64::from(header.original_size)));
        let skip = start - cursor.decoder.position();
        let result = cursor.decoder.read(&mut self.reader, skip, &mut |_| {}).and_then(|()| {
            cursor
                .decoder
                .read(&mut self.reader, u64::from(header.original_size), &mut |bytes| data.extend_from_slice(bytes))
        });
        match result {
            Ok(()) => Ok(data),
            Err(error) => {
                // The decoder state is unusable after an error; the next read starts over.
                self.cursor = None;
                Err(match error {
                    DecodeError::Io(e) => ArchiveError::Io(e),
                    DecodeError::Corrupt(reason) => corrupt(&header.name, reason),
                })
            }
        }
    }
}

/// Reads the entry headers and assigns entries to groups
struct HeaderScan {
    position: u64,
    end: u64,
    entries: Vec<LzxEntry>,
    groups: Vec<Group>,
    /// Entries of the current group, which ends with the next entry that has packed data
    open_group: Vec<usize>,
    open_size: u64,
}

impl HeaderScan {
    fn run<R: Read + Seek>(&mut self, reader: &mut BufReader<R>) -> Result<()> {
        // A tail too short for a header is ignored (padding added by transfer protocols).
        while self.end - self.position >= ENTRY_HEADER_SIZE as u64 {
            let mut header = [0u8; ENTRY_HEADER_SIZE];
            reader.read_exact(&mut header)?;
            let mut name = vec![0; usize::from(header[30])];
            let mut comment = vec![0; usize::from(header[14])];
            reader.read_exact(&mut name)?;
            reader.read_exact(&mut comment)?;
            let name_text = latin1(&name);

            let stored_crc = u32::from_le_bytes([header[26], header[27], header[28], header[29]]);
            header[26..30].fill(0);
            let mut hasher = crc32fast::Hasher::new();
            hasher.update(&header);
            hasher.update(&name);
            hasher.update(&comment);
            if hasher.finalize() != stored_crc {
                return Err(corrupt(&name_text, "header CRC mismatch"));
            }

            let original_size = u32::from_le_bytes([header[2], header[3], header[4], header[5]]);
            let packed_size = u32::from_le_bytes([header[6], header[7], header[8], header[9]]);
            let data_offset = self.position + (ENTRY_HEADER_SIZE + name.len() + comment.len()) as u64;
            self.open_group.push(self.entries.len());
            self.entries.push(LzxEntry {
                name: name_text,
                comment: latin1(&comment),
                attributes: header[0],
                original_size,
                packed_size,
                pack_mode: PackMode::from(header[11]),
                merged: false,
                modified_time: modified_time(u32::from_be_bytes([header[18], header[19], header[20], header[21]])),
                crc32: u32::from_le_bytes([header[22], header[23], header[24], header[25]]),
                group: self.groups.len(),
                group_offset: self.open_size,
            });
            self.open_size += u64::from(original_size);

            let data_end = data_offset + u64::from(packed_size);
            if data_end > self.end {
                return Err(corrupt(
                    &self.entries[self.entries.len() - 1].name,
                    "entry data extends past the end of the archive",
                ));
            }
            self.position = data_end;
            if packed_size > 0 {
                // The entry with packed data ends its group.
                self.close_group(Some((data_offset, u64::from(packed_size))), PackMode::from(header[11]));
                reader.seek_relative(i64::from(packed_size))?;
            } else if original_size == 0 && header[12] & MERGED_FLAG == 0 && self.open_group.len() == 1 {
                // An empty file or a directory, not part of a merged group
                self.close_group(None, PackMode::from(header[11]));
            }
        }
        Ok(())
    }

    fn close_group(&mut self, data: Option<(u64, u64)>, pack_mode: PackMode) {
        let merged = self.open_group.len() > 1;
        for &index in &self.open_group {
            self.entries[index].merged = merged;
        }
        self.groups.push(Group {
            data,
            pack_mode,
            size: self.open_size,
        });
        self.open_group.clear();
        self.open_size = 0;
    }

    /// Entries after the last one with packed data have no data; reading them fails
    /// unless they are empty.
    fn close_unterminated_group(&mut self) {
        if let Some(&last) = self.open_group.last() {
            let pack_mode = self.entries[last].pack_mode;
            self.close_group(None, pack_mode);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stamp(day: u32, month: u32, year_field: u32, hour: u32, minute: u32, second: u32) -> u32 {
        day << 27 | (month - 1) << 23 | year_field << 17 | hour << 12 | minute << 6 | second
    }

    fn ymd_hms(time: DosDateTime) -> (u16, u8, u8, u8, u8, u8) {
        (time.year(), time.month(), time.day(), time.hour(), time.minute(), time.second())
    }

    #[test]
    fn date_stamp_year_ranges() {
        let date = |field| modified_time(stamp(23, 2, field, 18, 26, 19)).map(ymd_hms);
        assert_eq!(date(27), Some((1997, 2, 23, 18, 26, 18)));
        assert_eq!(date(29), Some((1999, 2, 23, 18, 26, 18)));
        assert_eq!(date(58), Some((2000, 2, 23, 18, 26, 18)));
        assert_eq!(date(63), Some((2005, 2, 23, 18, 26, 18)));
        assert_eq!(date(30), Some((2006, 2, 23, 18, 26, 18)));
        assert_eq!(date(57), Some((2033, 2, 23, 18, 26, 18)));
        assert_eq!(date(0), Some((2034, 2, 23, 18, 26, 18)));
        assert_eq!(date(7), Some((2041, 2, 23, 18, 26, 18)));
        // 1978 and 1979 cannot be represented as DOS dates.
        assert_eq!(date(8), None);
        assert_eq!(date(9), None);
    }

    #[test]
    fn invalid_date_stamps_are_missing() {
        assert_eq!(modified_time(stamp(0, 1, 27, 0, 0, 0)), None);
        assert_eq!(modified_time(stamp(1, 13, 27, 0, 0, 0)), None);
        assert_eq!(modified_time(stamp(1, 1, 27, 24, 0, 0)), None);
        assert_eq!(modified_time(stamp(1, 1, 27, 0, 60, 0)), None);
        assert_eq!(modified_time(stamp(1, 1, 27, 0, 0, 60)), None);
        assert_eq!(modified_time(0), None);
    }

    #[test]
    fn protection_bits() {
        let entry = |attributes| LzxEntry {
            name: String::new(),
            comment: String::new(),
            attributes,
            original_size: 0,
            packed_size: 0,
            pack_mode: PackMode::Stored,
            merged: false,
            modified_time: None,
            crc32: 0,
            group: 0,
            group_offset: 0,
        };
        assert_eq!(entry(0x0F).protection(), "----rwed");
        assert_eq!(entry(0x07).protection(), "----rw-d");
        assert_eq!(entry(0xF0).protection(), "hspa----");
    }
}
