//! HSC decompression for HA archives
//!
//! HSC uses PPM (Prediction by Partial Matching) with arithmetic coding.
//! Based on: Cleary, J. & Witten, I. (1984) "Data Compression Using
//! Adaptive Coding and Partial String Matching"

use std::io::Read;

use super::acoder::ArithmeticDecoder;
use crate::error::Result;

// ============================================================================
// Algorithm Constants (from PPM specification)
// ============================================================================

/// Maximum order (context length) for PPM model
const MAX_ORDER: usize = 4;

/// Number of context slots in the model
const CONTEXT_POOL_SIZE: usize = 10000;

/// Number of frequency blocks for symbol storage
const FREQ_BLOCK_POOL_SIZE: usize = 32760;

/// Hash table size (power of 2 for fast modulo)
const HASH_TABLE_SIZE: usize = 16384;

/// Frequency threshold below which symbols contribute to escape probability
const LOW_FREQ_THRESHOLD: u16 = 3;

/// Maximum total frequency before rescaling
const MAX_TOTAL_FREQ: u16 = 8000;

/// Initial rescale factor multiplier
const RESCALE_FACTOR_INIT: u8 = 4;

/// Initial escape counter limit
const ESCAPE_COUNTER_LIMIT: u8 = 32;

/// Non-escape count threshold for frequency scaling
const NON_ESCAPE_THRESHOLD: u8 = 5;

/// Non-escape count maximum
const NON_ESCAPE_MAX: u8 = 10;

/// Total frequency limit for non-escape scaling
const NON_ESCAPE_TOTAL_LIMIT: u16 = 4;

/// Sentinel value for null/empty pointers
const NULL_PTR: u16 = 0xFFFF;

/// Symbol value representing escape (end-of-context or end-of-stream)
const ESCAPE_SYMBOL: u16 = 256;

// ============================================================================
// PPM Context Model
// ============================================================================

/// Four-byte context window (most recent byte at index 0)
type ContextBytes = [u8; 4];

/// PPM Decompressor using order-4 context modeling with arithmetic coding
pub struct HscDecoder<R: Read> {
    /// Arithmetic coding engine
    coder: ArithmeticDecoder<R>,

    /// Current context bytes (rolling window)
    context_window: ContextBytes,

    // --- Hash Table for Context Lookup ---
    /// Hash table heads (context_id or NULL_PTR)
    hash_heads: Vec<u16>,
    /// Hash chain next pointers
    hash_chain: Vec<u16>,
    /// Randomization table for hashing
    hash_rand: Vec<u16>,

    // --- Context Pool ---
    /// Context bytes for each slot
    ctx_bytes: Vec<ContextBytes>,
    /// Context length (0-4, or 0xFF if unallocated)
    ctx_length: Vec<u8>,
    /// Unique character count per context
    ctx_char_count: Vec<u8>,
    /// Total frequency sum per context
    ctx_total_freq: Vec<u16>,
    /// Count of low-frequency symbols (freq < threshold)
    ctx_low_freq_count: Vec<u8>,
    /// Rescale factor per context
    ctx_rescale_factor: Vec<u8>,

    // --- LRU Expire List ---
    /// Previous pointer in LRU list
    lru_prev: Vec<u16>,
    /// Next pointer in LRU list
    lru_next: Vec<u16>,
    /// Front of LRU list (most recently used)
    lru_front: u16,
    /// Back of LRU list (least recently used, victim)
    lru_back: u16,

    // --- Frequency Block Pool ---
    /// Symbol frequency values
    freq_value: Vec<u16>,
    /// Symbol character values
    freq_char: Vec<u8>,
    /// Next block in chain (linked list)
    freq_next: Vec<u16>,
    /// Head of free block list
    free_block_head: u16,
    /// Context index for block reclamation
    reclaim_cursor: u16,

    // --- Exclusion Mechanism ---
    /// Bit mask for excluded characters
    excluded: [bool; 256],
    /// Stack of excluded characters (for fast reset)
    excluded_stack: Vec<u8>,

    // --- Escape Probability Tracking ---
    /// Consecutive non-escape decode counter
    non_escape_count: u8,
    /// Initial escape counters per context length
    initial_escape: [u8; MAX_ORDER + 1],

    // --- Update Stack (deferred model updates) ---
    /// Stack depth
    update_depth: usize,
    /// Context pointers (high bit = new symbol flag)
    update_contexts: [u16; MAX_ORDER + 1],
    /// Frequency block pointers
    update_blocks: [u16; MAX_ORDER + 1],

    // --- Context Search State ---
    /// Precomputed hash values per order
    order_hashes: [u16; MAX_ORDER + 1],
    /// Current search order
    search_order: i16,

    // --- Memory Management ---
    /// Counter for max-order context creation (triggers order reduction)
    order_reduction_counter: i16,
    /// Current maximum context order
    current_max_order: u8,
}

impl<R: Read> HscDecoder<R> {
    /// Initialize the PPM decoder
    pub fn new(reader: R) -> Result<Self> {
        let coder = ArithmeticDecoder::new(reader)?;

        // Initialize hash table with null pointers
        let hash_heads = vec![NULL_PTR; HASH_TABLE_SIZE];
        let hash_chain = vec![0u16; CONTEXT_POOL_SIZE];

        // Initialize LRU doubly-linked list
        let mut lru_prev = vec![0u16; CONTEXT_POOL_SIZE];
        let mut lru_next = vec![0u16; CONTEXT_POOL_SIZE];
        for i in 0..CONTEXT_POOL_SIZE {
            lru_next[i] = (i + 1) as u16;
            lru_prev[i] = i.wrapping_sub(1) as u16;
        }

        // Initialize context pool
        let ctx_bytes = vec![[0u8; 4]; CONTEXT_POOL_SIZE];
        let ctx_length = vec![0xFFu8; CONTEXT_POOL_SIZE]; // 0xFF = unallocated
        let ctx_char_count = vec![0u8; CONTEXT_POOL_SIZE];
        let ctx_total_freq = vec![0u16; CONTEXT_POOL_SIZE];
        let ctx_low_freq_count = vec![0u8; CONTEXT_POOL_SIZE];
        let ctx_rescale_factor = vec![0u8; CONTEXT_POOL_SIZE];

        // Initialize frequency block pool
        let freq_value = vec![0u16; FREQ_BLOCK_POOL_SIZE];
        let freq_char = vec![0u8; FREQ_BLOCK_POOL_SIZE];
        let mut freq_next = vec![NULL_PTR; FREQ_BLOCK_POOL_SIZE];

        // Chain free blocks (starting after context pool reserved blocks)
        for (i, item) in freq_next.iter_mut().enumerate().take(FREQ_BLOCK_POOL_SIZE - 1).skip(CONTEXT_POOL_SIZE) {
            *item = (i + 1) as u16;
        }
        let free_block_head = CONTEXT_POOL_SIZE as u16;

        // Build hash randomization table using LCG
        let hash_rand = Self::build_hash_table();

        // Initialize escape probability tracking
        let mut initial_escape = [0u8; MAX_ORDER + 1];
        initial_escape[0] = ESCAPE_COUNTER_LIMIT >> 1;
        for item in initial_escape.iter_mut().skip(1) {
            *item = (ESCAPE_COUNTER_LIMIT >> 1) - 1;
        }

        Ok(Self {
            coder,
            context_window: [0; 4],
            hash_heads,
            hash_chain,
            hash_rand,
            ctx_bytes,
            ctx_length,
            ctx_char_count,
            ctx_total_freq,
            ctx_low_freq_count,
            ctx_rescale_factor,
            lru_prev,
            lru_next,
            lru_front: 0,
            lru_back: (CONTEXT_POOL_SIZE - 1) as u16,
            freq_value,
            freq_char,
            freq_next,
            free_block_head,
            reclaim_cursor: 0,
            excluded: [false; 256],
            excluded_stack: Vec::with_capacity(256),
            non_escape_count: 0,
            initial_escape,
            update_depth: 0,
            update_contexts: [0; MAX_ORDER + 1],
            update_blocks: [0; MAX_ORDER + 1],
            order_hashes: [0; MAX_ORDER + 1],
            search_order: 0,
            order_reduction_counter: (CONTEXT_POOL_SIZE / 4) as i16,
            current_max_order: MAX_ORDER as u8,
        })
    }

    /// Build randomization table for context hashing
    fn build_hash_table() -> Vec<u16> {
        let mut table = vec![0u16; HASH_TABLE_SIZE];
        let mut seed: i64 = 10;

        for item in table.iter_mut() {
            // Linear congruential generator step
            let quotient = seed / (2147483647i64 / 16807);
            let remainder = seed % (2147483647i64 / 16807);
            let product = 16807i64 * remainder - (2147483647i64 % 16807) * quotient;
            seed = if product > 0 { product } else { product + 2147483647 };
            *item = (seed as u16) & ((HASH_TABLE_SIZE - 1) as u16);
        }

        table
    }

    /// Compute hash value for a context of given length
    #[inline]
    fn compute_hash(&self, bytes: &[u8], length: usize) -> u16 {
        let mask = (HASH_TABLE_SIZE - 1) as u16;
        let mut h: u16 = 0;

        for byte in bytes.iter().take(length.min(4)) {
            h = self.hash_rand[((*byte as u16).wrapping_add(h) & mask) as usize];
        }

        h
    }

    /// Slide context window by one character
    #[inline]
    fn advance_context(&mut self, byte: u8) {
        self.context_window[3] = self.context_window[2];
        self.context_window[2] = self.context_window[1];
        self.context_window[1] = self.context_window[0];
        self.context_window[0] = byte;
    }

    /// Move context to front of LRU list
    #[inline]
    fn promote_to_front(&mut self, ctx_id: u16) {
        if ctx_id == self.lru_front {
            return;
        }

        let idx = ctx_id as usize;

        // Unlink from current position
        if ctx_id == self.lru_back {
            self.lru_back = self.lru_prev[idx];
        } else {
            let next = self.lru_next[idx];
            let prev = self.lru_prev[idx];
            self.lru_prev[next as usize] = prev;
            self.lru_next[prev as usize] = next;
        }

        // Insert at front
        self.lru_prev[self.lru_front as usize] = ctx_id;
        self.lru_next[idx] = self.lru_front;
        self.lru_front = ctx_id;
    }

    /// Precompute hash values for all context orders
    fn prepare_context_search(&mut self) {
        let mask = (HASH_TABLE_SIZE - 1) as u16;

        self.order_hashes[1] = self.hash_rand[self.context_window[0] as usize];
        self.order_hashes[2] = self.hash_rand[(self.context_window[1] as u16).wrapping_add(self.order_hashes[1]) as usize & mask as usize];
        self.order_hashes[3] = self.hash_rand[(self.context_window[2] as u16).wrapping_add(self.order_hashes[2]) as usize & mask as usize];
        self.order_hashes[4] = self.hash_rand[(self.context_window[3] as u16).wrapping_add(self.order_hashes[3]) as usize & mask as usize];

        // Reset search state
        self.update_depth = 0;
        self.excluded_stack.clear();
        self.excluded = [false; 256];
        self.search_order = (MAX_ORDER + 1) as i16;
    }

    /// Find longest matching context, returns context ID or NULL_PTR
    fn find_longest_context(&mut self) -> u16 {
        self.prepare_context_search();
        self.find_next_context()
    }

    /// Find next shorter matching context
    fn find_next_context(&mut self) -> u16 {
        for order in (0..self.search_order as usize).rev() {
            let hash = self.order_hashes[order] as usize;
            let mut ctx_id = self.hash_heads[hash];

            while ctx_id != NULL_PTR {
                let idx = ctx_id as usize;

                if self.ctx_length[idx] as usize == order && self.context_matches(idx, order) {
                    self.search_order = order as i16;
                    return ctx_id;
                }

                ctx_id = self.hash_chain[idx];
            }
        }

        NULL_PTR
    }

    /// Check if context at given index matches current context window
    #[inline]
    fn context_matches(&self, idx: usize, order: usize) -> bool {
        match order {
            4 => {
                self.ctx_bytes[idx][0] == self.context_window[0]
                    && self.ctx_bytes[idx][1] == self.context_window[1]
                    && self.ctx_bytes[idx][2] == self.context_window[2]
                    && self.ctx_bytes[idx][3] == self.context_window[3]
            }
            3 => {
                self.ctx_bytes[idx][0] == self.context_window[0]
                    && self.ctx_bytes[idx][1] == self.context_window[1]
                    && self.ctx_bytes[idx][2] == self.context_window[2]
            }
            2 => self.ctx_bytes[idx][0] == self.context_window[0] && self.ctx_bytes[idx][1] == self.context_window[1],
            1 => self.ctx_bytes[idx][0] == self.context_window[0],
            0 => true,
            _ => false,
        }
    }

    /// Calculate adjusted escape probability
    #[inline]
    fn calculate_escape_probability(&self, low_freq_count: u16, ctx_id: u16) -> u16 {
        let idx = ctx_id as usize;
        let total = self.ctx_total_freq[idx];
        let char_count = self.ctx_char_count[idx];

        // New context: use initial escape counter
        if total == 1 {
            return if self.initial_escape[self.ctx_length[idx] as usize] >= (ESCAPE_COUNTER_LIMIT >> 1) {
                2
            } else {
                1
            };
        }

        // Context with all 256 characters seen
        if char_count == 255 {
            return 1;
        }

        let mut escape = low_freq_count;

        // Scale escape probability based on character diversity
        if char_count > 0 && ((char_count as u16 + 1) << 1) >= total {
            escape = (escape as u32 * ((char_count as u32 + 1) << 1) / total as u32) as u16;

            if char_count as u16 + 1 == total {
                escape += (char_count as u16 + 1) >> 1;
            }
        }

        escape.max(1)
    }

    /// Decode symbol from context without exclusions (first attempt)
    fn decode_without_exclusions(&mut self, ctx_id: u16) -> Result<u16> {
        let idx = ctx_id as usize;
        let escape = self.calculate_escape_probability(self.ctx_low_freq_count[idx] as u16, ctx_id);
        let mut total = self.ctx_total_freq[idx];

        // Apply frequency scaling for high non-escape count
        let scale: u8;
        let threshold: u16;

        if self.non_escape_count >= NON_ESCAPE_THRESHOLD {
            scale = if total <= NON_ESCAPE_TOTAL_LIMIT && self.non_escape_count == NON_ESCAPE_MAX {
                2
            } else {
                1
            };
            total <<= scale;
            threshold = self.coder.threshold_val(total + escape) >> scale;
        } else {
            scale = 0;
            threshold = self.coder.threshold_val(total + escape);
        }

        // Walk frequency list to find symbol
        let mut block = ctx_id;
        let mut cumulative = 0u16;
        let mut symbol_freq = 0u16;

        while block != NULL_PTR {
            let freq = self.freq_value[block as usize];

            if cumulative + freq > threshold {
                symbol_freq = freq;
                if scale > 0 {
                    symbol_freq <<= scale;
                }
                break;
            }

            cumulative += freq;
            block = self.freq_next[block as usize];
        }

        if scale > 0 {
            cumulative <<= scale;
        }

        self.update_depth = 1;

        if block != NULL_PTR {
            // Decoded a character
            self.coder.decode_update(cumulative, cumulative + symbol_freq, total + escape)?;

            // Update initial escape counter
            if self.ctx_total_freq[idx] == 1 && self.initial_escape[self.ctx_length[idx] as usize] > 0 {
                self.initial_escape[self.ctx_length[idx] as usize] -= 1;
            }

            self.update_blocks[0] = block;
            self.update_contexts[0] = ctx_id;

            if self.non_escape_count < NON_ESCAPE_MAX {
                self.non_escape_count += 1;
            }

            Ok(self.freq_char[block as usize] as u16)
        } else {
            // Escape
            self.coder.decode_update(total, total + escape, total + escape)?;

            // Update initial escape counter
            if self.ctx_total_freq[idx] == 1 && self.initial_escape[self.ctx_length[idx] as usize] < ESCAPE_COUNTER_LIMIT {
                self.initial_escape[self.ctx_length[idx] as usize] += 1;
            }

            // Add all symbols to exclusion mask
            let mut blk = ctx_id;
            let mut last = 0u16;
            while blk != NULL_PTR {
                let ch = self.freq_char[blk as usize];
                self.excluded_stack.push(ch);
                self.excluded[ch as usize] = true;
                last = blk;
                blk = self.freq_next[blk as usize];
            }

            self.update_contexts[0] = 0x8000 | ctx_id;
            self.update_blocks[0] = last;
            self.non_escape_count = 0;

            Ok(ESCAPE_SYMBOL)
        }
    }

    /// Decode symbol from context with exclusions (after escape)
    fn decode_with_exclusions(&mut self, ctx_id: u16) -> Result<u16> {
        let idx = ctx_id as usize;

        // Calculate frequencies excluding masked symbols
        let mut total = 0u16;
        let mut low_count = 0u16;
        let mut blk = ctx_id;

        while blk != NULL_PTR {
            let ch = self.freq_char[blk as usize];
            if !self.excluded[ch as usize] {
                let freq = self.freq_value[blk as usize];
                total += freq;
                if freq < LOW_FREQ_THRESHOLD {
                    low_count += 1;
                }
            }
            blk = self.freq_next[blk as usize];
        }

        let escape = self.calculate_escape_probability(low_count, ctx_id);
        let threshold = self.coder.threshold_val(total + escape);

        // Find symbol skipping excluded characters
        let mut block = ctx_id;
        let mut cumulative = 0u16;
        let mut symbol_freq = 0u16;

        while block != NULL_PTR {
            let ch = self.freq_char[block as usize];

            if !self.excluded[ch as usize] {
                let freq = self.freq_value[block as usize];

                if cumulative + freq > threshold {
                    symbol_freq = freq;
                    break;
                }

                cumulative += freq;
            }

            block = self.freq_next[block as usize];
        }

        if block != NULL_PTR {
            // Decoded a character
            self.coder.decode_update(cumulative, cumulative + symbol_freq, total + escape)?;

            if self.ctx_total_freq[idx] == 1 && self.initial_escape[self.ctx_length[idx] as usize] > 0 {
                self.initial_escape[self.ctx_length[idx] as usize] -= 1;
            }

            self.update_blocks[self.update_depth] = block;
            self.update_contexts[self.update_depth] = ctx_id;
            self.update_depth += 1;
            self.non_escape_count += 1;

            Ok(self.freq_char[block as usize] as u16)
        } else {
            // Escape
            self.coder.decode_update(total, total + escape, total + escape)?;

            if self.ctx_total_freq[idx] == 1 && self.initial_escape[self.ctx_length[idx] as usize] < ESCAPE_COUNTER_LIMIT {
                self.initial_escape[self.ctx_length[idx] as usize] += 1;
            }

            // Add unmasked symbols to exclusion list
            let mut blk = ctx_id;
            let mut last = 0u16;
            while blk != NULL_PTR {
                let ch = self.freq_char[blk as usize];
                if !self.excluded[ch as usize] {
                    self.excluded_stack.push(ch);
                    self.excluded[ch as usize] = true;
                }
                last = blk;
                blk = self.freq_next[blk as usize];
            }

            self.update_contexts[self.update_depth] = 0x8000 | ctx_id;
            self.update_blocks[self.update_depth] = last;
            self.update_depth += 1;

            Ok(ESCAPE_SYMBOL)
        }
    }

    /// Decode symbol from context (routes to appropriate method)
    #[inline]
    fn decode_from_context(&mut self, ctx_id: u16) -> Result<u16> {
        if self.excluded_stack.is_empty() {
            self.decode_without_exclusions(ctx_id)
        } else {
            self.decode_with_exclusions(ctx_id)
        }
    }

    /// Decode new character (uniform distribution, no context)
    fn decode_uniform(&mut self) -> Result<u16> {
        let unmasked_count = 257 - self.excluded_stack.len() as u16;
        let threshold = self.coder.threshold_val(unmasked_count);

        let mut symbol = 0u16;
        let mut cumulative = 0u16;

        while symbol < 256 {
            if self.excluded[symbol as usize] {
                symbol += 1;
                continue;
            }

            if cumulative + 1 > threshold {
                break;
            }

            cumulative += 1;
            symbol += 1;
        }

        self.coder.decode_update(cumulative, cumulative + 1, unmasked_count)?;

        Ok(symbol)
    }

    /// Reclaim frequency blocks from least-important context
    fn reclaim_blocks(&mut self) {
        // Find a reclaimable context not in update stack
        loop {
            loop {
                self.reclaim_cursor += 1;
                if self.reclaim_cursor >= CONTEXT_POOL_SIZE as u16 {
                    self.reclaim_cursor = 0;
                }

                if self.freq_next[self.reclaim_cursor as usize] != NULL_PTR {
                    break;
                }
            }

            // Check if in update stack
            let mut in_stack = false;
            for i in 0..=self.update_depth {
                if (self.update_contexts[i] & 0x7FFF) == self.reclaim_cursor {
                    in_stack = true;
                    break;
                }
            }

            if !in_stack {
                break;
            }
        }

        let ctx = self.reclaim_cursor as usize;

        // Find minimum frequency in context
        let mut min_freq = self.freq_value[ctx];
        let mut blk = self.freq_next[ctx];
        while blk != NULL_PTR {
            if self.freq_value[blk as usize] < min_freq {
                min_freq = self.freq_value[blk as usize];
            }
            blk = self.freq_next[blk as usize];
        }
        min_freq += 1;

        // Handle first block specially if it's below threshold
        if self.freq_value[ctx] < min_freq {
            let mut blk = self.freq_next[ctx];
            while self.freq_value[blk as usize] < min_freq && self.freq_next[blk as usize] != NULL_PTR {
                blk = self.freq_next[blk as usize];
            }

            self.freq_value[ctx] = self.freq_value[blk as usize];
            self.freq_char[ctx] = self.freq_char[blk as usize];

            let next = self.freq_next[blk as usize];
            self.freq_next[blk as usize] = self.free_block_head;
            self.free_block_head = self.freq_next[ctx];
            self.freq_next[ctx] = next;

            if next == NULL_PTR {
                self.ctx_char_count[ctx] = 0;
                self.ctx_total_freq[ctx] = self.freq_value[ctx];
                self.ctx_low_freq_count[ctx] = if self.ctx_total_freq[ctx] < LOW_FREQ_THRESHOLD { 1 } else { 0 };
                return;
            }
        }

        // Scale down all frequencies
        self.freq_value[ctx] /= min_freq;
        self.ctx_total_freq[ctx] = self.freq_value[ctx];
        self.ctx_low_freq_count[ctx] = if self.ctx_total_freq[ctx] < LOW_FREQ_THRESHOLD { 1 } else { 0 };
        self.ctx_char_count[ctx] = 0;

        let mut prev = ctx;
        let mut blk = self.freq_next[prev];

        while blk != NULL_PTR {
            if self.freq_value[blk as usize] < min_freq {
                // Free this block
                self.freq_next[prev] = self.freq_next[blk as usize];
                self.freq_next[blk as usize] = self.free_block_head;
                self.free_block_head = blk;
                blk = self.freq_next[prev];
            } else {
                self.ctx_char_count[ctx] += 1;
                self.freq_value[blk as usize] /= min_freq;
                self.ctx_total_freq[ctx] += self.freq_value[blk as usize];
                if self.freq_value[blk as usize] < LOW_FREQ_THRESHOLD {
                    self.ctx_low_freq_count[ctx] += 1;
                }
                prev = blk as usize;
                blk = self.freq_next[prev];
            }
        }
    }

    /// Allocate a new context slot
    fn allocate_context(&mut self, order: u8, first_char: u8) -> u16 {
        // Get slot from LRU back
        let new_ctx = self.lru_back;
        self.lru_back = self.lru_prev[new_ctx as usize];

        // Move to front
        self.lru_prev[self.lru_front as usize] = new_ctx;
        self.lru_next[new_ctx as usize] = self.lru_front;
        self.lru_front = new_ctx;

        let idx = new_ctx as usize;

        // Deallocate old context if present
        if self.ctx_length[idx] != 0xFF {
            if self.ctx_length[idx] == MAX_ORDER as u8 {
                self.order_reduction_counter -= 1;
                if self.order_reduction_counter == 0 {
                    self.current_max_order = (MAX_ORDER - 1) as u8;
                }
            }

            // Remove from hash table
            let hash = self.compute_hash(&self.ctx_bytes[idx], self.ctx_length[idx] as usize);

            if self.hash_heads[hash as usize] == new_ctx {
                self.hash_heads[hash as usize] = self.hash_chain[idx];
            } else {
                let mut prev = self.hash_heads[hash as usize];
                while self.hash_chain[prev as usize] != new_ctx {
                    prev = self.hash_chain[prev as usize];
                }
                self.hash_chain[prev as usize] = self.hash_chain[idx];
            }

            // Free frequency blocks
            if self.freq_next[idx] != NULL_PTR {
                let mut last = self.freq_next[idx];
                while self.freq_next[last as usize] != NULL_PTR {
                    last = self.freq_next[last as usize];
                }
                self.freq_next[last as usize] = self.free_block_head;
                self.free_block_head = self.freq_next[idx];
            }
        }

        // Initialize new context
        self.freq_next[idx] = NULL_PTR;
        self.ctx_low_freq_count[idx] = 1;
        self.ctx_total_freq[idx] = 1;
        self.freq_value[idx] = 1;
        self.freq_char[idx] = first_char;
        self.ctx_rescale_factor[idx] = RESCALE_FACTOR_INIT;
        self.ctx_char_count[idx] = 0;
        self.ctx_length[idx] = order;
        self.ctx_bytes[idx] = self.context_window;

        // Add to hash table
        let hash = self.compute_hash(&self.context_window, order as usize);
        self.hash_chain[idx] = self.hash_heads[hash as usize];
        self.hash_heads[hash as usize] = new_ctx;

        new_ctx
    }

    /// Update model with decoded character
    fn update_model(&mut self, character: u8) {
        while self.update_depth != 0 {
            self.update_depth -= 1;

            let block = self.update_blocks[self.update_depth];
            let mut ctx_id = self.update_contexts[self.update_depth];

            if ctx_id & 0x8000 != 0 {
                // New character in context
                ctx_id &= 0x7FFF;
                let idx = ctx_id as usize;

                // Allocate new frequency block
                if self.free_block_head == NULL_PTR {
                    self.reclaim_blocks();
                }

                let new_block = self.free_block_head;
                self.freq_next[block as usize] = new_block;
                self.free_block_head = self.freq_next[new_block as usize];
                self.freq_next[new_block as usize] = NULL_PTR;
                self.freq_value[new_block as usize] = 1;
                self.freq_char[new_block as usize] = character;
                self.ctx_char_count[idx] += 1;
                self.ctx_low_freq_count[idx] += 1;

                self.update_context_stats(idx, new_block);
            } else {
                // Increment existing frequency
                let idx = ctx_id as usize;
                self.freq_value[block as usize] += 1;

                if self.freq_value[block as usize] == LOW_FREQ_THRESHOLD {
                    self.ctx_low_freq_count[idx] = self.ctx_low_freq_count[idx].saturating_sub(1);
                }

                self.update_context_stats(idx, block);
            }
        }
    }

    /// Update context statistics and rescale if needed
    fn update_context_stats(&mut self, idx: usize, block: u16) {
        self.ctx_total_freq[idx] += 1;

        // Adjust rescale factor based on frequency distribution
        let char_divisor = self.ctx_char_count[idx] as u16 + 1;
        if (self.freq_value[block as usize] << 1) < self.ctx_total_freq[idx] / char_divisor {
            self.ctx_rescale_factor[idx] = self.ctx_rescale_factor[idx].saturating_sub(1);
        } else if self.ctx_rescale_factor[idx] < RESCALE_FACTOR_INIT {
            self.ctx_rescale_factor[idx] += 1;
        }

        // Rescale if needed
        if self.ctx_rescale_factor[idx] == 0 || self.ctx_total_freq[idx] >= MAX_TOTAL_FREQ {
            self.ctx_rescale_factor[idx] += 1;
            self.ctx_low_freq_count[idx] = 0;
            self.ctx_total_freq[idx] = 0;

            let mut blk = idx as u16;
            while blk != NULL_PTR {
                if self.freq_value[blk as usize] > 1 {
                    self.freq_value[blk as usize] >>= 1;
                    self.ctx_total_freq[idx] += self.freq_value[blk as usize];
                    if self.freq_value[blk as usize] < LOW_FREQ_THRESHOLD {
                        self.ctx_low_freq_count[idx] += 1;
                    }
                } else {
                    self.ctx_total_freq[idx] += 1;
                    self.ctx_low_freq_count[idx] += 1;
                }
                blk = self.freq_next[blk as usize];
            }
        }
    }

    /// Main decompression loop
    pub fn decompress(&mut self) -> Result<Vec<u8>> {
        let mut output = Vec::new();

        loop {
            // Find longest matching context
            let mut ctx_id = self.find_longest_context();

            let min_order = if ctx_id == NULL_PTR { 0 } else { self.ctx_length[ctx_id as usize] + 1 };
            let mut max_order = self.current_max_order + 1;

            // Decode symbol (with escape fallback to shorter contexts)
            let decoded;
            loop {
                if ctx_id == NULL_PTR {
                    decoded = self.decode_uniform()?;
                    break;
                }

                let result = self.decode_from_context(ctx_id)?;

                if result != ESCAPE_SYMBOL {
                    self.promote_to_front(ctx_id);
                    decoded = result;
                    break;
                }

                ctx_id = self.find_next_context();
            }

            // End of stream
            if decoded == ESCAPE_SYMBOL {
                break;
            }

            let character = decoded as u8;

            // Update model
            self.update_model(character);

            // Create new contexts for orders that didn't match
            while max_order > min_order {
                max_order -= 1;
                self.allocate_context(max_order, character);
            }

            output.push(character);
            self.advance_context(character);
        }

        Ok(output)
    }
}

/// Decompress HSC compressed data
pub fn decompress_hsc<R: Read>(reader: R) -> Result<Vec<u8>> {
    let mut decoder = HscDecoder::new(reader)?;
    decoder.decompress()
}
