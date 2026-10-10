//! Builds FAT12 floppy images for tests, independently of the reader.
#![allow(dead_code)]

pub const PAYLOAD: &[u8] = b"hello FAT12";

#[derive(Clone, Copy)]
pub struct Layout {
    pub bps: usize,
    pub spc: usize,
    pub reserved: usize,
    pub fats: usize,
    pub root_entries: usize,
    pub spf: usize,
    pub total: usize,
}

/// PC 720K (3.5" DD).
pub const PC720: Layout = Layout {
    bps: 512,
    spc: 2,
    reserved: 1,
    fats: 2,
    root_entries: 112,
    spf: 3,
    total: 1440,
};
/// Atari ST TOS double-sided 720K: same sizes as PC720 but five sectors per FAT.
pub const ATARI720: Layout = Layout {
    bps: 512,
    spc: 2,
    reserved: 1,
    fats: 2,
    root_entries: 112,
    spf: 5,
    total: 1440,
};

impl Layout {
    pub fn root_offset(&self) -> usize {
        (self.reserved + self.fats * self.spf) * self.bps
    }
    pub fn data_offset(&self) -> usize {
        self.root_offset() + (self.root_entries * 32).div_ceil(self.bps) * self.bps
    }
    pub fn cluster_size(&self) -> usize {
        self.bps * self.spc
    }
    pub fn cluster_offset(&self, cluster: usize) -> usize {
        self.data_offset() + (cluster - 2) * self.cluster_size()
    }
}

/// An empty filesystem. `atari` writes a 68000 branch instead of an x86 jump and no 0x55AA signature.
pub fn blank(layout: Layout, atari: bool) -> Vec<u8> {
    let mut image = vec![0; layout.total * layout.bps];
    let boot = &mut image[..512];
    if atari {
        boot[..2].copy_from_slice(&[0x60, 0x38]);
    } else {
        boot[..3].copy_from_slice(&[0xEB, 0x3C, 0x90]);
        boot[510..512].copy_from_slice(&[0x55, 0xAA]);
    }
    boot[11..13].copy_from_slice(&(layout.bps as u16).to_le_bytes());
    boot[13] = layout.spc as u8;
    boot[14..16].copy_from_slice(&(layout.reserved as u16).to_le_bytes());
    boot[16] = layout.fats as u8;
    boot[17..19].copy_from_slice(&(layout.root_entries as u16).to_le_bytes());
    boot[19..21].copy_from_slice(&(layout.total as u16).to_le_bytes());
    boot[21] = 0xF9;
    boot[22..24].copy_from_slice(&(layout.spf as u16).to_le_bytes());
    for fat in 0..layout.fats {
        let offset = (layout.reserved + fat * layout.spf) * layout.bps;
        image[offset..offset + 3].copy_from_slice(&[0xF9, 0xFF, 0xFF]);
    }
    image
}

/// Sets a 12-bit FAT entry in every FAT copy.
pub fn set_fat(image: &mut [u8], layout: Layout, cluster: usize, value: u16) {
    for fat in 0..layout.fats {
        let offset = (layout.reserved + fat * layout.spf) * layout.bps + cluster * 3 / 2;
        let old = u16::from_le_bytes([image[offset], image[offset + 1]]);
        let new = if cluster.is_multiple_of(2) {
            (old & 0xF000) | value
        } else {
            (old & 0x000F) | (value << 4)
        };
        image[offset..offset + 2].copy_from_slice(&new.to_le_bytes());
    }
}

/// Links `clusters` into one chain ending with an end-of-chain mark.
pub fn chain(image: &mut [u8], layout: Layout, clusters: &[usize]) {
    for pair in clusters.windows(2) {
        set_fat(image, layout, pair[0], pair[1] as u16);
    }
    set_fat(image, layout, *clusters.last().unwrap(), 0xFFF);
}

/// A 32-byte directory entry. `name` is the raw 11-byte 8.3 name.
pub fn entry(name: &[u8; 11], attributes: u8, cluster: u16, size: u32) -> [u8; 32] {
    let mut entry = [0; 32];
    entry[..11].copy_from_slice(name);
    entry[11] = attributes;
    // 1994-03-12 10:22:30
    entry[22..24].copy_from_slice(&((10u16 << 11) | (22 << 5) | 15).to_le_bytes());
    entry[24..26].copy_from_slice(&(((1994u16 - 1980) << 9) | (3 << 5) | 12).to_le_bytes());
    entry[26..28].copy_from_slice(&cluster.to_le_bytes());
    entry[28..32].copy_from_slice(&size.to_le_bytes());
    entry
}

pub fn short_name_checksum(name: &[u8; 11]) -> u8 {
    name.iter().fold(0u8, |sum, &b| sum.rotate_right(1).wrapping_add(b))
}

/// VFAT entries for `long`, last part first, as stored on disk.
pub fn long_name_entries(long: &str, short: &[u8; 11]) -> Vec<[u8; 32]> {
    let mut units: Vec<u16> = long.encode_utf16().collect();
    if !units.len().is_multiple_of(13) {
        units.push(0);
        while !units.len().is_multiple_of(13) {
            units.push(0xFFFF);
        }
    }
    let parts = units.len() / 13;
    let checksum = short_name_checksum(short);
    (1..=parts)
        .rev()
        .map(|order| {
            let mut entry = [0; 32];
            entry[0] = order as u8 | if order == parts { 0x40 } else { 0 };
            entry[11] = 0x0F;
            entry[13] = checksum;
            for (unit, offset) in units[(order - 1) * 13..order * 13].iter().zip([1, 3, 5, 7, 9, 14, 16, 18, 20, 22, 24, 28, 30]) {
                entry[offset..offset + 2].copy_from_slice(&unit.to_le_bytes());
            }
            entry
        })
        .collect()
}

pub fn put_entries(image: &mut [u8], offset: usize, entries: &[[u8; 32]]) {
    for (index, entry) in entries.iter().enumerate() {
        image[offset + index * 32..offset + (index + 1) * 32].copy_from_slice(entry);
    }
}

/// Root: `DOCS/` (cluster 2) and `FILE.BIN` (clusters 3, 4, 5); `DOCS/README.TXT` (cluster 6) holds `PAYLOAD`.
pub fn simple(layout: Layout, atari: bool) -> Vec<u8> {
    let mut image = blank(layout, atari);
    let file_size = 2 * layout.cluster_size() + 7;
    put_entries(
        &mut image,
        layout.root_offset(),
        &[entry(b"DOCS       ", 0x10, 2, 0), entry(b"FILE    BIN", 0x20, 3, file_size as u32)],
    );
    chain(&mut image, layout, &[2]);
    chain(&mut image, layout, &[3, 4, 5]);
    chain(&mut image, layout, &[6]);
    let docs = layout.cluster_offset(2);
    put_entries(
        &mut image,
        docs,
        &[
            entry(b".          ", 0x10, 2, 0),
            entry(b"..         ", 0x10, 0, 0),
            entry(b"README  TXT", 0x20, 6, PAYLOAD.len() as u32),
        ],
    );
    let readme = layout.cluster_offset(6);
    image[readme..readme + PAYLOAD.len()].copy_from_slice(PAYLOAD);
    let file = layout.cluster_offset(3);
    for (index, byte) in image[file..file + file_size].iter_mut().enumerate() {
        *byte = (index % 251) as u8;
    }
    image
}

pub fn file_bin(layout: Layout) -> Vec<u8> {
    (0..2 * layout.cluster_size() + 7).map(|index| (index % 251) as u8).collect()
}
