//! JAR's word transform, including its on-wire English dictionary codes.
use crate::error::{ArchiveError, Result};

// The format fixes these codes; they are not an adaptive dictionary.
// Resource 1 of the reference JAR 1.02 executable, decoded by 0x421a50.
const DICTIONARY: &[u8; 65536] = include_bytes!("dictionary.bin");

pub(super) fn bad(reason: &str) -> ArchiveError {
    ArchiveError::decompression_failed("JAR", reason)
}

fn word(code: u16) -> Result<Vec<u8>> {
    let d = DICTIONARY;
    let index = (code as usize >> 4) & 0xfffe;
    let mut p = u16::from_le_bytes([d[index], d[index + 1]]) as usize;
    let mut header = p;
    let mut base = u16::from_le_bytes([
        *d.get(p + 2).ok_or_else(|| bad("Invalid dictionary code"))?,
        *d.get(p + 3).ok_or_else(|| bad("Invalid dictionary code"))?,
    ]);
    p += 7;
    loop {
        let n = *d.get(p).ok_or_else(|| bad("Invalid dictionary code"))?;
        if n == 0 {
            return Err(bad("Unknown dictionary code"));
        }
        if code == base || (n & 128 == 0 && code >= base && code - base <= 7) {
            let len = (n & 127) as usize;
            if len < 3 {
                return Err(bad("Invalid dictionary length"));
            }
            let mut result = d.get(header + 4..header + 7).ok_or_else(|| bad("Invalid dictionary prefix"))?.to_vec();
            result.extend_from_slice(d.get(p + 1..p + len - 2).ok_or_else(|| bad("Invalid dictionary suffix"))?);
            let suffixes: [&[u8]; 8] = [b"", b"ing", b"ers", b"ed", b"er", b"es", b"e", b"s"];
            result.extend_from_slice(suffixes.get((code - base) as usize).ok_or_else(|| bad("Invalid word suffix"))?);
            return Ok(result);
        }
        base = base
            .checked_add(if n & 128 != 0 { 1 } else { 8 })
            .ok_or_else(|| bad("Invalid dictionary code"))?;
        if n & 127 < 3 {
            return Err(bad("Invalid dictionary link"));
        }
        p += (n & 127) as usize - 2;
        if *d.get(p).ok_or_else(|| bad("Invalid dictionary link"))? >= 254 {
            header = p + 1;
            p += 8;
        }
    }
}

#[derive(Debug)]
pub(super) struct Record {
    pub tag: u16,
    pub data: Vec<u8>,
}

fn append_symbol(symbols: &[u16], i: &mut usize, out: &mut Vec<u8>) -> Result<()> {
    let s = symbols[*i];
    *i += 1;
    match s {
        0..=255 => out.push(s as u8),
        0x104 => out.extend_from_slice(b"\r\n"),
        0x800..=0x9ff => out.resize(out.len() + if s < 0x900 { 6 } else { 32 }, s as u8),
        0x2000..=0xffff => {
            let mut w = word(s)?;
            match symbols.get(*i).copied() {
                Some(0x100) => *i += 1,
                Some(0x101) => {
                    *i += 1;
                    w.make_ascii_uppercase();
                }
                Some(0x102) => {
                    *i += 1;
                    if let Some(first) = w.first_mut() {
                        first.make_ascii_uppercase();
                    }
                }
                _ => w.push(b' '),
            }
            out.extend_from_slice(&w);
        }
        _ => return Err(bad("Unexpected word-transform symbol")),
    }
    Ok(())
}

/// Preserve control records instead of silently emitting them as file bytes.
pub(super) fn decode(symbols: &[u16], limit: usize) -> Result<Vec<Record>> {
    let mut records = Vec::new();
    let mut current = Record { tag: 0, data: Vec::new() };
    let mut i = 0;
    let mut total = 0usize;
    let mut ended = false;
    while i < symbols.len() {
        let s = symbols[i];
        if s == 0xa00 {
            records.push(current);
            ended = true;
            if symbols[i + 1..].iter().any(|&s| s != 0) {
                return Err(bad("Nonzero stream padding"));
            }
            break;
        }
        let before = current.data.len();
        if s == 0xb00 || s == 0xb01 {
            i += 1;
            let end = symbols[i..]
                .iter()
                .position(|&v| v == s)
                .map(|n| i + n)
                .ok_or_else(|| bad("Unterminated transform"))?;
            if s == 0xb00 {
                let mut bytes = Vec::new();
                while i < end {
                    append_symbol(&symbols[..end], &mut i, &mut bytes)?;
                }
                for b in bytes {
                    current.data.extend_from_slice(&[b, 0]);
                }
            } else {
                let values = &symbols[i..end];
                if values.len() < 4 || values.len() % 2 != 0 || values.iter().any(|&v| v > 255) {
                    return Err(bad("Invalid delta transform"));
                }
                let mut left = values[0] | values[1] << 8;
                let mut right = values[2] | values[3] << 8;
                current.data.extend_from_slice(&left.to_le_bytes());
                current.data.extend_from_slice(&right.to_le_bytes());
                let count = (values.len() - 4) / 2;
                for n in 0..count {
                    left = left.wrapping_add(values[4 + n] as u8 as i8 as u16);
                    right = right.wrapping_add(values[4 + count + n] as u8 as i8 as u16);
                    current.data.extend_from_slice(&left.to_le_bytes());
                    current.data.extend_from_slice(&right.to_le_bytes());
                }
            }
            i = end + 1;
        } else if (0xe00..=0xeff).contains(&s) {
            records.push(current);
            current = Record { tag: s, data: Vec::new() };
            i += 1;
            continue;
        } else {
            append_symbol(symbols, &mut i, &mut current.data)?;
        }
        total += current.data.len() - before;
        if total > limit {
            return Err(bad("Decompressed stream exceeds size limit"));
        }
    }
    if !ended {
        return Err(bad("Missing stream terminator"));
    }
    Ok(records)
}
