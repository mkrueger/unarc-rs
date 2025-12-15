use delharc::decode::{Decoder, DecoderAny};
use salzweg::CodeSizeStrategy;
use std::sync::OnceLock;

use crate::error::{ArchiveError, Result};

#[derive(Clone, Copy, Debug)]
enum SqzLenMapping {
    Identity,
    /// Deflate-style length decoding using 29 codes mapped onto SQZ symbols
    /// 256..=284 (i.e. Deflate 257..=285).
    DeflateLike29,
    /// Generic base+extra scheme: 256 codes split into 16-code groups;
    /// group index determines extra-bit count.
    Grouped16,
    /// Use the base/extra tables embedded in the original `SQZ.EXE`.
    ExeTables,
    /// SQZ native: symbols 256-447 are direct lengths, 448+ have 1 extra bit
    SqzNative,
}

#[derive(Clone, Copy, Debug)]
enum SqzDistMapping {
    PowerOfTwo,
    DeflateLike,
    /// Use the base/extra tables embedded in the original `SQZ.EXE`.
    ExeTables,
}

struct SqzExeTables {
    // Length codes: only 32 entries needed (for codes 256-287, representing lengths 3-258)
    len_extra: [u8; 32],
    len_base: [u16; 32],

    // For method>=3, SQZ.EXE uses NP=0x1F codes for distance.
    dist_extra: [u8; 0x20],
    dist_base: [u16; 0x20],
}

static SQZ_EXE_TABLES: OnceLock<SqzExeTables> = OnceLock::new();

fn sqz_exe_tables() -> Result<&'static SqzExeTables> {
    if let Some(t) = SQZ_EXE_TABLES.get() {
        return Ok(t);
    }

    // Hardcoded tables extracted from SQZ.EXE via xxd analysis.
    // These are SQZ-specific (not Deflate) length/distance encoding tables.

    // Length extra bits (28 entries for length codes 256-283)
    // Extracted from SQZ.EXE offset 0x10cd8
    // Note: Only first 28 entries are valid (for symbols 256-283)
    // Pattern: 4 entries per group with increasing extra bits: 0,0,0,0, 1,1,1,1, 2,2,2,2, ...
    let len_extra: [u8; 32] = [
        0, 0, 0, 0,  // codes 0-3 (sym 256-259): 0 extra bits -> len 3-6
        1, 1, 1, 1,  // codes 4-7 (sym 260-263): 1 extra bit
        2, 2, 2, 2,  // codes 8-11 (sym 264-267): 2 extra bits
        3, 3, 3, 3,  // codes 12-15 (sym 268-271): 3 extra bits
        4, 4, 4, 4,  // codes 16-19 (sym 272-275): 4 extra bits
        5, 5, 5, 5,  // codes 20-23 (sym 276-279): 5 extra bits
        6, 6, 6, 6,  // codes 24-27 (sym 280-283): 6 extra bits
        0, 0, 0, 0,  // codes 28-31: unused/padding
    ];

    // Length base values - SQZ uses different progression than Deflate
    // Extracted from SQZ.EXE offset 0x10c98 (28 words = 56 bytes)
    // These are the base lengths before adding extra bits
    let len_base: [u16; 32] = [
        3, 4, 5, 6,         // with 0 extra bits: lengths 3-6
        7, 8, 10, 12,       // with 1 extra bit: lengths 7-8, 10-11, 12-13
        14, 16, 20, 24,     // with 2 extra bits
        28, 32, 40, 48,     // with 3 extra bits
        56, 64, 80, 96,     // with 4 extra bits
        112, 128, 160, 192, // with 5 extra bits
        224, 256, 320, 384, // with 6 extra bits
        0, 0, 0, 0,         // unused/padding
    ];

    // Distance extra bits (32 entries) - SQZ variant
    // Extracted from SQZ.EXE offset 0x10d34 (32 bytes)
    // Note: First 5 entries (codes 0-4) have 0 extra bits, then progression starts
    let dist_extra: [u8; 0x20] = [
        0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6,
        6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 0,
    ];

    // Distance base values - SQZ uses different progression than Deflate
    // Extracted from SQZ.EXE offset 0x10cf4 (32 words = 64 bytes)
    // Note: These differ from Deflate! E.g. code 5 → base 5, not 7
    let dist_base: [u16; 0x20] = [
        0, 1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129,
        193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577, 32768,
    ];

    let t = SqzExeTables {
        len_extra,
        len_base,
        dist_extra,
        dist_base,
    };

    SQZ_EXE_TABLES
        .set(t)
        .map_err(|_| ArchiveError::decompression_failed("SQZ", "SQZ.EXE tables init failed"))?;
    Ok(SQZ_EXE_TABLES.get().expect("tables set"))
}

#[derive(Clone, Copy, Debug)]
enum SqzHuffCodeOrder {
    Normal,
    Reversed,
}

#[derive(Clone, Copy, Debug)]
enum SqzBitOrder {
    Msb,
    Lsb,
}

trait SqzBitRead {
    fn ensure_bits(&mut self, need: u8) -> Result<()>;
    fn get_bits_u16(&mut self, n: u8) -> Result<u16>;
    fn peek_bits(&mut self, n: u8) -> Result<u16>;
    fn skip_bits(&mut self, n: u8) -> Result<()>;
    fn get_bit(&mut self) -> Result<u16> {
        self.get_bits_u16(1)
    }
    fn debug_position(&self) -> String;
}

#[derive(Clone, Debug)]
struct BitReaderMsb<'a> {
    data: &'a [u8],
    pos: usize,
    bitbuf: u32,
    bits: u8,
}

impl<'a> BitReaderMsb<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            pos: 0,
            bitbuf: 0,
            bits: 0,
        }
    }

    fn ensure_bits_internal(&mut self, need: u8) -> Result<()> {
        while self.bits < need {
            let byte = *self
                .data
                .get(self.pos)
                .ok_or_else(|| {
                    ArchiveError::decompression_failed("SQZ", "Unexpected EOF in bitstream")
                })?;
            self.pos += 1;
            self.bitbuf = (self.bitbuf << 8) | (byte as u32);
            self.bits = self.bits.saturating_add(8);
        }
        Ok(())
    }

    fn get_bits_u16_internal(&mut self, n: u8) -> Result<u16> {
        if n > 16 {
            return Err(ArchiveError::decompression_failed(
                "SQZ",
                format!("Too many bits requested: {n}"),
            ));
        }
        if n == 0 {
            return Ok(0);
        }
        self.ensure_bits_internal(n)?;
        let shift = self.bits - n;
        let val = ((self.bitbuf >> shift) & ((1u32 << n) - 1)) as u16;
        self.bits -= n;
        self.bitbuf &= if self.bits == 0 { 0 } else { (1u32 << self.bits) - 1 };
        Ok(val)
    }
}

impl SqzBitRead for BitReaderMsb<'_> {
    fn ensure_bits(&mut self, need: u8) -> Result<()> {
        self.ensure_bits_internal(need)
    }

    fn get_bits_u16(&mut self, n: u8) -> Result<u16> {
        self.get_bits_u16_internal(n)
    }

    fn peek_bits(&mut self, n: u8) -> Result<u16> {
        if n > 16 || n == 0 {
            return Ok(0);
        }
        self.ensure_bits_internal(n)?;
        let shift = self.bits - n;
        Ok(((self.bitbuf >> shift) & ((1u32 << n) - 1)) as u16)
    }

    fn skip_bits(&mut self, n: u8) -> Result<()> {
        if n == 0 {
            return Ok(());
        }
        self.ensure_bits_internal(n)?;
        self.bits -= n;
        self.bitbuf &= if self.bits == 0 { 0 } else { (1u32 << self.bits) - 1 };
        Ok(())
    }

    fn debug_position(&self) -> String {
        let bit_offset = self.pos * 8 - self.bits as usize;
        format!("pos={}, bits_in_buf={}, bit_offset={}", self.pos, self.bits, bit_offset)
    }
}

#[derive(Clone, Debug)]
struct BitReaderLsb<'a> {
    data: &'a [u8],
    pos: usize,
    bitbuf: u32,
    bits: u8,
}

impl<'a> BitReaderLsb<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            pos: 0,
            bitbuf: 0,
            bits: 0,
        }
    }

    fn ensure_bits_internal(&mut self, need: u8) -> Result<()> {
        while self.bits < need {
            let byte = *self
                .data
                .get(self.pos)
                .ok_or_else(|| {
                    ArchiveError::decompression_failed("SQZ", "Unexpected EOF in bitstream")
                })?;
            self.pos += 1;
            self.bitbuf |= (byte as u32) << self.bits;
            self.bits = self.bits.saturating_add(8);
        }
        Ok(())
    }

    fn get_bits_u16_internal(&mut self, n: u8) -> Result<u16> {
        if n > 16 {
            return Err(ArchiveError::decompression_failed(
                "SQZ",
                format!("Too many bits requested: {n}"),
            ));
        }
        if n == 0 {
            return Ok(0);
        }
        self.ensure_bits_internal(n)?;
        let mask = (1u32 << n) - 1;
        let val = (self.bitbuf & mask) as u16;
        self.bitbuf >>= n;
        self.bits -= n;
        Ok(val)
    }
}

impl SqzBitRead for BitReaderLsb<'_> {
    fn ensure_bits(&mut self, need: u8) -> Result<()> {
        self.ensure_bits_internal(need)
    }

    fn get_bits_u16(&mut self, n: u8) -> Result<u16> {
        self.get_bits_u16_internal(n)
    }

    fn peek_bits(&mut self, n: u8) -> Result<u16> {
        if n > 16 || n == 0 {
            return Ok(0);
        }
        self.ensure_bits_internal(n)?;
        let mask = (1u32 << n) - 1;
        Ok((self.bitbuf & mask) as u16)
    }

    fn skip_bits(&mut self, n: u8) -> Result<()> {
        if n == 0 {
            return Ok(());
        }
        self.ensure_bits_internal(n)?;
        self.bitbuf >>= n;
        self.bits -= n;
        Ok(())
    }

    fn debug_position(&self) -> String {
        let bit_offset = self.pos * 8 - self.bits as usize;
        format!("pos={}, bits_in_buf={}, bit_offset={}", self.pos, self.bits, bit_offset)
    }
}

#[derive(Clone, Debug)]
enum HuffmanDecoder {
    Constant(u16),
    Tree {
        nodes: Vec<HuffNode>,
    },
}

#[derive(Clone, Debug)]
enum HuffNode {
    Branch { left: usize, right: usize },
    Leaf(u16),
}

impl HuffmanDecoder {
    fn from_bit_lengths(bit_lengths: &[u8]) -> Result<Self> {
        Self::from_bit_lengths_with_order(bit_lengths, SqzHuffCodeOrder::Normal)
    }

    fn from_bit_lengths_with_order(
        bit_lengths: &[u8],
        order: SqzHuffCodeOrder,
    ) -> Result<Self> {
        let mut max_len = 0u8;
        for &len in bit_lengths {
            max_len = max_len.max(len);
        }

        // All symbols unused. The original implementation can still run with a
        // constant table (symbol returned with 0 bits consumed). We represent
        // that explicitly when callers request it.
        if max_len == 0 {
            return Err(ArchiveError::decompression_failed(
                "SQZ",
                "Empty Huffman tree",
            ));
        }

        let max_len_usize = max_len as usize;
        let mut bl_count = vec![0u16; max_len_usize + 1];
        for &len in bit_lengths {
            if len != 0 {
                bl_count[len as usize] = bl_count[len as usize].saturating_add(1);
            }
        }

        let mut next_code = vec![0u32; max_len_usize + 1];
        let mut code = 0u32;
        for bits in 1..=max_len_usize {
            code = (code + bl_count[bits - 1] as u32) << 1;
            next_code[bits] = code;
        }

        // Build a binary trie.
        let mut nodes = vec![HuffNode::Branch { left: 0, right: 0 }];

        for (sym, &len) in bit_lengths.iter().enumerate() {
            if len == 0 {
                continue;
            }
            let len_usize = len as usize;
            let sym_code = next_code[len_usize];
            next_code[len_usize] += 1;

            // Debug: show codes for key symbols
            if sym == 32 || sym == 98 || sym == 271 || sym == 286 || (sym >= 256 && sym <= 260) || (sym >= 20 && sym <= 30) {
                eprintln!("[DEBUG] Huffman code: sym={} len={} code=0x{:x} binary={:0width$b}", 
                          sym, len, sym_code, sym_code, width = len_usize);
            }

            // For Normal order: iterate MSB-first (bit_idx from len-1 down to 0)
            // For Reversed order: iterate LSB-first (bit_idx from 0 up to len-1)
            let mut node_idx = 0usize;
            for step in 0..len_usize {
                let bit_idx = match order {
                    SqzHuffCodeOrder::Normal => len_usize - 1 - step, // MSB first
                    SqzHuffCodeOrder::Reversed => step,               // LSB first
                };
                let is_last = step == len_usize - 1;
                
                let bit = ((sym_code >> bit_idx) & 1) as usize;
                let (left, right) = match nodes[node_idx] {
                    HuffNode::Branch { left, right } => (left, right),
                    HuffNode::Leaf(_) => {
                        return Err(ArchiveError::decompression_failed(
                            "SQZ",
                            format!(
                                "Invalid Huffman tree (leaf on path; sym={sym} len={len} code=0x{sym_code:x})"
                            ),
                        ));
                    }
                };

                let child = if bit == 0 { left } else { right };
                if child == 0 {
                    let new_idx = nodes.len();
                    let new_node = if is_last {
                        HuffNode::Leaf(sym as u16)
                    } else {
                        HuffNode::Branch { left: 0, right: 0 }
                    };
                    nodes.push(new_node);
                    match &mut nodes[node_idx] {
                        HuffNode::Branch { left, right } => {
                            if bit == 0 {
                                *left = new_idx;
                            } else {
                                *right = new_idx;
                            }
                        }
                        HuffNode::Leaf(_) => unreachable!(),
                    }
                    node_idx = new_idx;
                } else {
                    node_idx = child;
                    if is_last {
                        // Leaf already exists; reject duplicates.
                        if matches!(nodes[node_idx], HuffNode::Leaf(_)) {
                            return Err(ArchiveError::decompression_failed(
                                "SQZ",
                                format!(
                                    "Invalid Huffman tree (duplicate code; sym={sym} len={len} code=0x{sym_code:x})"
                                ),
                            ));
                        }
                        nodes[node_idx] = HuffNode::Leaf(sym as u16);
                    }
                }
            }
        }

        Ok(HuffmanDecoder::Tree { nodes })
    }

    fn decode<R: SqzBitRead>(&self, br: &mut R) -> Result<u16> {
        match self {
            HuffmanDecoder::Constant(sym) => Ok(*sym),
            HuffmanDecoder::Tree { nodes } => {
                let mut idx = 0usize;
                let mut bits_read = Vec::new();
                loop {
                    match nodes
                        .get(idx)
                        .ok_or_else(|| {
                            ArchiveError::decompression_failed("SQZ", "Bad Huffman node")
                        })?
                    {
                        HuffNode::Leaf(sym) => {
                            if bits_read.len() <= 12 && *sym <= 300 {
                                // Only log for short codes that might be relevant
                                // eprintln!("[DEBUG] Huffman decode: bits={:?} → sym={}", bits_read, sym);
                            }
                            return Ok(*sym);
                        }
                        HuffNode::Branch { left, right } => {
                            let bit = br.get_bit()? as usize;
                            bits_read.push(bit as u8);
                            idx = if bit == 0 { *left } else { *right };
                            if idx == 0 {
                                // SQZ.EXE uses table-based decoding where unused prefixes
                                // decode to 0. This is required to parse some streams.
                                return Ok(0);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn read_pt_len<R: SqzBitRead>(
    br: &mut R,
    n_symbols: usize,
    nbit: u8,
    special: Option<usize>,
    order: SqzHuffCodeOrder,
) -> Result<HuffmanDecoder> {
    let n = br.get_bits_u16(nbit)? as usize;
    eprintln!("[DEBUG] read_pt_len: n={}, n_symbols={}, special={:?}", n, n_symbols, special);
    if n == 0 {
        let c = br.get_bits_u16(nbit)?;
        eprintln!("[DEBUG] read_pt_len: constant={}", c);
        return Ok(HuffmanDecoder::Constant(c));
    }

    let mut pt_len = vec![0u8; n_symbols];

    let mut i = 0usize;
    while i < n {
        // SQZ.EXE at 0x7D47-0x7D77:
        // 1. Peek top 3 bits (bitbuf >> 13)
        // 2. If c == 7, test bits 12, 11, 10, ... and increment c for each 1
        // 3. Consume: if c < 7 → 3 bits; if c >= 7 → (c - 3) bits
        //
        // This is different from LHA-style! The escape 1-bits are WITHIN the 
        // consumed region, not additional bits after consuming 3.
        let peek3 = br.peek_bits(3)? as u8;
        let mut c = peek3;
        if c == 7 {
            // Test bits starting at position 3 (4th bit, 0-indexed from MSB)
            // SQZ.EXE tests mask 0x1000, 0x0800, 0x0400, etc.
            // We need to peek more bits and check each one individually
            let mut extra = 0u8;
            loop {
                // Peek 4+extra bits, then check if the lowest bit is 1
                // This tests bit at position (3+extra) in the stream
                let peek_val = br.peek_bits((4 + extra) as u8)?;
                let bit = peek_val & 1;
                if bit == 0 {
                    break;
                }
                c = c.saturating_add(1);
                extra += 1;
                if c >= 16 || extra > 12 {
                    break; // safety limit - max code length
                }
            }
        }
        // Consume bits: 3 if c < 7, else (c - 3)
        let bits_to_consume = if c < 7 { 3 } else { c.saturating_sub(3) };
        br.skip_bits(bits_to_consume)?;
        if i < pt_len.len() {
            pt_len[i] = c;
        }
        i += 1;

        if let Some(special) = special {
            if i == special {
                // SQZ.EXE: getbits(2) gives zeros count (0-3), NOT zeros+1
                // The loop is: while (--zeros >= 0) { write zero; }
                let zeros = br.get_bits_u16(2)? as usize;
                for _ in 0..zeros {
                    if i >= n {
                        break;
                    }
                    if i < pt_len.len() {
                        pt_len[i] = 0;
                    }
                    i += 1;
                }
            }
        }
    }

    // Remaining lengths are already 0.
    // Debug: show PT lengths
    let nonzero_pt: Vec<(usize, u8)> = pt_len.iter().enumerate()
        .filter(|(_, &len)| len > 0)
        .map(|(i, &len)| (i, len))
        .collect();
    eprintln!("[DEBUG] read_pt_len: non-zero pt_len={:?}", nonzero_pt);
    
    HuffmanDecoder::from_bit_lengths_with_order(&pt_len, order).map_err(|e| {
        ArchiveError::decompression_failed(
            "SQZ",
            format!(
                "PT tree build failed (n={n}, n_symbols={n_symbols}, nbit={nbit}, special={special:?}, pt_len={pt_len:?}): {e}"
            ),
        )
    })
}

fn read_c_len<R: SqzBitRead>(
    br: &mut R,
    pt: &HuffmanDecoder,
    order: SqzHuffCodeOrder,
) -> Result<HuffmanDecoder> {
    const NC: usize = 0x1ff;

    let n = br.get_bits_u16(9)? as usize;
    eprintln!("[DEBUG] read_c_len: n={}", n);
    if n == 0 {
        let c = br.get_bits_u16(9)?;
        eprintln!("[DEBUG] read_c_len: constant c={}", c);
        return Ok(HuffmanDecoder::Constant(c));
    }

    let mut c_len = vec![0u8; NC];
    let mut i = 0usize;
    let mut sym_count = 0;
    while i < n {
        let sym = pt
            .decode(br)
            .map_err(|e| {
                ArchiveError::decompression_failed(
                    "SQZ",
                    format!("PT decode failed while reading C lengths (i={i}, n={n}): {e}"),
                )
            })? as u16;
        if sym <= 2 {
            // SQZ.EXE at 0x7E7A-0x7ED4:
            // sym == 0: count = 1
            // sym == 1: count = getbits(4) + 3
            // sym == 2: count = 0x14, then loop adding getbits(7) until not 0x7f
            // Loop at 0x7EC9: while (--count >= 0) write zero → writes 'count' zeros
            let run = match sym {
                0 => 1usize,
                1 => (br.get_bits_u16(4)? as usize) + 3,
                2 => {
                    let mut total = 0x14usize;
                    loop {
                        let add = br.get_bits_u16(7)? as usize;
                        total += add;
                        if add != 0x7f {
                            break;
                        }
                    }
                    total
                }
                _ => unreachable!(),
            };
            if sym_count < 10 {
                eprintln!("[DEBUG] read_c_len sym {}: PT sym={} → run {} zeros at i={}", sym_count, sym, run, i);
            }
            sym_count += 1;
            for _ in 0..run {
                if i >= n {
                    break;
                }
                c_len[i] = 0;
                i += 1;
            }
        } else {
            if sym_count < 10 {
                eprintln!("[DEBUG] read_c_len sym {}: PT sym={} → c_len[{}]={}", sym_count, sym, i, sym - 2);
            }
            sym_count += 1;
            c_len[i] = (sym - 2) as u8;
            i += 1;
        }
    }

    // Show first non-zero c_len entries
    let nonzero_entries: Vec<(usize, u8)> = c_len.iter().enumerate()
        .filter(|(_, &len)| len > 0)
        .take(20)
        .map(|(i, &len)| (i, len))
        .collect();
    eprintln!("[DEBUG] read_c_len: first 20 non-zero c_len entries: {:?}", nonzero_entries);
    
    // Show ALL c_len entries for codes >= 256
    let all_match_entries: Vec<(usize, u8)> = c_len.iter().enumerate()
        .filter(|(i, &len)| *i >= 256 && len > 0)
        .map(|(i, &len)| (i, len))
        .collect();
    eprintln!("[DEBUG] read_c_len: ALL match length c_len entries (>=256): {:?}", all_match_entries);

    HuffmanDecoder::from_bit_lengths_with_order(&c_len, order)
}

fn length_extra_bits_deflate_like(sym: u16) -> u8 {
    // Sym is assumed to be in the "final" length-code space 256..=511 where
    // length = sym - 0xFD.
    //
    // This matches Deflate-style length ranges but expressed as "sym".
    match sym {
        256..=263 => 0,
        264 | 266 | 268 | 270 => 1,
        272 | 276 | 280 | 284 => 2,
        288 | 296 | 304 | 312 => 3,
        320 | 336 | 352 | 368 => 4,
        384 | 416 | 448 | 480 => 5,
        511 => 0,
        _ => 0,
    }
}

fn decode_len_code<R: SqzBitRead>(
    br: &mut R,
    c_dec: &HuffmanDecoder,
    mapping: SqzLenMapping,
) -> Result<u16> {
    let sym = c_dec
        .decode(br)
        .map_err(|_| ArchiveError::decompression_failed("SQZ", "C decode failed"))?;
    if sym <= 0xff {
        return Ok(sym);
    }
    match mapping {
        SqzLenMapping::Identity => Ok(sym),
        SqzLenMapping::DeflateLike29 => {
            // Interpret SQZ symbols 256..=284 as Deflate length codes 257..=285.
            // Return value must stay in the SQZ "C space" where caller computes
            // length as (c - 0xFD).
            if !(256..=284).contains(&sym) {
                return Ok(sym);
            }

            const LEN_BASE: [u16; 29] = [
                3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67,
                83, 99, 115, 131, 163, 195, 227, 258,
            ];
            const LEN_EXTRA: [u8; 29] = [
                0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4,
                5, 5, 5, 5, 0,
            ];

            let idx = (sym - 256) as usize;
            let base = LEN_BASE[idx];
            let extra = LEN_EXTRA[idx];
            let add = br.get_bits_u16(extra)?;
            let length = base.saturating_add(add);
            Ok(length.saturating_add(0xFD))
        }
        SqzLenMapping::Grouped16 => {
            // Sym is 256..=511 (or lower/higher if the stream is odd); treat anything
            // outside the expected range as identity.
            if sym < 256 {
                return Ok(sym);
            }
            let idx = (sym - 256) as u16;
            let group = (idx / 16) as u8; // 0..15
            let within = (idx % 16) as u16;

            let length = if group == 0 {
                3u16.saturating_add(within)
            } else {
                // base = 3 + 16 + sum_{k=1..group-1} 16*2^k + within*2^group
                let mut base: u32 = 3 + 16;
                for k in 1..group {
                    base = base.saturating_add(16u32.saturating_mul(1u32 << k));
                }
                base = base.saturating_add((within as u32).saturating_mul(1u32 << group));
                let add = br.get_bits_u16(group)? as u32;
                base.saturating_add(add) as u16
            };

            Ok(length.saturating_add(0xFD))
        }
        SqzLenMapping::ExeTables => {
            // SQZ.EXE decode_c at 0x8025-0x803D:
            // For symbols 256-283, uses lookup tables to determine actual length.
            // 
            // Algorithm:
            //   idx = symbol - 256
            //   extra_bits = len_extra[idx]
            //   if extra_bits == 0: return symbol unchanged
            //   base = len_base[idx]
            //   add = getbits(extra_bits)
            //   return base + add + 256
            //
            // The caller then does: length = result - 0xFD (253)
            // So: length = base + add + 256 - 253 = base + add + 3
            
            if sym < 256 {
                return Ok(sym); // literal byte
            }
            let t = sqz_exe_tables()?;
            let idx = (sym - 256) as usize;
            if idx >= 28 {
                // Only 28 valid length codes (256-283)
                return Err(ArchiveError::decompression_failed(
                    "SQZ",
                    format!("SQZ.EXE len code out of range: sym={sym}"),
                ));
            }

            let extra_bits = t.len_extra[idx];
            
            // SQZ.EXE at 0x802D: if extra_bits == 0, jump to return (no transformation)
            if extra_bits == 0 {
                // Symbols 256-259: no extra bits, return symbol directly
                // Caller does: length = sym - 253, so:
                // sym=256 -> len=3, sym=257 -> len=4, etc.
                return Ok(sym);
            }

            let base = t.len_base[idx];
            let add = br.get_bits_u16(extra_bits)?;
            
            // SQZ.EXE at 0x803D: add si, 0x100 (add 256)
            let result = base + add + 256;
            
            // Debug first few
            static DEBUG_COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let count = DEBUG_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if count < 10 {
                eprintln!("[DEBUG] ExeTables: sym={}, idx={}, base={}, extra_bits={}, add={}, result={}, final_len={}", 
                          sym, idx, base, extra_bits, add, result, result.saturating_sub(253));
            }
            
            Ok(result)
        }
        SqzLenMapping::SqzNative => {
            // SQZ.EXE decode_c at 0x7F8A-0x7FA9:
            // - sym < 0x1C0 (448): no extra bits, return sym directly
            // - sym >= 0x1C0: read 1 extra bit, compute: (sym - 0x1C0) * 2 + extra + 0x1C0
            if sym < 256 {
                return Ok(sym); // literal
            }
            if sym < 0x1C0 {
                // Symbols 256-447: direct length codes
                Ok(sym)
            } else {
                // Symbols 448-510: 1 extra bit extends the range
                let extra = br.get_bits_u16(1)?;
                let adjusted = (sym - 0x1C0) * 2 + extra + 0x1C0;
                Ok(adjusted)
            }
        }
    }
}

fn decode_distance<R: SqzBitRead>(br: &mut R, p_dec: &HuffmanDecoder, mapping: SqzDistMapping) -> Result<u16> {
    let sym = p_dec
        .decode(br)
        .map_err(|_| ArchiveError::decompression_failed("SQZ", "P decode failed"))?
        as usize;

    match mapping {
        SqzDistMapping::PowerOfTwo => {
            if sym < 2 {
                return Ok(sym as u16);
            }
            let extra = (sym - 1) as u8;
            if extra > 15 {
                return Err(ArchiveError::decompression_failed(
                    "SQZ",
                    format!("Distance symbol out of range: {}", sym),
                ));
            }
            let base = 1u16
                .checked_shl((sym - 1) as u32)
                .ok_or_else(|| {
                    ArchiveError::decompression_failed("SQZ", "Bad distance base")
                })?;
            let add = br.get_bits_u16(extra)?;
            Ok(base.wrapping_add(add))
        }
        SqzDistMapping::DeflateLike => {
            // Standard Deflate distance bases/extras for a 32KiB window.
            const DIST_BASE: [u16; 30] = [
                1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769,
                1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
            ];
            const DIST_EXTRA: [u8; 30] = [
                0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10,
                11, 11, 12, 12, 13, 13,
            ];

            let idx = sym.min(DIST_BASE.len() - 1);
            let base_minus_1 = DIST_BASE[idx] - 1;
            let extra = DIST_EXTRA[idx];
            let add = br.get_bits_u16(extra)?;
            Ok(base_minus_1 + add)
        }
        SqzDistMapping::ExeTables => {
            let t = sqz_exe_tables()?;
            if sym >= t.dist_extra.len() {
                return Err(ArchiveError::decompression_failed(
                    "SQZ",
                    format!("SQZ.EXE dist code out of range: sym={sym}"),
                ));
            }
            let extra = t.dist_extra[sym];
            let base = t.dist_base[sym];
            let add = if extra > 0 {
                br.get_bits_u16(extra)?
            } else {
                0
            };
            // Distance is 1-based: base + add gives the distance
            Ok(base.saturating_add(add))
        }
    }
}

fn unsqz_method4_impl_with_reader<R: SqzBitRead>(
    mut br: R,
    original_size: usize,
    len_mapping: SqzLenMapping,
    dist_mapping: SqzDistMapping,
    huff_order: SqzHuffCodeOrder,
    win_pos_init: usize,
    length_bias: u16,
    dist_bias: usize,
) -> Result<Vec<u8>> {
    // Parameters observed from SQZ.EXE (Squeeze It 1.08.3):
    // - 32KiB ring buffer
    // - blocks with 14-bit symbol count
    // - Huffman length coding with (NT=19, NC=511, NP=31)
    const NT: usize = 0x13;
    const NP: usize = 0x1f;

    // Initialize bit-buffer (SQZ.EXE does an initial fillbuf(16)).
    br.ensure_bits(16)?;

    // SQZ.EXE window initialization (0x810D-0x812F):
    // - First: memset(window, 0, 0x8000) - fills entire 32KB with zeros
    // - Then: memset(window + 0x7FC0, 0x20, 64) - fills last 64 bytes with spaces
    // This is critical for the first match which often references the end of the window
    // to copy leading spaces in text files.
    let mut window = vec![0u8; 0x8000];
    for i in 0x7FC0..0x8000 {
        window[i] = 0x20; // Fill last 64 bytes with space character
    }
    let window_mask: usize = 0x7fff;
    let mut win_pos: usize = win_pos_init & window_mask;

    let mut out = Vec::with_capacity(original_size.max(1));

    // Block counter: SQZ.EXE uses decrement-before-test, we use check-then-decrement
    // equivalent with initial = 0 and check for <= 0 after decrement.
    let mut block_remaining: i32 = 0;
    let mut block_index: usize = 0;
    let mut c_dec = HuffmanDecoder::Constant(0);
    let mut p_dec = HuffmanDecoder::Constant(0);
    let mut symbols_decoded_in_block: usize = 0;

    while out.len() < original_size {
        // Decrement first, then check (matching SQZ.EXE's dec-then-test pattern)
        block_remaining -= 1;
        if block_remaining < 0 {
            if block_index > 0 {
                eprintln!("[DEBUG] Block {} completed: decoded {} symbols, expected {}", block_index, symbols_decoded_in_block, block_remaining + symbols_decoded_in_block as i32 + 1);
            }
            symbols_decoded_in_block = 0;
            eprintln!("[DEBUG] Before block header read: {}", br.debug_position());
            // SQZ.EXE at 0x7F13-0x7F1D: blocksize = getbits(14) - 1
            let n = br.get_bits_u16(14)?;
            block_remaining = (n as i32) - 1;
            if n == 0 {
                return Err(ArchiveError::decompression_failed("SQZ", "Invalid block size (0)"));
            }
            block_index = block_index.saturating_add(1);
            eprintln!("[DEBUG] Block {block_index}: n={n}, block_remaining={block_remaining}, out.len()={}, {}", out.len(), br.debug_position());

            let pt_for_c = read_pt_len(&mut br, NT, 5, Some(3), huff_order).map_err(|e| {
                ArchiveError::decompression_failed(
                    "SQZ",
                    format!("block#{block_index} read_pt_len(CT): {e}"),
                )
            })?;
            eprintln!("[DEBUG] After PT(C) read: {}", br.debug_position());
            c_dec = read_c_len(&mut br, &pt_for_c, huff_order).map_err(|e| {
                ArchiveError::decompression_failed("SQZ", format!("block#{block_index} read_c_len: {e}"))
            })?;
            eprintln!("[DEBUG] After C read: {}", br.debug_position());
            p_dec = read_pt_len(&mut br, NP, 5, None, huff_order).map_err(|e| {
                ArchiveError::decompression_failed("SQZ", format!("block#{block_index} read_pt_len(P): {e}"))
            })?; // special=-1
            eprintln!("[DEBUG] After P read: {}", br.debug_position());
            
            // Show next 16 bits after P read (before first decode)
            let peek16 = br.peek_bits(16)?;
            eprintln!("[DEBUG] Next 16 bits after P read: 0x{:04x} = {:016b}", peek16, peek16);
        }

        symbols_decoded_in_block += 1;
        
        // Show bits before first few decodes
        if out.len() < 3 {
            let peek16 = br.peek_bits(16)?;
            eprintln!("[DEBUG] Before decode[{}]: next 16 bits = 0x{:04x} = {:016b}", out.len(), peek16, peek16);
        }
        
        let c = decode_len_code(&mut br, &c_dec, len_mapping)?;
        
        // Track the last few symbols of block 1
        if block_index == 1 && block_remaining <= 5 {
            eprintln!("[DEBUG] Block 1, remaining={}, c={}, {}", block_remaining, c, br.debug_position());
        }
        
        // Track first few symbols
        if out.len() < 50 {
            eprintln!("[DEBUG] DECODE[{}]: c={}", out.len(), c);
        }
        
        if c <= 0xff {
            let b = c as u8;
            out.push(b);
            window[win_pos] = b;
            win_pos = (win_pos + 1) & window_mask;
        } else {
            let length = (c as usize).saturating_sub(length_bias as usize);
            
            // Debug: show bits before P decode
            if out.len() < 50 {
                let peek16 = br.peek_bits(16)?;
                eprintln!("[DEBUG] Before P decode: next 16 bits = 0x{:04x} = {:016b}", peek16, peek16);
            }
            
            let p_sym = p_dec.decode(&mut br)? as usize;
            let dist = match dist_mapping {
                SqzDistMapping::PowerOfTwo => {
                    if p_sym < 2 {
                        p_sym as u16
                    } else {
                        let extra = (p_sym - 1) as u8;
                        if extra > 15 {
                            return Err(ArchiveError::decompression_failed(
                                "SQZ",
                                format!("Distance symbol out of range: {}", p_sym),
                            ));
                        }
                        let base = 1u16.checked_shl((p_sym - 1) as u32).ok_or_else(|| {
                            ArchiveError::decompression_failed("SQZ", "Bad distance base")
                        })?;
                        let add = br.get_bits_u16(extra)?;
                        base.wrapping_add(add)
                    }
                }
                SqzDistMapping::DeflateLike => {
                    const DIST_BASE: [u16; 30] = [
                        1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769,
                        1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
                    ];
                    const DIST_EXTRA: [u8; 30] = [
                        0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10,
                        11, 11, 12, 12, 13, 13,
                    ];
                    let idx = p_sym.min(DIST_BASE.len() - 1);
                    let base_minus_1 = DIST_BASE[idx] - 1;
                    let extra = DIST_EXTRA[idx];
                    let add = br.get_bits_u16(extra)?;
                    base_minus_1 + add
                }
                SqzDistMapping::ExeTables => {
                    let t = sqz_exe_tables()?;
                    if p_sym >= t.dist_extra.len() {
                        return Err(ArchiveError::decompression_failed(
                            "SQZ",
                            format!("SQZ.EXE dist code out of range: sym={p_sym}"),
                        ));
                    }
                    let extra = t.dist_extra[p_sym];
                    let base = t.dist_base[p_sym];
                    let add = if extra > 0 {
                        br.get_bits_u16(extra)?
                    } else {
                        0
                    };
                    base.saturating_add(add)
                }
            } as usize;
            
            // Debug for first few matches
            if out.len() < 100 || (block_index == 1 && block_remaining <= 10) {
                eprintln!("[DEBUG] MATCH: c={}, length={}, p_sym={}, dist={}, out.len()={}", c, length, p_sym, dist, out.len());
            }
            
            // Show bits after match decode for first few matches
            if out.len() < 50 {
                let peek16 = br.peek_bits(16)?;
                eprintln!("[DEBUG] After match (before next decode): next 16 bits = 0x{:04x} = {:016b}", peek16, peek16);
            }
            
            let mut src = win_pos.wrapping_sub(dist + dist_bias) & window_mask;
            for _ in 0..length {
                if out.len() >= original_size {
                    break;
                }
                let b = window[src];
                out.push(b);
                window[win_pos] = b;
                win_pos = (win_pos + 1) & window_mask;
                src = (src + 1) & window_mask;
            }
        }
    }

    Ok(out)
}

fn unsqz_method4_impl(
    buf: &[u8],
    original_size: usize,
    len_mapping: SqzLenMapping,
    dist_mapping: SqzDistMapping,
    huff_order: SqzHuffCodeOrder,
    bit_order: SqzBitOrder,
    win_pos_init: usize,
    length_bias: u16,
    dist_bias: usize,
) -> Result<Vec<u8>> {
    match bit_order {
        SqzBitOrder::Msb => unsqz_method4_impl_with_reader(
            BitReaderMsb::new(buf),
            original_size,
            len_mapping,
            dist_mapping,
            huff_order,
            win_pos_init,
            length_bias,
            dist_bias,
        ),
        SqzBitOrder::Lsb => unsqz_method4_impl_with_reader(
            BitReaderLsb::new(buf),
            original_size,
            len_mapping,
            dist_mapping,
            huff_order,
            win_pos_init,
            length_bias,
            dist_bias,
        ),
    }
}

fn unsqz_method4(buf: &[u8], original_size: usize, expected_crc32: u32) -> Result<Vec<u8>> {
    // Try a small set of plausible mappings; accept only if CRC matches.
    // This mirrors the SQZ.EXE core structure while staying conservative.
    let variants = [
        // Primary: match SQZ.EXE's base/extra tables for method>=3.
        (SqzLenMapping::ExeTables, SqzDistMapping::ExeTables, SqzHuffCodeOrder::Normal, SqzBitOrder::Msb),
    ];

    fn progress_rank(msg: &str) -> u16 {
        if msg.contains("CRC mismatched") {
            return 300;
        }
        if msg.contains("Distance symbol out of range") {
            return 220;
        }
        if msg.contains("P decode failed") {
            return 210;
        }
        if msg.contains("C decode failed") {
            return 200;
        }
        if msg.contains("read_pt_len(P)") {
            return 120;
        }
        if msg.contains("read_c_len") {
            return 110;
        }
        if msg.contains("read_pt_len(CT)") {
            return 100;
        }
        50
    }

    // Prefer the most "advanced" failure, so we don't hide useful signal behind
    // an obviously-wrong variant.
    let mut best: Option<(u16, ArchiveError)> = None;
    for (len_map, dist_map, order, bit_order) in variants {
        match unsqz_method4_impl(buf, original_size, len_map, dist_map, order, bit_order, 0, 0xFD, 1) {
            Ok(out) => {
                if let Some(out) = accept_candidate(out, original_size, expected_crc32) {
                    return Ok(out);
                }
                let err = ArchiveError::decompression_failed(
                    "SQZ",
                    format!(
                        "Method4 decode produced data but CRC mismatched (len={len_map:?}, dist={dist_map:?}, order={order:?}, bits={bit_order:?})"
                    ),
                );
                best = Some((300, err));
            }
            Err(e) => {
                let err = ArchiveError::decompression_failed(
                    "SQZ",
                    format!(
                        "Method4 variant failed (len={len_map:?}, dist={dist_map:?}, order={order:?}, bits={bit_order:?}): {e}"
                    ),
                );
                let rank = progress_rank(&format!("{err}"));
                if best.as_ref().map_or(true, |(best_rank, _)| rank > *best_rank) {
                    best = Some((rank, err));
                }
            }
        }
    }

    Err(best.map(|(_, e)| e).unwrap_or_else(|| {
        ArchiveError::unsupported_method("SQZ", "Compressed(method=4)")
    }))
}

fn accept_candidate(mut out: Vec<u8>, original_size: usize, expected_crc32: u32) -> Option<Vec<u8>> {
    if out.len() < original_size {
        return None;
    }
    out.truncate(original_size);
    let actual_crc = crc32fast::hash(&out);
    if actual_crc == expected_crc32 {
        Some(out)
    } else {
        eprintln!("[DEBUG] CRC mismatch: expected=0x{:08x}, actual=0x{:08x}, len={}", 
                  expected_crc32, actual_crc, out.len());
        // Show first 100 bytes as hex
        let first_100: Vec<String> = out.iter().take(100).map(|b| format!("{:02x}", b)).collect();
        eprintln!("[DEBUG] First 100 bytes (hex): {}", first_100.join(" "));
        // Show first 100 bytes as string (lossy)
        let first_100_str = String::from_utf8_lossy(&out[..100.min(out.len())]);
        eprintln!("[DEBUG] First 100 chars: {:?}", first_100_str);
        None
    }
}

pub fn unsqz(buf: &[u8], original_size: usize, method: u8, expected_crc32: u32) -> Result<Vec<u8>> {
    // SQZ.EXE exposes methods 0..4 (0=copy, 1..4=squeeze). The exact bitstream
    // format depends on the original implementation. Until the reverse-engineering
    // is complete, we try a small set of plausible decoders and validate by size.
    //
    // This is intentionally strict: if none match (including CRC-32), we report UnsupportedMethod.

    // 0) Known SQZ method=4 ("default") decoding (reverse engineered from SQZ.EXE).
    if method == 4 {
        // Before our in-house method=4 decoder, try a few LHA raw-stream decoders.
        // Some SQZ files embed an LHA-family bitstream.
        {
            use delharc::CompressionMethod as M;

            let methods = [M::Lh1, M::Lh5, M::Lh7];
            for m in methods {
                let mut decoder = DecoderAny::new_from_compression(m, buf);
                let mut out = vec![0u8; original_size];
                if decoder.fill_buffer(&mut out).is_ok() {
                    eprintln!("[DEBUG] LHA decoder {:?} succeeded, checking CRC", m);
                    if let Some(out) = accept_candidate(out, original_size, expected_crc32) {
                        return Ok(out);
                    }
                }
            }
        }

        eprintln!("[DEBUG] Trying unsqz_method4");
        return unsqz_method4(buf, original_size, expected_crc32);
    }

    // 1) Try LHA-family raw streams (some SQZ variants embed LHA-style compression)
    {
        use delharc::CompressionMethod as M;

        // Try the common LHA methods first, then the rest.
        let methods = [
            M::Lh5,
            M::Lh6,
            M::Lh7,
            M::Lh4,
            M::Lh1,
            M::Lhx,
            M::Lzs,
            M::Lz5,
            M::Lz4,
            M::Lh0,
            M::Pm2,
            M::Pm1,
            M::Pm0,
        ];

        for method in methods {
            let mut decoder = DecoderAny::new_from_compression(method, buf);
            let mut out = vec![0u8; original_size];
            if decoder.fill_buffer(&mut out).is_ok() {
                if let Some(out) = accept_candidate(out, original_size, expected_crc32) {
                    return Ok(out);
                }
            }
        }
    }

    // 2) Try salzweg variable-width LZW (Zoo-style).
    {
        let mut out = Vec::with_capacity(original_size.max(1));
        if salzweg::decoder::VariableDecoder::decode(
            buf,
            &mut out,
            8,
            salzweg::Endianness::LittleEndian,
            CodeSizeStrategy::Default,
        )
        .is_ok()
        {
            if let Some(out) = accept_candidate(out, original_size, expected_crc32) {
                return Ok(out);
            }
        }
    }

    // 3) Try classic Unix "compress" LZW variants (9..=16 bits, optional CLEAR code).
    // Many legacy DOS tools reused this exact LZW packing.
    for max_bits in 9u8..=16u8 {
        for block_mode in [false, true] {
            let mut lzw = crate::z::lzw::Lzw::new(max_bits, block_mode);
            if let Ok(out) = lzw.decomp(buf) {
                if let Some(out) = accept_candidate(out, original_size, expected_crc32) {
                    return Ok(out);
                }
            }
        }
    }

    // 4) Try ARC-style "squeeze" (Huffman tree + RLE) as last resort.
    if let Ok(out) = crate::arc::unsqueeze::unsqueeze(buf) {
        if let Some(out) = accept_candidate(out, original_size, expected_crc32) {
            return Ok(out);
        }
    }

    Err(ArchiveError::unsupported_method(
        "SQZ",
        format!("Compressed(method={})", method),
    ))
}

/// Decode SQZ-native LZHUF-like streams using the tables embedded in `SQZ.EXE`.
///
/// This intentionally does **not** probe other unrelated codecs (LHA/LZW/ARC);
/// it only tries a small set of bit-order / Huffman-code-order variants and
/// accepts a candidate strictly by CRC-32.
pub(crate) fn unsqz_sqzexe_only(
    buf: &[u8],
    original_size: usize,
    method: u8,
    expected_crc32: u32,
) -> Result<Vec<u8>> {
    if method == 0 {
        return Ok(buf.to_vec());
    }

    // The samples we have (methods 1..=4) all appear to use the same core
    // block/Huffman framing. What differs is most likely the mapping from
    // C/P symbols to match lengths and distances.
    //
    // We keep this conservative: try only a small set of plausible mappings and
    // ordering variants, and accept strictly by CRC.
    let mapping_variants = [
        // Variants observed to successfully decode SQZ-native streams (kept strict via CRC).
        (SqzLenMapping::Grouped16, SqzDistMapping::DeflateLike),
        (SqzLenMapping::Grouped16, SqzDistMapping::PowerOfTwo),
        (SqzLenMapping::Grouped16, SqzDistMapping::ExeTables),
        (SqzLenMapping::DeflateLike29, SqzDistMapping::DeflateLike),
        (SqzLenMapping::DeflateLike29, SqzDistMapping::PowerOfTwo),
        (SqzLenMapping::DeflateLike29, SqzDistMapping::ExeTables),
        (SqzLenMapping::Identity, SqzDistMapping::DeflateLike),
        (SqzLenMapping::Identity, SqzDistMapping::PowerOfTwo),
        (SqzLenMapping::Identity, SqzDistMapping::ExeTables),

        // Use the tables from SQZ.EXE - mapping based on disassembly.
        (SqzLenMapping::ExeTables, SqzDistMapping::ExeTables),
        (SqzLenMapping::ExeTables, SqzDistMapping::DeflateLike),
        (SqzLenMapping::ExeTables, SqzDistMapping::PowerOfTwo),
        // SQZ Native length + ExeTables distance (mixed mode).
        (SqzLenMapping::SqzNative, SqzDistMapping::ExeTables),
        // SQZ Native length + alternative distance models.
        (SqzLenMapping::SqzNative, SqzDistMapping::PowerOfTwo),
        (SqzLenMapping::SqzNative, SqzDistMapping::DeflateLike),
    ];

    // Most SQZ.EXE streams appear to use canonical Huffman codes with MSB-first bit reading.
    // Some variants (observed in method 3/4 samples) seem to require interpreting the
    // canonical code bits in reversed order to match CRC.
    let order_variants: &[SqzHuffCodeOrder] = if method >= 3 {
        &[SqzHuffCodeOrder::Normal, SqzHuffCodeOrder::Reversed]
    } else {
        &[SqzHuffCodeOrder::Normal]
    };
    // Method 4 samples sometimes appear to use LSB-first bit reading.
    // Keep method 1..=3 conservative (MSB only) to avoid excessive search.
    let bit_variants: &[SqzBitOrder] = if method >= 4 {
        &[SqzBitOrder::Msb, SqzBitOrder::Lsb]
    } else {
        &[SqzBitOrder::Msb]
    };
    let win_pos_variants: &[usize] = if method >= 4 { &[0, 0x7FC0] } else { &[0] };
    let bias_variants = [
        (0xFDu16, 1usize),
        (0xFEu16, 1usize),
        (0xFDu16, 0usize),
        (0xFEu16, 0usize),
    ];

    fn rank_error(msg: &str) -> u16 {
        if msg.contains("CRC mismatched") {
            return 300;
        }
        if msg.contains("Unexpected EOF") {
            return 250;
        }
        if msg.contains("Invalid block size") {
            return 240;
        }
        if msg.contains("read_pt_len") {
            return 200;
        }
        if msg.contains("read_c_len") {
            return 190;
        }
        if msg.contains("C decode failed") {
            return 180;
        }
        if msg.contains("P decode failed") {
            return 170;
        }
        50
    }

    let mut best: Option<(u16, ArchiveError)> = None;
    for (len_map, dist_map) in mapping_variants {
        for (length_bias, dist_bias) in bias_variants {
            for &order in order_variants {
                for &bit_order in bit_variants {
                    for &win_pos_init in win_pos_variants {
                        match unsqz_method4_impl(
                            buf,
                            original_size,
                            len_map,
                            dist_map,
                            order,
                            bit_order,
                            win_pos_init,
                            length_bias,
                            dist_bias,
                        ) {
            Ok(out) => {
                if let Some(out) = accept_candidate(out, original_size, expected_crc32) {
                    return Ok(out);
                }
                let err = ArchiveError::decompression_failed(
                    "SQZ",
                    format!(
                        "SQZ-native decode produced data but CRC mismatched (method={method}, len={len_map:?}, dist={dist_map:?}, win_pos_init=0x{win_pos_init:x}, len_bias=0x{length_bias:x}, dist_bias={dist_bias}, order={order:?}, bits={bit_order:?})"
                    ),
                );
                best = Some((300, err));
            }
            Err(e) => {
                let err = ArchiveError::decompression_failed(
                    "SQZ",
                    format!(
                        "SQZ-native variant failed (method={method}, len={len_map:?}, dist={dist_map:?}, win_pos_init=0x{win_pos_init:x}, len_bias=0x{length_bias:x}, dist_bias={dist_bias}, order={order:?}, bits={bit_order:?}): {e}"
                    ),
                );
                let r = rank_error(&format!("{err}"));
                if best.as_ref().map_or(true, |(best_r, _)| r > *best_r) {
                    best = Some((r, err));
                }
            }
                        }
                    }
                }
            }
        }
    }

    Err(best
        .map(|(_, e)| e)
        .unwrap_or_else(|| ArchiveError::unsupported_method("SQZ", format!("Compressed(method={})", method))))
}

/// Like `unsqz_sqzexe_only`, but returns the first successfully decoded output
/// without checking CRC-32 (still truncates to `original_size`).
///
/// This is intended for debugging/format reverse engineering.
pub(crate) fn unsqz_sqz_only_raw(
    buf: &[u8],
    original_size: usize,
    method: u8,
) -> Result<Vec<u8>> {
    if method == 0 {
        return Ok(buf.to_vec());
    }

    fn rank_error(msg: &str) -> u16 {
        if msg.contains("Unexpected EOF") {
            return 250;
        }
        if msg.contains("Invalid block size") {
            return 240;
        }
        if msg.contains("read_pt_len") {
            return 200;
        }
        if msg.contains("read_c_len") {
            return 190;
        }
        if msg.contains("C decode failed") {
            return 180;
        }
        if msg.contains("P decode failed") {
            return 170;
        }
        50
    }

    let mapping_variants = [
        (SqzLenMapping::Grouped16, SqzDistMapping::DeflateLike),
        (SqzLenMapping::Grouped16, SqzDistMapping::PowerOfTwo),
        (SqzLenMapping::Grouped16, SqzDistMapping::ExeTables),
        (SqzLenMapping::DeflateLike29, SqzDistMapping::DeflateLike),
        (SqzLenMapping::DeflateLike29, SqzDistMapping::PowerOfTwo),
        (SqzLenMapping::DeflateLike29, SqzDistMapping::ExeTables),
        (SqzLenMapping::Identity, SqzDistMapping::DeflateLike),
        (SqzLenMapping::Identity, SqzDistMapping::PowerOfTwo),
        (SqzLenMapping::Identity, SqzDistMapping::ExeTables),
        (SqzLenMapping::ExeTables, SqzDistMapping::ExeTables),
        (SqzLenMapping::ExeTables, SqzDistMapping::DeflateLike),
        (SqzLenMapping::ExeTables, SqzDistMapping::PowerOfTwo),
        (SqzLenMapping::SqzNative, SqzDistMapping::ExeTables),
        (SqzLenMapping::SqzNative, SqzDistMapping::PowerOfTwo),
        (SqzLenMapping::SqzNative, SqzDistMapping::DeflateLike),
    ];
    // SQZ.EXE uses canonical Huffman codes with MSB-first bit reading
    let order_variants = [SqzHuffCodeOrder::Normal];
    let bit_variants: &[SqzBitOrder] = if method >= 4 {
        &[SqzBitOrder::Msb, SqzBitOrder::Lsb]
    } else {
        &[SqzBitOrder::Msb]
    };
    let win_pos_variants: &[usize] = if method >= 4 { &[0, 0x7FC0] } else { &[0] };
    let bias_variants = [(0xFDu16, 1usize), (0xFEu16, 1usize), (0xFDu16, 0usize), (0xFEu16, 0usize)];

    let mut best: Option<(u16, ArchiveError)> = None;
    for (len_map, dist_map) in mapping_variants {
        for (length_bias, dist_bias) in bias_variants {
            for order in order_variants {
                for &bit_order in bit_variants {
                    for &win_pos_init in win_pos_variants {
                        match unsqz_method4_impl(
                            buf,
                            original_size,
                            len_map,
                            dist_map,
                            order,
                            bit_order,
                            win_pos_init,
                            length_bias,
                            dist_bias,
                        ) {
                            Ok(mut out) => {
                                if out.len() > original_size {
                                    out.truncate(original_size);
                                }
                                return Ok(out);
                            }
                            Err(e) => {
                                let err = ArchiveError::decompression_failed(
                                    "SQZ",
                                    format!(
                                        "SQZ-native raw variant failed (method={method}, len={len_map:?}, dist={dist_map:?}, win_pos_init=0x{win_pos_init:x}, len_bias=0x{length_bias:x}, dist_bias={dist_bias}, order={order:?}, bits={bit_order:?}): {e}"
                                    ),
                                );
                                let r = rank_error(&format!("{err}"));
                                if best.as_ref().map_or(true, |(best_r, _)| r > *best_r) {
                                    best = Some((r, err));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Err(best
        .map(|(_, e)| e)
        .unwrap_or_else(|| ArchiveError::unsupported_method("SQZ", format!("Compressed(method={})", method))))
}
