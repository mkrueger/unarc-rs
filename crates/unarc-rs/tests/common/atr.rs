//! Builds ATR images for tests, independently of the reader.
#![allow(dead_code)]

/// Sectors of a disk; sectors 1-3 always hold 128 bytes.
pub struct Disk {
    pub sector_size: usize,
    sectors: Vec<Vec<u8>>,
}

impl Disk {
    pub fn new(sector_size: usize, count: usize) -> Self {
        let sectors = (1..=count).map(|n| vec![0; if n <= 3 { 128 } else { sector_size }]).collect();
        Self { sector_size, sectors }
    }

    /// Sector `n`, counted from 1.
    pub fn sector(&mut self, n: usize) -> &mut [u8] {
        &mut self.sectors[n - 1]
    }

    /// The ATR file with 128-byte boot sectors.
    pub fn atr(&self) -> Vec<u8> {
        let body: Vec<u8> = self.sectors.concat();
        let mut image = header(body.len(), self.sector_size);
        image.extend(body);
        image
    }

    /// A double-density ATR file whose boot sectors fill 256-byte slots: each padded,
    /// or (`packed`) all three first, followed by 384 zero bytes.
    pub fn atr_padded(&self, packed: bool) -> Vec<u8> {
        let mut body = Vec::new();
        if packed {
            body.extend(self.sectors[..3].concat());
            body.resize(768, 0);
        } else {
            for boot in &self.sectors[..3] {
                body.extend(boot);
                body.resize(body.len() + 128, 0);
            }
        }
        body.extend(self.sectors[3..].concat());
        let mut image = header(body.len(), self.sector_size);
        image.extend(body);
        image
    }
}

pub fn header(len: usize, sector_size: usize) -> Vec<u8> {
    let paragraphs = len / 16;
    let mut header = vec![
        0x96,
        0x02,
        paragraphs as u8,
        (paragraphs >> 8) as u8,
        sector_size as u8,
        (sector_size >> 8) as u8,
        (paragraphs >> 16) as u8,
    ];
    header.resize(16, 0);
    header
}

/// An 8.3 name padded with spaces.
pub fn name(name: &str) -> [u8; 11] {
    let (base, extension) = name.split_once('.').unwrap_or((name, ""));
    let mut raw = [b' '; 11];
    raw[..base.len()].copy_from_slice(base.as_bytes());
    raw[8..8 + extension.len()].copy_from_slice(extension.as_bytes());
    raw
}

// Atari DOS 2 / MyDOS

pub const DOS2_FILE: u8 = 0x42;
pub const MYDOS_LONG_LINKS: u8 = 0x46;
pub const MYDOS_DIRECTORY: u8 = 0x10;

/// A VTOC with DOS code 2, 707 usable and `free` free sectors.
pub fn vtoc(disk: &mut Disk, free: u16) {
    let vtoc = disk.sector(360);
    vtoc[..5].copy_from_slice(&[2, 0xC3, 0x02, free as u8, (free >> 8) as u8]);
}

/// Writes directory entry `index` of the directory starting at `directory`.
pub fn entry(disk: &mut Disk, directory: usize, index: usize, flag: u8, count: u16, start: u16, name: &[u8; 11]) {
    let sector = disk.sector(directory + index / 8);
    let raw = &mut sector[(index % 8) * 16..(index % 8 + 1) * 16];
    raw[0] = flag;
    raw[1..3].copy_from_slice(&count.to_le_bytes());
    raw[3..5].copy_from_slice(&start.to_le_bytes());
    raw[5..].copy_from_slice(name);
}

/// Writes `data` as a sector chain over `sectors`, which must be long enough;
/// an empty file takes one sector. Returns the number of sectors used.
pub fn chain(disk: &mut Disk, sectors: &[usize], file_number: u8, data: &[u8], long_links: bool) -> usize {
    let payload = disk.sector_size - 3;
    let chunks: Vec<&[u8]> = if data.is_empty() { vec![&[]] } else { data.chunks(payload).collect() };
    for (i, chunk) in chunks.iter().enumerate() {
        let next = sectors.get(i + 1).filter(|_| i + 1 < chunks.len()).copied().unwrap_or(0);
        let sector = disk.sector(sectors[i]);
        let n = sector.len();
        sector[..chunk.len()].copy_from_slice(chunk);
        sector[n - 3] = if long_links {
            (next >> 8) as u8
        } else {
            file_number << 2 | (next >> 8) as u8
        };
        sector[n - 2] = next as u8;
        sector[n - 1] = chunk.len() as u8;
    }
    chunks.len()
}

/// Adds a DOS 2 file with consecutive sectors from `start`.
pub fn file(disk: &mut Disk, directory: usize, index: usize, start: usize, name: &str, data: &[u8]) -> usize {
    let sectors: Vec<usize> = (start..start + 1 + data.len() / (disk.sector_size - 3)).collect();
    let count = chain(disk, &sectors, index as u8, data, false);
    entry(disk, directory, index, DOS2_FILE, count as u16, start as u16, &self::name(name));
    count
}

/// Deterministic test data.
pub fn data(len: usize, seed: u8) -> Vec<u8> {
    (0..len).map(|i| (i * 7 + i / 251) as u8 ^ seed).collect()
}

// SpartaDOS

pub const SPARTA_FILE: u8 = 0x08;
pub const SPARTA_DIRECTORY: u8 = 0x28;

/// Builds a SpartaDOS disk, allocating sectors upwards from sector 4.
pub struct Sparta {
    pub disk: Disk,
    next: usize,
}

impl Sparta {
    pub fn new(sector_size: usize, count: usize) -> Self {
        let mut disk = Disk::new(sector_size, count);
        let boot = disk.sector(1);
        boot[1] = 3;
        boot[7] = 0x80;
        boot[11..13].copy_from_slice(&(count as u16).to_le_bytes());
        boot[22..30].copy_from_slice(b"UNARCRS ");
        boot[31] = if sector_size == 128 { 0x80 } else { 0 };
        boot[32] = 0x20;
        Self { disk, next: 4 }
    }

    pub fn allocate(&mut self) -> usize {
        self.next += 1;
        self.next - 1
    }

    /// Writes data sectors and sector maps; `None` leaves an unallocated (sparse) sector.
    /// Returns the first sector map.
    pub fn write_sectors(&mut self, chunks: &[Option<&[u8]>]) -> u16 {
        let ss = self.disk.sector_size;
        let numbers: Vec<u16> = chunks
            .iter()
            .map(|chunk| match chunk {
                Some(chunk) => {
                    let sector = self.allocate();
                    self.disk.sector(sector)[..chunk.len()].copy_from_slice(chunk);
                    sector as u16
                }
                None => 0,
            })
            .collect();
        let per_map = (ss - 4) / 2;
        let maps: Vec<usize> = (0..numbers.len().div_ceil(per_map).max(1)).map(|_| self.allocate()).collect();
        for (i, &map) in maps.iter().enumerate() {
            let next = maps.get(i + 1).copied().unwrap_or(0) as u16;
            let previous = if i == 0 { 0 } else { maps[i - 1] as u16 };
            let sector = self.disk.sector(map);
            sector[..2].copy_from_slice(&next.to_le_bytes());
            sector[2..4].copy_from_slice(&previous.to_le_bytes());
            for (slot, number) in numbers.iter().skip(i * per_map).take(per_map).enumerate() {
                sector[4 + 2 * slot..6 + 2 * slot].copy_from_slice(&number.to_le_bytes());
            }
        }
        maps[0] as u16
    }

    pub fn write(&mut self, data: &[u8]) -> u16 {
        let ss = self.disk.sector_size;
        let chunks: Vec<Option<&[u8]>> = data.chunks(ss).map(Some).collect();
        self.write_sectors(&chunks)
    }

    /// Writes a directory from (status, map, size, name, stamp) entries and returns its first map.
    pub fn directory(&mut self, name: &str, entries: &[(u8, u16, u32, &str, [u8; 6])]) -> u16 {
        let mut data = Vec::new();
        let size = (entries.len() + 1) * 23;
        data.push(SPARTA_DIRECTORY);
        data.extend([0, 0]);
        data.extend(&(size as u32).to_le_bytes()[..3]);
        data.extend(&self::name(name)[..8]);
        data.resize(23, 0);
        for &(status, map, len, entry_name, stamp) in entries {
            data.push(status);
            data.extend(map.to_le_bytes());
            data.extend(&len.to_le_bytes()[..3]);
            data.extend(self::name(entry_name));
            data.extend(stamp);
        }
        self.write(&data)
    }

    pub fn set_root(&mut self, map: u16) {
        self.disk.sector(1)[9..11].copy_from_slice(&map.to_le_bytes());
    }
}
