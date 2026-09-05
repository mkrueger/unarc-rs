//! JAR 1.02 solid streams: packet framing, Huffman/LZ, word transforms and records.
use super::{
    engine,
    words::{self, bad, Record},
};
use crate::error::{ArchiveError, Result};

pub(super) fn u32_at(data: &[u8], offset: usize) -> Result<u32> {
    let bytes = data.get(offset..offset + 4).ok_or_else(|| bad("Truncated integer"))?;
    Ok(u32::from_le_bytes(bytes.try_into().map_err(|_| bad("Truncated integer"))?))
}

pub(super) fn name(data: &[u8]) -> Result<String> {
    if data.last() != Some(&0) || data[..data.len() - 1].contains(&0) {
        return Err(bad("Invalid filename"));
    }
    Ok(data[..data.len() - 1].iter().map(|&b| if b == b'\\' { '/' } else { char::from(b) }).collect())
}

/// A stream begins with a six-byte signature, then length/flag-framed packets.
pub(super) fn decode_stream(data: &[u8], limit: usize) -> Result<(Vec<Record>, usize)> {
    if !data.starts_with(&[0x19, 0x96, 0x05, 0x30, 0x16, 0x30]) {
        return Err(bad("Invalid solid stream signature"));
    }
    let mut pos = 6;
    let mut payload = Vec::new();
    let mut first = true;
    loop {
        let header = data.get(pos..pos + 3).ok_or_else(|| bad("Truncated packet header"))?;
        let len = u16::from_le_bytes([header[0], header[1]]) as usize;
        let flags = header[2];
        if flags & !7 != 0 || (flags & 1 != 0) != first {
            return Err(bad("Invalid packet flags"));
        }
        pos += 3;
        let packet = data.get(pos..pos + len).ok_or_else(|| bad("Truncated packet"))?;
        payload.extend_from_slice(packet);
        pos += len;
        first = false;
        if flags & 2 != 0 {
            break;
        }
    }
    let header = payload.get(..10).ok_or_else(|| bad("Truncated compression header"))?;
    if header[0] != 0x10 || header[1] > 0x10 {
        return Err(ArchiveError::unsupported_method("JAR", format!("Engine version {}.{}", header[0], header[1])));
    }
    let flags = u16::from_le_bytes([header[4], header[5]]);
    // Bit 0 disables the fixed word dictionary; bit 1 disables the two
    // recent-symbol/match models. Neither changes the literal wire codes.
    if flags & !3 != 0 {
        return Err(ArchiveError::unsupported_method("JAR", format!("Engine flags {flags:#x}")));
    }
    // The final symbol block is padded with zeros to an 8192-symbol boundary.
    let symbol_limit = limit.checked_add(8192).ok_or_else(|| bad("Size limit overflow"))?;
    let (symbols, consumed) = engine::decode_symbols(&payload[10..], symbol_limit)?;
    if consumed + 10 != payload.len() {
        return Err(bad("Trailing data after Huffman stream"));
    }
    let records = words::decode(&symbols, limit)?;
    Ok((records, pos))
}

pub(super) fn record_crc(records: &[Record]) -> u32 {
    let mut hasher = crc32fast::Hasher::new();
    for record in records {
        if record.tag != 0 {
            hasher.update(&record.tag.to_le_bytes());
        }
        hasher.update(&record.data);
    }
    !hasher.finalize()
}

/// Decode a complete packet-framed JAR stream into its data records.
/// `method` is the engine identifier (0x10), not a byte in the Huffman table.
pub fn decompress(data: &[u8], method: u8, output_size: usize) -> Result<Vec<u8>> {
    if method != 0x10 {
        return Err(ArchiveError::unsupported_method("JAR", format!("Engine {method:#x}")));
    }
    let (records, _) = decode_stream(data, output_size.saturating_add(65536))?;
    let output: Vec<u8> = records.into_iter().filter(|r| r.tag == 0xe02).flat_map(|r| r.data).collect();
    if output.len() != output_size {
        return Err(bad("Incorrect decompressed size"));
    }
    Ok(output)
}
