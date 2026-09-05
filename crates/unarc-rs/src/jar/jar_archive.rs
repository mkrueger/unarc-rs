//! JAR Archive Format Support
//!
//! JAR (Just Another aRchiver) is a solid archive format created by
//! ARJ Software, Inc. (Robert Jung) in 1996-97. Not to be confused with Java JAR files.
//!
//! # Format Overview
//!
//! JAR archives use a solid compression approach where multiple files are compressed
//! together as a single stream. The archive structure consists of:
//! - A 64-byte archive header
//! - Packet-framed solid data streams
//! - A separately compressed metadata tree and its CRC
//!
//! # Compression Methods
//!
//! JAR supports 4 compression methods:
//! - Method 1 (m1): Fast compression
//! - Method 2 (m2): Normal compression
//! - Method 3 (m3): Better compression
//! - Method 4 (m4): Best compression
//!
//! All four presets use the same Huffman/LZ/word decoding pipeline, not ARJ.

use std::io::{Read, Seek, SeekFrom};

use crate::error::{ArchiveError, Result};

use super::{
    decoder,
    words::{bad, Record},
};

/// Magic bytes for JAR archive format at offset 0x0E
const JAR_MAGIC: [u8; 4] = [0x1A, b'J', b'a', b'r'];

/// Archive header size (always 64 bytes)
const HEADER_SIZE: usize = 64;

/// JAR Archive Header
#[derive(Debug, Clone)]
pub struct JarHeader {
    /// Header CRC (special checksum, bytes 0x00-0x03)
    pub header_crc: u32,
    /// Offset to compressed data (always 0x40 = 64)
    pub data_offset: u32,
    /// Size of compressed data
    pub compressed_size: u32,
    /// Version needed to extract (usually 27 = 0x1B)
    pub version_needed: u16,
    /// Auxiliary archive-header field (not the per-file data CRC)
    pub data_crc: u32,
    /// Archive flags
    pub flags: u32,
    /// Legacy field name: byte offset of the metadata stream relative to data_offset.
    /// This is NOT an uncompressed size. Prefer `JarArchive::directory_offset()`.
    pub uncompressed_size: u32,
    /// Secondary CRC
    pub secondary_crc: u32,
}

impl JarHeader {
    /// Load header from a byte slice (must be at least 64 bytes)
    pub fn load_from(data: &[u8]) -> Result<Self> {
        if data.len() < HEADER_SIZE {
            return Err(ArchiveError::invalid_header("JAR"));
        }

        // Check magic at offset 0x0E
        if &data[0x0E..0x12] != JAR_MAGIC {
            return Err(ArchiveError::invalid_header("JAR"));
        }

        let header_crc = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let data_offset = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        let compressed_size = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
        let version_needed = u16::from_le_bytes([data[0x12], data[0x13]]);
        let data_crc = u32::from_le_bytes([data[0x18], data[0x19], data[0x1A], data[0x1B]]);
        let flags = u32::from_le_bytes([data[0x1C], data[0x1D], data[0x1E], data[0x1F]]);
        let uncompressed_size = u32::from_le_bytes([data[0x20], data[0x21], data[0x22], data[0x23]]);
        let secondary_crc = u32::from_le_bytes([data[0x24], data[0x25], data[0x26], data[0x27]]);

        let mut checked = [0u8; HEADER_SIZE];
        checked.copy_from_slice(&data[..HEADER_SIZE]);
        checked[..4].fill(0);
        if (!crc32fast::hash(&checked)).rotate_left(21) != header_crc {
            return Err(bad("Archive header CRC mismatch"));
        }
        if version_needed != 27 {
            return Err(ArchiveError::unsupported_method("JAR", format!("Archive version {version_needed}")));
        }
        if data_offset < HEADER_SIZE as u32 || uncompressed_size > compressed_size {
            return Err(ArchiveError::invalid_header("JAR"));
        }

        Ok(Self {
            header_crc,
            data_offset,
            compressed_size,
            version_needed,
            data_crc,
            flags,
            uncompressed_size,
            secondary_crc,
        })
    }
}

/// Compression engine. The old Method/Stored variants are retained for API
/// compatibility, but are not inferred from arbitrary Huffman-table bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMethod {
    /// JAR's Huffman/LZ/word engine, shared by compression presets m1–m4.
    Huffman,
    /// Method 1 - Fast compression (byte 'A' = 0x41)
    Method1,
    /// Method 2 - Normal compression (byte 'C' = 0x43)
    Method2,
    /// Method 3/4 - Better/Best compression (byte 'B' = 0x42)
    Method34,
    /// Stored or minimal compression
    Stored,
    /// Unknown method
    Unknown(u8),
}

impl From<u8> for CompressionMethod {
    fn from(value: u8) -> Self {
        match value {
            0x10 => CompressionMethod::Huffman,
            0x41 => CompressionMethod::Method1,  // 'A'
            0x42 => CompressionMethod::Method34, // 'B'
            0x43 => CompressionMethod::Method2,  // 'C'
            0x27 => CompressionMethod::Stored,   // '\''
            0x00 => CompressionMethod::Stored,
            _ => CompressionMethod::Unknown(value),
        }
    }
}

/// A file entry within a JAR archive
///
/// File metadata is stored separately from the solid data. A decoded 0xE01
/// trailer also records the name and date; both sources are cross-checked.
#[derive(Debug, Clone)]
pub struct JarEntry {
    /// File name
    pub name: String,
    /// Original (uncompressed) size
    pub original_size: u32,
    /// Compressed size (0: not individually attributable in a solid stream)
    pub compressed_size: u32,
    /// Compression method
    pub compression_method: CompressionMethod,
    /// JAR CRC over the file's control records, bytes and descriptor (unfinalized CRC-32)
    pub crc32: u32,
    /// DOS modification date
    pub modification_date: u16,
    /// DOS modification time
    pub modification_time: u16,
    /// File attributes
    pub attributes: u8,
    /// Is this a directory?
    pub is_directory: bool,
}

/// JAR Archive Reader
///
/// JAR uses solid compression, meaning all files are compressed together
/// as a single stream. This makes random access to individual files
/// impractical - the entire archive must be decompressed sequentially.
pub struct JarArchive<R: Read + Seek> {
    reader: R,
    header: JarHeader,
    entries: Option<Vec<(JarEntry, Vec<u8>)>>,
    next_entry: usize,
    output_limit: usize,
}

impl<R: Read + Seek> JarArchive<R> {
    /// Create a new JAR archive reader
    pub fn new(mut reader: R) -> Result<Self> {
        // Read header
        let mut header_buf = [0u8; HEADER_SIZE];
        reader.read_exact(&mut header_buf)?;

        let header = JarHeader::load_from(&header_buf)?;

        // Seek to start of compressed data
        reader.seek(SeekFrom::Start(header.data_offset as u64))?;

        let end = reader.seek(SeekFrom::End(0))?;
        if u64::from(header.data_offset) + u64::from(header.compressed_size) > end {
            return Err(bad("Truncated archive"));
        }
        Ok(Self {
            reader,
            header,
            entries: None,
            next_entry: 0,
            output_limit: 256 * 1024 * 1024,
        })
    }

    /// Get the archive header
    pub fn header(&self) -> &JarHeader {
        &self.header
    }

    /// Legacy accessor: returns the metadata offset, NOT the uncompressed size.
    #[deprecated(note = "Use directory_offset() or total_original_size()")]
    pub fn uncompressed_size(&self) -> u32 {
        self.header.uncompressed_size
    }

    /// Offset of the compressed metadata, relative to the archive's data offset.
    pub fn directory_offset(&self) -> u32 {
        self.header.uncompressed_size
    }

    /// Sum of actual original file sizes, after decoding and validating the archive.
    pub fn total_original_size(&mut self) -> Result<u64> {
        self.load_entries()?;
        Ok(self
            .entries
            .as_ref()
            .ok_or_else(|| bad("Missing entries"))?
            .iter()
            .map(|(e, _)| u64::from(e.original_size))
            .sum())
    }

    /// Get the compressed size of the archive data
    pub fn compressed_size(&self) -> u32 {
        self.header.compressed_size
    }

    /// Check if the archive can be probed (has valid magic)
    pub fn probe(data: &[u8]) -> bool {
        if data.len() < HEADER_SIZE {
            return false;
        }
        // Check magic at offset 0x0E
        &data[0x0E..0x12] == JAR_MAGIC
    }

    /// Extract all files from the archive
    ///
    /// Since JAR uses solid compression, we must decompress the entire
    /// archive to extract any files. This method returns all entries
    /// with their decompressed data.
    pub fn extract_all(&mut self) -> Result<Vec<(JarEntry, Vec<u8>)>> {
        self.load_entries()?;
        Ok(self.entries.as_ref().ok_or_else(|| bad("Missing entries"))?.clone())
    }

    /// Set the maximum total expanded stream size (default: 256 MiB).
    pub fn set_output_limit(&mut self, bytes: usize) {
        self.output_limit = bytes;
    }

    /// Read the next file/directory, decoding the solid stream only once.
    pub fn get_next_entry(&mut self) -> Result<Option<JarEntry>> {
        self.load_entries()?;
        let entry = self.entries.as_ref().and_then(|v| v.get(self.next_entry)).map(|v| v.0.clone());
        if entry.is_some() {
            self.next_entry += 1;
        }
        Ok(entry)
    }

    pub fn read_entry(&mut self, entry: &JarEntry) -> Result<Vec<u8>> {
        self.load_entries()?;
        self.entries
            .as_ref()
            .and_then(|v| v.iter().find(|v| v.0.name == entry.name && v.0.crc32 == entry.crc32))
            .map(|v| v.1.clone())
            .ok_or_else(|| bad("Entry not found"))
    }

    fn load_entries(&mut self) -> Result<()> {
        if self.entries.is_some() {
            return Ok(());
        }
        let compressed = self.read_compressed_data()?;
        let directory_offset = self.header.uncompressed_size as usize;
        let (directory, used) = decoder::decode_stream(&compressed[directory_offset..], self.output_limit.min(16 * 1024 * 1024))?;
        if directory_offset + used + 4 != compressed.len() {
            return Err(bad("Unsupported archive layout"));
        }
        let crc_record = directory.iter().position(|r| r.tag == 0xe03).ok_or_else(|| bad("Missing directory CRC"))?;
        let crc = decoder::u32_at(&directory[crc_record].data, 0)?;
        if decoder::record_crc(&directory[..crc_record]) != crc || decoder::u32_at(&compressed, directory_offset + used)? != crc {
            return Err(bad("Directory CRC mismatch"));
        }
        let mut metadata = parse_directory(&directory[..crc_record])?;
        let expected = metadata
            .values()
            .try_fold(0usize, |sum, e| sum.checked_add(e.original_size as usize).ok_or_else(|| bad("Size overflow")))?;
        if expected > self.output_limit {
            return Err(bad("Archive exceeds output limit"));
        }
        let mut result = Vec::new();
        let mut offset = 0;
        let mut block_index = 0u32;
        while offset < directory_offset {
            let (records, used) = decoder::decode_stream(&compressed[offset..directory_offset], self.output_limit)?;
            offset += used;
            let mut file_index = 0u32;
            let mut start = 0;
            for i in 0..records.len() {
                if records[i].tag != 0xe03 {
                    continue;
                }
                while start < i && records[start].tag == 0xe04 && records[start].data.is_empty() {
                    start += 1;
                }
                let file_records = &records[start..i];
                let descriptor = file_records.iter().find(|r| r.tag == 0xe01).ok_or_else(|| bad("Missing file descriptor"))?;
                if descriptor.data.len() < 6 {
                    return Err(bad("Truncated file descriptor"));
                }
                let filename = decoder::name(&descriptor.data[5..])?;
                let crc = decoder::u32_at(&records[i].data, 0)?;
                if records[i].data.len() != 4 || decoder::record_crc(file_records) != crc {
                    return Err(bad(&format!("File CRC mismatch: {filename}")));
                }
                let entry = metadata.remove(&(block_index, file_index)).ok_or_else(|| bad("Missing file metadata"))?;
                if filename != entry.name || crc != entry.crc32 {
                    return Err(bad("File descriptor differs from directory"));
                }
                let mut bytes = Vec::new();
                for record in file_records {
                    match record.tag {
                        0xe02 => bytes.extend_from_slice(&record.data),
                        0 | 0xe04 if record.data.is_empty() => (),
                        0xe01 => (),
                        _ => return Err(bad("Unsupported file record")),
                    }
                }
                if bytes.len() != entry.original_size as usize {
                    return Err(bad("File size mismatch"));
                }
                result.push((entry, bytes));
                file_index += 1;
                start = i + 1;
            }
            if records[start..].iter().any(|r| r.tag != 0xe04 || !r.data.is_empty()) {
                return Err(bad("Incomplete file records"));
            }
            block_index += 1;
        }
        if !metadata.is_empty() {
            return Err(bad("Missing file data"));
        }
        self.entries = Some(result);
        Ok(())
    }

    /// Get the raw compressed data
    ///
    /// Returns the compressed data stream for analysis or external decompression.
    pub fn read_compressed_data(&mut self) -> Result<Vec<u8>> {
        if self.header.compressed_size as usize > self.output_limit.saturating_add(16 * 1024 * 1024) {
            return Err(bad("Compressed archive exceeds input limit"));
        }
        self.reader.seek(SeekFrom::Start(self.header.data_offset as u64))?;

        let mut data = vec![0u8; self.header.compressed_size as usize];
        self.reader.read_exact(&mut data)?;

        Ok(data)
    }
}

// File IDs are local to a solid block: ABL -> block ID -> file ID.
// Named BIN children describe blocks; VIN/CIN/AIN are separate metadata trees.
fn parse_directory(records: &[Record]) -> Result<std::collections::BTreeMap<(u32, u32), JarEntry>> {
    #[derive(Clone, Copy)]
    enum GroupKind {
        Other,
        ArchiveBlocks,
        Block(u32),
        File(u32),
    }
    struct Group {
        id: u32,
        kind: GroupKind,
        entry: Option<JarEntry>,
        name: Option<String>,
    }
    let mut groups = Vec::<Group>::new();
    let mut result = std::collections::BTreeMap::new();
    for record in records {
        match record.tag {
            0 if record.data.is_empty() => (),
            0xe06 => {
                if groups.len() >= 256 {
                    return Err(bad("Directory nesting too deep"));
                }
                let id = decoder::u32_at(&record.data, 0)?;
                let kind = if groups.is_empty() && &record.data[4..] == b"ABL\0" {
                    GroupKind::ArchiveBlocks
                } else if record.data.len() == 4 {
                    match groups.last().map(|g| g.kind) {
                        Some(GroupKind::ArchiveBlocks) => GroupKind::Block(id),
                        Some(GroupKind::Block(block)) => GroupKind::File(block),
                        _ => GroupKind::Other,
                    }
                } else {
                    GroupKind::Other
                };
                groups.push(Group {
                    id,
                    kind,
                    entry: None,
                    name: None,
                });
            }
            0xe07 => {
                let g = groups.last_mut().ok_or_else(|| bad("Record outside directory group"))?;
                let data = &record.data;
                if data.len() < 2 {
                    return Err(bad("Truncated directory record"));
                }
                if !matches!(g.kind, GroupKind::File(_)) {
                    continue;
                }
                match u16::from_le_bytes([data[0], data[1]]) {
                    0x100 => {
                        if data.len() != 95 || g.entry.is_some() {
                            return Err(bad("Unsupported file metadata"));
                        }
                        // Three eleven-byte timestamps: modification, access, creation.
                        let year = u16::from_le_bytes([data[2], data[3]]);
                        if !(1980..=2107).contains(&year)
                            || !(1..=12).contains(&data[4])
                            || !(1..=31).contains(&data[5])
                            || data[6] > 23
                            || data[7] > 59
                            || data[8] > 59
                        {
                            return Err(bad("Invalid modification timestamp"));
                        }
                        let attrs = decoder::u32_at(data, 35)?;
                        // JAR stores the archive attribute in bit 0, not DOS bit 5.
                        let attributes = ((attrs & 0x1e) | ((attrs & 1) << 5)) as u8;
                        if decoder::u32_at(data, 43)? != 0 {
                            return Err(bad("Files above 4 GiB are unsupported"));
                        }
                        g.entry = Some(JarEntry {
                            name: String::new(),
                            original_size: decoder::u32_at(data, 39)?,
                            compressed_size: 0,
                            compression_method: CompressionMethod::Huffman,
                            crc32: decoder::u32_at(data, 51)?,
                            modification_date: ((year - 1980) << 9) | (u16::from(data[4]) << 5) | u16::from(data[5]),
                            modification_time: (u16::from(data[6]) << 11) | (u16::from(data[7]) << 5) | u16::from(data[8] / 2),
                            attributes,
                            is_directory: attrs & 0x10 != 0,
                        });
                    }
                    0x101 => {
                        if g.name.is_some() {
                            return Err(bad("Duplicate filename record"));
                        }
                        g.name = Some(decoder::name(&data[2..])?);
                    }
                    _ => (),
                }
            }
            0xe08 => {
                if !record.data.is_empty() {
                    return Err(bad("Unexpected directory end data"));
                }
                if let Some(g) = groups.pop() {
                    if let (GroupKind::File(block), Some(mut entry)) = (g.kind, g.entry) {
                        entry.name = g.name.ok_or_else(|| bad("Missing filename"))?;
                        if result.insert((block, g.id), entry).is_some() {
                            return Err(bad("Duplicate file index"));
                        }
                    }
                }
            }
            _ => return Err(bad("Unsupported directory record")),
        }
    }
    if !groups.is_empty() {
        return Err(bad("Unclosed directory group"));
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn two_block_directory() -> Vec<Record> {
        let data = include_bytes!("../../tests/jar/two_blocks.j");
        let header = JarHeader::load_from(data).unwrap();
        let offset = (header.data_offset + header.uncompressed_size) as usize;
        let (mut records, _) = decoder::decode_stream(&data[offset..], 65536).unwrap();
        records.truncate(records.iter().position(|r| r.tag == 0xe03).unwrap());
        records
    }

    #[test]
    fn directory_file_indices_are_block_local() {
        let entries = parse_directory(&two_block_directory()).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[&(0, 0)].name, "FIRST.TXT");
        assert_eq!(entries[&(1, 0)].name, "SECOND.TXT");
    }

    #[test]
    fn directory_rejects_duplicate_file_index_in_same_block() {
        let mut records = two_block_directory();
        let mut depth = 0usize;
        let mut changed = false;
        for record in &mut records {
            if record.tag == 0xe06 {
                if depth == 1 && record.data == 1u32.to_le_bytes() {
                    record.data = 0u32.to_le_bytes().to_vec();
                    changed = true;
                    break;
                }
                depth += 1;
            } else if record.tag == 0xe08 {
                depth = depth.saturating_sub(1);
            }
        }
        assert!(changed);
        assert!(parse_directory(&records).unwrap_err().to_string().contains("Duplicate file index"));
    }

    #[test]
    fn directory_ignores_file_like_records_outside_abl() {
        let mut records = two_block_directory();
        let root = records.iter_mut().find(|r| r.tag == 0xe06).unwrap();
        assert_eq!(&root.data[4..], b"ABL\0");
        root.data[4..].copy_from_slice(b"VIN\0");
        assert!(parse_directory(&records).unwrap().is_empty());
    }

    #[test]
    fn test_jar_probe() {
        // Valid JAR header (first 64 bytes pattern)
        let mut data = vec![0u8; 64];
        // Magic at offset 0x0E
        data[0x0E] = 0x1A;
        data[0x0F] = b'J';
        data[0x10] = b'a';
        data[0x11] = b'r';

        assert!(JarArchive::<Cursor<&[u8]>>::probe(&data));

        // Invalid header
        let invalid = vec![0u8; 64];
        assert!(!JarArchive::<Cursor<&[u8]>>::probe(&invalid));
    }

    #[test]
    fn test_compression_method_from_byte() {
        assert_eq!(CompressionMethod::from(0x41), CompressionMethod::Method1);
        assert_eq!(CompressionMethod::from(0x42), CompressionMethod::Method34);
        assert_eq!(CompressionMethod::from(0x43), CompressionMethod::Method2);
        assert_eq!(CompressionMethod::from(0x27), CompressionMethod::Stored);
        assert_eq!(CompressionMethod::from(0xFF), CompressionMethod::Unknown(0xFF));
    }
}
