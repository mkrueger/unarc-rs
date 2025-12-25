//! Crushed (Method 10) decompression for PAK archives.
//!
//! ## Algorithm Overview
//!
//! Crushed is an enhanced LZW variant used by PAK v2.51. It differs from standard LZW in:
//!
//! 1. **Adaptive mode switching**: Tracks recent symbols in a ring buffer to determine
//!    whether to use a 1-bit literal prefix or direct symbol coding.
//!
//! 2. **Usage-based table replacement**: When the 8192-entry table is full, entries are
//!    replaced based on a usage counter, not the oldest entry.
//!
//! 3. **Dynamic code size**: String codes start at 1 bit and grow up to 13 bits.
//!
//! ## PAK.EXE Analysis
//!
//! Reverse-engineered from PAK.EXE v2.51 using radare2.
//! - Initialization: `fcn.00010a2b` (135 bytes)
//! - Main loop: `fcn.00010ab2` (669 bytes)
//! - Dispatch: Method 0x0A at address 0x5e2a
//!
//! ## Data Format
//!
//! The compressed stream contains:
//! - In "literal mode": 1 bit (0=literal, 1=string) + 8-bit literal or N-bit string code
//! - In "string mode": N-bit code where values < 0x100 are XOR'd with 0xFF for literals
//!
//! String codes 0-255 are literals, 256 is EOF, 257+ are dictionary entries.
//!
//! ## Post-processing
//!
//! Crushed data is RLE-encoded. The caller must apply RLE decompression after LZW.
//! RLE marker is 0x90 (DLE).
//!
//! ## Constants (from PAK.EXE)
//!
//! - Table size: 8192 (0x2000)
//! - Reserved entries: 257 (0-255 literals + 256 EOF)
//! - Ring buffer: 500 entries
//! - Mode threshold: 375 strings in ring buffer
//! - Max code size: 13 bits

use crate::error::{ArchiveError, Result};
use bitstream_io::{BitRead, BitReader, LittleEndian};

/// Maximum LZW table entries (from PAK.EXE: 0x2000).
const TABLE_SIZE: usize = 8192;

/// Reserved entries: 0-255 literals + 256 EOF marker.
const RESERVED_ENTRIES: usize = 257;

/// LZW table entry.
#[derive(Clone, Copy, Default)]
struct LzwEntry {
    /// Parent index (-1 for root/literal entries).
    parent: i32,
    /// Byte value at this position.
    byte: u8,
}

/// Crushed decompressor state.
///
/// This mirrors the state maintained by PAK.EXE during decompression.
struct CrushedState {
    /// LZW string table (8192 entries).
    table: Vec<LzwEntry>,
    /// Usage counters for replacement algorithm.
    usage: Vec<u8>,
    /// Current table size (starts at 257).
    table_size: usize,
    /// Current code bit size for string codes (1-13).
    code_bits: u32,
    /// Threshold for increasing code_bits.
    next_bump: usize,
    /// True = use 1-bit literal prefix, False = direct coding.
    literal_mode: bool,
    /// Ring buffer tracking recent symbol types (true = string).
    ring_buffer: [bool; 500],
    /// Current ring buffer position.
    ring_pos: usize,
    /// Count of strings in ring buffer.
    string_count: usize,
    /// Search start position for usage-based replacement.
    usage_pos: usize,
    /// Previous symbol for dictionary chaining.
    prev_sym: Option<usize>,
}

impl CrushedState {
    /// Create new decompressor state.
    fn new() -> Self {
        let mut table = vec![LzwEntry::default(); TABLE_SIZE];

        // Initialize literal entries (0-255).
        for i in 0..256 {
            table[i] = LzwEntry {
                parent: -1,
                byte: i as u8,
            };
        }
        // Entry 256 = EOF (unused but reserved).
        table[256] = LzwEntry {
            parent: -1,
            byte: 0,
        };

        // Initialize usage counters.
        let mut usage = vec![0u8; TABLE_SIZE];
        for i in 0..RESERVED_ENTRIES {
            usage[i] = 4; // Root entries start with high usage.
        }

        Self {
            table,
            usage,
            table_size: RESERVED_ENTRIES,
            code_bits: 1,
            next_bump: 2,
            literal_mode: true,
            ring_buffer: [false; 500],
            ring_pos: 0,
            string_count: 0,
            usage_pos: RESERVED_ENTRIES,
            prev_sym: None,
        }
    }

    /// Check if table is full.
    #[inline]
    fn table_full(&self) -> bool {
        self.table_size >= TABLE_SIZE
    }

    /// Get first byte of a string (walk to root).
    fn first_byte(&self, mut sym: usize) -> u8 {
        // Safety limit to prevent infinite loops on corrupt data
        let mut iterations = 0usize;
        while sym < TABLE_SIZE && self.table[sym].parent >= 0 {
            iterations += 1;
            if iterations > TABLE_SIZE {
                break; // Corrupt data, just return what we have
            }
            sym = self.table[sym].parent as usize;
        }
        self.table.get(sym).map(|e| e.byte).unwrap_or(0)
    }

    /// Mark symbol chain as recently used.
    fn mark_used(&mut self, mut sym: usize) {
        // Safety limit to prevent infinite loops on corrupt data
        let mut iterations = 0usize;
        while sym < TABLE_SIZE {
            iterations += 1;
            if iterations > TABLE_SIZE {
                break; // Corrupt data
            }
            self.usage[sym] = 4;
            let parent = self.table[sym].parent;
            if parent < 0 {
                break;
            }
            sym = parent as usize;
        }
    }

    /// Update literal/string mode based on ring buffer statistics.
    ///
    /// From PAK.EXE: Switch mode when string_count crosses 375 threshold.
    fn update_mode(&mut self, is_string: bool) {
        // Remove old entry from count.
        if self.ring_buffer[self.ring_pos] {
            self.string_count = self.string_count.saturating_sub(1);
        }

        // Add new entry.
        self.ring_buffer[self.ring_pos] = is_string;
        if is_string {
            self.string_count += 1;
        }

        // Advance ring position.
        self.ring_pos = (self.ring_pos + 1) % 500;

        // Check mode switch (threshold = 375 from PAK.EXE).
        let use_literal_mode = self.string_count < 375;
        if use_literal_mode != self.literal_mode {
            self.literal_mode = use_literal_mode;
            // Recalculate next bump threshold.
            self.next_bump = 1usize << self.code_bits;
            if !self.literal_mode {
                self.next_bump = self.next_bump.saturating_sub(0x100);
            }
        }
    }

    /// Check and increase code bit size if needed.
    fn check_code_size(&mut self) {
        let added = self.table_size.saturating_sub(RESERVED_ENTRIES);
        if added >= self.next_bump && self.code_bits < 13 {
            self.code_bits += 1;
            self.next_bump = 1usize << self.code_bits;
            if !self.literal_mode {
                self.next_bump = self.next_bump.saturating_sub(0x100);
            }
        }
    }

    /// Read next symbol from bitstream.
    fn read_symbol<R: BitRead>(&self, reader: &mut R) -> Result<usize> {
        let sym = if self.literal_mode {
            // Mode with 1-bit prefix: 0 = literal, 1 = string.
            let is_string = reader.read_bit().map_err(|e| {
                ArchiveError::decompression_failed("Crushed", format!("read error: {e}"))
            })?;
            if is_string {
                let code: u16 = reader.read_var(self.code_bits).map_err(|e| {
                    ArchiveError::decompression_failed("Crushed", format!("read error: {e}"))
                })?;
                code as usize + 256
            } else {
                reader.read::<8, u8>().map_err(|e| {
                    ArchiveError::decompression_failed("Crushed", format!("read error: {e}"))
                })? as usize
            }
        } else {
            // Direct coding: values < 0x100 are XOR'd with 0xFF.
            let code: u16 = reader.read_var(self.code_bits).map_err(|e| {
                ArchiveError::decompression_failed("Crushed", format!("read error: {e}"))
            })?;
            let code = code as usize;
            if code < 0x100 {
                code ^ 0xFF
            } else {
                code
            }
        };
        Ok(sym)
    }

    /// Decode string for symbol into buffer (reversed).
    fn decode_string(&self, sym: usize, out: &mut Vec<u8>) -> Result<()> {
        out.clear();

        // Handle KwKwK case (sym == table_size).
        if sym == self.table_size {
            let prev = self.prev_sym.ok_or_else(|| {
                ArchiveError::decompression_failed("Crushed", "KwKwK without previous symbol")
            })?;

            // Decode previous string with safety limit.
            let mut s = prev;
            let mut iterations = 0usize;
            while s < TABLE_SIZE {
                iterations += 1;
                if iterations > TABLE_SIZE {
                    return Err(ArchiveError::decompression_failed(
                        "Crushed",
                        "infinite loop detected (corrupt data or wrong password)",
                    ));
                }
                let entry = &self.table[s];
                out.push(entry.byte);
                if entry.parent < 0 {
                    break;
                }
                s = entry.parent as usize;
            }

            // KwKwK: first byte is also last byte.
            let first = *out.last().unwrap_or(&0);
            out.insert(0, first);
            return Ok(());
        }

        if sym >= TABLE_SIZE {
            return Err(ArchiveError::decompression_failed(
                "Crushed",
                format!("invalid symbol {sym}"),
            ));
        }

        // Normal decode: walk parent chain.
        // Safety limit to prevent infinite loops on corrupt data
        let mut s = sym;
        let mut iterations = 0usize;
        while s < TABLE_SIZE {
            iterations += 1;
            if iterations > TABLE_SIZE {
                return Err(ArchiveError::decompression_failed(
                    "Crushed",
                    "infinite loop detected (corrupt data or wrong password)",
                ));
            }
            let entry = &self.table[s];
            out.push(entry.byte);
            if entry.parent < 0 {
                break;
            }
            s = entry.parent as usize;
        }
        Ok(())
    }

    /// Add new table entry (prev_sym + first byte of current symbol).
    fn add_entry(&mut self, sym: usize) {
        let prev = match self.prev_sym {
            None => {
                self.prev_sym = Some(sym);
                return;
            }
            Some(p) => p,
        };

        let first = if sym == self.table_size {
            self.first_byte(prev)
        } else {
            self.first_byte(sym)
        };

        if !self.table_full() {
            // Simple append.
            self.table[self.table_size] = LzwEntry {
                parent: prev as i32,
                byte: first,
            };
            self.usage[self.table_size] = 2;
            self.table_size += 1;
        } else {
            // Find least-used entry to replace.
            let mut min_idx = RESERVED_ENTRIES;
            let mut min_use = u8::MAX;
            let mut idx = self.usage_pos;

            loop {
                idx += 1;
                if idx >= TABLE_SIZE {
                    idx = RESERVED_ENTRIES;
                }

                if self.usage[idx] < min_use {
                    min_idx = idx;
                    min_use = self.usage[idx];
                }

                if self.usage[idx] > 0 {
                    self.usage[idx] -= 1;
                }
                if self.usage[idx] == 0 || idx == self.usage_pos {
                    break;
                }
            }

            self.usage_pos = idx;

            self.table[min_idx] = LzwEntry {
                parent: prev as i32,
                byte: first,
            };
            self.usage[min_idx] = 2;
        }

        self.prev_sym = Some(sym);
    }
}

/// Decompress Crushed (Method 10) data.
///
/// # Arguments
/// * `input` - Compressed data bytes
///
/// # Returns
/// Decompressed data. Note: This still needs RLE decoding (0x90 marker).
pub fn decompress(input: &[u8]) -> Result<Vec<u8>> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    let mut state = CrushedState::new();
    let mut reader = BitReader::endian(input, LittleEndian);
    let mut output = Vec::new();
    let mut buffer = Vec::with_capacity(256);

    loop {
        let sym = state.read_symbol(&mut reader)?;

        // EOF marker (code 256).
        if sym == 0x100 {
            break;
        }

        // Validate symbol.
        if sym >= TABLE_SIZE {
            return Err(ArchiveError::decompression_failed(
                "Crushed",
                format!("invalid symbol {sym}"),
            ));
        }
        if sym > state.table_size {
            return Err(ArchiveError::decompression_failed(
                "Crushed",
                format!("symbol {sym} > table_size {}", state.table_size),
            ));
        }

        // Mark as used.
        if sym < state.table_size {
            state.mark_used(sym);
        } else if let Some(prev) = state.prev_sym {
            state.mark_used(prev);
        }

        // Update mode tracking.
        state.update_mode(sym >= 256);

        // Decode and output string.
        state.decode_string(sym, &mut buffer)?;
        output.reserve(buffer.len());
        for &b in buffer.iter().rev() {
            output.push(b);
        }

        // Add new dictionary entry.
        state.add_entry(sym);

        // Check code size.
        state.check_code_size();
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        assert_eq!(decompress(&[]).unwrap(), Vec::<u8>::new());
    }
}
