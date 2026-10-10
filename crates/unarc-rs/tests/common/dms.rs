#![allow(dead_code)]

pub const MODES: &[u8] = include_bytes!("../dms/modes.dms");
pub const HD: &[u8] = include_bytes!("../dms/hd.dms");
pub const NDOS: &[u8] = include_bytes!("../dms/ndos.dms");
pub const PARTIAL: &[u8] = include_bytes!("../dms/partial.dms");
pub const ENCRYPTED: &[u8] = include_bytes!("../dms/encrypted.dms");
pub const FIXTURES: [&[u8]; 5] = [MODES, HD, NDOS, PARTIAL, ENCRYPTED];

pub const LICENSE: &[u8] = include_bytes!("../../../../LICENSE");
pub const DISK_DIZ: &[u8] = include_bytes!("../dms/FILE_ID.DIZ");
pub const ATTACHED_DIZ: &[u8] = b"unarc-rs DMS fixture\nAttached FILE_ID.DIZ (track 80).\n";
pub const BANNER: &[u8] = b"\x1b[1munarc-rs\x1b[0m DMS test banner\n";
pub const BANNER2: &[u8] = b"A second banner, appended after the disk.\n";
pub const TRACK: usize = 11 * 1024;

/// The 3000-line text file stored as `docs/lines.txt`.
pub fn lines() -> Vec<u8> {
    (1..=3000)
        .flat_map(|i| format!("line {i} of the unarc-rs DMS test file\n").into_bytes())
        .collect()
}

pub fn crc(data: &[u8]) -> u16 {
    crc16::State::<crc16::ARC>::calculate(data)
}

pub fn word(data: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes([data[offset], data[offset + 1]])
}

pub fn put(data: &mut [u8], offset: usize, value: u16) {
    data[offset..offset + 2].copy_from_slice(&value.to_be_bytes());
}

/// Offsets of the 20-byte track headers, in file order.
pub fn tracks(data: &[u8]) -> Vec<usize> {
    let mut offsets = Vec::new();
    let mut pos = 56;
    while pos + 20 <= data.len() && &data[pos..pos + 2] == b"TR" {
        offsets.push(pos);
        pos += 20 + usize::from(word(data, pos + 6));
    }
    offsets
}

/// Offset of the header of track `number`.
pub fn track(data: &[u8], number: u16) -> usize {
    tracks(data).into_iter().find(|&pos| word(data, pos + 2) == number).unwrap()
}

pub fn fix_header_crc(data: &mut [u8]) {
    let value = crc(&data[4..54]);
    put(data, 54, value);
}

/// Recomputes the data and header CRCs of the track at `pos`.
pub fn fix_track_crc(data: &mut [u8], pos: usize) {
    let len = usize::from(word(data, pos + 6));
    let value = crc(&data[pos + 20..pos + 20 + len]);
    put(data, pos + 16, value);
    let value = crc(&data[pos..pos + 18]);
    put(data, pos + 18, value);
}
