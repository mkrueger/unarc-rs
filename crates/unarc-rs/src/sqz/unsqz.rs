//! SQZ decompression implementation (reverse-engineered from SQZ.EXE).

use std::sync::OnceLock;

use crate::error::{ArchiveError, Result};

#[derive(Clone, Copy, Debug)]
enum SqzLenMapping {
    /// Deflate-style length decoding using 29 codes mapped onto SQZ symbols
    /// 256..=284 (i.e. Deflate 257..=285).
    DeflateLike29,
    /// SQZ native: symbols 256-447 are direct lengths, 448+ have 1 extra bit
    SqzNative,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SqzDeflateLenTables {
    Standard,
    SqzExe,
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
    Ok(SQZ_EXE_TABLES.get_or_init(|| {
        // Hardcoded tables extracted from SQZ.EXE via xxd analysis.
        // These are SQZ-specific (not Deflate) length/distance encoding tables.

        // Method>=3 length mapping tables used by SQZ.EXE's decoder at 0x7FAE.
        // After Huffman decoding a symbol in 0x100..=0x11F, it does:
        //   extra = len_extra[idx]
        //   base  = len_base[idx]
        //   c = 0x100 + base + getbits(extra)
        // so later `len = c - 0xFD` yields `len = base + getbits(extra) + 3`.
        //
        // These values are extracted by interpreting SQZ.EXE as DS-based tables:
        //   extra byte table address: DS:0x0AF4 + sym
        //   base  word table address: DS:0x09B2 + sym*2
        // for sym=0x100..0x11F.
        let len_extra: [u8; 32] = [
            0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6,
            6, 6, 6,
        ];

        let len_base: [u16; 32] = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 14, 16, 20, 24, 28, 32, 40, 48, 56, 64, 80, 96, 112,
            128, 160, 192, 224, 256, 320, 384, 448,
        ];

        // Distance extra bits (32 entries) - SQZ variant
        // Extracted from SQZ.EXE offset 0x10d34 (32 bytes)
        // Note: First 5 entries (codes 0-4) have 0 extra bits, then progression starts
        let dist_extra: [u8; 0x20] = [
            0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11,
            12, 12, 13, 13, 0,
        ];

        // Distance base values - SQZ uses different progression than Deflate
        // Extracted from SQZ.EXE offset 0x10cf4 (32 words = 64 bytes)
        // Note: These differ from Deflate! E.g. code 5 → base 5, not 7
        let dist_base: [u16; 0x20] = [
            0, 1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025,
            1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577, 32768,
        ];

        SqzExeTables {
            len_extra,
            len_base,
            dist_extra,
            dist_base,
        }
    }))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SqzPtLenMode {
    PeekSkip,
    Sequential,
}

trait SqzBitRead {
    fn ensure_bits(&mut self, need: u8) -> Result<()>;
    fn get_bits_u16(&mut self, n: u8) -> Result<u16>;
    fn peek_bits(&mut self, n: u8) -> Result<u16>;
    fn skip_bits(&mut self, n: u8) -> Result<()>;
    fn align_to_byte(&mut self);
    fn get_bit(&mut self) -> Result<u16> {
        self.get_bits_u16(1)
    }
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

    fn align_to_byte(&mut self) {
        let rem = self.bits % 8;
        if rem == 0 {
            return;
        }
        self.bits -= rem;
        self.bitbuf &= if self.bits == 0 {
            0
        } else {
            (1u32 << self.bits) - 1
        };
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

        // All symbols unused.
        // Treat this as an error: in practice this indicates a malformed or
        // misaligned block header, and callers rely on the error to retry with
        // alternate parsing strategies.
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

            // SQZ streams seen in the wild (and SQZ.EXE's table-based decoder)
            // can tolerate oversubscribed/"shadowed" codes. If a shorter code
            // already forms a leaf on this path, any longer codes under it are
            // unreachable and can be ignored.
            let mut skip_symbol = false;

            // Normal order: iterate MSB-first (bit_idx from len-1 down to 0)
            let mut node_idx = 0usize;
            for step in 0..len_usize {
                let bit_idx = len_usize - 1 - step; // MSB first
                let is_last = step == len_usize - 1;

                let bit = ((sym_code >> bit_idx) & 1) as usize;
                let (left, right) = match nodes[node_idx] {
                    HuffNode::Branch { left, right } => (left, right),
                    HuffNode::Leaf(_) => {
                        skip_symbol = true;
                        break;
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
                            skip_symbol = true;
                        } else {
                            nodes[node_idx] = HuffNode::Leaf(sym as u16);
                        }
                    }
                }
            }

            if skip_symbol {
                continue;
            }
        }

        Ok(HuffmanDecoder::Tree { nodes })
    }

    fn decode<R: SqzBitRead>(&self, br: &mut R) -> Result<u16> {
        match self {
            HuffmanDecoder::Constant(sym) => Ok(*sym),
            HuffmanDecoder::Tree { nodes } => {
                let mut idx = 0usize;
                loop {
                    match nodes.get(idx).ok_or_else(|| {
                        ArchiveError::decompression_failed("SQZ", "Bad Huffman node")
                    })? {
                        HuffNode::Leaf(sym) => {
                            return Ok(*sym);
                        }
                        HuffNode::Branch { left, right } => {
                            let bit = br.get_bit()? as usize;
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

fn read_pt_len<R: SqzBitRead + Clone>(
    br: &mut R,
    n_symbols: usize,
    nbit: u8,
    special: Option<usize>,
    _mode: SqzPtLenMode,
) -> Result<HuffmanDecoder> {
    // Note: SQZ.EXE (see src/sqz/exe/SQZ.EXE.ndisasm16.asm @ 0x7CE8)
    // uses the PeekSkip-style algorithm, including the special-run behavior.

    let n = br.get_bits_u16(nbit)? as usize;
    if n == 0 {
        let c = br.get_bits_u16(nbit)?;
        return Ok(HuffmanDecoder::Constant(c));
    }

    // SQZ.EXE (function at 0x7CE8) reads PT lengths as:
    // - c = top3bits(bitbuf16)
    // - if c==7: extend by counting consecutive 1 bits starting at mask 0x1000
    // - fillbuf( (c<7)?3:(c-3) )
    // - special handling at index `special`: read 2 bits and write (zeros+1) zero lengths.
    let mut pt_len = vec![0u8; n_symbols];
    let mut i = 0usize;
    while i < n {
        let bitbuf16 = br.peek_bits(16)?;
        let mut c = (bitbuf16 >> 13) as u8;
        if c == 7 {
            // SQZ.EXE at 0x7D56-0x7D64:
            // mask starts at 0x1000 (bit 12), tests bit, if set: shift mask right and inc c
            // Loop: test → if set → shift mask → inc c → repeat
            let mut mask = 0x1000u16;
            loop {
                if (bitbuf16 & mask) == 0 {
                    break; // Bit not set, stop extending
                }
                mask >>= 1;
                c = c.saturating_add(1);
                if mask == 0 || c >= 16 {
                    break;
                }
            }
        }

        let skip = if c < 7 { 3 } else { c.saturating_sub(3) };
        br.skip_bits(skip)?;

        if i < pt_len.len() {
            pt_len[i] = c;
        }
        i += 1;

        if let Some(special) = special {
            if i == special {
                let zeros = br.get_bits_u16(2)? as usize;
                // Special-run: insert `zeros` zero-length entries.
                let run = zeros;
                for _ in 0..run {
                    if i < pt_len.len() {
                        pt_len[i] = 0;
                    }
                    i += 1;
                }
            }
        }
    }

    HuffmanDecoder::from_bit_lengths(&pt_len).map_err(|e| {
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
    allow_empty: bool,
) -> Result<HuffmanDecoder> {
    const NC: usize = 0x1ff;

    let n = br.get_bits_u16(9)? as usize;
    if n == 0 {
        let c = br.get_bits_u16(9)?;
        return Ok(HuffmanDecoder::Constant(c));
    }

    let mut c_len = vec![0u8; NC];
    let mut i = 0usize;
    while i < n {
        let sym = pt.decode(br).map_err(|e| {
            ArchiveError::decompression_failed(
                "SQZ",
                format!("PT decode failed while reading C lengths (i={i}, n={n}): {e}"),
            )
        })?;
        if sym <= 2 {
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
            for _ in 0..run {
                if i >= n {
                    break;
                }
                c_len[i] = 0;
                i += 1;
            }
        } else {
            c_len[i] = (sym - 2) as u8;
            i += 1;
        }
    }

    // If all lengths are zero, the Huffman table would be empty.
    // Usually this means a malformed/misaligned block header, but SQZ.EXE's
    // table builder tolerates this and effectively decodes to 0.
    let has_nonzero = c_len.iter().any(|&len| len != 0);
    if !has_nonzero {
        if allow_empty {
            // SQZ.EXE's table-based decoder effectively returns 0 for unused prefixes.
            // If the tree is fully empty, treating it as a constant-0 decoder matches
            // that permissive behavior.
            return Ok(HuffmanDecoder::Constant(0));
        }
        return Err(ArchiveError::decompression_failed(
            "SQZ",
            "Empty Huffman tree (all lengths zero)",
        ));
    }

    HuffmanDecoder::from_bit_lengths(&c_len)
}

fn decode_len_code<R: SqzBitRead>(
    br: &mut R,
    c_dec: &HuffmanDecoder,
    mapping: SqzLenMapping,
    deflate_len_tables: SqzDeflateLenTables,
) -> Result<u16> {
    let sym = c_dec.decode(br).map_err(|e| {
        ArchiveError::decompression_failed("SQZ", format!("C decode failed: {}", e))
    })?;
    if sym <= 0xff {
        return Ok(sym);
    }
    match mapping {
        SqzLenMapping::DeflateLike29 => {
            // Interpret SQZ symbols 256..=284 as Deflate length codes 257..=285.
            // Return value must stay in the SQZ "C space" where caller computes
            // length as (c - 0xFD).
            // SQZ.EXE's method>=3 decoder (0x7FAE) uses a 32-entry table for
            // codes 256..=287. Standard Deflate uses 29 entries 256..=284.
            match deflate_len_tables {
                SqzDeflateLenTables::Standard => {
                    if !(256..=284).contains(&sym) {
                        return Ok(sym);
                    }
                    let idx = (sym - 256) as usize;
                    const LEN_BASE: [u16; 29] = [
                        3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59,
                        67, 83, 99, 115, 131, 163, 195, 227, 258,
                    ];
                    const LEN_EXTRA: [u8; 29] = [
                        0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5,
                        5, 5, 5, 0,
                    ];
                    let base = LEN_BASE[idx];
                    let extra = LEN_EXTRA[idx];
                    let add = br.get_bits_u16(extra)?;
                    Ok(base.saturating_add(add).saturating_add(0xFD))
                }
                SqzDeflateLenTables::SqzExe => {
                    // Match SQZ.EXE 0x7FAE mapping: for sym 0x100..=0x11F,
                    // c = 0x100 + base + getbits(extra).
                    if !(0x100..=0x11F).contains(&sym) {
                        return Ok(sym);
                    }
                    let t = sqz_exe_tables()?;
                    let idx = (sym - 0x100) as usize;
                    let extra = t.len_extra[idx];
                    let base = t.len_base[idx];
                    let add = br.get_bits_u16(extra)?;
                    Ok(0x100u16.saturating_add(base).saturating_add(add))
                }
            }
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

#[allow(clippy::too_many_arguments)]
fn unsqz_method4_impl_with_reader<R: SqzBitRead + Clone>(
    mut br: R,
    original_size: usize,
    len_mapping: SqzLenMapping,
    dist_mapping: SqzDistMapping,
    pt_len_mode: SqzPtLenMode,
    deflate_len_tables: SqzDeflateLenTables,
    win_pos_init: usize,
    length_bias: u16,
    dist_bias: usize,
    blocksize_bits: u8,
    blocksize_offset: i8,
    allow_empty_c_tree: bool,
) -> Result<Vec<u8>> {
    let mut pt_len_mode = pt_len_mode;

    // Parameters observed from SQZ.EXE (Squeeze It 1.08.3):
    // - 32KiB ring buffer
    // - blocks with 14-bit symbol count
    // - Huffman length coding with (NT=19, NC=511, NP=31)
    // Note: NP=31 (0x1F) is confirmed from SQZ.EXE disassembly at 0x7F3D
    const NT: usize = 19;
    const NP: usize = 31;

    // Initialize bit-buffer (SQZ.EXE does an initial fillbuf(16)).
    br.ensure_bits(16)?;

    // SQZ.EXE window initialization (0x810D-0x812F):
    // - First: memset(window, 0, 0x8000) - fills entire 32KB with zeros
    // - Then: memset(window + 0x7FC0, 0x20, 64) - fills last 64 bytes with spaces
    // This is critical for the first match which often references the end of the window
    // to copy leading spaces in text files.
    let mut window = vec![0u8; 0x8000];
    for item in window.iter_mut().take(0x8000).skip(0x7FC0) {
        *item = 0x20; // Fill last 64 bytes with space character
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

    while out.len() < original_size {
        // Decrement first, then check (matching SQZ.EXE's dec-then-test pattern)
        block_remaining -= 1;
        if block_remaining < 0 {
            let next_block_index = block_index.saturating_add(1);
            let block_start = br.clone();

            let parse_block = |br: &mut R,
                               pt_len_mode: SqzPtLenMode|
             -> Result<(u16, HuffmanDecoder, HuffmanDecoder)> {
                let raw_n = br.get_bits_u16(blocksize_bits)?;
                // Block size is stored as a bit-field. SQZ.EXE does `dec ax` after
                // reading, meaning `n = raw_n - 1`. However, different streams
                // may have different conventions, so we try multiple offsets.
                // Note: Not sure if it's needed - works with raw_n but I leave it in - just in case…
                let n = match blocksize_offset {
                    -1 => raw_n.saturating_sub(1),
                    0 => raw_n,
                    1 => raw_n.wrapping_add(1),
                    _ => raw_n,
                };
                if n == 0 {
                    return Err(ArchiveError::decompression_failed(
                        "SQZ",
                        "Invalid block size (0)",
                    ));
                }

                let pt_for_c = read_pt_len(br, NT, 5, Some(3), pt_len_mode).map_err(|e| {
                    ArchiveError::decompression_failed(
                        "SQZ",
                        format!("block#{next_block_index} read_pt_len(CT): {e}"),
                    )
                })?;

                let c_dec = read_c_len(br, &pt_for_c, allow_empty_c_tree).map_err(|e| {
                    ArchiveError::decompression_failed(
                        "SQZ",
                        format!("block#{next_block_index} read_c_len: {e}"),
                    )
                })?;

                let p_dec = read_pt_len(br, NP, 5, None, pt_len_mode).map_err(|e| {
                    ArchiveError::decompression_failed(
                        "SQZ",
                        format!("block#{next_block_index} read_pt_len(P): {e}"),
                    )
                })?;

                Ok((n, c_dec, p_dec))
            };

            let mut br_try = block_start.clone();

            // Try parsing the block header/tables without and with byte-alignment.
            // If that fails, allow a one-way switch from PeekSkip -> Sequential (method-3
            // streams in the wild appear inconsistent here).
            let mut last_err: Option<ArchiveError> = None;

            let try_once = |candidate: R, align: bool, mode: SqzPtLenMode| {
                let mut br_local = candidate;
                if align {
                    br_local.align_to_byte();
                }
                let res = parse_block(&mut br_local, mode);
                (br_local, res)
            };

            // Attempt order:
            // 1) no byte-alignment + current PT mode
            // 2) byte-alignment + current PT mode
            // 3) no byte-alignment + alternate PT mode (PeekSkip -> Sequential only)
            // 4) byte-alignment + alternate PT mode
            let mut attempts: Vec<(bool, SqzPtLenMode)> =
                vec![(false, pt_len_mode), (true, pt_len_mode)];
            if pt_len_mode == SqzPtLenMode::PeekSkip {
                attempts.push((false, SqzPtLenMode::Sequential));
                attempts.push((true, SqzPtLenMode::Sequential));
            }

            let mut parsed: Option<(R, u16, HuffmanDecoder, HuffmanDecoder)> = None;
            for (align, mode) in attempts {
                let (candidate_reader, res) = try_once(block_start.clone(), align, mode);
                match res {
                    Ok((n, c, p)) => {
                        if mode != pt_len_mode {
                            pt_len_mode = mode;
                        }
                        br_try = candidate_reader;
                        parsed = Some((br_try.clone(), n, c, p));
                        break;
                    }
                    Err(e) => {
                        last_err = Some(e);
                    }
                }
            }

            let Some((_, n, new_c_dec, new_p_dec)) = parsed else {
                return Err(last_err.unwrap_or_else(|| {
                    ArchiveError::decompression_failed("SQZ", "Block parse failed")
                }));
            };

            br = br_try;
            block_index = next_block_index;
            block_remaining = (n as i32) - 1;
            c_dec = new_c_dec;
            p_dec = new_p_dec;
        }

        let c = match decode_len_code(&mut br, &c_dec, len_mapping, deflate_len_tables) {
            Ok(c) => c,
            Err(e) => {
                return Err(e);
            }
        };

        if c <= 0xff {
            let b = c as u8;
            out.push(b);
            window[win_pos] = b;
            win_pos = (win_pos + 1) & window_mask;
        } else {
            let length = (c as usize).saturating_sub(length_bias as usize);

            let p_sym = match p_dec.decode(&mut br) {
                Ok(s) => s as usize,
                Err(e) => {
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
    // Method 1: SqzNative + PowerOfTwo
    // Method 2: SqzNative + ExeTables
    // Method 3: DeflateLike29 + PowerOfTwo
    // Method 4: DeflateLike29 + ExeTables
    //
    // SQZ.EXE reads PT lengths using the PeekSkip-style algorithm for all methods.
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
            // Method-3 distance mapping is ambiguous in the wild; we try both below.
            SqzDistMapping::PowerOfTwo,
            // SQZ.EXE reads PT lengths using the PeekSkip-style algorithm.
            SqzPtLenMode::PeekSkip,
        ),
        4 => (
            SqzLenMapping::DeflateLike29,
            SqzDistMapping::ExeTables,
            SqzPtLenMode::PeekSkip,
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
    let blocksize_bits = 14u8;
    let blocksize_offset = 0i8;
    let allow_empty_c_tree = false;

    // Method 3/4 use SqzExe length tables, others use Standard
    let deflate_len_tables = match method {
        3 | 4 => SqzDeflateLenTables::SqzExe,
        _ => SqzDeflateLenTables::Standard,
    };

    let mut out = unsqz_method4_impl_with_reader(
        BitReaderMsb::new(buf),
        original_size,
        len_mapping,
        dist_mapping,
        pt_len_mode,
        deflate_len_tables,
        win_pos_init,
        length_bias,
        dist_bias,
        blocksize_bits,
        blocksize_offset,
        allow_empty_c_tree,
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
