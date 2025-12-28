use std::io::{self, ErrorKind};

use super::consts::*;
use super::supermaster_decompressed::SUPERMASTER;

// UC2 decompression implementation
// Based on ULTRACMP.CPP from the original UC2 source code

/// Master dictionary selection for UC2 decompression
pub enum MasterDict<'a> {
    /// 512 bytes of zeros ("NoMaster")
    NoMaster,
    /// Built-in SuperMaster dictionary (49152 bytes)
    SuperMaster,
    /// Custom master dictionary supplied by caller
    Custom(&'a [u8]),
}

struct HuffmanTable {
    entries: [u32; LOOKUP_SIZE],
}

impl HuffmanTable {
    const INVALID: u32 = 0;

    fn new() -> Self {
        Self {
            entries: [Self::INVALID; LOOKUP_SIZE],
        }
    }

    fn build_with_codes(&mut self, lengths: &[u8], num_literals: usize, num_dists: usize, dist_codes: &[u32]) -> io::Result<()> {
        let codes = &dist_codes[..num_dists.min(dist_codes.len())];
        self.fill(lengths, num_literals, codes)
    }

    fn build_lengths(&mut self, lengths: &[u8], num_symbols: usize) -> io::Result<()> {
        let mut codes = [0u32; NUM_LEN_SYM];
        for i in 0..num_symbols.min(NUM_LEN_SYM) {
            let (base, extra) = LEN_CODES[i];
            codes[i] = base | ((extra as u32) << 20);
        }
        self.fill(lengths, 0, &codes[..num_symbols.min(NUM_LEN_SYM)])
    }

    fn build_symbols(&mut self, lengths: &[u8], num_symbols: usize) -> io::Result<()> {
        self.fill(lengths, num_symbols, &[])
    }

    fn fill(&mut self, lengths: &[u8], num_literals: usize, codes: &[u32]) -> io::Result<()> {
        let num_codes = codes.len();
        let total_symbols = num_literals + num_codes;
        self.entries.fill(Self::INVALID);

        let mut pos = 0usize;
        for len in 1..=MAX_CODE_BITS {
            for symbol in 0..total_symbols.min(lengths.len()) {
                if lengths[symbol] as usize != len {
                    continue;
                }

                let span = 1usize << (MAX_CODE_BITS - len);
                if pos + span > LOOKUP_SIZE {
                    return Err(io::Error::new(ErrorKind::InvalidData, "invalid Huffman table"));
                }

                let value = if symbol < num_literals { symbol as u32 } else { codes[symbol - num_literals] } | ((len as u32) << 24);

                self.entries[pos..pos + span].fill(value);
                pos += span;
            }
        }

        Ok(())
    }

    fn decode(&self, bits: &mut Uc2BitReader) -> io::Result<u16> {
        self.decode_raw(bits).map(|v| v as u16)
    }

    fn decode_u32(&self, bits: &mut Uc2BitReader) -> io::Result<u32> {
        self.decode_raw(bits)
    }

    fn decode_raw(&self, bits: &mut Uc2BitReader) -> io::Result<u32> {
        let index = bits.peek_bits(MAX_CODE_BITS as u32)? as usize;
        let entry = self.entries[index];
        let bits_used = (entry >> 24) as u8;
        if bits_used == 0 {
            return Err(io::Error::new(ErrorKind::InvalidData, "invalid Huffman entry"));
        }
        bits.skip_bits(bits_used as u32)?;
        Ok(entry & 0x00FF_FFFF)
    }
}

/// UC2 bit reader - reads 16-bit LE words but extracts bits MSB-first
struct Uc2BitReader<'a> {
    data: &'a [u8],
    pos: usize,
    cache: u32,
    cache_bits: u32,
}

impl<'a> Uc2BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            pos: 0,
            cache: 0,
            cache_bits: 0,
        }
    }

    /// Fill cache with at least n bits
    #[inline]
    fn fill_cache(&mut self, n: u32) -> io::Result<()> {
        while self.cache_bits < n {
            if self.pos + 1 >= self.data.len() {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "truncated data"));
            }
            // Read 16-bit little-endian word
            let word = u16::from_le_bytes([self.data[self.pos], self.data[self.pos + 1]]) as u32;
            self.pos += 2;
            self.cache = (self.cache << 16) | word;
            self.cache_bits += 16;
        }
        Ok(())
    }

    /// Peek n bits without consuming them
    #[inline]
    fn peek_bits(&mut self, n: u32) -> io::Result<u32> {
        self.fill_cache(n)?;
        Ok((self.cache >> (self.cache_bits - n)) & ((1 << n) - 1))
    }

    /// Read n bits and consume them
    #[inline]
    fn read_bits(&mut self, n: u32) -> io::Result<u32> {
        let val = self.peek_bits(n)?;
        self.cache_bits -= n;
        Ok(val)
    }

    /// Skip n bits
    #[inline]
    fn skip_bits(&mut self, n: u32) -> io::Result<()> {
        self.fill_cache(n)?;
        self.cache_bits -= n;
        Ok(())
    }
}

/// Circular buffer for LZ77 decompression
struct CircularBuffer {
    buffer: Vec<u8>,
    pos: usize,
}

impl CircularBuffer {
    /// Create circular buffer with zeros (for NoMaster mode)
    fn new_with_zeros(initial_data_size: usize) -> Self {
        // Buffer is always 65536 bytes (u16 addressing), but only initial_data_size bytes are initialized
        let buffer = vec![0u8; 65536];
        // For NoMaster, only the first initial_data_size bytes are zeros (which they already are)
        Self {
            buffer,
            pos: initial_data_size, // Start position after the initial data
        }
    }

    /// Create circular buffer with custom master data
    fn new_with_custom_master(master_data: &[u8]) -> Self {
        let mut buffer = vec![0u8; 65536];
        let len = master_data.len().min(65536);
        buffer[0..len].copy_from_slice(&master_data[0..len]);
        Self {
            buffer,
            pos: len, // Start position after the master data
        }
    }

    /// Create circular buffer with pre-decompressed SuperMaster (for SuperMaster mode)
    fn new_with_supermaster() -> Result<Self, io::Error> {
        // Use the pre-decompressed SuperMaster (49152 bytes)
        // This avoids the need to recursively decompress it with NoMaster mode
        if SUPERMASTER.len() != 49152 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("SuperMaster has wrong size: {} (expected 49152)", SUPERMASTER.len()),
            ));
        }

        // Create 65536 byte buffer with SuperMaster at the START (not end!)
        // Buffer layout: [49152 bytes SuperMaster][rest zeros]
        let mut buffer = vec![0u8; 65536];
        buffer[0..49152].copy_from_slice(SUPERMASTER);

        Ok(Self {
            buffer,
            pos: 49152, // Start position AFTER the SuperMaster
        })
    }

    fn put(&mut self, byte: u8, output: &mut Vec<u8>) {
        self.buffer[self.pos] = byte;
        self.pos = (self.pos + 1) % self.buffer.len();
        output.push(byte);
    }

    fn copy_from_distance(&mut self, distance: usize, length: usize, output: &mut Vec<u8>) {
        let buf_size = self.buffer.len();

        // The C code does: ultra->cb.data[ultra->cb.tail] = ultra->cb.data[(u16)(ultra->cb.tail - dist)];
        // Since tail and dist are u16, the subtraction wraps around automatically.
        // In Rust, we need to explicitly handle wraparound with wrapping_sub.
        // This calculates the source position as: (pos - distance) % buf_size
        let mut src_pos = self.pos.wrapping_sub(distance) & (buf_size - 1); // buf_size is power of 2

        for _i in 0..length {
            let byte = self.buffer[src_pos];
            self.buffer[self.pos] = byte;
            output.push(byte);
            src_pos = (src_pos + 1) & (buf_size - 1); // Wraparound
            self.pos = (self.pos + 1) & (buf_size - 1);
        }
    }
}

// Default lengths
fn get_default_lengths() -> [u8; NUM_LD_SYM + NUM_LEN_SYM] {
    let mut lengths = [0u8; NUM_LD_SYM + NUM_LEN_SYM];

    let mut idx = 0;
    for &(count, value) in DEFAULT_LENGTHS_RLE {
        for _ in 0..count {
            if idx < lengths.len() {
                lengths[idx] = value;
                idx += 1;
            }
        }
    }
    lengths
}

/// Decode tree lengths using delta-coded Huffman tree  
fn decode_tree_lengths(bits: &mut Uc2BitReader, symprev: &mut [u8; NUM_LD_SYM + NUM_LEN_SYM]) -> io::Result<[u8; NUM_LD_SYM + NUM_LEN_SYM]> {
    let mut lengths = [0u8; NUM_LD_SYM + NUM_LEN_SYM];

    // Read tree type flag (1 bit)
    let has_tree = bits.read_bits(1)?;
    if has_tree == 0 {
        let defaults = get_default_lengths();
        *symprev = defaults;
        return Ok(defaults);
    }

    // Read tree type (2 bits)
    let tree_type = bits.read_bits(2)?;

    // Read lengths for the length codes table (15 * 3 bits)
    let mut tlengths = [0u8; NUM_LEN_CODES];
    for item in &mut tlengths {
        *item = bits.read_bits(3)? as u8;
    }

    // Build temporary Huffman table
    let mut temp_table = HuffmanTable::new();
    temp_table.build_symbols(&tlengths, NUM_LEN_CODES)?;

    // Decode the delta stream
    const NUM_SYMBOLS: usize = NUM_LD_SYM + NUM_LEN_SYM; // 344 symbols total

    let num_syms =
        NUM_SYMBOLS - NUM_LO_ASCII - NUM_HI_BYTE + if tree_type & 1 != 0 { NUM_LO_ASCII } else { 0 } + if tree_type & 2 != 0 { NUM_HI_BYTE } else { 0 };

    let mut stream = Vec::with_capacity(num_syms);
    let mut val = 0u8;

    while stream.len() < num_syms {
        let c = temp_table.decode(bits)? as usize;

        if c == REPEAT_CODE {
            let c2 = temp_table.decode(bits)? as usize;
            let n = c2 + MIN_REPEAT - 1;
            for _ in 0..n {
                stream.push(val);
            }
        } else {
            val = c as u8;
            stream.push(val);
        }
    }

    let rle_pattern = &RLE_PATTERNS[tree_type as usize];
    let mut idx = 0;
    let mut stream_idx = 0;

    for &v in rle_pattern.iter() {
        if v == 0 {
            break;
        }

        let count = (v & 0x1ff) as usize;
        let has_values = (v & 0x200) != 0;

        for _ in 0..count {
            if idx >= lengths.len() {
                break;
            }

            if has_values && stream_idx < stream.len() {
                let prev = symprev[idx] as usize;
                let delta = stream[stream_idx] as usize;
                if prev < 14 && delta < 14 {
                    lengths[idx] = VVAL[prev][delta];
                } else {
                    lengths[idx] = 0;
                }
                stream_idx += 1;
            } else {
                lengths[idx] = 0;
            }
            idx += 1;
        }
    }

    *symprev = lengths;
    Ok(lengths)
}

/// Decompress a single block
fn decompress_block(
    bits: &mut Uc2BitReader,
    ld_table: &HuffmanTable,
    len_table: &HuffmanTable,
    circ_buf: &mut CircularBuffer,
    output: &mut Vec<u8>,
    max_output: usize,
) -> io::Result<bool> {
    loop {
        if output.len() >= max_output {
            return Ok(false);
        }

        // Decode symbol from BD table (returns packed value for distance codes)
        let ld_entry = ld_table.decode_u32(bits)?;

        // Check if this is a literal (< 256) or distance code (has bit 16 set)
        let is_distance = (ld_entry & (1 << 16)) != 0;

        if !is_distance {
            // Literal byte (entry is just the byte value)
            let byte_val = ld_entry as u8;
            circ_buf.put(byte_val, output);
        } else {
            // Distance code: extract base_dist and extra_bits from packed value
            // Format: base_dist | (extra_bits << 20) | (1 << 16)
            let base_dist = ld_entry & 0xFFFF;
            let extra_bits_count = (ld_entry >> 20) & 0xF;

            let extra_bits_value = if extra_bits_count > 0 { bits.read_bits(extra_bits_count)? } else { 0 };
            let distance = base_dist + extra_bits_value;

            // Decode length first (C code reads length before EOB check)
            let len_entry = len_table.decode_u32(bits)?;

            if distance == EOB_MARK {
                return Ok(true); // End of block
            }

            let mut length = len_entry & 0xFFFF;
            let len_extra_bits = (len_entry >> 20) & 0xF;
            if len_extra_bits > 0 {
                length += bits.read_bits(len_extra_bits)?;
            }

            circ_buf.copy_from_distance(distance as usize, length as usize, output);
        }
    }
}

fn create_circular_buffer(dict: MasterDict) -> io::Result<CircularBuffer> {
    match dict {
        MasterDict::NoMaster => Ok(CircularBuffer::new_with_zeros(512)),
        MasterDict::SuperMaster => CircularBuffer::new_with_supermaster(),
        MasterDict::Custom(data) => Ok(CircularBuffer::new_with_custom_master(data)),
    }
}

fn decompress_impl(compressed: &[u8], expected_size: Option<usize>, dict: MasterDict) -> io::Result<Vec<u8>> {
    let mut bits = Uc2BitReader::new(compressed);
    let mut circ_buf = create_circular_buffer(dict)?;
    let mut output = Vec::with_capacity(expected_size.unwrap_or(0));
    let mut symprev = get_default_lengths();
    let max_output = expected_size.unwrap_or(usize::MAX);

    loop {
        let has_block = bits.read_bits(1)?;
        if has_block == 0 {
            break;
        }

        let lengths = decode_tree_lengths(&mut bits, &mut symprev)?;

        let mut ld_table = HuffmanTable::new();
        ld_table.build_with_codes(&lengths[..NUM_LD_SYM], NUM_BYTE_SYM, NUM_DIST_SYM, &PACKED_DIST_CODES)?;

        let mut len_table = HuffmanTable::new();
        len_table.build_lengths(&lengths[NUM_LD_SYM..], NUM_LEN_SYM)?;

        let _ = decompress_block(&mut bits, &ld_table, &len_table, &mut circ_buf, &mut output, max_output)?;

        if let Some(limit) = expected_size {
            if output.len() >= limit {
                break;
            }
        }
    }

    if let Some(limit) = expected_size {
        if output.len() < limit {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                format!("decompressed output shorter than expected ({} < {})", output.len(), limit),
            ));
        }
        if output.len() > limit {
            output.truncate(limit);
        }
    }

    Ok(output)
}

/// UC2 decompression with arbitrary dictionary configuration
pub fn decompress_with_dict<'a>(compressed: &[u8], expected_size: usize, dict: MasterDict<'a>) -> io::Result<Vec<u8>> {
    decompress_impl(compressed, Some(expected_size), dict)
}

/// Decompress a block that uses the "NoMaster" dictionary and unknown output size (e.g. CDIR)
pub fn decompress_no_master(compressed: &[u8]) -> io::Result<Vec<u8>> {
    decompress_impl(compressed, None, MasterDict::NoMaster)
}
