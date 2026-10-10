//! Reader for D64 images of Commodore 1541 floppy disks.
//!
//! A D64 file is a plain dump of the disk's 256-byte sectors, optionally followed
//! by one error byte per sector. It has no signature: images are recognised by
//! their size and by a plausible block availability map (BAM) on track 18.
//!
//! See `doc/d64.md` for the layout.

use std::io::{Read, Seek, SeekFrom};

use crate::cbm::{entry_name, petscii_to_string, CbmFileType};
use crate::error::{ArchiveError, Result};

const SECTOR_SIZE: usize = 256;
const DIRECTORY_TRACK: u8 = 18;
/// The first directory sector is always 18/1; the link in the BAM sector is not used.
const FIRST_DIRECTORY_SECTOR: u8 = 1;
const ENTRY_SIZE: usize = 32;
/// Track counts of the supported layouts: standard, 40 and 42 track images.
const TRACK_COUNTS: [u8; 3] = [35, 40, 42];

/// Number of sectors on a 1541 track (zone bit recording).
fn sectors_per_track(track: u8) -> u8 {
    match track {
        1..=17 => 21,
        18..=24 => 19,
        25..=30 => 18,
        _ => 17,
    }
}

fn total_sectors(tracks: u8) -> usize {
    (1..=tracks).map(|t| usize::from(sectors_per_track(t))).sum()
}

/// Returns the track count for a D64 image of `len` bytes, with or without error bytes.
fn tracks_for_size(len: u64) -> Option<u8> {
    TRACK_COUNTS.into_iter().find(|&tracks| {
        let sectors = total_sectors(tracks) as u64;
        len == sectors * SECTOR_SIZE as u64 || len == sectors * (SECTOR_SIZE as u64 + 1)
    })
}

/// Index of a sector in the image, or `None` if the track/sector pair is not on the disk.
fn sector_index(tracks: u8, track: u8, sector: u8) -> Option<usize> {
    if track == 0 || track > tracks || sector >= sectors_per_track(track) {
        return None;
    }
    Some(total_sectors(track - 1) + usize::from(sector))
}

/// Checks the free-block counts of tracks 1-35 against their bitmaps and the DOS type bytes.
fn bam_is_plausible(bam: &[u8; SECTOR_SIZE]) -> bool {
    let dos_type = bam[2] == b'A' || &bam[0xA5..0xA7] == b"2A";
    dos_type
        && (1..=35u8).all(|track| {
            let entry = &bam[4 * usize::from(track)..4 * usize::from(track) + 4];
            let mask = (1u32 << sectors_per_track(track)) - 1;
            let bits = u32::from_le_bytes([entry[1], entry[2], entry[3], 0]) & mask;
            u32::from(entry[0]) == bits.count_ones()
        })
}

/// A file in the directory of a D64 image.
#[derive(Debug, Clone)]
pub struct D64Entry {
    /// Entry name: the converted PETSCII name plus an extension for the file type, e.g. `GAME.prg`.
    pub name: String,
    /// The raw PETSCII name from the directory, padded with `0xA0`.
    pub petscii_name: [u8; 16],
    /// CBM file type.
    pub file_type: CbmFileType,
    /// The raw directory type byte (bit 7 = closed, bit 6 = locked).
    pub type_byte: u8,
    /// Track of the first data sector.
    pub track: u8,
    /// Sector of the first data sector.
    pub sector: u8,
    /// Size in blocks as recorded in the directory (not trusted for extraction).
    pub blocks: u16,
    /// Record length of a REL file, otherwise 0.
    pub record_length: u8,
    /// File size in bytes, determined by following the sector chain.
    pub size: u64,
}

impl D64Entry {
    /// Returns true if the file was closed properly (unclosed files are listed as `*PRG` etc.).
    pub fn is_closed(&self) -> bool {
        self.type_byte & 0x80 != 0
    }

    /// Returns true if the file is locked against scratching.
    pub fn is_locked(&self) -> bool {
        self.type_byte & 0x40 != 0
    }
}

/// D64 disk image reader.
///
/// The image is at most about 200 KiB, so it is read into memory when opened.
pub struct D64Archive {
    image: Vec<u8>,
    tracks: u8,
    entries: Vec<D64Entry>,
    next: usize,
}

impl D64Archive {
    /// Opens a D64 image. The image must have one of the standard sizes
    /// (35, 40 or 42 tracks, each with or without error bytes).
    pub fn new<R: Read + Seek>(mut reader: R) -> Result<Self> {
        let start = reader.stream_position()?;
        let len = reader.seek(SeekFrom::End(0))?.saturating_sub(start);
        let tracks = tracks_for_size(len).ok_or_else(|| ArchiveError::invalid_header("D64"))?;
        reader.seek(SeekFrom::Start(start))?;
        let image = crate::limits::read_exact_vec(&mut reader, (total_sectors(tracks) * SECTOR_SIZE) as u64)?;

        let mut archive = Self {
            image,
            tracks,
            entries: Vec::new(),
            next: 0,
        };
        archive.entries = archive.read_directory();
        Ok(archive)
    }

    /// Returns true if the reader holds an image of a D64 size whose BAM is plausible.
    ///
    /// The reader position is restored afterwards.
    pub fn probe<R: Read + Seek>(reader: &mut R) -> std::io::Result<bool> {
        let start = reader.stream_position()?;
        let result = Self::probe_at(reader, start);
        reader.seek(SeekFrom::Start(start))?;
        result
    }

    fn probe_at<R: Read + Seek>(reader: &mut R, start: u64) -> std::io::Result<bool> {
        let len = reader.seek(SeekFrom::End(0))?.saturating_sub(start);
        let Some(tracks) = tracks_for_size(len) else {
            return Ok(false);
        };
        let Some(bam_index) = sector_index(tracks, DIRECTORY_TRACK, 0) else {
            return Ok(false);
        };
        reader.seek(SeekFrom::Start(start + (bam_index * SECTOR_SIZE) as u64))?;
        // 18/0 (BAM) is directly followed by 18/1, the first directory sector.
        let mut bam = [0; SECTOR_SIZE];
        let mut directory = [0; SECTOR_SIZE];
        reader.read_exact(&mut bam)?;
        reader.read_exact(&mut directory)?;
        let link_ok = directory[0] == 0 || sector_index(tracks, directory[0], directory[1]).is_some();
        Ok(link_ok && bam_is_plausible(&bam))
    }

    /// Returns true if the reader's remaining length is one of the D64 image sizes.
    ///
    /// The reader position is restored afterwards.
    pub fn has_image_size<R: Read + Seek>(reader: &mut R) -> std::io::Result<bool> {
        let start = reader.stream_position()?;
        let len = reader.seek(SeekFrom::End(0))?.saturating_sub(start);
        reader.seek(SeekFrom::Start(start))?;
        Ok(tracks_for_size(len).is_some())
    }

    /// Number of tracks in the image (35, 40 or 42).
    pub fn tracks(&self) -> u8 {
        self.tracks
    }

    /// The disk name from the BAM sector, converted from PETSCII.
    pub fn disk_name(&self) -> String {
        self.sector(DIRECTORY_TRACK, 0)
            .map(|bam| petscii_to_string(&bam[0x90..0xA0]))
            .unwrap_or_default()
    }

    fn sector(&self, track: u8, sector: u8) -> Option<&[u8]> {
        let index = sector_index(self.tracks, track, sector)?;
        self.image.get(index * SECTOR_SIZE..(index + 1) * SECTOR_SIZE)
    }

    /// Follows a sector chain, passing the used part of every sector to `visit`.
    ///
    /// Fails on links outside the disk and on chains that revisit a sector, so
    /// the walk ends after at most one visit per sector.
    fn walk_chain(&self, mut track: u8, mut sector: u8, mut visit: impl FnMut(&[u8])) -> std::result::Result<(), String> {
        let mut visited = vec![false; total_sectors(self.tracks)];
        while track != 0 {
            let index = sector_index(self.tracks, track, sector).ok_or_else(|| format!("invalid track/sector link {track}/{sector}"))?;
            if std::mem::replace(&mut visited[index], true) {
                return Err(format!("sector chain loops back to {track}/{sector}"));
            }
            let data = &self.image[index * SECTOR_SIZE..(index + 1) * SECTOR_SIZE];
            if data[0] == 0 {
                // Last sector: the second byte is the index of the last used byte.
                let end = usize::from(data[1]) + 1;
                if end > 2 {
                    visit(&data[2..end]);
                }
                return Ok(());
            }
            visit(&data[2..]);
            (track, sector) = (data[0], data[1]);
        }
        Ok(())
    }

    /// Reads the directory chain starting at 18/1.
    ///
    /// The directory is listed up to the first invalid or repeated link, so a
    /// damaged chain still yields the entries before the damage.
    fn read_directory(&self) -> Vec<D64Entry> {
        let mut entries = Vec::new();
        let mut visited = vec![false; total_sectors(self.tracks)];
        let (mut track, mut sector) = (DIRECTORY_TRACK, FIRST_DIRECTORY_SECTOR);
        while let Some(index) = sector_index(self.tracks, track, sector) {
            if std::mem::replace(&mut visited[index], true) {
                break;
            }
            let data = &self.image[index * SECTOR_SIZE..(index + 1) * SECTOR_SIZE];
            for raw in data.chunks_exact(ENTRY_SIZE) {
                if let Some(entry) = self.parse_entry(raw) {
                    entries.push(entry);
                }
            }
            (track, sector) = (data[0], data[1]);
        }
        entries
    }

    fn parse_entry(&self, raw: &[u8]) -> Option<D64Entry> {
        let type_byte = raw[2];
        if type_byte == 0 {
            // Scratched or unused slot.
            return None;
        }
        let file_type = CbmFileType::from_dos_type(type_byte);
        let mut petscii_name = [0; 16];
        petscii_name.copy_from_slice(&raw[5..21]);
        let (track, sector) = (raw[3], raw[4]);
        let mut size = 0u64;
        // A broken chain is reported when the entry is read; the size covers the readable part.
        let _ = self.walk_chain(track, sector, |data| size += data.len() as u64);
        Some(D64Entry {
            name: entry_name(&petscii_name, file_type),
            petscii_name,
            file_type,
            type_byte,
            track,
            sector,
            blocks: u16::from_le_bytes([raw[30], raw[31]]),
            record_length: raw[23],
            size,
        })
    }

    /// Returns the next directory entry, or `None` after the last one.
    pub fn get_next_entry(&mut self) -> Result<Option<D64Entry>> {
        let entry = self.entries.get(self.next).cloned();
        if entry.is_some() {
            self.next += 1;
        }
        Ok(entry)
    }

    /// Reads a file by following its sector chain.
    ///
    /// PRG files include their two-byte load address, as stored on disk.
    pub fn read(&self, entry: &D64Entry) -> Result<Vec<u8>> {
        let mut data = Vec::with_capacity(crate::limits::capacity_hint(entry.size));
        self.walk_chain(entry.track, entry.sector, |sector| data.extend_from_slice(sector))
            .map_err(|reason| ArchiveError::corrupted_entry_named("D64", &entry.name, reason))?;
        Ok(data)
    }

    /// Skips an entry. Entries are read from memory, so there is nothing to do.
    pub fn skip(&mut self, _entry: &D64Entry) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry() {
        assert_eq!(total_sectors(35), 683);
        assert_eq!(total_sectors(40), 768);
        assert_eq!(total_sectors(42), 802);
        assert_eq!(tracks_for_size(174_848), Some(35));
        assert_eq!(tracks_for_size(175_531), Some(35));
        assert_eq!(tracks_for_size(196_608), Some(40));
        assert_eq!(tracks_for_size(197_376), Some(40));
        assert_eq!(tracks_for_size(205_312), Some(42));
        assert_eq!(tracks_for_size(206_114), Some(42));
        assert_eq!(tracks_for_size(174_847), None);
        assert_eq!(sector_index(35, 18, 0), Some(357));
        assert_eq!(sector_index(35, 18, 19), None);
        assert_eq!(sector_index(35, 36, 0), None);
        assert_eq!(sector_index(40, 40, 16), Some(767));
        assert_eq!(sector_index(35, 0, 0), None);
    }
}
