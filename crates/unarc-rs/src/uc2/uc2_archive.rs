use std::{
    collections::HashMap,
    io::{Read, Seek, SeekFrom},
};

use crate::error::{ArchiveError, Result};

use super::decompress::{self, MasterDict};
use super::{CompressInfo, EntryType, ExtendedHeader, FileAttributes, Location};

#[derive(Debug, Clone)]
pub struct Uc2ArchiveEntry {
    pub name: String,
    pub parent_dir: u32,
    pub attributes: FileAttributes,
    pub dos_time: u32,
    pub compress_info: CompressInfo,
    pub location: super::Location,
    pub original_size: u32,
    pub crc: u16,
}

#[derive(Debug, Clone)]
struct MasterRecord {
    size: usize,
    location: Location,
    compress_info: CompressInfo,
    data: Option<Vec<u8>>,
}

pub struct Uc2Archive<T: Read + Seek> {
    reader: T,
    x_head: ExtendedHeader,
    entries: Vec<Uc2ArchiveEntry>,
    masters: HashMap<u32, MasterRecord>,
    current_entry: usize,
}

const HEADER_SIZE: usize = 4 + 8 + 1;
const ID: u32 = 0x1A324355; // UC2\x1A
const AMAG: u32 = 0x01B2C3D4;
// UltraCrypt-encrypted UC2 archives start with "UE2" followed by a version byte.
// In little-endian u32 this matches: (id & 0x00FF_FFFF) == 0x00324555.
const UE2_LABEL: u32 = 0x00324555;

impl<T: Read + Seek> Uc2Archive<T> {
    pub fn new(mut reader: T) -> Result<Self> {
        let (_archive_len, _is_damage_protected) = read_header(&mut reader)?;
        let x_head = ExtendedHeader::load_from(&mut reader)?;

        Ok(Self {
            reader,
            x_head,
            entries: Vec::new(),
            masters: HashMap::new(),
            current_entry: 0,
        })
    }

    pub fn get_next_entry(&mut self) -> Result<Option<Uc2ArchiveEntry>> {
        self.ensure_cdir_parsed()?;
        if self.current_entry >= self.entries.len() {
            return Ok(None);
        }
        let entry = self.entries[self.current_entry].clone();
        self.current_entry += 1;
        Ok(Some(entry))
    }

    fn parse_cdir(&mut self) -> Result<()> {
        if !self.entries.is_empty() {
            return Ok(());
        }

        const SUPER_MASTER_SENTINEL: u32 = 0xDEDE_DEDE;

        let cdir_offset = self.x_head.c_dir_loc.offset as u64;
        self.reader.seek(SeekFrom::Start(cdir_offset))?;

        let compress_info = CompressInfo::load_from(&mut self.reader)?;

        if compress_info.master_prefix != 1 {
            return Err(ArchiveError::corrupted_entry_named(
                "UC2",
                "CDIR",
                format!(
                    "unexpected CDIR master prefix: {}",
                    compress_info.master_prefix
                ),
            ));
        }

        // If compressed_length is 0, it means the CDIR extends to the end of the file
        // Read all remaining data as compressed CDIR
        let compressed_data = if compress_info.compressed_length == 0 {
            let mut buffer = Vec::new();
            self.reader.read_to_end(&mut buffer)?;
            buffer
        } else {
            let mut buffer = vec![0u8; compress_info.compressed_length as usize];
            self.reader.read_exact(&mut buffer)?;
            buffer
        };

        let mut cursor: &[u8] = &decompress::decompress_no_master(&compressed_data)?;

        let mut entries = Vec::new();
        let mut masters = HashMap::new();

        while !cursor.is_empty() {
            let raw_type = read_u8(&mut cursor)?;
            let entry_type = EntryType::try_from(raw_type)?;
            match entry_type {
                EntryType::EndOfCdir => break,
                EntryType::DirEntry => {
                    let _parent = read_u32_le(&mut cursor)?;
                    let _attrib = read_u8(&mut cursor)?;
                    let _time = read_u32_le(&mut cursor)?;
                    let _name_bytes = take(&mut cursor, 11)?;
                    let _hidden = read_u8(&mut cursor)?;
                    let tag_flag = read_u8(&mut cursor)?;
                    if tag_flag != 0 {
                        let _ = read_tags(&mut cursor)?;
                    }
                    let _index = read_u32_le(&mut cursor)?;
                }
                EntryType::FileEntry => {
                    let parent = read_u32_le(&mut cursor)?;
                    let attrib = read_u8(&mut cursor)?;
                    let dos_time = read_u32_le(&mut cursor)?;
                    let mut dos_name = [0u8; 11];
                    dos_name.copy_from_slice(take(&mut cursor, 11)?);
                    let _hidden = read_u8(&mut cursor)?;
                    let tag_flag = read_u8(&mut cursor)?;

                    let original_size = read_u32_le(&mut cursor)?;
                    let crc = read_u16_le(&mut cursor)?;

                    let mut compress = read_compress(&mut cursor)?;
                    if compress.master_prefix == SUPER_MASTER_SENTINEL {
                        compress.master_prefix = 0;
                    }

                    let location = read_location(&mut cursor)?;

                    if location.volume > 1 {
                        return Err(ArchiveError::unsupported_method(
                            "UC2",
                            "multi-volume archives",
                        ));
                    }

                    let mut name = decode_dos_name(&dos_name);
                    if tag_flag != 0 {
                        if let Some(long_name) = read_tags(&mut cursor)? {
                            name = long_name;
                        }
                    }

                    entries.push(Uc2ArchiveEntry {
                        name,
                        parent_dir: parent,
                        attributes: FileAttributes::from(attrib),
                        dos_time,
                        compress_info: compress,
                        location,
                        original_size,
                        crc,
                    });
                }
                EntryType::MasterEntry => {
                    let id = read_u32_le(&mut cursor)?;
                    let _key = read_u32_le(&mut cursor)?;
                    let _ref_len = read_u32_le(&mut cursor)?;
                    let _ref_ctr = read_u32_le(&mut cursor)?;
                    let size = read_u16_le(&mut cursor)? as usize;
                    let _crc = read_u16_le(&mut cursor)?;

                    let mut compress = read_compress(&mut cursor)?;
                    if compress.master_prefix == SUPER_MASTER_SENTINEL {
                        compress.master_prefix = 0;
                    }
                    let location = read_location(&mut cursor)?;

                    if location.volume > 1 {
                        return Err(ArchiveError::unsupported_method(
                            "UC2",
                            "multi-volume master entries",
                        ));
                    }

                    masters.insert(
                        id,
                        MasterRecord {
                            size,
                            location,
                            compress_info: compress,
                            data: None,
                        },
                    );
                }
            }
        }

        self.entries = entries;
        self.masters = masters;
        Ok(())
    }

    fn ensure_cdir_parsed(&mut self) -> Result<()> {
        if self.entries.is_empty() {
            self.parse_cdir()?;
        }
        Ok(())
    }

    fn ensure_master(&mut self, id: u32) -> Result<()> {
        if id < 2 {
            return Ok(());
        }

        let Some(record) = self.masters.get(&id) else {
            return Err(ArchiveError::corrupted_entry_named(
                "UC2",
                "master",
                format!("master {} not found in CDIR", id),
            ));
        };

        if record.data.is_some() {
            return Ok(());
        }

        let (size, offset, compressed_len, master_prefix) = (
            record.size,
            record.location.offset,
            record.compress_info.compressed_length as usize,
            record.compress_info.master_prefix,
        );

        self.reader.seek(SeekFrom::Start(offset as u64))?;
        let mut compressed = vec![0u8; compressed_len];
        self.reader.read_exact(&mut compressed)?;

        let data = match master_prefix {
            0 => decompress::decompress_with_dict(&compressed, size, MasterDict::SuperMaster)?,
            1 => decompress::decompress_with_dict(&compressed, size, MasterDict::NoMaster)?,
            parent => {
                self.ensure_master(parent)?;
                let slice = self
                    .masters
                    .get(&parent)
                    .and_then(|m| m.data.as_ref())
                    .ok_or_else(|| {
                        ArchiveError::corrupted_entry_named(
                            "UC2",
                            "master",
                            "missing parent master",
                        )
                    })?;
                decompress::decompress_with_dict(
                    &compressed,
                    size,
                    MasterDict::Custom(slice.as_slice()),
                )?
            }
        };

        if let Some(record) = self.masters.get_mut(&id) {
            record.data = Some(data);
        }

        Ok(())
    }

    pub fn read(&mut self, entry: &Uc2ArchiveEntry) -> Result<Vec<u8>> {
        self.ensure_cdir_parsed()?;

        let compressed_len = entry.compress_info.compressed_length as usize;
        let offset = entry.location.offset as u64;
        let original_size = entry.original_size as usize;

        eprintln!(
            "📁 read(): compressed_len={}, offset={}, original_size={}",
            compressed_len, offset, original_size
        );

        self.reader.seek(SeekFrom::Start(offset))?;
        let mut compressed = vec![0u8; compressed_len];
        self.reader.read_exact(&mut compressed)?;

        let dict = entry.compress_info.master_prefix;

        let output = match dict {
            0 => decompress::decompress_with_dict(
                &compressed,
                original_size,
                MasterDict::SuperMaster,
            )?,
            1 => {
                decompress::decompress_with_dict(&compressed, original_size, MasterDict::NoMaster)?
            }
            master_id => {
                self.ensure_master(master_id)?;
                let slice = self
                    .masters
                    .get(&master_id)
                    .and_then(|m| m.data.as_ref())
                    .ok_or_else(|| {
                        ArchiveError::corrupted_entry_named(
                            "UC2",
                            &entry.name,
                            "missing master data",
                        )
                    })?;
                decompress::decompress_with_dict(
                    &compressed,
                    original_size,
                    MasterDict::Custom(slice.as_slice()),
                )?
            }
        };

        Ok(output)
    }

    pub fn skip(&mut self, _entry: &Uc2ArchiveEntry) -> Result<()> {
        // UC2 archives are already fully parsed during get_next_entry,
        // so skip is a no-op - we just move to the next entry
        Ok(())
    }
}

fn read_header<T: Read + Seek>(reader: &mut T) -> Result<(u32, bool)> {
    let mut header_bytes = vec![0; HEADER_SIZE];
    reader.read_exact(&mut header_bytes)?;
    let mut header_bytes = header_bytes.as_slice();
    convert_u32!(id, header_bytes);

    // Detect UltraCrypt / UE2 early and return a clear error.
    if (id & 0x00FF_FFFF) == UE2_LABEL {
        return Err(ArchiveError::unsupported_method(
            "UC2",
            "UltraCrypt (UE2) encrypted archive (decrypt with UCRYPT first)",
        ));
    }
    if id != ID {
        return Err(ArchiveError::invalid_header("UC2"));
    }
    convert_u32!(archive_len, header_bytes);
    convert_u32!(archive_len2, header_bytes);
    if archive_len.wrapping_add(AMAG) != archive_len2 {
        return Err(ArchiveError::corrupted_entry_named(
            "UC2",
            "header",
            "file header is damaged",
        ));
    }
    convert_u8!(damage_protected_flag, header_bytes);
    Ok((archive_len, damage_protected_flag != 0))
}

fn take<'a>(cursor: &mut &'a [u8], len: usize) -> Result<&'a [u8]> {
    if cursor.len() < len {
        return Err(ArchiveError::corrupted_entry("UC2", "truncated UC2 CDIR"));
    }
    let (head, tail) = cursor.split_at(len);
    *cursor = tail;
    Ok(head)
}

fn read_u8(cursor: &mut &[u8]) -> Result<u8> {
    Ok(take(cursor, 1)?[0])
}

fn read_u16_le(cursor: &mut &[u8]) -> Result<u16> {
    let bytes = take(cursor, 2)?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32_le(cursor: &mut &[u8]) -> Result<u32> {
    let bytes = take(cursor, 4)?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_compress(cursor: &mut &[u8]) -> Result<CompressInfo> {
    Ok(CompressInfo {
        compressed_length: read_u32_le(cursor)?,
        method: read_u16_le(cursor)?,
        master_prefix: read_u32_le(cursor)?,
    })
}

fn read_location(cursor: &mut &[u8]) -> Result<Location> {
    Ok(Location {
        volume: read_u32_le(cursor)?,
        offset: read_u32_le(cursor)?,
    })
}

fn decode_dos_name(raw: &[u8; 11]) -> String {
    let prefix_len = raw[..8]
        .iter()
        .rposition(|&b| b != b' ')
        .map(|idx| idx + 1)
        .unwrap_or(0);
    let suffix_len = raw[8..]
        .iter()
        .rposition(|&b| b != b' ')
        .map(|idx| idx + 1)
        .unwrap_or(0);

    let mut name = String::new();
    if prefix_len > 0 {
        name.push_str(&String::from_utf8_lossy(&raw[..prefix_len]));
    }
    if suffix_len > 0 {
        if !name.is_empty() {
            name.push('.');
        }
        name.push_str(&String::from_utf8_lossy(&raw[8..8 + suffix_len]));
    }
    if name.is_empty() {
        name.push_str("noname");
    }
    name
}

fn read_tags(cursor: &mut &[u8]) -> Result<Option<String>> {
    let mut long_name = None;
    loop {
        let raw_tag = take(cursor, 16)?;
        let tag_end = raw_tag.iter().position(|&b| b == 0).unwrap_or(16);
        let tag_name = std::str::from_utf8(&raw_tag[..tag_end]).unwrap_or("");
        let size = read_u32_le(cursor)? as usize;
        let next = read_u8(cursor)?;
        let data = take(cursor, size)?;
        if tag_name == "AIP:Win95 LongN" {
            let str_end = data.iter().position(|&b| b == 0).unwrap_or(data.len());
            long_name = Some(String::from_utf8_lossy(&data[..str_end]).to_string());
        }
        if next == 0 {
            break;
        }
    }
    Ok(long_name)
}
