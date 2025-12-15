//! SQZ decompression implementation (reverse-engineered from SQZ.EXE).

use std::sync::OnceLock;

use crate::error::{ArchiveError, Result};

static SQZ_TRACE_LEVEL: OnceLock<u8> = OnceLock::new();

fn sqz_trace_level() -> u8 {
    *SQZ_TRACE_LEVEL.get_or_init(|| match std::env::var("UNARC_SQZ_TRACE") {
        Ok(v) => {
            if let Ok(n) = v.parse::<u8>() {
                n
            } else if matches!(v.as_str(), "true" | "yes" | "on") {
                1
            } else {
                0
            }
        }
        Err(_) => 0,
    })
}

macro_rules! sqz_trace {
    ($level:expr, $($arg:tt)*) => {{
        if sqz_trace_level() >= ($level) {
            eprintln!($($arg)*);
        }
    }};
}

#[derive(Clone, Copy, Debug)]
enum SqzLenMapping {
    /// Deflate-style length decoding using 29 codes mapped onto SQZ symbols
    /// 256..=284 (i.e. Deflate 257..=285).
    DeflateLike29,
    /// SQZ native: symbols 256-447 are direct lengths, 448+ have 1 extra bit
    SqzNative,
}

#[derive(Clone, Copy, Debug)]
enum SqzDistMapping {
    PowerOfTwo,
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
        0, 0, 0, 0, // codes 0-3 (sym 256-259): 0 extra bits -> len 3-6
        1, 1, 1, 1, // codes 4-7 (sym 260-263): 1 extra bit
        2, 2, 2, 2, // codes 8-11 (sym 264-267): 2 extra bits
        3, 3, 3, 3, // codes 12-15 (sym 268-271): 3 extra bits
        4, 4, 4, 4, // codes 16-19 (sym 272-275): 4 extra bits
        5, 5, 5, 5, // codes 20-23 (sym 276-279): 5 extra bits
        6, 6, 6, 6, // codes 24-27 (sym 280-283): 6 extra bits
        0, 0, 0, 0, // codes 28-31: unused/padding
    ];

    // Length base values - SQZ uses different progression than Deflate
    // Extracted from SQZ.EXE offset 0x10c98 (28 words = 56 bytes)
    // These are the base lengths before adding extra bits
    let len_base: [u16; 32] = [
        3, 4, 5, 6, // with 0 extra bits: lengths 3-6
        7, 8, 10, 12, // with 1 extra bit: lengths 7-8, 10-11, 12-13
        14, 16, 20, 24, // with 2 extra bits
        28, 32, 40, 48, // with 3 extra bits
        56, 64, 80, 96, // with 4 extra bits
        112, 128, 160, 192, // with 5 extra bits
        224, 256, 320, 384, // with 6 extra bits
        0, 0, 0, 0, // unused/padding
    ];

    // Distance extra bits (32 entries) - SQZ variant
    // Extracted from SQZ.EXE offset 0x10d34 (32 bytes)
    // Note: First 5 entries (codes 0-4) have 0 extra bits, then progression starts
    let dist_extra: [u8; 0x20] = [
        0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12,
        12, 13, 13, 0,
    ];

    // Distance base values - SQZ uses different progression than Deflate
    // Extracted from SQZ.EXE offset 0x10cf4 (32 words = 64 bytes)
    // Note: These differ from Deflate! E.g. code 5 → base 5, not 7
    let dist_base: [u16; 0x20] = [
        0, 1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025,
        1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577, 32768,
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
enum SqzPtLenMode {
    PeekSkip,
    Sequential,
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
    fn debug_bitbuf(&self) -> u32;
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
            let byte = *self.data.get(self.pos).ok_or_else(|| {
                ArchiveError::decompression_failed("SQZ", "Unexpected EOF in bitstream")
            })?;
            // Ultra-verbose debug for block boundary analysis
            if self.pos >= 16566 && self.pos <= 16572 {
                sqz_trace!(2, "[DEBUG] ensure_bits: loading byte at pos={}: 0x{:02x}, bitbuf before=0x{:08x}, bits={}", 
                    self.pos, byte, self.bitbuf, self.bits);
            }
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
        self.bitbuf &= if self.bits == 0 {
            0
        } else {
            (1u32 << self.bits) - 1
        };
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
        self.bitbuf &= if self.bits == 0 {
            0
        } else {
            (1u32 << self.bits) - 1
        };
        Ok(())
    }

    fn debug_position(&self) -> String {
        let bit_offset = self.pos * 8 - self.bits as usize;
        format!(
            "pos={}, bits_in_buf={}, bit_offset={}",
            self.pos, self.bits, bit_offset
        )
    }

    fn debug_bitbuf(&self) -> u32 {
        self.bitbuf
    }
}

#[derive(Clone, Debug)]
enum HuffmanDecoder {
    Constant(u16),
    Tree { nodes: Vec<HuffNode> },
}

#[derive(Clone, Debug)]
enum HuffNode {
    Branch { left: usize, right: usize },
    Leaf(u16),
}

impl HuffmanDecoder {
    fn from_bit_lengths(bit_lengths: &[u8]) -> Result<Self> {
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
            if sym == 32
                || sym == 98
                || sym == 271
                || sym == 286
                || (sym >= 256 && sym <= 260)
                || (sym >= 20 && sym <= 30)
            {
                sqz_trace!(
                    2,
                    "[DEBUG] Huffman code: sym={} len={} code=0x{:x} binary={:0width$b}",
                    sym,
                    len,
                    sym_code,
                    sym_code,
                    width = len_usize
                );
            }

            // Normal order: iterate MSB-first (bit_idx from len-1 down to 0)
            let mut node_idx = 0usize;
            for step in 0..len_usize {
                let bit_idx = len_usize - 1 - step; // MSB first
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
                    match nodes.get(idx).ok_or_else(|| {
                        ArchiveError::decompression_failed("SQZ", "Bad Huffman node")
                    })? {
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
    mode: SqzPtLenMode,
) -> Result<HuffmanDecoder> {
    let n = br.get_bits_u16(nbit)? as usize;
    sqz_trace!(
        2,
        "[DEBUG] read_pt_len: n={}, n_symbols={}, special={:?}",
        n,
        n_symbols,
        special
    );
    if n == 0 {
        let c = br.get_bits_u16(nbit)?;
        sqz_trace!(2, "[DEBUG] read_pt_len: constant={}", c);
        return Ok(HuffmanDecoder::Constant(c));
    }

    let mut pt_len = vec![0u8; n_symbols];

    let mut i = 0usize;
    while i < n {
        let mut c: u8;
        // Debug first few PT length reads for block 2
        let debug_this = i < 5;
        if debug_this {
            let peek16 = br.peek_bits(16)?;
            sqz_trace!(
                2,
                "[DEBUG] read_pt_len i={}: before read, next 16 bits = 0x{:04x} = {:016b}",
                i,
                peek16,
                peek16
            );
        }
        match mode {
            SqzPtLenMode::PeekSkip => {
                // SQZ.EXE at 0x7D47-0x7D77:
                // Peek top 3 bits. If all are 1 (c==7), then continue to peek additional bits.
                // Each additional 1-bit increments c; stop at the first 0-bit.
                // Consume: 3 bits if c < 7, else (3 + num_extra + 1) bits.
                let peek3 = br.peek_bits(3)? as u8;
                if peek3 == 7 {
                    c = 7;
                    let mut extra = 0u8;
                    loop {
                        // Peek (4 + extra) bits total, extract the last bit
                        let peek_extended = br.peek_bits(4 + extra)?;
                        let bit = (peek_extended & 1) as u8;
                        if bit == 0 {
                            // Terminating 0 bit found
                            br.skip_bits(3 + extra + 1)?;
                            break;
                        }
                        c = c.saturating_add(1);
                        extra += 1;
                        if c >= 16 || extra > 12 {
                            br.skip_bits(3 + extra)?;
                            break;
                        }
                    }
                } else {
                    c = peek3;
                    br.skip_bits(3)?;
                }
                if debug_this {
                    sqz_trace!(2, "[DEBUG] read_pt_len i={}: peek3={}, c={}", i, peek3, c);
                }
            }
            SqzPtLenMode::Sequential => {
                // Alternative, reference-style decoding (LHA/LZHUF-like):
                // c = getbits(3); if c==7 { while(getbit()==1) c++; getbit()==0 terminates }
                c = br.get_bits_u16(3)? as u8;
                if c == 7 {
                    loop {
                        let b = br.get_bit()? as u8;
                        if b == 0 {
                            break;
                        }
                        c = c.saturating_add(1);
                        if c >= 16 {
                            break;
                        }
                    }
                }
            }
        }
        if i < pt_len.len() {
            pt_len[i] = c;
        }
        i += 1;

        if let Some(special) = special {
            if i == special {
                let peek_before_zerorun = br.peek_bits(16)?;
                sqz_trace!(2,
                    "[DEBUG] read_pt_len: at i={}, special found, reading zero-run. Next 16 bits = 0x{:04x} = {:016b}",
                    i - 1, peek_before_zerorun, peek_before_zerorun
                );
                // SQZ.EXE: getbits(2) gives zeros count (0-3), NOT zeros+1
                // The loop is: while (--zeros >= 0) { write zero; }
                let zeros = br.get_bits_u16(2)? as usize;
                sqz_trace!(2, "[DEBUG] read_pt_len: zeros={}, i={}, n={}", zeros, i, n);
                for _ in 0..zeros {
                    if i >= n {
                        sqz_trace!(2, "[DEBUG] read_pt_len: breaking early at i={}, n={}", i, n);
                        break;
                    }
                    if i < pt_len.len() {
                        pt_len[i] = 0;
                    }
                    i += 1;
                }
                sqz_trace!(2, "[DEBUG] read_pt_len: after zero-run, i={}", i);
            }
        }
    }

    // Remaining lengths are already 0.
    // Debug: show PT lengths
    let nonzero_pt: Vec<(usize, u8)> = pt_len
        .iter()
        .enumerate()
        .filter(|(_, &len)| len > 0)
        .map(|(i, &len)| (i, len))
        .collect();
    sqz_trace!(2, "[DEBUG] read_pt_len: non-zero pt_len={:?}", nonzero_pt);

    HuffmanDecoder::from_bit_lengths(&pt_len).map_err(|e| {
        ArchiveError::decompression_failed(
            "SQZ",
            format!(
                "PT tree build failed (n={n}, n_symbols={n_symbols}, nbit={nbit}, special={special:?}, pt_len={pt_len:?}): {e}"
            ),
        )
    })
}

fn read_c_len<R: SqzBitRead>(br: &mut R, pt: &HuffmanDecoder) -> Result<HuffmanDecoder> {
    const NC: usize = 0x1ff;

    let n = br.get_bits_u16(9)? as usize;
    sqz_trace!(2, "[DEBUG] read_c_len: n={}", n);
    if n == 0 {
        let c = br.get_bits_u16(9)?;
        sqz_trace!(2, "[DEBUG] read_c_len: constant c={}", c);
        return Ok(HuffmanDecoder::Constant(c));
    }

    let mut c_len = vec![0u8; NC];
    let mut i = 0usize;
    let mut sym_count = 0;
    while i < n {
        let sym = pt.decode(br).map_err(|e| {
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
                sqz_trace!(
                    2,
                    "[DEBUG] read_c_len sym {}: PT sym={} → run {} zeros at i={}",
                    sym_count,
                    sym,
                    run,
                    i
                );
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
                sqz_trace!(
                    2,
                    "[DEBUG] read_c_len sym {}: PT sym={} → c_len[{}]={}",
                    sym_count,
                    sym,
                    i,
                    sym - 2
                );
            }
            sym_count += 1;
            c_len[i] = (sym - 2) as u8;
            i += 1;
        }
    }

    // Show first non-zero c_len entries
    let nonzero_entries: Vec<(usize, u8)> = c_len
        .iter()
        .enumerate()
        .filter(|(_, &len)| len > 0)
        .take(20)
        .map(|(i, &len)| (i, len))
        .collect();
    sqz_trace!(
        2,
        "[DEBUG] read_c_len: first 20 non-zero c_len entries: {:?}",
        nonzero_entries
    );

    // Show ALL c_len entries for codes >= 256
    let all_match_entries: Vec<(usize, u8)> = c_len
        .iter()
        .enumerate()
        .filter(|(i, &len)| *i >= 256 && len > 0)
        .map(|(i, &len)| (i, len))
        .collect();
    sqz_trace!(
        2,
        "[DEBUG] read_c_len: ALL match length c_len entries (>=256): {:?}",
        all_match_entries
    );

    HuffmanDecoder::from_bit_lengths(&c_len)
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
        SqzLenMapping::DeflateLike29 => {
            // Interpret SQZ symbols 256..=284 as Deflate length codes 257..=285.
            // Return value must stay in the SQZ "C space" where caller computes
            // length as (c - 0xFD).
            if !(256..=284).contains(&sym) {
                return Ok(sym);
            }

            const LEN_BASE: [u16; 29] = [
                3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83,
                99, 115, 131, 163, 195, 227, 258,
            ];
            const LEN_EXTRA: [u8; 29] = [
                0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5,
                0,
            ];

            let idx = (sym - 256) as usize;
            let base = LEN_BASE[idx];
            let extra = LEN_EXTRA[idx];
            let add = br.get_bits_u16(extra)?;
            let length = base.saturating_add(add);
            Ok(length.saturating_add(0xFD))
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

fn unsqz_method4_impl_with_reader<R: SqzBitRead>(
    mut br: R,
    original_size: usize,
    len_mapping: SqzLenMapping,
    dist_mapping: SqzDistMapping,
    pt_len_mode: SqzPtLenMode,
    win_pos_init: usize,
    length_bias: u16,
    dist_bias: usize,
) -> Result<Vec<u8>> {
    sqz_trace!(2, "[DEBUG] VARIANT: len={:?}, dist={:?}, pt_len={:?}, win_pos=0x{:x}, len_bias=0x{:x}, dist_bias={}",
        len_mapping, dist_mapping, pt_len_mode, win_pos_init, length_bias, dist_bias);
    // Parameters observed from SQZ.EXE (Squeeze It 1.08.3):
    // - 32KiB ring buffer
    // - blocks with 14-bit symbol count
    // - Huffman length coding with (NT=19, NC=511, NP=31)
    const NT: usize = 0x20;
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
                sqz_trace!(
                    2,
                    "[DEBUG] Block {} completed: decoded {} symbols, expected {}",
                    block_index,
                    symbols_decoded_in_block,
                    block_remaining + symbols_decoded_in_block as i32 + 1
                );
            }
            symbols_decoded_in_block = 0;
            // Debug: show exact bitbuf state before block header read
            sqz_trace!(
                2,
                "[DEBUG] Before block header read: {} bitbuf=0x{:08x}",
                br.debug_position(),
                br.debug_bitbuf()
            );
            // SQZ.EXE at 0x7F13-0x7F1D: blocksize = getbits(14) - 1
            let n = br.get_bits_u16(14)?;
            sqz_trace!(
                2,
                "[DEBUG] Read block size n={} (0x{:04x}), now: {} bitbuf=0x{:08x}",
                n,
                n,
                br.debug_position(),
                br.debug_bitbuf()
            );
            block_remaining = (n as i32) - 1;
            if n == 0 {
                return Err(ArchiveError::decompression_failed(
                    "SQZ",
                    "Invalid block size (0)",
                ));
            }
            block_index = block_index.saturating_add(1);
            sqz_trace!(2, "[DEBUG] Block {block_index}: n={n}, block_remaining={block_remaining}, out.len()={}, {}", out.len(), br.debug_position());

            let pt_for_c = read_pt_len(&mut br, NT, 5, Some(3), pt_len_mode).map_err(|e| {
                ArchiveError::decompression_failed(
                    "SQZ",
                    format!("block#{block_index} read_pt_len(CT): {e}"),
                )
            })?;
            sqz_trace!(2, "[DEBUG] After PT(C) read: {}", br.debug_position());
            c_dec = read_c_len(&mut br, &pt_for_c).map_err(|e| {
                ArchiveError::decompression_failed(
                    "SQZ",
                    format!("block#{block_index} read_c_len: {e}"),
                )
            })?;
            sqz_trace!(2, "[DEBUG] After C read: {}", br.debug_position());
            p_dec = read_pt_len(&mut br, NP, 5, None, pt_len_mode).map_err(|e| {
                ArchiveError::decompression_failed(
                    "SQZ",
                    format!("block#{block_index} read_pt_len(P): {e}"),
                )
            })?; // special=-1
            sqz_trace!(2, "[DEBUG] After P read: {}", br.debug_position());

            // Show next 16 bits after P read (before first decode)
            let peek16 = br.peek_bits(16)?;
            sqz_trace!(
                2,
                "[DEBUG] Next 16 bits after P read: 0x{:04x} = {:016b}",
                peek16,
                peek16
            );
        }

        symbols_decoded_in_block += 1;

        // Show bits before first few decodes
        if out.len() < 3 {
            let peek16 = br.peek_bits(16)?;
            sqz_trace!(
                2,
                "[DEBUG] Before decode[{}]: next 16 bits = 0x{:04x} = {:016b}",
                out.len(),
                peek16,
                peek16
            );
        }

        let c = match decode_len_code(&mut br, &c_dec, len_mapping) {
            Ok(c) => c,
            Err(e) => {
                sqz_trace!(1, "[DEBUG] decode_len_code FAILED at block {block_index}, out.len()={}, symbols_decoded={}, {}: {e}", out.len(), symbols_decoded_in_block, br.debug_position());
                return Err(e);
            }
        };

        // Track the last few symbols of block 1
        if block_index == 1 && block_remaining <= 5 {
            sqz_trace!(
                2,
                "[DEBUG] Block 1, remaining={}, c={}, {}",
                block_remaining,
                c,
                br.debug_position()
            );
        }

        // Track first few symbols
        if out.len() < 50 {
            sqz_trace!(2, "[DEBUG] DECODE[{}]: c={}", out.len(), c);
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
                sqz_trace!(
                    2,
                    "[DEBUG] Before P decode: next 16 bits = 0x{:04x} = {:016b}",
                    peek16,
                    peek16
                );
            }

            let p_sym = match p_dec.decode(&mut br) {
                Ok(s) => s as usize,
                Err(e) => {
                    sqz_trace!(1, "[DEBUG] p_dec.decode FAILED at block {block_index}, out.len()={}, symbols_decoded={}, {}: {e}", out.len(), symbols_decoded_in_block, br.debug_position());
                    return Err(e);
                }
            };
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
                sqz_trace!(
                    2,
                    "[DEBUG] MATCH: c={}, length={}, p_sym={}, dist={}, out.len()={}",
                    c,
                    length,
                    p_sym,
                    dist,
                    out.len()
                );
            }

            // Show bits after match decode for first few matches
            if out.len() < 50 {
                let peek16 = br.peek_bits(16)?;
                sqz_trace!(
                    2,
                    "[DEBUG] After match (before next decode): next 16 bits = 0x{:04x} = {:016b}",
                    peek16,
                    peek16
                );
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
    pt_len_mode: SqzPtLenMode,
    win_pos_init: usize,
    length_bias: u16,
    dist_bias: usize,
) -> Result<Vec<u8>> {
    unsqz_method4_impl_with_reader(
        BitReaderMsb::new(buf),
        original_size,
        len_mapping,
        dist_mapping,
        pt_len_mode,
        win_pos_init,
        length_bias,
        dist_bias,
    )
}

/// Decode SQZ-native LZHUF-like streams using the tables embedded in `SQZ.EXE`.
///
/// Each compression method (1-4) uses a specific combination of length/distance
/// mappings determined by analysis of working archives.
pub(crate) fn unsqz_compressed(
    buf: &[u8],
    original_size: usize,
    method: u8,
    expected_crc32: u32,
) -> Result<Vec<u8>> {
    if method == 0 {
        return Ok(buf.to_vec());
    }

    // Each method uses specific, known mappings (determined by testing):
    // Method 1: SqzNative + PowerOfTwo, PeekSkip
    // Method 2: SqzNative + ExeTables, PeekSkip
    // Method 3: DeflateLike29 + PowerOfTwo, Sequential
    // Method 4: DeflateLike29 + ExeTables, Sequential
    let (len_mapping, dist_mapping, pt_len_mode) = match method {
        1 => (
            SqzLenMapping::SqzNative,
            SqzDistMapping::PowerOfTwo,
            SqzPtLenMode::PeekSkip,
        ),
        2 => (
            SqzLenMapping::SqzNative,
            SqzDistMapping::ExeTables,
            SqzPtLenMode::PeekSkip,
        ),
        3 => (
            SqzLenMapping::DeflateLike29,
            SqzDistMapping::PowerOfTwo,
            SqzPtLenMode::Sequential,
        ),
        4 => (
            SqzLenMapping::DeflateLike29,
            SqzDistMapping::ExeTables,
            SqzPtLenMode::Sequential,
        ),
        _ => {
            return Err(ArchiveError::unsupported_method(
                "SQZ",
                format!("Compressed(method={})", method),
            ));
        }
    };

    // Common parameters for all methods
    let win_pos_init = 0usize;
    let length_bias = 0xFDu16;
    let dist_bias = 1usize;

    let mut out = unsqz_method4_impl(
        buf,
        original_size,
        len_mapping,
        dist_mapping,
        pt_len_mode,
        win_pos_init,
        length_bias,
        dist_bias,
    )?;

    if out.len() < original_size {
        return Err(ArchiveError::decompression_failed(
            "SQZ",
            format!(
                "SQZ decode produced too-short output: {} < expected {}",
                out.len(),
                original_size
            ),
        ));
    }

    out.truncate(original_size);
    let actual_crc = crc32fast::hash(&out);
    if actual_crc != expected_crc32 {
        return Err(ArchiveError::crc_mismatch(
            "SQZ",
            expected_crc32,
            actual_crc,
        ));
    }

    Ok(out)
}
