use std::collections::{HashSet, VecDeque};
use std::io::{Read, Seek, SeekFrom};

use chrono::NaiveDate;

use crate::date_time::DosDateTime;
use crate::error::{ArchiveError, Result};

/// Longest entry path, in UTF-8 bytes.
pub const MAX_PATH_BYTES: usize = 4096;
/// FAT12 holds at most 4084 data clusters; more means FAT16 or FAT32.
const MAX_FAT12_CLUSTERS: u32 = 4084;
const BOOT_SECTOR: usize = 512;
const ENTRY_SIZE: usize = 32;
const ATTR_VOLUME_LABEL: u8 = 0x08;
const ATTR_DIRECTORY: u8 = 0x10;
const ATTR_LONG_NAME: u8 = 0x0F;
/// Cluster values from here on end a chain; 0xFF7 marks a bad cluster.
const END_OF_CHAIN: u16 = 0xFF8;

/// Upper half of code page 437 for PC short names.
const CP437_HIGH: [char; 128] = [
    'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å', //
    'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ', //
    'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»', //
    '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐', //
    '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧', //
    '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀', //
    'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩', //
    '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '\u{a0}',
];

fn corrupt(reason: impl Into<String>) -> ArchiveError {
    ArchiveError::corrupted_entry("FAT", reason)
}

fn u16_at(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([data[offset], data[offset + 1]])
}

fn u32_at(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
}

fn cp437(byte: u8) -> char {
    if byte < 0x80 {
        char::from(byte)
    } else {
        CP437_HIGH[usize::from(byte - 0x80)]
    }
}

fn atari_st(byte: u8) -> char {
    match byte {
        0x9E => '\u{df}',
        0xB0..=0xC1 => [
            '\u{e3}', '\u{f5}', '\u{d8}', '\u{f8}', '\u{153}', '\u{152}', '\u{c0}', '\u{c3}', '\u{d5}', '\u{a8}', '\u{b4}', '\u{2020}', '\u{b6}', '\u{a9}',
            '\u{ae}', '\u{2122}', '\u{133}', '\u{132}',
        ][usize::from(byte - 0xB0)],
        0xC2..=0xDC => [
            '\u{5d0}', '\u{5d1}', '\u{5d2}', '\u{5d3}', '\u{5d4}', '\u{5d5}', '\u{5d6}', '\u{5d7}', '\u{5d8}', '\u{5d9}', '\u{5db}', '\u{5dc}', '\u{5de}',
            '\u{5e0}', '\u{5e1}', '\u{5e2}', '\u{5e4}', '\u{5e6}', '\u{5e7}', '\u{5e8}', '\u{5e9}', '\u{5ea}', '\u{5df}', '\u{5da}', '\u{5dd}', '\u{5e3}',
            '\u{5e5}',
        ][usize::from(byte - 0xC2)],
        0xDD => '\u{a7}',
        0xDE => '\u{2227}',
        0xDF => '\u{221e}',
        0xE1 => '\u{3b2}',
        0xEC => '\u{222e}',
        0xEE => '\u{2208}',
        0xFE => '\u{b3}',
        0xFF => '\u{af}',
        _ => cp437(byte),
    }
}

/// Converts a DOS date and time, treating impossible values as missing.
fn dos_time(date: u16, time: u16) -> Option<DosDateTime> {
    let (year, month, day) = (1980 + i32::from(date >> 9), u32::from((date >> 5) & 0x0F), u32::from(date & 0x1F));
    let (hour, minute, second) = (time >> 11, (time >> 5) & 0x3F, (time & 0x1F) * 2);
    if date == 0 || hour > 23 || minute > 59 || second > 59 || NaiveDate::from_ymd_opt(year, month, day).is_none() {
        return None;
    }
    Some(DosDateTime::from((date, time)))
}

/// The checksum a VFAT long name records for its 8.3 alias.
fn short_name_checksum(raw: &[u8]) -> u8 {
    raw[..11].iter().fold(0u8, |sum, &b| sum.rotate_right(1).wrapping_add(b))
}

/// Volume layout from the BIOS parameter block (BPB) in the boot sector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FatGeometry {
    pub bytes_per_sector: u32,
    pub sectors_per_cluster: u32,
    pub reserved_sectors: u32,
    pub fat_count: u32,
    pub root_entries: u32,
    pub total_sectors: u32,
    pub sectors_per_fat: u32,
    /// Media descriptor. Atari ST floppies often record an arbitrary value.
    pub media: u8,
}

impl FatGeometry {
    fn fat_offset(&self) -> u64 {
        u64::from(self.reserved_sectors) * u64::from(self.bytes_per_sector)
    }

    fn root_offset(&self) -> u64 {
        (u64::from(self.reserved_sectors) + u64::from(self.fat_count) * u64::from(self.sectors_per_fat)) * u64::from(self.bytes_per_sector)
    }

    fn root_sectors(&self) -> u64 {
        (u64::from(self.root_entries) * ENTRY_SIZE as u64).div_ceil(u64::from(self.bytes_per_sector))
    }

    fn data_sector(&self) -> u64 {
        u64::from(self.reserved_sectors) + u64::from(self.fat_count) * u64::from(self.sectors_per_fat) + self.root_sectors()
    }

    fn data_offset(&self) -> u64 {
        self.data_sector() * u64::from(self.bytes_per_sector)
    }

    /// Bytes per cluster.
    pub fn cluster_size(&self) -> u64 {
        u64::from(self.bytes_per_sector) * u64::from(self.sectors_per_cluster)
    }

    /// Number of data clusters (numbered from 2).
    pub fn clusters(&self) -> u32 {
        ((u64::from(self.total_sectors) - self.data_sector()) / u64::from(self.sectors_per_cluster)) as u32
    }

    /// Size of the filesystem in bytes.
    pub fn size(&self) -> u64 {
        u64::from(self.total_sectors) * u64::from(self.bytes_per_sector)
    }

    /// Bytes of the first FAT that describe clusters 0 through `clusters() + 1`.
    fn fat_bytes(&self) -> usize {
        ((self.clusters() as usize + 2) * 3).div_ceil(2)
    }

    /// Reads and checks a BPB. `Err(true)` means a valid FAT16/FAT32 layout.
    fn from_boot_sector(boot: &[u8]) -> std::result::Result<Self, bool> {
        let bytes_per_sector = u32::from(u16_at(boot, 11));
        let sectors_per_cluster = u32::from(boot[13]);
        // Atari boot sectors keep boot code where the 32-bit total would be; only a zero 16-bit total uses it.
        let total = match u16_at(boot, 19) {
            0 => u32_at(boot, 32),
            total => u32::from(total),
        };
        let geometry = Self {
            bytes_per_sector,
            sectors_per_cluster,
            reserved_sectors: u32::from(u16_at(boot, 14)),
            fat_count: u32::from(boot[16]),
            root_entries: u32::from(u16_at(boot, 17)),
            total_sectors: total,
            sectors_per_fat: u32::from(u16_at(boot, 22)),
            media: boot[21],
        };
        if !(512..=4096).contains(&bytes_per_sector)
            || !bytes_per_sector.is_power_of_two()
            || !(1..=128).contains(&sectors_per_cluster)
            || !sectors_per_cluster.is_power_of_two()
            || geometry.cluster_size() > 65_536
            || geometry.reserved_sectors == 0
            || !(1..=4).contains(&geometry.fat_count)
            || geometry.sectors_per_fat == 0
            || geometry.data_sector() >= u64::from(total)
        {
            return Err(false);
        }
        if geometry.clusters() == 0 {
            return Err(false);
        }
        // FAT32 records no root entries and a zero 16-bit FAT size; both mean "not FAT12".
        if geometry.clusters() > MAX_FAT12_CLUSTERS || geometry.root_entries == 0 {
            return Err(true);
        }
        if geometry.fat_bytes() as u64 > u64::from(geometry.sectors_per_fat) * u64::from(bytes_per_sector) {
            return Err(false);
        }
        Ok(geometry)
    }

    /// DOS 1.x disks have no BPB; their geometry follows from the image size and media byte.
    fn dos1(len: u64, fat_head: &[u8]) -> Option<Self> {
        let (media, sectors_per_cluster, root_entries, sectors_per_fat, total_sectors) = match len {
            163_840 => (0xFE, 1, 64, 1, 320),
            184_320 => (0xFC, 1, 64, 2, 360),
            327_680 => (0xFF, 2, 112, 1, 640),
            368_640 => (0xFD, 2, 112, 2, 720),
            _ => return None,
        };
        (fat_head == [media, 0xFF, 0xFF]).then_some(Self {
            bytes_per_sector: 512,
            sectors_per_cluster,
            reserved_sectors: 1,
            fat_count: 2,
            root_entries,
            total_sectors,
            sectors_per_fat,
            media,
        })
    }
}

/// A file or directory in a FAT image.
#[derive(Debug, Clone)]
pub struct FatEntry {
    /// Path with `/` separators; directories end with `/`. VFAT long names are used when valid.
    pub name: String,
    pub size: u64,
    pub is_directory: bool,
    /// DOS attributes: 0x01 read-only, 0x02 hidden, 0x04 system, 0x10 directory, 0x20 archive.
    pub attributes: u8,
    pub modified_time: Option<DosDateTime>,
    pub start_cluster: u32,
    index: usize,
}

/// A listed entry. Only the last path component is stored, so memory stays
/// linear in the number of entries however deep the tree is.
#[derive(Debug, Clone)]
struct Node {
    parent: Option<usize>,
    component: String,
    path_len: usize,
    size: u64,
    is_directory: bool,
    attributes: u8,
    modified_time: Option<DosDateTime>,
    start_cluster: u32,
}

/// A partially collected VFAT long name.
struct LongName {
    parts: Vec<[u16; 13]>,
    next: u8,
    checksum: u8,
}

/// A seek-based FAT12 floppy-image reader. Only metadata is read when opening.
pub struct FatArchive<T: Read + Seek> {
    reader: T,
    start: u64,
    len: u64,
    geometry: FatGeometry,
    fat: Vec<u8>,
    /// Clusters that hold subdirectories, which no file may share.
    directory_clusters: Vec<bool>,
    nodes: Vec<Node>,
    volume_label: Option<String>,
    next: usize,
    atari_names: bool,
}

impl<T: Read + Seek> FatArchive<T> {
    /// Opens a FAT12 floppy image (PC or Atari ST) starting at the reader's position.
    pub fn new(reader: T) -> Result<Self> {
        Self::open(reader, None)
    }

    /// Opens an Atari image using the Atari ST character set even when its boot
    /// sector has PC-compatible or absent boot code.
    pub fn new_atari(reader: T) -> Result<Self> {
        Self::open(reader, Some(true))
    }

    pub(crate) fn open(mut reader: T, atari_names: Option<bool>) -> Result<Self> {
        let start = reader.stream_position()?;
        let len = reader.seek(SeekFrom::End(0))?.saturating_sub(start);
        reader.seek(SeekFrom::Start(start))?;
        let mut opcode = [0; 2];
        reader.read_exact(&mut opcode)?;
        let atari_names = atari_names.unwrap_or(opcode[0] == 0x60);
        let geometry = Self::read_geometry(&mut reader, start, len)?.map_err(|fat16| {
            if fat16 {
                ArchiveError::unsupported_format("FAT16/FAT32 images (only FAT12 floppy images are supported)")
            } else {
                ArchiveError::invalid_header("FAT (no FAT12 boot sector; partitioned disk images are not supported)")
            }
        })?;
        if len < geometry.data_offset() {
            return Err(corrupt("image ends before the data area"));
        }
        let mut archive = Self {
            reader,
            start,
            len,
            geometry,
            fat: Vec::new(),
            directory_clusters: vec![false; geometry.clusters() as usize + 2],
            nodes: Vec::new(),
            volume_label: None,
            next: 0,
            atari_names,
        };
        archive.fat = archive.read_at(geometry.fat_offset(), geometry.fat_bytes())?;
        if archive.fat_entry(0) < 0xFF0 || archive.fat_entry(1) < END_OF_CHAIN {
            return Err(corrupt("invalid reserved FAT entries"));
        }
        archive.list_entries()?;
        Ok(archive)
    }

    fn read_geometry(reader: &mut T, start: u64, len: u64) -> std::io::Result<std::result::Result<FatGeometry, bool>> {
        if len < 2 * BOOT_SECTOR as u64 {
            return Ok(Err(false));
        }
        let mut boot = [0; BOOT_SECTOR + 3];
        reader.seek(SeekFrom::Start(start))?;
        reader.read_exact(&mut boot)?;
        Ok(match FatGeometry::from_boot_sector(&boot) {
            Err(false) => FatGeometry::dos1(len, &boot[BOOT_SECTOR..]).ok_or(false),
            result => result,
        })
    }

    /// Returns true if the reader holds a FAT12 image whose size fits its boot sector.
    ///
    /// FAT images have no signature, so this is only a fallback after formats with
    /// magic bytes. The reader position is restored afterwards.
    pub fn probe(reader: &mut T) -> std::io::Result<bool> {
        let start = reader.stream_position()?;
        let len = reader.seek(SeekFrom::End(0))?.saturating_sub(start);
        let result = (|| {
            let Ok(geometry) = Self::read_geometry(reader, start, len)? else {
                return Ok(false);
            };
            if len < geometry.size() || len > geometry.size() + 65_536 {
                return Ok(false);
            }
            reader.seek(SeekFrom::Start(start + geometry.fat_offset()))?;
            let mut head = [0; 3];
            reader.read_exact(&mut head)?;
            Ok(u16_at(&head, 0) & 0x0FFF >= 0xFF0 && u16_at(&head, 1) >> 4 >= END_OF_CHAIN)
        })();
        reader.seek(SeekFrom::Start(start))?;
        result
    }

    /// The layout of the filesystem.
    pub fn geometry(&self) -> FatGeometry {
        self.geometry
    }

    /// The volume label from the root directory, if any.
    pub fn volume_label(&self) -> Option<&str> {
        self.volume_label.as_deref()
    }

    fn read_at(&mut self, offset: u64, size: usize) -> Result<Vec<u8>> {
        if offset > self.len || size as u64 > self.len - offset {
            return Err(corrupt("data lies past the end of the image"));
        }
        self.reader.seek(SeekFrom::Start(self.start + offset))?;
        let mut data = vec![0; size];
        self.reader.read_exact(&mut data)?;
        Ok(data)
    }

    fn fat_entry(&self, cluster: u32) -> u16 {
        let offset = cluster as usize * 3 / 2;
        let value = u16_at(&self.fat, offset);
        if cluster & 1 == 0 {
            value & 0x0FFF
        } else {
            value >> 4
        }
    }

    fn valid_cluster(&self, cluster: u32) -> bool {
        (2..self.geometry.clusters() + 2).contains(&cluster) && cluster < 0xFF0
    }

    /// Follows a cluster chain. Files stop after `needed` clusters; directories run to the end mark.
    fn chain(&self, start: u32, needed: Option<u64>, visited: &mut [bool]) -> Result<Vec<u32>> {
        let mut chain = Vec::new();
        let mut cluster = start;
        loop {
            if !self.valid_cluster(cluster) {
                return Err(corrupt(format!("cluster {cluster:#x} is not a data cluster")));
            }
            if std::mem::replace(&mut visited[cluster as usize], true) {
                return Err(corrupt(format!("cluster chain loops or is shared at cluster {cluster:#x}")));
            }
            chain.push(cluster);
            let next = self.fat_entry(cluster);
            if next < END_OF_CHAIN && !self.valid_cluster(u32::from(next)) {
                return Err(corrupt(format!("cluster {next:#x} is not a data cluster")));
            }
            if needed == Some(chain.len() as u64) {
                // Clusters past the recorded size are ignored, as DOS does.
                return Ok(chain);
            }
            if next >= END_OF_CHAIN {
                return match needed {
                    Some(_) => Err(corrupt("cluster chain ends before the recorded file size")),
                    None => Ok(chain),
                };
            }
            cluster = u32::from(next);
        }
    }

    fn read_clusters(&mut self, chain: &[u32], size: u64) -> Result<Vec<u8>> {
        let cluster_size = self.geometry.cluster_size();
        let mut data = Vec::with_capacity(crate::limits::capacity_hint(size));
        for &cluster in chain {
            let offset = self.geometry.data_offset() + u64::from(cluster - 2) * cluster_size;
            let take = (size - data.len() as u64).min(cluster_size) as usize;
            data.extend_from_slice(&self.read_at(offset, take)?);
        }
        Ok(data)
    }

    fn list_entries(&mut self) -> Result<()> {
        let root = self.read_at(self.geometry.root_offset(), self.geometry.root_entries as usize * ENTRY_SIZE)?;
        let mut pending = VecDeque::from([(None, root)]);
        let mut directory_visited = vec![false; self.directory_clusters.len()];
        while let Some((parent, data)) = pending.pop_front() {
            for (cluster, index) in self.parse_directory(&data, parent)? {
                let chain = self.chain(cluster, None, &mut directory_visited)?;
                for &c in &chain {
                    self.directory_clusters[c as usize] = true;
                }
                let size = chain.len() as u64 * self.geometry.cluster_size();
                pending.push_back((Some(index), self.read_clusters(&chain, size)?));
            }
        }
        Ok(())
    }

    /// Adds a directory's entries and returns its subdirectories as (start cluster, node index).
    fn parse_directory(&mut self, data: &[u8], parent: Option<usize>) -> Result<Vec<(u32, usize)>> {
        let parent_len = parent.map_or(0, |p| self.nodes[p].path_len);
        let mut subdirectories = Vec::new();
        let mut siblings = HashSet::new();
        let mut long_name: Option<LongName> = None;
        for entry in data.as_chunks::<ENTRY_SIZE>().0 {
            let attributes = entry[11];
            match entry[0] {
                0x00 => break,
                0xE5 => {
                    long_name = None;
                    continue;
                }
                _ => {}
            }
            if attributes & 0x3F == ATTR_LONG_NAME {
                long_name = Self::collect_long_name(long_name.take(), entry);
                continue;
            }
            let long = long_name.take();
            if attributes & ATTR_VOLUME_LABEL != 0 {
                if parent.is_none() && self.volume_label.is_none() {
                    self.volume_label = Some(entry[..11].iter().map(|&b| self.decode_byte(b)).collect::<String>().trim_end().to_string());
                }
                continue;
            }
            if entry[0] == b'.' && entry[1..11].iter().all(|&b| b == b' ' || b == b'.') {
                // "." and ".." refer to this directory and its parent.
                continue;
            }
            let component = match long.filter(|l| l.next == 0 && l.checksum == short_name_checksum(entry)) {
                Some(long) => Self::decode_long_name(&long).map_or_else(|| self.short_name(entry), Ok)?,
                None => self.short_name(entry)?,
            };
            if component.is_empty() || component == "." || component == ".." || component.chars().any(|c| c < ' ' || c == '/' || c == '\\') {
                return Err(corrupt(format!("invalid file name {component:?}")));
            }
            if !siblings.insert(component.to_lowercase()) {
                return Err(corrupt(format!("duplicate file name {component:?}")));
            }
            let is_directory = attributes & ATTR_DIRECTORY != 0;
            let path_len = parent_len + component.len() + usize::from(is_directory);
            if path_len > MAX_PATH_BYTES {
                return Err(corrupt(format!("path exceeds {MAX_PATH_BYTES} bytes")));
            }
            let start_cluster = u32::from(u16_at(entry, 26));
            let index = self.nodes.len();
            if is_directory {
                if start_cluster == 0 {
                    return Err(corrupt(format!("directory {component:?} has no clusters")));
                }
                subdirectories.push((start_cluster, index));
            }
            self.nodes.push(Node {
                parent,
                component,
                path_len,
                size: if is_directory { 0 } else { u64::from(u32_at(entry, 28)) },
                is_directory,
                attributes,
                modified_time: dos_time(u16_at(entry, 24), u16_at(entry, 22)),
                start_cluster,
            });
        }
        Ok(subdirectories)
    }

    fn collect_long_name(current: Option<LongName>, entry: &[u8; ENTRY_SIZE]) -> Option<LongName> {
        if entry[0] & 0xA0 != 0 || entry[12] != 0 || u16_at(entry, 26) != 0 {
            return None;
        }
        let order = entry[0] & 0x1F;
        let mut long = if entry[0] & 0x40 != 0 {
            // The last part comes first; at most 20 parts (255 characters).
            if !(1..=20).contains(&order) {
                return None;
            }
            LongName {
                parts: vec![[0; 13]; usize::from(order)],
                next: order,
                checksum: entry[13],
            }
        } else {
            current?
        };
        if order == 0 || long.next != order || long.checksum != entry[13] {
            return None;
        }
        let part = &mut long.parts[usize::from(order) - 1];
        for (slot, offset) in part.iter_mut().zip([1, 3, 5, 7, 9, 14, 16, 18, 20, 22, 24, 28, 30]) {
            *slot = u16_at(entry, offset);
        }
        long.next = order - 1;
        Some(long)
    }

    /// Returns `None` for malformed UTF-16, which falls back to the 8.3 name.
    fn decode_long_name(long: &LongName) -> Option<String> {
        let units: Vec<_> = long.parts.iter().flatten().copied().collect();
        let length = units.iter().position(|&unit| unit == 0).unwrap_or(units.len());
        if length > 255 || units[..length].contains(&0xFFFF) || units.get(length + 1..).is_some_and(|padding| padding.iter().any(|&unit| unit != 0xFFFF)) {
            return None;
        }
        char::decode_utf16(units[..length].iter().copied())
            .collect::<std::result::Result<String, _>>()
            .ok()
    }

    fn decode_byte(&self, byte: u8) -> char {
        if self.atari_names {
            atari_st(byte)
        } else {
            cp437(byte)
        }
    }

    fn short_name(&self, entry: &[u8; ENTRY_SIZE]) -> Result<String> {
        let mut raw = [0; 11];
        raw.copy_from_slice(&entry[..11]);
        if raw[0] == 0x05 {
            // 0x05 stands for a leading 0xE5, which would otherwise mark a deleted entry.
            raw[0] = 0xE5;
        }
        // Windows NT records all-lowercase base names and extensions as flags.
        let (lower_base, lower_extension) = (!self.atari_names && entry[12] & 0x08 != 0, !self.atari_names && entry[12] & 0x10 != 0);
        let decode = |bytes: &[u8], lower: bool| -> String {
            let text = bytes.iter().map(|&b| self.decode_byte(b)).collect::<String>();
            let text = text.trim_end_matches(' ');
            if lower {
                text.to_ascii_lowercase()
            } else {
                text.to_string()
            }
        };
        let base = decode(&raw[..8], lower_base);
        let extension = decode(&raw[8..], lower_extension);
        Ok(if extension.is_empty() { base } else { format!("{base}.{extension}") })
    }

    fn path(&self, index: usize) -> String {
        let mut components = Vec::new();
        let mut current = Some(index);
        while let Some(i) = current {
            components.push(&self.nodes[i]);
            current = self.nodes[i].parent;
        }
        let mut path = String::with_capacity(self.nodes[index].path_len);
        for node in components.iter().rev() {
            path.push_str(&node.component);
            if node.is_directory {
                path.push('/');
            }
        }
        path
    }

    fn entry(&self, index: usize) -> FatEntry {
        let node = &self.nodes[index];
        FatEntry {
            name: self.path(index),
            size: node.size,
            is_directory: node.is_directory,
            attributes: node.attributes,
            modified_time: node.modified_time,
            start_cluster: node.start_cluster,
            index,
        }
    }

    pub fn get_next_entry(&mut self) -> Result<Option<FatEntry>> {
        if self.next >= self.nodes.len() {
            return Ok(None);
        }
        self.next += 1;
        Ok(Some(self.entry(self.next - 1)))
    }

    /// Returns this archive's own record for `entry`; callers' copies may have been modified.
    fn stored(&self, entry: &FatEntry) -> Result<&Node> {
        self.nodes
            .get(entry.index)
            .filter(|node| node.start_cluster == entry.start_cluster && node.is_directory == entry.is_directory && self.path(entry.index) == entry.name)
            .ok_or_else(|| ArchiveError::IndexMismatch("Entry does not belong to this FAT image".into()))
    }

    /// Reads an entry with the default per-file limit.
    pub fn read(&mut self, entry: &FatEntry) -> Result<Vec<u8>> {
        self.read_with_limit(entry, Some(crate::limits::DEFAULT_MAX_ENTRY_SIZE))
    }

    pub fn read_with_limit(&mut self, entry: &FatEntry, limit: Option<u64>) -> Result<Vec<u8>> {
        let node = self.stored(entry)?.clone();
        crate::limits::check_size(node.size, limit, &entry.name)?;
        if node.is_directory || node.size == 0 {
            return Ok(Vec::new());
        }
        let needed = node.size.div_ceil(self.geometry.cluster_size());
        if needed > u64::from(self.geometry.clusters()) {
            return Err(corrupt("file size exceeds the volume"));
        }
        let mut visited = vec![false; self.directory_clusters.len()];
        let chain = self.chain(node.start_cluster, Some(needed), &mut visited)?;
        if chain.iter().any(|&c| self.directory_clusters[c as usize]) {
            return Err(corrupt("file data shares clusters with a directory"));
        }
        self.read_clusters(&chain, node.size)
    }

    pub fn skip(&mut self, entry: &FatEntry) -> Result<()> {
        self.stored(entry).map(|_| ())
    }
}
