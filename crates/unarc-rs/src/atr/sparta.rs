//! SpartaDOS: files and directories located through sector maps, 23-byte directory
//! entries with sizes and time stamps.

use std::collections::VecDeque;

use chrono::NaiveDate;

use crate::date_time::DosDateTime;
use crate::error::Result;

use super::archive::{corrupt, file_name, u16_at, Data, Image, NewNode, Tree, Visited};

const ENTRY_SIZE: usize = 23;
/// A sector map starts with links to the next and previous maps.
const MAP_HEADER: usize = 4;

const STATUS_LOCKED: u8 = 0x01;
const STATUS_IN_USE: u8 = 0x08;
const STATUS_DELETED: u8 = 0x10;
const STATUS_DIRECTORY: u8 = 0x20;

fn u24_at(data: &[u8], offset: usize) -> u64 {
    u64::from(data[offset]) | u64::from(data[offset + 1]) << 8 | u64::from(data[offset + 2]) << 16
}

/// Returns the first sector map of the root directory if the boot sector describes a
/// SpartaDOS filesystem with this image's sector size.
fn root_map(image: &Image) -> Option<u16> {
    let boot = image.sector(1)?;
    let sector_size = match boot[31] {
        0x80 => 128,
        0x00 => 256,
        _ => return None,
    };
    let root = u16_at(boot, 9);
    // Byte 7 is the low byte of the boot code address, $80 on SpartaDOS disks; byte 32 the filesystem version.
    (boot[7] == 0x80 && matches!(boot[32], 0x11 | 0x20 | 0x21) && sector_size == image.sector_size() && image.sector(usize::from(root)).is_some())
        .then_some(root)
}

/// Collects the first `count` data sector numbers (0 for unallocated sectors) from a map
/// chain, passing each map's sector number to `on_map`.
fn data_sectors(image: &Image, visited: &mut Visited, first_map: u16, count: u64, mut on_map: impl FnMut(u16)) -> std::result::Result<Vec<u16>, String> {
    visited.start();
    let mut sectors = Vec::with_capacity(crate::limits::capacity_hint(count.saturating_mul(2)) / 2);
    let mut map = first_map;
    while (sectors.len() as u64) < count {
        if map == 0 {
            return Err("sector map ends before the end of the file".into());
        }
        let data = image
            .sector(usize::from(map))
            .ok_or_else(|| format!("sector map {map} lies beyond the end of the image"))?;
        visited.visit(usize::from(map))?;
        on_map(map);
        let wanted = (count - sectors.len() as u64).min(((data.len() - MAP_HEADER) / 2) as u64) as usize;
        sectors.extend(data[MAP_HEADER..].as_chunks::<2>().0[..wanted].iter().map(|&pair| u16::from_le_bytes(pair)));
        map = u16_at(data, 0);
    }
    Ok(sectors)
}

/// Returns the number of sectors that hold `size` bytes, failing if the disk cannot hold them.
fn sector_count(image: &Image, size: u64) -> std::result::Result<u64, String> {
    let count = size.div_ceil(image.sector_size() as u64);
    if count > image.sectors() as u64 {
        return Err(format!("size {size} exceeds the disk"));
    }
    Ok(count)
}

/// Reads the first `size` bytes of `sectors`.
fn read_sectors(image: &Image, visited: &mut Visited, sectors: &[u16], size: u64) -> std::result::Result<Vec<u8>, String> {
    let sector_size = image.sector_size() as u64;
    visited.spend(sectors.len())?;
    let mut data = Vec::with_capacity(crate::limits::capacity_hint(size));
    for &sector in sectors {
        let take = (size - data.len() as u64).min(sector_size) as usize;
        if sector == 0 {
            // Unallocated sectors of sparse files read as zeros.
            data.resize(data.len() + take, 0);
            continue;
        }
        match image.sector(usize::from(sector)) {
            Some(bytes) if bytes.len() as u64 == sector_size => data.extend_from_slice(&bytes[..take]),
            Some(_) => return Err(format!("data in boot sector {sector}")),
            None => return Err(format!("sector {sector} lies beyond the end of the image")),
        }
    }
    Ok(data)
}

/// Reads `size` bytes of a file through its sector maps.
pub(super) fn read(image: &Image, visited: &mut Visited, first_map: u16, size: u64) -> std::result::Result<Vec<u8>, String> {
    let count = sector_count(image, size)?;
    let sectors = data_sectors(image, visited, first_map, count, |_| {})?;
    read_sectors(image, visited, &sectors, size)
}

/// Converts a day/month/year, hour/minute/second stamp; years 80-99 are 1980-1999, 0-79 are 2000-2079.
fn time_stamp(stamp: &[u8]) -> Option<DosDateTime> {
    let (day, month, year, hour, minute, second) = (stamp[0], stamp[1], stamp[2], stamp[3], stamp[4], stamp[5]);
    let year = if year >= 80 { 1900 + i32::from(year) } else { 2000 + i32::from(year) };
    if year > 2079 || hour > 23 || minute > 59 || second > 59 || NaiveDate::from_ymd_opt(year, u32::from(month), u32::from(day)).is_none() {
        return None;
    }
    let date = ((year - 1980) as u16) << 9 | u16::from(month) << 5 | u16::from(day);
    let time = u16::from(hour) << 11 | u16::from(minute) << 5 | u16::from(second / 2);
    Some(DosDateTime::from((date, time)))
}

/// Reads a directory: its first 23-byte entry holds the directory's own length.
///
/// The directory's maps and data sectors may not belong to another directory (or appear
/// twice), and directories are never sparse, so all directories together hold at most
/// the image's bytes and their entries cannot be multiplied by sharing sectors. `owned`
/// marks the sectors of directories read so far.
fn read_directory(image: &Image, visited: &mut Visited, owned: &mut [bool], map: u16) -> std::result::Result<Vec<u8>, String> {
    if owned.get(usize::from(map)).is_some_and(|&owned| owned) {
        return Err(format!("sector map {map} belongs to another directory"));
    }
    let header = read(image, visited, map, ENTRY_SIZE as u64)?;
    let size = u24_at(&header, 3);
    if size < ENTRY_SIZE as u64 {
        return Err(format!("directory length {size}"));
    }
    let count = sector_count(image, size)?;
    let mut maps = Vec::new();
    let sectors = data_sectors(image, visited, map, count, |map| maps.push(map))?;
    let mut claimed = Vec::with_capacity(maps.len() + sectors.len());
    for &sector in maps.iter().chain(&sectors) {
        let sector = usize::from(sector);
        let problem = match owned.get(sector) {
            _ if sector == 0 => Some("an unallocated sector".to_string()),
            Some(false) => None,
            Some(true) => Some(format!("sector {sector}, which belongs to another directory")),
            None => Some(format!("sector {sector}, which lies beyond the end of the image")),
        };
        if let Some(problem) = problem {
            // Release this directory's sectors so a later, valid directory can claim them.
            for &sector in &claimed {
                owned[sector] = false;
            }
            return Err(format!("directory has {problem}"));
        }
        owned[sector] = true;
        claimed.push(sector);
    }
    let result = read_sectors(image, visited, &sectors, size);
    if result.is_err() {
        for &sector in &claimed {
            owned[sector] = false;
        }
    }
    result
}

/// Lists a SpartaDOS filesystem, or returns `None` if the boot sector describes none.
///
/// A subdirectory that cannot be read is listed as an empty directory, so the rest of
/// the disk stays listable; an unreadable root directory fails.
pub(super) fn list(image: &Image, visited: &mut Visited) -> Result<Option<Tree>> {
    let Some(root) = root_map(image) else {
        return Ok(None);
    };
    let mut tree = Tree::new(image);
    let mut owned = vec![false; image.sectors() + 1];
    let root_directory = read_directory(image, visited, &mut owned, root).map_err(|reason| corrupt(format!("root directory: {reason}")))?;
    let mut pending = VecDeque::from([(None, root_directory)]);
    while let Some((parent, directory)) = pending.pop_front() {
        // The first entry describes the directory itself.
        for entry in directory.as_chunks::<ENTRY_SIZE>().0.iter().skip(1) {
            let status = entry[0];
            if status == 0 {
                break;
            }
            if status & STATUS_IN_USE == 0 || status & STATUS_DELETED != 0 {
                continue;
            }
            let map = u16_at(entry, 1);
            let is_directory = status & STATUS_DIRECTORY != 0;
            let index = tree.push(NewNode {
                parent,
                component: file_name(&entry[6..17]),
                size: if is_directory { 0 } else { u24_at(entry, 3) },
                is_directory,
                locked: status & STATUS_LOCKED != 0,
                modified_time: time_stamp(&entry[17..23]),
                start_sector: map,
                data: if is_directory { Data::None } else { Data::Sparta { map } },
            })?;
            if is_directory {
                if let Ok(directory) = read_directory(image, visited, &mut owned, map) {
                    pending.push_back((Some(index), directory));
                }
            }
        }
    }
    visited.check_budget()?;
    Ok(Some(tree))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_stamps() {
        let time = time_stamp(&[7, 10, 85, 15, 52, 44]).unwrap();
        assert_eq!(
            (time.year(), time.month(), time.day(), time.hour(), time.minute(), time.second()),
            (1985, 10, 7, 15, 52, 44)
        );
        assert_eq!(time_stamp(&[31, 12, 79, 23, 59, 58]).unwrap().year(), 2079);
        assert!(time_stamp(&[0, 0, 0, 0, 0, 0]).is_none());
        assert!(time_stamp(&[30, 2, 90, 0, 0, 0]).is_none());
        assert!(time_stamp(&[1, 1, 90, 24, 0, 0]).is_none());
    }
}
