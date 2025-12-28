//! ASC decompression for HA archives
use std::io::Read;

use super::acoder::ArithmeticDecoder;
use crate::error::Result;

const WINDOW_CAPACITY: u16 = 31200;

const SHORT_LEN_COUNT: u16 = 16;
const LONG_LEN_COUNT: u16 = 48;
const LONG_LEN_BITS: u16 = 4;
const LONG_LEN_RANGE: u16 = 16;
const TOTAL_LENGTHS: u16 = SHORT_LEN_COUNT + LONG_LEN_COUNT * LONG_LEN_RANGE;

const LEN_TABLE_SIZE: usize = (SHORT_LEN_COUNT + LONG_LEN_COUNT) as usize;
const CHAR_TABLE_SIZE: usize = 256;
const POS_TABLE_SIZE: usize = 16;

const LEN_STEP: u16 = 8;
const POS_STEP: u16 = 24;
const TYPE_STEP: u16 = 40;
const TOTAL_MAX: u16 = 6000;

const CHAR_MAX: u16 = 1000;

const CHAR_LOCALITY: usize = 8;
const LEN_LOCALITY: usize = 4;
const TYPE_CONTEXTS: usize = 4;

/// A frequency table stored as a binary tree for efficient cumulative lookups
struct BinaryTreeTable {
    /// Storage: index 0 unused, 1 = root (total), 1..size internal, size..2*size leaves
    storage: Vec<u16>,
    /// Number of symbols (leaves)
    leaf_count: usize,
}

impl BinaryTreeTable {
    /// Create table with all leaves set to initial_value
    fn create(leaf_count: usize, initial_value: u16) -> Self {
        let mut storage = vec![0u16; leaf_count * 2];

        // Set leaf values
        for item in storage.iter_mut().take(2 * leaf_count).skip(leaf_count) {
            *item = initial_value;
        }

        // Compute internal nodes bottom-up
        Self::recompute_internals(&mut storage, leaf_count);

        Self { storage, leaf_count }
    }

    /// Recompute internal nodes from leaf values
    fn recompute_internals(storage: &mut [u16], leaf_count: usize) {
        let mut src = (leaf_count << 1) - 2;
        for dest in (1..leaf_count).rev() {
            storage[dest] = storage[src] + storage[src + 1];
            src -= 2;
        }
    }

    /// Root value = sum of all frequencies
    #[inline]
    fn root_sum(&self) -> u16 {
        self.storage[1]
    }

    /// Get frequency of a specific symbol
    #[inline]
    fn symbol_freq(&self, symbol: usize) -> u16 {
        self.storage[self.leaf_count + symbol]
    }

    /// Navigate tree to find symbol for threshold, return (symbol, cumulative_before)
    fn navigate_to_symbol(&self, threshold: u16) -> (usize, u16) {
        let mut node = 2;
        let mut cumulative = 0u16;

        loop {
            let left_child_sum = self.storage[node];
            if cumulative + left_child_sum <= threshold {
                cumulative += left_child_sum;
                node += 1; // move to right sibling
            }
            if node >= self.leaf_count {
                return (node - self.leaf_count, cumulative);
            }
            node <<= 1; // descend to left child
        }
    }

    /// Add step to symbol frequency and propagate up
    fn add_frequency(&mut self, symbol: usize, step: u16, max_total: u16) {
        let mut idx = symbol + self.leaf_count;
        while idx > 0 {
            self.storage[idx] += step;
            idx >>= 1;
        }

        if self.storage[1] >= max_total {
            self.halve_all();
        }
    }

    /// Halve all frequencies (minimum 1)
    fn halve_all(&mut self) {
        for idx in self.leaf_count..(2 * self.leaf_count) {
            if self.storage[idx] > 1 {
                self.storage[idx] >>= 1;
            }
        }
        Self::recompute_internals(&mut self.storage, self.leaf_count);
    }

    /// Remove symbol's frequency from tree
    fn remove_symbol(&mut self, symbol: usize) {
        let mut idx = symbol + self.leaf_count;
        let amount = self.storage[idx];
        while idx > 0 {
            self.storage[idx] -= amount;
            idx >>= 1;
        }
    }
}

/// Tracks literal vs match frequencies per context
struct TypeContextModel {
    /// frequencies[context][0] = literal, frequencies[context][1] = match
    frequencies: [[u16; 2]; TYPE_CONTEXTS],
    /// Current context (0-3)
    active_context: usize,
}

impl TypeContextModel {
    fn create() -> Self {
        Self {
            frequencies: [[TYPE_STEP; 2]; TYPE_CONTEXTS],
            active_context: 0,
        }
    }

    fn combined_frequency(&self) -> u16 {
        self.frequencies[self.active_context][0] + self.frequencies[self.active_context][1]
    }

    fn literal_frequency(&self) -> u16 {
        self.frequencies[self.active_context][0]
    }

    fn record_literal(&mut self) {
        let total = self.combined_frequency();
        self.frequencies[self.active_context][0] += TYPE_STEP;
        if total >= TOTAL_MAX {
            self.scale_context();
        }
        self.active_context = (self.active_context << 1) & 0x3;
    }

    fn record_match(&mut self) {
        let total = self.combined_frequency();
        self.frequencies[self.active_context][1] += TYPE_STEP;
        if total >= TOTAL_MAX {
            self.scale_context();
        }
        self.active_context = ((self.active_context << 1) | 1) & 0x3;
    }

    fn scale_context(&mut self) {
        let ctx = self.active_context;
        self.frequencies[ctx][0] = (self.frequencies[ctx][0] >> 1).max(1);
        self.frequencies[ctx][1] = (self.frequencies[ctx][1] >> 1).max(1);
    }
}

/// Circular buffer for LZ77 back-references
struct SlidingWindow {
    data: Vec<u8>,
    write_position: usize,
}

impl SlidingWindow {
    fn create(capacity: usize) -> Self {
        Self {
            data: vec![0u8; capacity],
            write_position: 0,
        }
    }

    fn store_byte(&mut self, byte: u8) {
        self.data[self.write_position] = byte;
        self.write_position += 1;
        if self.write_position >= self.data.len() {
            self.write_position = 0;
        }
    }

    fn copy_sequence(&mut self, length: u16, offset: u16, output: &mut Vec<u8>) {
        let capacity = self.data.len();

        // Calculate read start: offset positions back from current
        let mut read_pos = if self.write_position as u16 > offset {
            self.write_position - 1 - offset as usize
        } else {
            capacity - 1 - offset as usize + self.write_position
        };

        for _ in 0..length {
            let byte = self.data[read_pos];
            output.push(byte);
            self.data[self.write_position] = byte;

            self.write_position += 1;
            if self.write_position >= capacity {
                self.write_position = 0;
            }
            read_pos += 1;
            if read_pos >= capacity {
                read_pos = 0;
            }
        }
    }
}

pub struct AscDecoder<R: Read> {
    /// Arithmetic coding engine
    coder: ArithmeticDecoder<R>,

    /// Type model (literal vs match)
    type_model: TypeContextModel,

    /// Character main table
    char_main: BinaryTreeTable,
    /// Character escape table
    char_escape: BinaryTreeTable,
    /// Character escape weight
    char_escape_weight: u16,

    /// Length main table
    len_main: BinaryTreeTable,
    /// Length escape table
    len_escape: BinaryTreeTable,
    /// Length escape weight
    len_escape_weight: u16,

    /// Position table
    pos_table: BinaryTreeTable,
    /// Active position codes
    pos_codes_active: u16,
    /// Maximum addressable position
    pos_max_value: u16,

    /// Sliding window buffer
    window: SlidingWindow,
    /// Bytes written so far
    bytes_emitted: u16,
}

impl<R: Read> AscDecoder<R> {
    /// Initialize decoder
    pub fn new(reader: R) -> Result<Self> {
        let coder = ArithmeticDecoder::new(reader)?;

        // Initialize tables
        let char_main = BinaryTreeTable::create(CHAR_TABLE_SIZE, 0);
        let char_escape = BinaryTreeTable::create(CHAR_TABLE_SIZE, 1);

        let len_main = BinaryTreeTable::create(LEN_TABLE_SIZE, 0);
        let len_escape = BinaryTreeTable::create(LEN_TABLE_SIZE, 1);

        let mut pos_table = BinaryTreeTable::create(POS_TABLE_SIZE, 0);
        pos_table.add_frequency(0, POS_STEP, TOTAL_MAX);

        Ok(Self {
            coder,
            type_model: TypeContextModel::create(),
            char_main,
            char_escape,
            char_escape_weight: 1,
            len_main,
            len_escape,
            len_escape_weight: LEN_STEP,
            pos_table,
            pos_codes_active: 1,
            pos_max_value: 1,
            window: SlidingWindow::create(WINDOW_CAPACITY as usize),
            bytes_emitted: 0,
        })
    }

    pub fn decompress(&mut self) -> Result<Vec<u8>> {
        let mut output = Vec::new();

        loop {
            let type_total = self.type_model.combined_frequency();
            let threshold = self.coder.threshold_val(type_total + 1);
            let lit_freq = self.type_model.literal_frequency();

            if lit_freq > threshold {
                self.coder.decode_update(0, lit_freq, type_total + 1)?;
                self.type_model.record_literal();

                let byte = self.decode_character()?;
                output.push(byte);
                self.window.store_byte(byte);

                if self.bytes_emitted < WINDOW_CAPACITY {
                    self.bytes_emitted += 1;
                }
            } else if type_total > threshold {
                self.coder.decode_update(lit_freq, type_total, type_total + 1)?;
                self.type_model.record_match();

                while self.bytes_emitted > self.pos_max_value {
                    self.pos_table.add_frequency(self.pos_codes_active as usize, POS_STEP, TOTAL_MAX);
                    self.pos_codes_active += 1;
                    self.pos_max_value <<= 1;
                }

                let position = self.decode_position()?;
                let length = self.decode_length()?;

                self.window.copy_sequence(length, position, &mut output);

                if self.bytes_emitted < WINDOW_CAPACITY {
                    self.bytes_emitted += length;
                    if self.bytes_emitted > WINDOW_CAPACITY {
                        self.bytes_emitted = WINDOW_CAPACITY;
                    }
                }
            } else {
                self.coder.decode_update(type_total, type_total + 1, type_total + 1)?;
                break;
            }
        }

        Ok(output)
    }

    fn decode_character(&mut self) -> Result<u8> {
        let main_total = self.char_main.root_sum();
        let combined = main_total + self.char_escape_weight;
        let threshold = self.coder.threshold_val(combined);

        let symbol = if threshold >= main_total {
            self.coder.decode_update(main_total, combined, combined)?;

            let esc_total = self.char_escape.root_sum();
            let esc_threshold = self.coder.threshold_val(esc_total);
            let (sym, lt) = self.char_escape.navigate_to_symbol(esc_threshold);
            let freq = self.char_escape.symbol_freq(sym);
            self.coder.decode_update(lt, lt + freq, esc_total)?;

            self.char_escape.remove_symbol(sym);

            if self.char_escape.root_sum() != 0 {
                self.char_escape_weight += 1;
            } else {
                self.char_escape_weight = 0;
            }

            let start = sym.saturating_sub(CHAR_LOCALITY);
            let end = (sym + CHAR_LOCALITY).min(CHAR_TABLE_SIZE - 1);
            for i in start..end {
                if self.char_escape.symbol_freq(i) > 0 {
                    self.char_escape.add_frequency(i, 1, CHAR_MAX);
                }
            }
            sym
        } else {
            let (sym, lt) = self.char_main.navigate_to_symbol(threshold);
            let freq = self.char_main.symbol_freq(sym);
            self.coder.decode_update(lt, lt + freq, combined)?;
            sym
        };

        self.char_main.add_frequency(symbol, 1, CHAR_MAX);

        if self.char_main.symbol_freq(symbol) == 3 {
            if 1 < self.char_escape_weight {
                self.char_escape_weight -= 1;
            } else {
                self.char_escape_weight = self.char_escape_weight.saturating_sub(1).max(1);
            }
        }

        Ok(symbol as u8)
    }

    fn decode_position(&mut self) -> Result<u16> {
        let total = self.pos_table.root_sum();
        let threshold = self.coder.threshold_val(total);
        let (code, lt) = self.pos_table.navigate_to_symbol(threshold);
        let freq = self.pos_table.symbol_freq(code);
        self.coder.decode_update(lt, lt + freq, total)?;

        self.pos_table.add_frequency(code, POS_STEP, TOTAL_MAX);

        let position = if code > 1 {
            let base = 1u16 << (code - 1);
            let range = if base == (self.pos_max_value >> 1) {
                self.bytes_emitted - (self.pos_max_value >> 1)
            } else {
                base
            };

            let extra = self.coder.threshold_val(range);
            self.coder.decode_update(extra, extra + 1, range)?;
            extra + base
        } else {
            code as u16
        };

        Ok(position)
    }

    fn decode_length(&mut self) -> Result<u16> {
        let main_total = self.len_main.root_sum();
        let combined = main_total + self.len_escape_weight;
        let threshold = self.coder.threshold_val(combined);

        let code = if threshold >= main_total {
            self.coder.decode_update(main_total, combined, combined)?;

            let esc_total = self.len_escape.root_sum();
            let esc_threshold = self.coder.threshold_val(esc_total);
            let (sym, lt) = self.len_escape.navigate_to_symbol(esc_threshold);
            let freq = self.len_escape.symbol_freq(sym);
            self.coder.decode_update(lt, lt + freq, esc_total)?;

            self.len_escape.remove_symbol(sym);

            if self.len_escape.root_sum() != 0 {
                self.len_escape_weight += LEN_STEP;
            } else {
                self.len_escape_weight = 0;
            }

            let start = sym.saturating_sub(LEN_LOCALITY);
            let end = (sym + LEN_LOCALITY).min(LEN_TABLE_SIZE - 1);
            for i in start..end {
                if self.len_escape.symbol_freq(i) > 0 {
                    self.len_escape.add_frequency(i, 1, TOTAL_MAX);
                }
            }
            sym
        } else {
            let (sym, lt) = self.len_main.navigate_to_symbol(threshold);
            let freq = self.len_main.symbol_freq(sym);
            self.coder.decode_update(lt, lt + freq, combined)?;
            sym
        };

        self.len_main.add_frequency(code, LEN_STEP, TOTAL_MAX);

        if self.len_main.symbol_freq(code) == 3 * LEN_STEP {
            if LEN_STEP < self.len_escape_weight {
                self.len_escape_weight -= LEN_STEP;
            } else {
                self.len_escape_weight = self.len_escape_weight.saturating_sub(1).max(1);
            }
        }

        let raw_length = if code == (SHORT_LEN_COUNT as usize - 1) {
            TOTAL_LENGTHS - 1
        } else if code >= SHORT_LEN_COUNT as usize {
            let extra = self.coder.threshold_val(LONG_LEN_RANGE);
            self.coder.decode_update(extra, extra + 1, LONG_LEN_RANGE)?;
            ((code as u16 - SHORT_LEN_COUNT) << LONG_LEN_BITS) + extra + SHORT_LEN_COUNT - 1
        } else {
            code as u16
        };

        const MIN_MATCH: u16 = 3;
        Ok(raw_length + MIN_MATCH)
    }
}

pub fn decompress_asc<R: Read>(reader: R) -> Result<Vec<u8>> {
    let mut decoder = AscDecoder::new(reader)?;
    decoder.decompress()
}
