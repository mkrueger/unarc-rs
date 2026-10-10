//! Contents of the self-authored files stored in the D64, T64 and Lynx fixtures.
//! See `tests/d64/README.md` for how the fixtures were made.
#![allow(dead_code)]

/// `10 PRINT"HELLO, C64"` as a PRG loading at $0801.
pub fn hello() -> Vec<u8> {
    let mut body = vec![0x0A, 0x00, 0x99];
    body.extend_from_slice(b"\"HELLO, C64\"\0");
    let link = 0x0801 + 2 + body.len() as u16;
    let mut prg = vec![0x01, 0x08];
    prg.extend_from_slice(&link.to_le_bytes());
    prg.extend(body);
    prg.extend_from_slice(&[0, 0]);
    prg
}

/// 30 text lines, 690 bytes (three blocks).
pub fn data() -> Vec<u8> {
    (0..30).flat_map(|i| format!("LINE {i:03} OF A SEQ FILE\r").into_bytes()).collect()
}

/// A PRG of exactly two blocks (508 bytes) loading at $C000.
pub fn exact() -> Vec<u8> {
    let mut prg = vec![0x00, 0xC0];
    prg.extend((0..506u32).map(|i| (i * 7 + 3) as u8));
    prg
}

/// A PRG of 1502 bytes loading at $2000.
pub fn long() -> Vec<u8> {
    let mut prg = vec![0x00, 0x20];
    prg.extend((0..1500u32).map(|i| ((i * 13) ^ (i >> 3)) as u8));
    prg
}

pub fn note() -> Vec<u8> {
    b"USR FILE CONTENT\r".to_vec()
}

/// 200 records of 32 bytes for a REL file.
pub fn records() -> Vec<u8> {
    (0..200).flat_map(|i| format!("{:.<32}", format!("RECORD {i:03}")).into_bytes()).collect()
}
