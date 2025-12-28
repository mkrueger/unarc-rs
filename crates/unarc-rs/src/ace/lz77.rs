//! LZ77 decompression for ACE archives

use std::io::Read;

use crate::error::{ArchiveError, Result};

use super::bitstream::BitStream;
use super::huffman::HuffmanTree;

/// LZ77 constants
#[allow(dead_code)]
pub const MAX_CODE_WIDTH: u8 = 11;
#[allow(dead_code)]
pub const MAX_LEN: usize = 259;
pub const MAX_DIC_BITS: usize = 22;
pub const MIN_DIC_SIZE: usize = 1 << 10;
pub const MAX_DIC_SIZE: usize = 1 << MAX_DIC_BITS;
pub const TYPE_CODE: u16 = 260 + MAX_DIC_BITS as u16 + 1;
pub const NUM_MAIN_CODES: usize = 260 + MAX_DIC_BITS + 2;
pub const NUM_LEN_CODES: usize = 255;
pub const MAX_DIST_AT_LEN2: usize = 255;
pub const MAX_DIST_AT_LEN3: usize = 8191;

/// Distance history for LZ77
struct DistanceHistory {
    history: [usize; 4],
}

impl DistanceHistory {
    fn new() -> Self {
        Self { history: [0; 4] }
    }

    fn append(&mut self, dist: usize) {
        self.history[0] = self.history[1];
        self.history[1] = self.history[2];
        self.history[2] = self.history[3];
        self.history[3] = dist;
    }

    fn retrieve(&mut self, offset: usize) -> usize {
        let idx = 3 - offset;
        let dist = self.history[idx];
        // Move to end (most recent)
        for i in idx..3 {
            self.history[i] = self.history[i + 1];
        }
        self.history[3] = dist;
        dist
    }
}

/// LZ77 dictionary (sliding window)
struct Dictionary {
    data: Vec<u8>,
    min_size: usize,
    max_size: usize,
}

impl Dictionary {
    fn new(min_size: usize, max_size: usize) -> Self {
        Self {
            data: Vec::new(),
            min_size,
            max_size,
        }
    }

    fn set_size(&mut self, size: usize) {
        // Size can only grow
        self.min_size = self.min_size.max(size).min(self.max_size);
    }

    fn push(&mut self, byte: u8) {
        self.data.push(byte);
    }

    #[allow(dead_code)]
    fn extend(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    fn copy_from_history(&mut self, dist: usize, len: usize) -> Result<()> {
        let src_pos = self
            .data
            .len()
            .checked_sub(dist)
            .ok_or_else(|| ArchiveError::decompression_failed("lz77", "copy source out of bounds"))?;

        // Copy byte by byte for overlapping regions
        for i in 0..len {
            let byte = self.data[src_pos + i];
            self.data.push(byte);
        }
        Ok(())
    }

    fn take_output(&mut self, n: usize) -> Vec<u8> {
        if n == 0 {
            return Vec::new();
        }

        let output = self.data[self.data.len() - n..].to_vec();
        self.truncate();
        output
    }

    fn truncate(&mut self) {
        // Only truncate when dictionary exceeds 4x its size
        if self.data.len() > 4 * self.min_size {
            let keep = self.data.len().saturating_sub(self.min_size);
            self.data.drain(0..keep);
        }
    }

    #[allow(dead_code)]
    fn register(&mut self, bytes: &[u8]) {
        self.extend(bytes);
        self.truncate();
    }
}

/// Symbol reader for LZ77 blocks
struct SymbolReader {
    main_tree: Option<HuffmanTree>,
    len_tree: Option<HuffmanTree>,
    symbols_remaining: usize,
}

impl SymbolReader {
    fn new() -> Self {
        Self {
            main_tree: None,
            len_tree: None,
            symbols_remaining: 0,
        }
    }

    fn read_trees<R: Read>(&mut self, bs: &mut BitStream<R>) -> Result<()> {
        self.main_tree = Some(HuffmanTree::read_from(bs, MAX_CODE_WIDTH, NUM_MAIN_CODES)?);
        self.len_tree = Some(HuffmanTree::read_from(bs, MAX_CODE_WIDTH, NUM_LEN_CODES)?);
        self.symbols_remaining = bs.read_bits(15)? as usize;
        Ok(())
    }

    fn read_main_symbol<R: Read>(&mut self, bs: &mut BitStream<R>) -> Result<u16> {
        if self.symbols_remaining == 0 {
            self.read_trees(bs)?;
        }
        self.symbols_remaining -= 1;
        self.main_tree.as_ref().unwrap().read_symbol(bs)
    }

    fn read_len_symbol<R: Read>(&mut self, bs: &mut BitStream<R>) -> Result<u16> {
        self.len_tree.as_ref().unwrap().read_symbol(bs)
    }
}

/// LZ77 decompression engine
pub struct Lz77Decoder {
    dictionary: Dictionary,
    dist_history: DistanceHistory,
    symbol_reader: SymbolReader,
}

impl Lz77Decoder {
    pub fn new() -> Self {
        Self {
            dictionary: Dictionary::new(MIN_DIC_SIZE, MAX_DIC_SIZE),
            dist_history: DistanceHistory::new(),
            symbol_reader: SymbolReader::new(),
        }
    }

    pub fn reset(&mut self) {
        self.symbol_reader = SymbolReader::new();
        self.dist_history = DistanceHistory::new();
    }

    pub fn set_dictionary_size(&mut self, size: usize) {
        self.dictionary.set_size(size);
    }

    #[allow(dead_code)]
    pub fn register_data(&mut self, data: &[u8]) {
        self.dictionary.register(data);
    }

    /// Decompress data from bit stream
    pub fn decompress<R: Read>(&mut self, bs: &mut BitStream<R>, want_size: usize) -> Result<Vec<u8>> {
        let mut have_size = 0;

        while have_size < want_size {
            let symbol = self.symbol_reader.read_main_symbol(bs)?;

            if symbol <= 255 {
                // Literal byte
                self.dictionary.push(symbol as u8);
                have_size += 1;
            } else if symbol < TYPE_CODE {
                // Copy from dictionary
                let (copy_dist, copy_len) = if symbol <= 259 {
                    // Use distance history
                    let offset = (symbol & 0x03) as usize;
                    let len = self.symbol_reader.read_len_symbol(bs)? as usize;
                    let dist = self.dist_history.retrieve(offset);
                    let len = if offset > 1 { len + 3 } else { len + 2 };
                    (dist, len)
                } else {
                    // Read distance from bitstream
                    let dist_bits = (symbol - 260) as u8;
                    let dist = bs.read_known_width_uint(dist_bits)? as usize;
                    let len = self.symbol_reader.read_len_symbol(bs)? as usize;
                    self.dist_history.append(dist);

                    let len = if dist <= MAX_DIST_AT_LEN2 {
                        len + 2
                    } else if dist <= MAX_DIST_AT_LEN3 {
                        len + 3
                    } else {
                        len + 4
                    };
                    (dist, len)
                };

                if have_size + copy_len > want_size {
                    return Err(ArchiveError::decompression_failed("lz77", "copy exceeds expected size"));
                }

                self.dictionary.copy_from_history(copy_dist + 1, copy_len)?;
                have_size += copy_len;
            } else if symbol == TYPE_CODE {
                // Mode change - skip for now (basic LZ77)
                let _mode = bs.read_bits(8)?;
                // Handle DELTA and EXE modes if needed
            } else {
                return Err(ArchiveError::decompression_failed("lz77", "invalid symbol"));
            }
        }

        Ok(self.dictionary.take_output(have_size))
    }
}
