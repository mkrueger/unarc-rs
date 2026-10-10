//! Atari DOS 2.0S/2.0D/2.5 and MyDOS: a 64-entry directory in sectors 361-368 and
//! files stored as linked sector chains.

use std::collections::VecDeque;

use crate::error::{ArchiveError, Result};

use super::archive::{file_name, u16_at, AtrFilesystem, Data, Image, NewNode, Tree, Visited};

const VTOC_SECTOR: usize = 360;
const ROOT_DIRECTORY: usize = 361;
const DIRECTORY_SECTORS: usize = 8;
const ENTRIES_PER_SECTOR: usize = 8;
const ENTRY_SIZE: usize = 16;
/// Each data sector ends with the file number and link (two bytes) and the byte count.
const SECTOR_TRAILER: usize = 3;

const FLAG_OPEN_FOR_OUTPUT: u8 = 0x01;
const FLAG_DOS2: u8 = 0x02;
/// MyDOS: links hold a 16-bit sector number instead of a file number and 10-bit link.
const FLAG_LONG_LINKS: u8 = 0x04;
/// MyDOS subdirectory, possibly locked; no other bits are set.
const FLAG_DIRECTORY: u8 = 0x10;
const FLAG_LOCKED: u8 = 0x20;
const FLAG_IN_USE: u8 = 0x40;
const FLAG_DELETED: u8 = 0x80;
/// DOS 2.5: a closed file that uses sectors above 719, hidden from DOS 2.0 by the clear in-use bit.
const FLAG_DOS25_EXTENDED: u8 = FLAG_DOS2 | FLAG_OPEN_FOR_OUTPUT;

/// A file's sector chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Chain {
    start: u16,
    long_links: bool,
    /// The file number every sector must carry. `None` when the first sector does not
    /// carry the entry's directory index, as on disks written by some other DOSes.
    file_number: Option<u8>,
}

impl Chain {
    fn new(image: &Image, start: u16, long_links: bool, index: usize) -> Self {
        let file_number = (!long_links)
            .then(|| image.sector(usize::from(start)))
            .flatten()
            .map(|data| data[data.len() - SECTOR_TRAILER] >> 2)
            .filter(|&number| usize::from(number) == index);
        Self {
            start,
            long_links,
            file_number,
        }
    }

    /// Follows the chain, passing the used part of each sector to `visit`.
    fn walk(&self, image: &Image, visited: &mut Visited, mut visit: impl FnMut(&[u8])) -> std::result::Result<(), String> {
        visited.start();
        let mut sector = usize::from(self.start);
        while sector != 0 {
            let data = image
                .sector(sector)
                .ok_or_else(|| format!("sector {sector} lies beyond the end of the image"))?;
            visited.visit(sector)?;
            let (payload, trailer) = data.split_at(data.len() - SECTOR_TRAILER);
            let count = usize::from(trailer[2]);
            if count > payload.len() {
                return Err(format!("sector {sector} claims {count} data bytes"));
            }
            // Files whose length is a multiple of the sector data size often link to a final
            // sector that was never written. It holds no data, so its file number is not checked.
            let empty_last_sector = count == 0 && trailer[..2] == [0, 0];
            let next = if self.long_links {
                u16::from_be_bytes([trailer[0], trailer[1]])
            } else {
                if let Some(number) = self.file_number.filter(|&number| trailer[0] >> 2 != number && !empty_last_sector) {
                    // DOS reports error 164 (file number mismatch) here.
                    return Err(format!("sector {sector} belongs to file number {}, not {number}", trailer[0] >> 2));
                }
                u16::from(trailer[0] & 0x03) << 8 | u16::from(trailer[1])
            };
            visit(&payload[..count]);
            sector = usize::from(next);
        }
        Ok(())
    }

    /// Returns the number of readable bytes and whether the whole chain is intact.
    fn size(&self, image: &Image, visited: &mut Visited) -> (u64, bool) {
        let mut size = 0u64;
        let intact = self.walk(image, visited, |data| size += data.len() as u64).is_ok();
        (size, intact)
    }

    pub(super) fn read(&self, image: &Image, visited: &mut Visited, size: u64) -> std::result::Result<Vec<u8>, String> {
        let mut data = Vec::with_capacity(crate::limits::capacity_hint(size));
        self.walk(image, visited, |sector| data.extend_from_slice(sector))?;
        Ok(data)
    }
}

/// A directory entry that DOS lists.
struct Entry {
    index: usize,
    flag: u8,
    start: u16,
    name: [u8; 11],
}

impl Entry {
    fn is_directory(&self) -> bool {
        self.flag & !FLAG_LOCKED == FLAG_DIRECTORY
    }
}

/// Returns the listed entries of the directory in the 8 sectors from `first`, up to the
/// first entry that was never used.
///
/// Listed are files in use, including those never closed after writing (`$43`), DOS 2.5
/// files that use sectors above 719 (flag `$03`, hidden from DOS 2.0) and MyDOS
/// subdirectories (`$10`). Deleted files are hidden.
fn read_directory(image: &Image, first: usize) -> Vec<Entry> {
    let mut entries = Vec::new();
    for (index, raw) in (first..first + DIRECTORY_SECTORS)
        .filter_map(|sector| image.sector(sector))
        .flat_map(|data| data[..ENTRIES_PER_SECTOR * ENTRY_SIZE].as_chunks::<ENTRY_SIZE>().0)
        .enumerate()
    {
        let flag = raw[0];
        if flag == 0 {
            break;
        }
        let mut name = [0; 11];
        name.copy_from_slice(&raw[5..]);
        let entry = Entry {
            index,
            flag,
            start: u16_at(raw, 3),
            name,
        };
        // DOS hides unclosed files, but on real disks they are often complete (DOS.SYS
        // among them), and a damaged chain still fails when the file is read.
        let listed = flag & FLAG_DELETED == 0 && (entry.is_directory() || flag & FLAG_IN_USE != 0 || flag & !FLAG_LOCKED == FLAG_DOS25_EXTENDED);
        if listed {
            entries.push(entry);
        }
    }
    entries
}

/// A VTOC as DOS 2 or MyDOS writes it: a known DOS code and no more free than usable sectors.
fn vtoc_is_plausible(image: &Image) -> bool {
    image.sector(VTOC_SECTOR).is_some_and(|vtoc| {
        let (total, free) = (u16_at(vtoc, 1), u16_at(vtoc, 3));
        (2..=0x40).contains(&vtoc[0]) && total > 0 && free <= total
    })
}

/// Returns true if a root directory entry is a file with an intact, non-empty sector chain
/// whose sectors carry its file number (or MyDOS 16-bit links).
fn is_valid_file(image: &Image, visited: &mut Visited, entry: &Entry) -> bool {
    if entry.is_directory() || entry.start == 0 {
        return false;
    }
    let chain = Chain::new(image, entry.start, entry.flag & FLAG_LONG_LINKS != 0, entry.index);
    (chain.long_links || chain.file_number.is_some()) && matches!(chain.size(image, visited), (1.., true))
}

/// Lists a DOS 2 or MyDOS filesystem, or returns `None` if the image has none.
///
/// Garbage in sectors 361-368 must not pass for a directory, so at least one root
/// entry must be a valid file. A directory without files (empty, or only directory
/// art) also needs a plausible VTOC.
pub(super) fn list(image: &Image, visited: &mut Visited) -> Result<Option<(AtrFilesystem, Tree)>> {
    if image.sectors() < ROOT_DIRECTORY + DIRECTORY_SECTORS - 1 {
        return Ok(None);
    }
    let root = read_directory(image, ROOT_DIRECTORY);
    let has_data = root.iter().any(|entry| !entry.is_directory() && entry.start != 0);
    let valid = root.iter().any(|entry| is_valid_file(image, visited, entry));
    visited.check_budget()?;
    if !valid && (has_data || !vtoc_is_plausible(image)) {
        return Ok(None);
    }
    if image.sector(VTOC_SECTOR).is_some_and(|vtoc| vtoc[0] == 1) {
        // DOS 1.0 uses the byte-count byte of a sector differently.
        return Err(ArchiveError::unsupported_format("Atari DOS 1.0 disks"));
    }

    let mut tree = Tree::default();
    let mut mydos = false;
    // Each sector may belong to one directory only, so subdirectories cannot loop.
    let mut directory_sectors = vec![false; image.sectors() + 1];
    directory_sectors[ROOT_DIRECTORY..ROOT_DIRECTORY + DIRECTORY_SECTORS].fill(true);
    let mut pending = VecDeque::from([(None, root)]);
    while let Some((parent, entries)) = pending.pop_front() {
        for entry in entries {
            let start = usize::from(entry.start);
            let component = file_name(&entry.name);
            let locked = entry.flag & FLAG_LOCKED != 0;
            if entry.is_directory() {
                mydos = true;
                let index = tree.push(NewNode {
                    parent,
                    component,
                    size: 0,
                    is_directory: true,
                    locked,
                    modified_time: None,
                    start_sector: entry.start,
                    data: Data::None,
                })?;
                let sectors = start..start + DIRECTORY_SECTORS;
                if start != 0 && sectors.end <= image.sectors() + 1 && !directory_sectors[sectors.clone()].contains(&true) {
                    directory_sectors[sectors].fill(true);
                    pending.push_back((Some(index), read_directory(image, start)));
                }
                continue;
            }
            let long_links = entry.flag & FLAG_LONG_LINKS != 0;
            mydos |= long_links;
            let (size, data) = if start == 0 {
                // Directory art: title or separator lines without data.
                (0, Data::None)
            } else {
                let chain = Chain::new(image, entry.start, long_links, entry.index);
                let (size, _) = chain.size(image, visited);
                (size, Data::Dos2(chain))
            };
            tree.push(NewNode {
                parent,
                component,
                size,
                is_directory: false,
                locked,
                modified_time: None,
                start_sector: entry.start,
                data,
            })?;
        }
    }
    visited.check_budget()?;
    let filesystem = if mydos { AtrFilesystem::MyDos } else { AtrFilesystem::AtariDos2 };
    Ok(Some((filesystem, tree)))
}
