use std::collections::{HashMap, HashSet};
use std::io::{Read, Seek, SeekFrom};

use chrono::{Datelike, Duration, NaiveDate, Timelike};

use crate::date_time::DosDateTime;
use crate::error::{ArchiveError, Result};

const SECTOR_SIZE: u64 = 512;
const ADF_DD_SIZE: u64 = 901_120;
const ADF_HD_SIZE: u64 = 1_802_240;
const END: u32 = u32::MAX;

fn corrupt(reason: impl Into<String>) -> ArchiveError {
    ArchiveError::corrupted_entry("Amiga", reason)
}

fn word(block: &[u8], index: usize) -> u32 {
    let offset = index * 4;
    u32::from_be_bytes([block[offset], block[offset + 1], block[offset + 2], block[offset + 3]])
}

fn tail(block: &[u8], bytes: usize) -> u32 {
    word(block, (block.len() - bytes) / 4)
}

fn checksum(block: &[u8]) -> bool {
    block
        .as_chunks::<4>()
        .0
        .iter()
        .fold(0u32, |sum, bytes| sum.wrapping_add(u32::from_be_bytes(*bytes)))
        == 0
}

fn block_size(size: u64) -> Result<usize> {
    if !(512..=65_536).contains(&size) || !size.is_power_of_two() {
        return Err(ArchiveError::unsupported_format(format!(
            "Amiga block size {size} (supported: powers of two from 512 to 65536)"
        )));
    }
    Ok(size as usize)
}

fn name(block: &[u8], offset: usize, max: usize) -> Result<String> {
    let len = usize::from(block[offset]);
    if len == 0 || len > max {
        return Err(corrupt("invalid Amiga name length"));
    }
    let bytes = &block[offset + 1..offset + 1 + len];
    if bytes.iter().any(|b| matches!(b, 0 | b'/' | b':')) {
        return Err(corrupt("invalid Amiga name"));
    }
    Ok(bytes.iter().map(|&b| char::from(b)).collect())
}

fn modified(block: &[u8]) -> Result<Option<DosDateTime>> {
    let (days, minutes, ticks) = (tail(block, 92), tail(block, 88), tail(block, 84));
    if minutes >= 1440 || ticks >= 3000 {
        return Err(corrupt("invalid Amiga timestamp"));
    }
    let date = NaiveDate::from_ymd_opt(1978, 1, 1)
        .and_then(|epoch| epoch.checked_add_signed(Duration::days(i64::from(days))))
        .ok_or_else(|| corrupt("Amiga timestamp out of range"))?;
    if !(1980..=2107).contains(&date.year()) {
        // The unified API's DOS timestamp cannot represent the full Amiga epoch.
        return Ok(None);
    }
    let date_word = ((date.year() - 1980) as u16) << 9 | (date.month() as u16) << 5 | date.day() as u16;
    let time = date
        .and_hms_opt(minutes / 60, minutes % 60, ticks / 50)
        .ok_or_else(|| corrupt("invalid Amiga timestamp"))?;
    let time_word = (time.hour() as u16) << 11 | (time.minute() as u16) << 5 | (time.second() as u16 / 2);
    Ok(Some(DosDateTime::from((date_word, time_word))))
}

/// Entry types recorded by an Amiga filesystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmigaEntryKind {
    File,
    Directory,
    SymbolicLink,
    HardLink,
}

/// A file, directory or link in an Amiga volume.
#[derive(Debug, Clone)]
pub struct AmigaEntry {
    pub name: String,
    pub size: u64,
    pub kind: AmigaEntryKind,
    pub modified_time: Option<DosDateTime>,
    pub protection: u32,
    pub comment: String,
    pub link_target: Option<String>,
    volume: usize,
    block: u32,
    real_entry: u32,
    hard_directory: bool,
}

impl AmigaEntry {
    /// Index into [`AmigaArchive::volumes`].
    pub fn volume_index(&self) -> usize {
        self.volume
    }
}

/// An OFS/FFS volume, either the whole image or one RDB partition.
#[derive(Debug, Clone)]
pub struct AmigaVolume {
    /// RDB device name, such as `DH0`, or `None` for a filesystem-only image.
    pub partition_name: Option<String>,
    pub name: String,
    /// DOS type byte: 0/2/4 OFS, 1/3/5 FFS.
    pub dos_type: u8,
    pub block_size: usize,
    offset: u64,
    blocks: u64,
    reserved: u32,
    root: u32,
}

impl AmigaVolume {
    pub fn filesystem(&self) -> &'static str {
        if self.dos_type & 1 == 0 {
            "OFS"
        } else {
            "FFS"
        }
    }
}

/// A seek-based Amiga disk-image reader. HDF images are not loaded into memory.
pub struct AmigaArchive<T: Read + Seek> {
    reader: T,
    start: u64,
    len: u64,
    volumes: Vec<AmigaVolume>,
    entries: Vec<AmigaEntry>,
    next: usize,
}

impl<T: Read + Seek> AmigaArchive<T> {
    /// Opens a standard DD or HD ADF with 512-byte sectors.
    pub fn open_adf(reader: T) -> Result<Self> {
        let mut archive = Self::empty(reader)?;
        if !matches!(archive.len, ADF_DD_SIZE | ADF_HD_SIZE) {
            return Err(ArchiveError::invalid_header("ADF (expected DD or HD sector image)"));
        }
        archive.add_volume(0, archive.len, 512, 2, None)?;
        archive.list_entries()?;
        Ok(archive)
    }

    /// Opens a filesystem-only HDF or an RDB-partitioned hard disk.
    pub fn open_hdf(reader: T) -> Result<Self> {
        let mut archive = Self::empty(reader)?;
        if archive.len < 3 * SECTOR_SIZE || archive.len % SECTOR_SIZE != 0 {
            return Err(ArchiveError::invalid_header("HDF"));
        }
        let boot = archive.read_at(0, 4)?;
        if &boot[..3] == b"DOS" {
            archive.add_flat_volume()?;
        } else if let Some(offset) = archive.find_rdb()? {
            archive.read_partitions(offset)?;
        } else {
            archive.add_flat_volume()?;
        }
        archive.list_entries()?;
        Ok(archive)
    }

    fn empty(mut reader: T) -> Result<Self> {
        let start = reader.stream_position()?;
        let len = reader
            .seek(SeekFrom::End(0))?
            .checked_sub(start)
            .ok_or_else(|| corrupt("image starts past EOF"))?;
        Ok(Self {
            reader,
            start,
            len,
            volumes: Vec::new(),
            entries: Vec::new(),
            next: 0,
        })
    }

    /// Detects Amiga images, preserving the reader position. `true` means HDF.
    ///
    /// Filesystem-only images of standard floppy sizes are classified as ADF.
    pub fn probe(reader: &mut T) -> std::io::Result<Option<bool>> {
        let start = reader.stream_position()?;
        let result = (|| {
            let len = reader.seek(SeekFrom::End(0))?.saturating_sub(start);
            if len < 512 || len % 512 != 0 {
                return Ok(None);
            }
            let mut magic = [0; 4];
            reader.seek(SeekFrom::Start(start))?;
            reader.read_exact(&mut magic)?;
            if &magic[..3] == b"DOS" {
                return Ok(Some(!matches!(len, ADF_DD_SIZE | ADF_HD_SIZE)));
            }
            // RDBs can reside in any of the first sixteen physical blocks.
            for offset in (0..len.min(16 * 512)).step_by(512) {
                reader.seek(SeekFrom::Start(start + offset))?;
                let mut header = [0; 20];
                reader.read_exact(&mut header)?;
                if &header[..4] == b"RDSK" {
                    let size = u64::from(word(&header, 4));
                    if size == 512 {
                        return Ok(Some(true));
                    }
                }
            }
            Ok(None)
        })();
        reader.seek(SeekFrom::Start(start))?;
        result
    }

    fn read_at(&mut self, offset: u64, size: usize) -> Result<Vec<u8>> {
        if offset > self.len || size as u64 > self.len - offset {
            return Err(corrupt("block points outside the image"));
        }
        self.reader.seek(SeekFrom::Start(self.start + offset))?;
        let mut data = vec![0; size];
        self.reader.read_exact(&mut data)?;
        Ok(data)
    }

    fn find_rdb(&mut self) -> Result<Option<u64>> {
        for offset in (0..self.len.min(16 * 512)).step_by(512) {
            let data = self.read_at(offset, 20)?;
            if &data[..4] == b"RDSK" {
                if word(&data, 4) != 512 {
                    return Err(ArchiveError::unsupported_format("Amiga RDB physical sectors other than 512 bytes"));
                }
                return Ok(Some(offset));
            }
        }
        Ok(None)
    }

    fn rdb_block(&mut self, offset: u64, size: usize, magic: &[u8; 4]) -> Result<Vec<u8>> {
        let data = self.read_at(offset, size)?;
        let longs = usize::try_from(word(&data, 1)).map_err(|_| corrupt("RDB checksum length overflow"))?;
        if &data[..4] != magic || longs < 64 || longs > size / 4 || !checksum(&data[..longs * 4]) {
            return Err(corrupt("invalid RDB/PART signature, size or checksum"));
        }
        Ok(data)
    }

    fn read_partitions(&mut self, offset: u64) -> Result<()> {
        let header = self.read_at(offset, 20)?;
        let size = block_size(u64::from(word(&header, 4)))?;
        let rdb = self.rdb_block(offset, size, b"RDSK")?;
        if word(&rdb, 6) != END {
            return Err(ArchiveError::unsupported_format("Amiga RDB bad-block replacement lists"));
        }
        let rdb_lo = u64::from(word(&rdb, 32));
        let rdb_hi = u64::from(word(&rdb, 33));
        let rdb_index = offset / size as u64;
        if rdb_lo > rdb_hi || rdb_hi >= self.len / size as u64 || rdb_index < rdb_lo || rdb_index > rdb_hi {
            return Err(corrupt("invalid RDB reserved area"));
        }
        let mut block = word(&rdb, 7);
        let mut visited = HashSet::new();
        let mut names = HashSet::new();
        let mut ranges = Vec::new();
        while block != END {
            if u64::from(block) < rdb_lo || u64::from(block) > rdb_hi || !visited.insert(block) {
                return Err(corrupt("invalid or cyclic RDB partition list"));
            }
            let part = self.rdb_block(u64::from(block) * size as u64, size, b"PART")?;
            let partition_name = name(&part, 36, 31)?;
            if !names.insert(partition_name.to_ascii_lowercase()) {
                return Err(corrupt("duplicate RDB partition name"));
            }
            let env_len = word(&part, 32);
            if !(16..=31).contains(&env_len) || 128 + (env_len as usize + 1) * 4 > usize::try_from(word(&part, 1)).unwrap_or(0) * 4 {
                return Err(corrupt("invalid RDB DOS environment vector"));
            }
            if word(&part, 34) != 0 {
                return Err(ArchiveError::unsupported_format("Amiga RDB nonzero sector origin"));
            }
            let sector_bytes = u64::from(word(&part, 33)) * 4;
            let sectors_per_block = u64::from(word(&part, 36));
            if sector_bytes != size as u64 {
                return Err(ArchiveError::unsupported_format("Amiga partition sector size differs from the RDB sector size"));
            }
            let fs_size = block_size(
                sector_bytes
                    .checked_mul(sectors_per_block)
                    .ok_or_else(|| corrupt("RDB filesystem block size overflow"))?,
            )?;
            let heads = u64::from(word(&part, 35));
            let sectors = u64::from(word(&part, 37));
            let low = u64::from(word(&part, 41));
            let high = u64::from(word(&part, 42));
            let cylinder_bytes = heads
                .checked_mul(sectors)
                .and_then(|n| n.checked_mul(size as u64))
                .ok_or_else(|| corrupt("RDB geometry overflow"))?;
            let begin = low.checked_mul(cylinder_bytes).ok_or_else(|| corrupt("RDB partition offset overflow"))?;
            let end = high
                .checked_add(1)
                .and_then(|n| n.checked_mul(cylinder_bytes))
                .ok_or_else(|| corrupt("RDB partition end overflow"))?;
            if cylinder_bytes == 0 || low > high || end > self.len || begin < (rdb_hi + 1) * size as u64 || ranges.iter().any(|&(a, b)| begin < b && end > a) {
                return Err(corrupt("invalid or overlapping RDB partition geometry"));
            }
            ranges.push((begin, end));
            self.add_volume(begin, end - begin, fs_size, word(&part, 38), Some(partition_name))?;
            block = word(&part, 4);
        }
        if self.volumes.is_empty() {
            return Err(corrupt("RDB contains no partitions"));
        }
        Ok(())
    }

    fn add_volume(&mut self, offset: u64, len: u64, size: usize, reserved: u32, partition_name: Option<String>) -> Result<()> {
        let blocks = len / size as u64;
        if !len.is_multiple_of(size as u64) || reserved == 0 || blocks <= u64::from(reserved) || blocks > u64::from(u32::MAX) {
            return Err(corrupt("invalid Amiga volume size or reserved blocks"));
        }

        let boot = self.read_at(offset, size)?;
        if &boot[..3] != b"DOS" || boot[3] > 5 {
            return Err(ArchiveError::unsupported_format(format!(
                "Amiga filesystem {:02x?} (only DOS0 through DOS5 supported)",
                &boot[..4]
            )));
        }
        let mut volume = AmigaVolume {
            partition_name,
            name: String::new(),
            dos_type: boot[3],
            block_size: size,
            offset,
            blocks,
            reserved,
            root: ((u64::from(reserved) + blocks - 1) / 2) as u32,
        };
        let root = self.fs_block(&volume, volume.root)?;
        if word(&root, 0) != 2 || tail(&root, 4) != 1 || word(&root, 3) as usize != size / 4 - 56 {
            return Err(corrupt("invalid Amiga root block"));
        }
        volume.name = name(&root, size - 80, 30)?;
        self.volumes.push(volume);
        Ok(())
    }

    fn add_flat_volume(&mut self) -> Result<()> {
        let boot = self.read_at(0, 512)?;
        if &boot[..3] != b"DOS" || boot[3] > 5 {
            return Err(ArchiveError::unsupported_format(format!(
                "Amiga filesystem {:02x?} (only DOS0 through DOS5 supported)",
                &boot[..4]
            )));
        }
        for size in [512, 1024, 2048, 4096, 8192, 16384, 32768, 65536] {
            if !self.len.is_multiple_of(size as u64) {
                continue;
            }
            let blocks = self.len / size as u64;
            if blocks < 3 || blocks > u64::from(u32::MAX) {
                continue;
            }
            for reserved in [2, 1] {
                let root = (reserved + blocks - 1) / 2;
                let data = self.read_at(root * size as u64, size)?;
                if word(&data, 0) == 2 && tail(&data, 4) == 1 && word(&data, 3) as usize == size / 4 - 56 && checksum(&data) {
                    return self.add_volume(0, self.len, size, reserved as u32, None);
                }
            }
        }
        Err(corrupt("no valid OFS/FFS root block at the volume midpoint"))
    }

    fn fs_block(&mut self, volume: &AmigaVolume, block: u32) -> Result<Vec<u8>> {
        let data = self.data_block(volume, block)?;
        if !checksum(&data) {
            return Err(corrupt(format!("block {block} checksum mismatch")));
        }
        Ok(data)
    }

    fn data_block(&mut self, volume: &AmigaVolume, block: u32) -> Result<Vec<u8>> {
        if block < volume.reserved || u64::from(block) >= volume.blocks {
            return Err(corrupt(format!("block {block} points outside its volume")));
        }
        self.read_at(volume.offset + u64::from(block) * volume.block_size as u64, volume.block_size)
    }

    fn list_entries(&mut self) -> Result<()> {
        for volume_index in 0..self.volumes.len() {
            let volume = self.volumes[volume_index].clone();
            let prefix = volume.partition_name.as_ref().map(|n| format!("{n}/")).unwrap_or_default();
            let root = self.fs_block(&volume, volume.root)?;
            if !prefix.is_empty() {
                self.entries.push(AmigaEntry {
                    name: prefix.clone(),
                    size: 0,
                    kind: AmigaEntryKind::Directory,
                    modified_time: modified(&root)?,
                    protection: 0,
                    comment: String::new(),
                    link_target: None,
                    volume: volume_index,
                    block: volume.root,
                    real_entry: 0,
                    hard_directory: false,
                });
            }
            let mut pending = vec![(volume.root, prefix)];
            let mut visited = HashSet::from([volume.root]);
            while let Some((parent, prefix)) = pending.pop() {
                let directory = self.fs_block(&volume, parent)?;
                let mut siblings = HashSet::new();
                for bucket in 0..volume.block_size / 4 - 56 {
                    let mut block = word(&directory, 6 + bucket);
                    while block != 0 {
                        if !visited.insert(block) {
                            return Err(corrupt("cyclic or multiply referenced directory entry"));
                        }
                        let data = self.fs_block(&volume, block)?;
                        if word(&data, 0) != 2 || word(&data, 1) != block || tail(&data, 12) != parent {
                            return Err(corrupt("invalid directory entry header or parent"));
                        }
                        let component = name(&data, volume.block_size - 80, 30)?;
                        let folded: Vec<_> = component
                            .chars()
                            .map(|c| {
                                let b = c as u8;
                                if b.is_ascii_lowercase() || (volume.dos_type >= 2 && (224..=254).contains(&b) && b != 247) {
                                    b - 32
                                } else {
                                    b
                                }
                            })
                            .collect();
                        if !siblings.insert(folded) {
                            return Err(corrupt("duplicate Amiga directory name"));
                        }
                        let subtype = tail(&data, 4) as i32;
                        let kind = match subtype {
                            -3 => AmigaEntryKind::File,
                            2 => AmigaEntryKind::Directory,
                            3 => AmigaEntryKind::SymbolicLink,
                            -4 | 4 => AmigaEntryKind::HardLink,
                            _ => return Err(ArchiveError::unsupported_method("Amiga", format!("directory entry type {subtype}"))),
                        };
                        let mut entry = AmigaEntry {
                            name: format!("{prefix}{component}"),
                            size: if subtype == -3 { u64::from(tail(&data, 188)) } else { 0 },
                            kind,
                            modified_time: modified(&data)?,
                            protection: tail(&data, 192),
                            comment: String::new(),
                            link_target: None,
                            volume: volume_index,
                            block,
                            real_entry: tail(&data, 44),
                            hard_directory: subtype == 4,
                        };
                        let comment_offset = volume.block_size - 184;
                        let comment_len = usize::from(data[comment_offset]);
                        if comment_len > 79 {
                            return Err(corrupt("invalid Amiga comment length"));
                        }
                        entry.comment = data[comment_offset + 1..comment_offset + 1 + comment_len]
                            .iter()
                            .map(|&b| char::from(b))
                            .collect();
                        if kind == AmigaEntryKind::Directory {
                            entry.name.push('/');
                            pending.push((block, entry.name.clone()));
                        } else if kind == AmigaEntryKind::SymbolicLink {
                            let target = &data[24..volume.block_size - 200];
                            let len = target.iter().position(|&b| b == 0).ok_or_else(|| corrupt("unterminated Amiga symbolic link"))?;
                            entry.link_target = Some(target[..len].iter().map(|&b| char::from(b)).collect());
                            entry.size = len as u64;
                        }
                        self.entries.push(entry);
                        block = tail(&data, 16);
                    }
                }
            }
        }
        let targets: HashMap<_, _> = self.entries.iter().map(|e| ((e.volume, e.block), (e.name.clone(), e.size, e.kind))).collect();
        for entry in &mut self.entries {
            if entry.kind == AmigaEntryKind::HardLink {
                let (target, size, kind) = targets
                    .get(&(entry.volume, entry.real_entry))
                    .ok_or_else(|| corrupt("hard link target is not in the directory tree"))?;
                if *kind
                    != if entry.hard_directory {
                        AmigaEntryKind::Directory
                    } else {
                        AmigaEntryKind::File
                    }
                {
                    return Err(corrupt("hard link target type mismatch"));
                }
                entry.link_target = Some(target.clone());
                entry.size = *size;
            }
        }
        Ok(())
    }

    pub fn volumes(&self) -> &[AmigaVolume] {
        &self.volumes
    }

    pub fn get_next_entry(&mut self) -> Result<Option<AmigaEntry>> {
        let entry = self.entries.get(self.next).cloned();
        if entry.is_some() {
            self.next += 1;
        }
        Ok(entry)
    }

    /// Reads an entry with the default per-file limit.
    pub fn read(&mut self, entry: &AmigaEntry) -> Result<Vec<u8>> {
        self.read_with_limit(entry, Some(crate::limits::DEFAULT_MAX_ENTRY_SIZE))
    }

    pub fn read_with_limit(&mut self, entry: &AmigaEntry, limit: Option<u64>) -> Result<Vec<u8>> {
        let stored = self
            .entries
            .iter()
            .find(|e| e.volume == entry.volume && e.block == entry.block && e.name == entry.name)
            .ok_or_else(|| ArchiveError::IndexMismatch("Entry does not belong to this Amiga image".into()))?
            .clone();
        crate::limits::check_size(stored.size, limit, &stored.name)?;
        if stored.kind == AmigaEntryKind::Directory {
            return Ok(Vec::new());
        }
        if stored.kind == AmigaEntryKind::SymbolicLink {
            let target = stored.link_target.ok_or_else(|| corrupt("missing symbolic link target"))?;
            return Ok(target.chars().map(|c| c as u8).collect());
        }
        if stored.hard_directory {
            return Err(ArchiveError::unsupported_method("Amiga", "reading a hard link to a directory"));
        }
        let volume = self.volumes[stored.volume].clone();
        let header_block = if stored.kind == AmigaEntryKind::HardLink {
            stored.real_entry
        } else {
            stored.block
        };
        let mut header = self.fs_block(&volume, header_block)?;
        let payload = volume.block_size - if volume.dos_type & 1 == 0 { 24 } else { 0 };
        let expected_blocks = stored.size.div_ceil(payload as u64);
        if expected_blocks > volume.blocks - u64::from(volume.reserved) {
            return Err(corrupt("file size exceeds volume capacity"));
        }
        let mut pointers = Vec::new();
        let metadata: HashSet<_> = self
            .entries
            .iter()
            .filter(|e| e.volume == stored.volume)
            .map(|e| e.block)
            .chain([volume.root])
            .collect();
        let mut visited = HashSet::from([header_block]);
        let first_data = word(&header, 4);
        let mut table_block = header_block;
        loop {
            let expected_type = if table_block == header_block { 2 } else { 16 };
            if word(&header, 0) != expected_type
                || word(&header, 1) != table_block
                || tail(&header, 4) as i32 != -3
                || (table_block != header_block && tail(&header, 12) != header_block)
            {
                return Err(corrupt("invalid file header or extension block"));
            }
            let count = word(&header, 2) as usize;
            if count > volume.block_size / 4 - 56 || pointers.len() as u64 + count as u64 > expected_blocks {
                return Err(corrupt("file data-block count disagrees with size"));
            }
            for index in 0..count {
                let block = tail(&header, 204 + index * 4);
                if metadata.contains(&block) || !visited.insert(block) {
                    return Err(corrupt("cyclic or repeated file data/extension block"));
                }
                pointers.push(block);
            }
            let extension = tail(&header, 8);
            if extension == 0 {
                break;
            }
            if count != volume.block_size / 4 - 56 || !visited.insert(extension) {
                return Err(corrupt("invalid or cyclic file extension chain"));
            }
            table_block = extension;
            header = self.fs_block(&volume, extension)?;
        }
        if pointers.len() as u64 != expected_blocks || (volume.dos_type & 1 == 0 && first_data != pointers.first().copied().unwrap_or(0)) {
            return Err(corrupt("missing file data blocks"));
        }
        let mut output = Vec::with_capacity(crate::limits::capacity_hint(stored.size));
        for (index, &block) in pointers.iter().enumerate() {
            let data = self.data_block(&volume, block)?;
            let remaining = stored.size - output.len() as u64;
            let take = remaining.min(payload as u64) as usize;
            if volume.dos_type & 1 == 0 {
                let next = pointers.get(index + 1).copied().unwrap_or(0);
                if !checksum(&data)
                    || word(&data, 0) != 8
                    || word(&data, 1) != header_block
                    || word(&data, 2) as usize != index + 1
                    || word(&data, 3) as usize != take
                    || word(&data, 4) != next
                {
                    return Err(corrupt("invalid OFS data block checksum, owner, sequence, size or link"));
                }
                output.extend_from_slice(&data[24..24 + take]);
            } else {
                output.extend_from_slice(&data[..take]);
            }
        }
        Ok(output)
    }

    pub fn skip(&mut self, entry: &AmigaEntry) -> Result<()> {
        if self
            .entries
            .iter()
            .any(|e| e.volume == entry.volume && e.block == entry.block && e.name == entry.name)
        {
            Ok(())
        } else {
            Err(ArchiveError::IndexMismatch("Entry does not belong to this Amiga image".into()))
        }
    }
}
