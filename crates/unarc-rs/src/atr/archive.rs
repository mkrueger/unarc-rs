use std::fmt;
use std::io::{Read, Seek, SeekFrom};

use crate::date_time::DosDateTime;
use crate::error::{ArchiveError, Result};

use super::{dos2, sparta};

/// Longest entry path, in UTF-8 bytes.
pub const MAX_PATH_BYTES: usize = 4096;
const HEADER_SIZE: usize = 16;
const MAGIC: [u8; 2] = [0x96, 0x02];
/// Sector numbers are 16 bits wide in every supported filesystem.
const MAX_SECTORS: usize = 65_535;
/// Sectors 1-3 hold 128 bytes even on double-density disks.
const BOOT_SECTORS: usize = 3;
const BOOT_SECTOR_SIZE: usize = 128;
/// Sector reads allowed while listing, as a multiple of the sector count. A
/// consistent filesystem reads each sector at most once or twice.
const LISTING_READS_PER_SECTOR: usize = 16;

pub(super) fn corrupt(reason: impl Into<String>) -> ArchiveError {
    ArchiveError::corrupted_entry("ATR", reason)
}

/// ATASCII 0x00-0x1F: box-drawing and other graphics characters.
const ATASCII_GRAPHICS: [char; 32] = [
    '♥', '┣', '┃', '┛', '┫', '┓', '╱', '╲', '◢', '▗', '◣', '▝', '▘', '▔', '▁', '▖', //
    '♣', '┏', '━', '╋', '●', '▄', '▎', '┳', '┻', '▌', '┗', '␛', '↑', '↓', '←', '→',
];

/// Converts an ATASCII byte. Inverse-video characters (bit 7) map to their normal form.
fn atascii(byte: u8) -> char {
    match byte & 0x7F {
        b @ 0x00..=0x1F => ATASCII_GRAPHICS[usize::from(b)],
        0x60 => '♦',
        0x7B => '♠',
        0x7D => '↰',
        0x7E => '◀',
        0x7F => '▶',
        b => char::from(b),
    }
}

/// Converts a space-padded 8.3 directory name into an entry name component.
///
/// `/` and `\` become `_`, and names that would be empty or consist only of
/// dots (title lines in directory art) have each character replaced by `_`.
pub(super) fn file_name(raw: &[u8]) -> String {
    let part = |bytes: &[u8]| -> String {
        let end = bytes.iter().rposition(|&b| b != b' ').map_or(0, |last| last + 1);
        bytes[..end]
            .iter()
            .map(|&b| match atascii(b) {
                '/' | '\\' => '_',
                c => c,
            })
            .collect()
    };
    let (base, extension) = (part(&raw[..8]), part(&raw[8..11]));
    let name = if extension.is_empty() { base } else { format!("{base}.{extension}") };
    if name.chars().all(|c| c == '.') {
        "_".repeat(name.len().max(1))
    } else {
        name
    }
}

pub(super) fn u16_at(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([data[offset], data[offset + 1]])
}

/// The sectors of an ATR image, read into memory.
pub(super) struct Image {
    data: Vec<u8>,
    sector_size: usize,
    sectors: usize,
    /// Distance between the boot sectors: 256 when they are padded to full double-density sectors.
    boot_stride: usize,
    /// Offset of sector 4.
    data_offset: usize,
}

impl Image {
    /// Lays out the sectors. With `full_boot_slots`, a double-density image whose length is
    /// a multiple of 256 keeps its boot sectors in 256-byte slots: either each padded
    /// ("physical"), or all three packed first and followed by 384 zero bytes.
    fn new(data: Vec<u8>, sector_size: usize, full_boot_slots: bool) -> Self {
        let len = data.len();
        let (boot_stride, data_offset, sectors) = if sector_size == BOOT_SECTOR_SIZE {
            (BOOT_SECTOR_SIZE, BOOT_SECTORS * BOOT_SECTOR_SIZE, len / sector_size)
        } else if full_boot_slots && Self::has_full_boot_slots(&data, sector_size) {
            let packed = data[BOOT_SECTORS * BOOT_SECTOR_SIZE..2 * BOOT_SECTORS * BOOT_SECTOR_SIZE]
                .iter()
                .all(|&b| b == 0);
            let stride = if packed { BOOT_SECTOR_SIZE } else { sector_size };
            (stride, BOOT_SECTORS * sector_size, len / sector_size)
        } else if len >= BOOT_SECTORS * BOOT_SECTOR_SIZE {
            let data_offset = BOOT_SECTORS * BOOT_SECTOR_SIZE;
            (BOOT_SECTOR_SIZE, data_offset, BOOT_SECTORS + (len - data_offset) / sector_size)
        } else {
            (BOOT_SECTOR_SIZE, BOOT_SECTORS * BOOT_SECTOR_SIZE, len / BOOT_SECTOR_SIZE)
        };
        Self {
            data,
            sector_size,
            sectors: sectors.min(MAX_SECTORS),
            boot_stride,
            data_offset,
        }
    }

    fn has_full_boot_slots(data: &[u8], sector_size: usize) -> bool {
        sector_size != BOOT_SECTOR_SIZE && data.len().is_multiple_of(sector_size) && data.len() >= 2 * BOOT_SECTORS * BOOT_SECTOR_SIZE
    }

    pub(super) fn sector_size(&self) -> usize {
        self.sector_size
    }

    pub(super) fn sectors(&self) -> usize {
        self.sectors
    }

    /// Returns sector `number` (counted from 1), or `None` if the image does not contain it.
    pub(super) fn sector(&self, number: usize) -> Option<&[u8]> {
        if number == 0 || number > self.sectors {
            return None;
        }
        let (offset, size) = if number <= BOOT_SECTORS {
            ((number - 1) * self.boot_stride, BOOT_SECTOR_SIZE)
        } else {
            (self.data_offset + (number - BOOT_SECTORS - 1) * self.sector_size, self.sector_size)
        };
        self.data.get(offset..offset + size)
    }
}

/// Marks visited sectors to detect loops, and limits the total number of sector reads.
pub(super) struct Visited {
    marks: Vec<u32>,
    generation: u32,
    budget: usize,
    exhausted: bool,
}

impl Visited {
    pub(super) fn new(sectors: usize, budget: usize) -> Self {
        Self {
            marks: vec![0; sectors + 1],
            generation: 0,
            budget,
            exhausted: false,
        }
    }

    /// Starts a new walk; sectors visited by earlier walks may be visited again.
    pub(super) fn start(&mut self) {
        if self.generation == u32::MAX {
            self.marks.fill(0);
            self.generation = 0;
        }
        self.generation += 1;
    }

    /// Records a visit, failing if this walk already visited `sector` or the read budget is spent.
    /// `sector` must be at most the sector count passed to [`Visited::new`].
    pub(super) fn visit(&mut self, sector: usize) -> std::result::Result<(), String> {
        self.spend(1)?;
        if std::mem::replace(&mut self.marks[sector], self.generation) == self.generation {
            return Err(format!("sector chain loops back to sector {sector}"));
        }
        Ok(())
    }

    /// Charges `reads` sector reads that need no loop check.
    pub(super) fn spend(&mut self, reads: usize) -> std::result::Result<(), String> {
        match self.budget.checked_sub(reads) {
            Some(budget) => self.budget = budget,
            None => {
                self.exhausted = true;
                return Err("too many sector reads; files or directories share sectors".into());
            }
        }
        Ok(())
    }

    /// Fails if the read budget ran out, which only happens on images with shared or looping structures.
    pub(super) fn check_budget(&self) -> Result<()> {
        if self.exhausted {
            return Err(corrupt("too many sector reads; files or directories share sectors"));
        }
        Ok(())
    }
}

/// Where an entry's data lives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Data {
    /// Directories, and directory art without data.
    None,
    Dos2(dos2::Chain),
    Sparta {
        map: u16,
    },
}

/// A listed entry. Only the last path component is stored, so memory stays
/// linear in the number of entries however deep the tree is.
#[derive(Debug, Clone)]
pub(super) struct Node {
    parent: Option<usize>,
    component: String,
    path_len: usize,
    size: u64,
    is_directory: bool,
    locked: bool,
    modified_time: Option<DosDateTime>,
    start_sector: u16,
    data: Data,
}

/// Collects the entries of a filesystem.
pub(super) struct Tree {
    nodes: Vec<Node>,
    max_nodes: usize,
}

/// An entry to add to a [`Tree`].
pub(super) struct NewNode {
    pub parent: Option<usize>,
    pub component: String,
    pub size: u64,
    pub is_directory: bool,
    pub locked: bool,
    pub modified_time: Option<DosDateTime>,
    pub start_sector: u16,
    pub data: Data,
}

impl Tree {
    /// A tree for `image`. Directories may not share sectors, so an image holds at most
    /// one entry per 16 bytes (DOS 2) or 23 bytes (SpartaDOS); more entries than one
    /// per 16 bytes are refused as a safeguard.
    pub(super) fn new(image: &Image) -> Self {
        Self {
            nodes: Vec::new(),
            max_nodes: image.sectors() * image.sector_size() / 16,
        }
    }

    /// Adds an entry and returns its index.
    pub(super) fn push(&mut self, node: NewNode) -> Result<usize> {
        if self.nodes.len() >= self.max_nodes {
            return Err(corrupt("more entries than the image can hold"));
        }
        let parent_len = node.parent.map_or(0, |p| self.nodes[p].path_len);
        let path_len = parent_len + node.component.len() + usize::from(node.is_directory);
        if path_len > MAX_PATH_BYTES {
            return Err(corrupt(format!("path exceeds {MAX_PATH_BYTES} bytes")));
        }
        self.nodes.push(Node {
            parent: node.parent,
            component: node.component,
            path_len,
            size: node.size,
            is_directory: node.is_directory,
            locked: node.locked,
            modified_time: node.modified_time,
            start_sector: node.start_sector,
            data: node.data,
        });
        Ok(self.nodes.len() - 1)
    }
}

/// The filesystem found on an ATR image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtrFilesystem {
    /// Atari DOS 2.0S, 2.0D or 2.5, or a compatible DOS.
    AtariDos2,
    /// A DOS 2 compatible disk with MyDOS subdirectories or 16-bit sector links.
    MyDos,
    /// SpartaDOS (or a compatible DOS such as BW-DOS).
    SpartaDos,
}

impl fmt::Display for AtrFilesystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            AtrFilesystem::AtariDos2 => "Atari DOS 2",
            AtrFilesystem::MyDos => "MyDOS",
            AtrFilesystem::SpartaDos => "SpartaDOS",
        })
    }
}

/// A file or directory in an ATR image.
#[derive(Debug, Clone)]
pub struct AtrEntry {
    /// Path with `/` separators; directories end with `/`.
    pub name: String,
    /// Size in bytes. For a DOS 2 file whose sector chain is damaged, the readable part.
    pub size: u64,
    pub is_directory: bool,
    /// Locked (DOS 2) or write-protected (SpartaDOS).
    pub locked: bool,
    /// SpartaDOS time stamp; DOS 2 and MyDOS record none.
    pub modified_time: Option<DosDateTime>,
    /// First data sector (DOS 2), first directory sector (MyDOS directories), or first
    /// sector map (SpartaDOS). 0 for directory art without data.
    pub start_sector: u16,
    index: usize,
}

/// An ATR disk image reader.
///
/// The image (at most 65535 sectors, about 16 MiB) is read into memory when opened.
pub struct AtrArchive {
    image: Image,
    filesystem: AtrFilesystem,
    nodes: Vec<Node>,
    next: usize,
}

impl AtrArchive {
    /// Opens an ATR image starting at the reader's position.
    ///
    /// Images without a supported filesystem, such as boot-only game disks, fail with
    /// [`ArchiveError::UnsupportedFormat`].
    pub fn new<R: Read + Seek>(mut reader: R) -> Result<Self> {
        let mut header = [0; HEADER_SIZE];
        if crate::limits::read_up_to(&mut reader, &mut header)? < HEADER_SIZE || header[..2] != MAGIC {
            return Err(ArchiveError::invalid_header("ATR"));
        }
        let sector_size = usize::from(u16_at(&header, 4));
        if sector_size != 128 && sector_size != 256 {
            return Err(ArchiveError::unsupported_format(format!("ATR images with {sector_size}-byte sectors")));
        }
        // The paragraph count in the header is often wrong, so the file length decides the sector count.
        let limit = (BOOT_SECTORS * sector_size + MAX_SECTORS * sector_size) as u64;
        let start = reader.stream_position()?;
        let available = reader.seek(SeekFrom::End(0))?.saturating_sub(start);
        reader.seek(SeekFrom::Start(start))?;
        let data = crate::limits::read_exact_vec(&mut reader, available.min(limit))?;
        let ambiguous = Image::has_full_boot_slots(&data, sector_size);
        let image = Image::new(data, sector_size, true);
        let mut result = Self::list(&image);
        let image = if ambiguous && !matches!(result, Ok(Some(_))) {
            // The length suggests 256-byte boot slots, but the image may instead have
            // 128-byte boot sectors followed by trailing data. Errors refer to the first layout.
            let logical = Image::new(image.data, sector_size, false);
            if let Ok(Some(found)) = Self::list(&logical) {
                result = Ok(Some(found));
            }
            logical
        } else {
            image
        };
        let (filesystem, tree) = result?
            .ok_or_else(|| ArchiveError::unsupported_format("ATR image without an Atari DOS 2, MyDOS or SpartaDOS filesystem (boot-only or other DOS)"))?;
        Ok(Self {
            image,
            filesystem,
            nodes: tree.nodes,
            next: 0,
        })
    }

    fn list(image: &Image) -> Result<Option<(AtrFilesystem, Tree)>> {
        let mut visited = Visited::new(image.sectors(), LISTING_READS_PER_SECTOR * (image.sectors() + 1));
        if let Some(tree) = sparta::list(image, &mut visited)? {
            return Ok(Some((AtrFilesystem::SpartaDos, tree)));
        }
        dos2::list(image, &mut visited)
    }

    /// The filesystem on the image.
    pub fn filesystem(&self) -> AtrFilesystem {
        self.filesystem
    }

    /// Bytes per sector: 128 (single or enhanced density) or 256 (double density).
    pub fn sector_size(&self) -> usize {
        self.image.sector_size()
    }

    /// Number of sectors in the image.
    pub fn sectors(&self) -> usize {
        self.image.sectors()
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

    /// Returns the next entry, or `None` after the last one.
    pub fn get_next_entry(&mut self) -> Result<Option<AtrEntry>> {
        let Some(node) = self.nodes.get(self.next) else {
            return Ok(None);
        };
        let entry = AtrEntry {
            name: self.path(self.next),
            size: node.size,
            is_directory: node.is_directory,
            locked: node.locked,
            modified_time: node.modified_time,
            start_sector: node.start_sector,
            index: self.next,
        };
        self.next += 1;
        Ok(Some(entry))
    }

    /// Returns this archive's own record for `entry`; callers' copies may have been modified.
    fn stored(&self, entry: &AtrEntry) -> Result<&Node> {
        self.nodes
            .get(entry.index)
            .filter(|node| node.start_sector == entry.start_sector && node.is_directory == entry.is_directory && self.path(entry.index) == entry.name)
            .ok_or_else(|| ArchiveError::IndexMismatch("Entry does not belong to this ATR image".into()))
    }

    /// Reads an entry with the default per-file limit.
    pub fn read(&self, entry: &AtrEntry) -> Result<Vec<u8>> {
        self.read_with_limit(entry, Some(crate::limits::DEFAULT_MAX_ENTRY_SIZE))
    }

    /// Reads an entry, failing if it is larger than `limit` bytes. Directories read as empty.
    pub fn read_with_limit(&self, entry: &AtrEntry, limit: Option<u64>) -> Result<Vec<u8>> {
        let node = self.stored(entry)?;
        crate::limits::check_size(node.size, limit, &entry.name)?;
        // Sector maps and data sectors: each sector at most once.
        let mut visited = Visited::new(self.image.sectors(), 2 * (self.image.sectors() + 1));
        let result = match node.data {
            Data::None => return Ok(Vec::new()),
            Data::Dos2(chain) => chain.read(&self.image, &mut visited, node.size),
            Data::Sparta { map } => sparta::read(&self.image, &mut visited, map, node.size),
        };
        result.map_err(|reason| ArchiveError::corrupted_entry_named("ATR", &entry.name, reason))
    }

    /// Skips an entry. Entries are read from memory, so there is nothing to do.
    pub fn skip(&mut self, entry: &AtrEntry) -> Result<()> {
        self.stored(entry).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atascii_names() {
        assert_eq!(file_name(b"DOS     SYS"), "DOS.SYS");
        assert_eq!(file_name(b"README     "), "README");
        assert_eq!(file_name(b"        ..."), "____");
        assert_eq!(file_name(b".          "), "_");
        assert_eq!(file_name(b"           "), "_");
        assert_eq!(file_name(b"A/B\\C   D/E"), "A_B_C.D_E");
        // Graphics, inverse video, and the four arrows/symbols outside ASCII.
        assert_eq!(file_name(&[0x11, 0x12, 0x05, 0xC1, b' ', b' ', b' ', b' ', 0x60, 0x7B, 0x7F]), "┏━┓A.♦♠▶");
        assert_eq!(file_name(&[0x9B, b'x', 0x7C, b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ']), "␛x|");
    }

    fn image(sector_size: usize, len: usize) -> Image {
        let mut data = vec![0; len];
        for (i, b) in data.iter_mut().enumerate() {
            *b = (i / 128) as u8 | 0x80;
        }
        Image::new(data, sector_size, true)
    }

    #[test]
    fn entries_are_limited_by_the_image_size() {
        // 4 sectors of 128 bytes hold at most 32 entries of 16 bytes.
        let mut tree = Tree::new(&image(128, 4 * 128));
        let node = || NewNode {
            parent: None,
            component: "A".into(),
            size: 0,
            is_directory: false,
            locked: false,
            modified_time: None,
            start_sector: 0,
            data: Data::None,
        };
        for _ in 0..32 {
            tree.push(node()).unwrap();
        }
        assert!(matches!(tree.push(node()), Err(ArchiveError::CorruptedEntry { .. })));
    }

    #[test]
    fn sector_layouts() {
        let sd = image(128, 720 * 128);
        assert_eq!(sd.sectors(), 720);
        assert_eq!(sd.sector(360).unwrap()[0], (359 % 256) as u8 | 0x80);
        assert!(sd.sector(0).is_none() && sd.sector(721).is_none());

        // Double density with 128-byte boot sectors ("logical" layout).
        let dd = image(256, 720 * 256 - 384);
        assert_eq!(dd.sectors(), 720);
        assert_eq!(dd.sector(2).unwrap().len(), 128);
        assert_eq!(dd.sector(3).unwrap()[0], 0x82);
        assert_eq!(dd.sector(4).unwrap().len(), 256);
        assert_eq!(dd.sector(4).unwrap()[0], 0x83);

        // Boot sectors padded to 256 bytes.
        let padded = image(256, 720 * 256);
        assert_eq!(padded.sectors(), 720);
        assert_eq!(padded.sector(2).unwrap()[0], 0x82);
        assert_eq!(padded.sector(4).unwrap()[0], 0x86);

        // Boot sectors packed first, then 384 zero bytes.
        let mut data = image(256, 720 * 256).data;
        data[384..768].fill(0);
        let packed = Image::new(data, 256, true);
        assert_eq!(packed.sector(2).unwrap()[0], 0x81);
        assert_eq!(packed.sector(4).unwrap()[0], 0x86);

        // Partial trailing sectors are ignored.
        assert_eq!(image(128, 100 * 128 + 50).sectors(), 100);
        assert_eq!(image(256, 2 * 128).sectors(), 2);
    }
}
