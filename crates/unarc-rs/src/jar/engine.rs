//! JAR 1.02 Huffman and LZ symbol decoder (shared by presets m1–m4).
//!
//! This module is a faithful Rust port of the entropy-decoder front-end of
//! JAR's `engine\huff.c`, reverse-engineered from `JAR32.EXE`
//! (see `disasm/jar32_objdump.asm`). It implements, exactly:
//!
//! * the LSB-first bit reader (orig `0x422d20`),
//! * the code-length table reader (orig `0x422d70`),
//! * the Huffman tree builder (orig `0x416410`),
//! * the per-symbol decoder (orig `0x422fb0`).
//!
//! The LZ emit layer reconstructs 16-bit symbols using a full symbol history,
//! a 0x1010-entry recent-special-symbol history and a recent-match history.
//! `words` performs the subsequent dictionary and binary transforms.
//!
//! ## Validation invariant
//! A complete Huffman code-length table satisfies `sum(weights) == 0x100000`
//! (each weight is `2^20 >> code_len`) and fills exactly `NSYM` entries. This is
//! used to locate the true start of the bitstream.

/// Number of symbols in the JAR literal/length alphabet (`0x300`).
pub const NSYM: usize = 0x300;
/// Total tree node slots (leaves + internal), `0x600`.
const NNODES: usize = 0x600;
/// Sentinel "no node" value (the engine uses `0xffffffff`).
const NIL: u32 = 0xffff_ffff;

/// LSB-first bit reader over little-endian bytes (port of `0x422d20`).
///
/// Reads a 40-bit little-endian window at the current byte position, shifts out
/// the already-consumed low bits, and masks the requested count.
pub struct BitReader<'a> {
    buf: &'a [u8],
    byte_pos: usize,
    bit_pos: u32,
}

impl<'a> BitReader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, byte_pos: 0, bit_pos: 0 }
    }

    pub fn with_offset(buf: &'a [u8], byte_pos: usize) -> Self {
        Self { buf, byte_pos, bit_pos: 0 }
    }

    /// Skip `n` bits forward (used by the entry-point probe).
    pub fn skip_bits(&mut self, n: u32) {
        let total = self.bit_pos + n;
        self.byte_pos += (total >> 3) as usize;
        self.bit_pos = total & 7;
    }

    /// Number of whole bits consumed so far.
    pub fn bits_consumed(&self) -> usize {
        self.byte_pos * 8 + self.bit_pos as usize
    }

    fn peek_word(&self) -> u64 {
        let mut v = 0u64;
        for i in 0..5 {
            let idx = self.byte_pos + i;
            if idx < self.buf.len() {
                v |= (self.buf[idx] as u64) << (i * 8);
            }
        }
        v
    }

    /// Read up to 32 bits, including unaligned distance codes.
    pub fn read(&mut self, n: u32) -> u32 {
        assert!(n <= 32);
        if n == 0 {
            return 0;
        }
        let word = self.peek_word();
        let mask = (1u64 << n) - 1;
        let val = ((word >> self.bit_pos) & mask) as u32;
        let total = self.bit_pos + n;
        self.byte_pos += (total >> 3) as usize;
        self.bit_pos = total & 7;
        val
    }

    /// True once the reader has run past the end of the buffer.
    pub fn exhausted(&self) -> bool {
        self.byte_pos >= self.buf.len()
    }
}

/// Result of reading one code-length table.
pub struct CodeLengthTable {
    /// Per-symbol weights: `2^20 >> code_len`, or 0 for absent symbols.
    pub weights: [u32; NSYM],
}

impl CodeLengthTable {
    /// A complete Huffman code has `sum(weights) == 2^20`.
    pub fn is_complete(&self) -> bool {
        self.weights.iter().map(|&w| w as u64).sum::<u64>() == 0x10_0000
    }
}

/// Outcome of attempting to read a code-length table.
pub enum TableRead {
    /// A table that filled exactly `NSYM` symbols (boxed: it holds a 3 KiB
    /// weight array, so boxing keeps the enum small).
    Filled(Box<CodeLengthTable>),
    /// The 8-bit start marker was 0 (end-of-stream signal).
    End,
    /// The token stream overran or underran the symbol table.
    Malformed,
}

/// Port of `0x422d70`: read the run-length-coded code-length table.
///
/// `expect_marker` controls whether the leading 8-bit sync marker (which the
/// engine expects to be 1) is consumed first.
pub fn read_code_length_table(br: &mut BitReader, expect_marker: bool) -> TableRead {
    if expect_marker {
        br.skip_bits(((8 - br.bits_consumed() % 8) % 8) as u32);
        if br.exhausted() {
            return TableRead::Malformed;
        }
        let marker = br.read(8);
        if marker == 0 {
            return TableRead::End;
        }
        if marker != 1 {
            return TableRead::Malformed;
        }
    }

    let mut weights = [0u32; NSYM];
    let mut idx = 0usize;
    let mut prev_len: u32 = 0;

    while idx < NSYM {
        if br.exhausted() {
            return TableRead::Malformed;
        }
        let token = br.read(3);
        match token {
            0 => {
                weights[idx] = 0;
                idx += 1;
                prev_len = 0;
            }
            1..=4 => {
                let len = token + 7; // 8..=11
                weights[idx] = 0x10_0000 >> len;
                idx += 1;
                prev_len = len;
            }
            7 => {
                let n = br.read(4);
                let len = if n == 0 {
                    // original refills here; treat as a hard stop
                    return TableRead::Malformed;
                } else if n < 8 {
                    n
                } else {
                    n + 4
                };
                weights[idx] = 0x10_0000 >> len;
                idx += 1;
                prev_len = len;
            }
            5 | 6 => {
                if idx == 0 {
                    return TableRead::Malformed;
                }
                let count = if token == 5 { br.read(2) + 2 } else { br.read(12) };
                let w = if prev_len != 0 { 0x10_0000 >> prev_len } else { 0 };
                if count == 0 || count as usize > NSYM - idx {
                    return TableRead::Malformed;
                }
                for _ in 0..count {
                    weights[idx] = w;
                    idx += 1;
                }
                prev_len = 0;
            }
            _ => unreachable!("3-bit token is always 0..=7"),
        }
    }

    if idx == NSYM && br.bits_consumed() <= br.buf.len() * 8 {
        TableRead::Filled(Box::new(CodeLengthTable { weights }))
    } else {
        TableRead::Malformed
    }
}

/// Huffman decode tree, faithful port of the arrays built by `0x416410`.
///
/// After construction, `left[node]` is followed on bit 0 and `right[node]` on
/// bit 1 (the bit reader is LSB-first). A node value `< NSYM` is a leaf whose
/// index is the decoded symbol; values `>= NSYM` are internal nodes.
pub struct HuffmanTree {
    left: [u32; NNODES],
    right: [u32; NNODES],
    root: u32,
}

impl HuffmanTree {
    /// Faithful port of `0x416410` (+ the root walk at `0x422f52`) building the
    /// decode tree from per-symbol `weights` (`2^20 >> code_len`, 0 if absent).
    ///
    /// Phases mirror the disassembly exactly:
    /// * C — insert every live leaf into a weight-keyed BST rooted at `first`;
    /// * D — repeatedly extract the two minimum-weight nodes (the right spine of
    ///   the BST) and merge them under a fresh internal node, threading parent
    ///   links through `pair`;
    /// * E/F — rebuild `left`/`right` as a decode tree by walking each leaf up
    ///   its `pair` chain (first claimant of a node takes the right/bit-1 slot);
    /// * root — walk `pair` up from the first live leaf to the node whose parent
    ///   is `NIL`.
    #[allow(clippy::cognitive_complexity)]
    pub fn build(weights_in: &[u32; NSYM]) -> Option<Self> {
        let mut weight = [0u32; NNODES];
        weight[..NSYM].copy_from_slice(weights_in);

        // pair = parent links (0x457be0); left = 0x45a000; right = 0x45dc30.
        let mut pair = [NIL; NNODES];
        let mut left = [NIL; NNODES];
        let mut right = [NIL; NNODES];

        // first live (nonzero-weight) leaf; all-zero table is invalid.
        let first = (0..NSYM).find(|&i| weight[i] != 0)?;

        // If there is only a single live leaf, the engine bumps weights[0] and
        // weights[1] so a two-leaf tree can be formed.
        let has_second = ((first + 1)..NSYM).any(|i| weight[i] != 0);
        if !has_second {
            weight[0] = weight[0].wrapping_add(1);
            weight[1] = weight[1].wrapping_add(1);
        }

        // --- Phase C: weight-keyed BST of leaves, rooted at `first`. ---
        // At a node: lighter than the insert key descends left, else right.
        for node in 0..NSYM {
            let w = weight[node];
            if w == 0 || node == first {
                continue;
            }
            right[node] = NIL;
            left[node] = NIL;
            let mut cur = first;
            loop {
                if weight[cur] < w {
                    if left[cur] == NIL {
                        left[cur] = node as u32;
                        break;
                    }
                    cur = left[cur] as usize;
                } else if right[cur] == NIL {
                    right[cur] = node as u32;
                    break;
                } else {
                    cur = right[cur] as usize;
                }
            }
        }

        // --- Phase D: two-minimum merge. `root` is the BST root (NIL once
        // empty); the rightmost node along `right` links is the minimum. ---
        let mut bst_root = first as u32;
        let mut next_node = NSYM as u32; // fresh internal-node id (0x300..)
        let tree_root;
        loop {
            // Extract first minimum (rightmost of bst_root).
            let min1 = Self::extract_min(&mut bst_root, &mut left, &mut right);
            if bst_root == NIL {
                // Should not happen mid-merge; guard against malformed input.
                return None;
            }
            // Extract second minimum.
            let min2 = Self::extract_min(&mut bst_root, &mut left, &mut right);

            // Create the parent of the two minima.
            weight[next_node as usize] = weight[min1 as usize].wrapping_add(weight[min2 as usize]);
            pair[min1 as usize] = next_node;
            pair[min2 as usize] = next_node;
            pair[next_node as usize] = NIL;

            if bst_root == NIL {
                // The merge that empties the BST yields the tree root.
                tree_root = next_node;
                break;
            }

            // Re-insert the new internal node into the BST.
            right[next_node as usize] = NIL;
            left[next_node as usize] = NIL;
            let combined = weight[next_node as usize];
            let mut cur = bst_root as usize;
            loop {
                if weight[cur] < combined {
                    if left[cur] == NIL {
                        left[cur] = next_node;
                        break;
                    }
                    cur = left[cur] as usize;
                } else if right[cur] == NIL {
                    right[cur] = next_node;
                    break;
                } else {
                    cur = right[cur] as usize;
                }
            }
            next_node += 1;
        }

        // --- Phase E/F: rebuild left/right as the decode tree. ---
        for n in left.iter_mut() {
            *n = NIL;
        }
        for n in right.iter_mut() {
            *n = NIL;
        }
        for sym in 0..NSYM {
            if weight[sym] == 0 {
                continue;
            }
            let mut child = sym as u32;
            let mut hn = pair[sym];
            while hn != NIL {
                let h = hn as usize;
                if right[h] == NIL {
                    right[h] = child;
                }
                if right[h] != child {
                    // The right slot is taken by another leaf: take left/bit-0.
                    if left[h] == NIL {
                        left[h] = child;
                    }
                }
                child = hn;
                hn = pair[h];
            }
        }

        Some(Self { left, right, root: tree_root })
    }

    /// Extract the minimum-weight node from the weight BST: follow `right`
    /// links to the end, splice it out by replacing it with its `left` subtree,
    /// and update `root` if the minimum was the root itself.
    fn extract_min(root: &mut u32, left: &mut [u32; NNODES], right: &mut [u32; NNODES]) -> u32 {
        let r = *root;
        if right[r as usize] == NIL {
            // Root is the minimum.
            *root = left[r as usize];
            return r;
        }
        let mut parent = r;
        let mut node = right[r as usize];
        while right[node as usize] != NIL {
            parent = node;
            node = right[node as usize];
        }
        right[parent as usize] = left[node as usize];
        node
    }

    /// Decode one symbol by walking the tree from the root (cache-free port of
    /// `0x422fb0`): bit 1 takes `right`, bit 0 takes `left`, stopping at a leaf
    /// (`node < NSYM`).
    pub fn decode_symbol(&self, br: &mut BitReader) -> Option<u32> {
        let mut node = self.root;
        for _ in 0..32 {
            if node == NIL {
                return None;
            }
            if (node as usize) < NSYM {
                return Some(node);
            }
            if br.exhausted() {
                return None;
            }
            let bit = br.read(1);
            node = if bit == 1 { self.right[node as usize] } else { self.left[node as usize] };
        }
        None
    }

    pub fn root(&self) -> u32 {
        self.root
    }
}

/// A decoded match/length/distance operation produced from a symbol `>= 0x108`.
///
/// This is the structured result of `0x423140`: it parses the symbol plus any
/// trailing extra bits into one of the engine's operation kinds. The actual
/// *application* of these ops is performed by [`decode_symbols`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchCode {
    /// `sym` in `0x108..=0x10F`: a secondary-model literal symbol carrying 13
    /// extra bits — `value = ((sym << 13) + read(13)) & 0xFFFF` (orig `0x40ff50`
    /// literal path).
    LiteralSymbol(u16),
    /// `sym` in `0x110..=0x11F`: a length using the *remembered* distance
    /// (a repeat match); `len` already includes the decoded extra bits
    /// (orig `0x40ff20`).
    RepeatLength(u32),
    /// `sym` in `0x120..=0x2FF`, low nibble `0`: a distance taken from the
    /// recent-special-symbol model (orig `0x40f740`); `index` selects a previous
    /// symbol >= 0x108, not a byte distance.
    DistanceA(u32),
    /// `sym` in `0x120..=0x2FF`, low nibble `0xF`: a distance from the secondary
    /// recent-match model (orig `0x40f950`), recalling a position and length.
    DistanceB(u32),
    /// `sym` in `0x120..=0x2FF`, low nibble `1..=14`: a combined match — a
    /// `len` (from the nibble length table) and a direct symbol distance `index`
    /// (orig `0x40fef0`).
    Match { len: u32, index: u32 },
}

/// Decode the JAR length-token sub-code shared by the `0x110` and combined
/// paths (orig at `0x42318b` / `0x42324d`): nibble `n` selects a fixed length,
/// a `(1<<k)+read(k)+7` range, or a flat 15-bit length.
fn decode_length_nibble(n: u32, br: &mut BitReader) -> u32 {
    if n <= 7 {
        n + 1
    } else if n <= 13 {
        let k = n - 7;
        (1u32 << k) + br.read(k) + 7
    } else {
        br.read(15)
    }
}

/// Decode the distance-model index used by the `0x120+` paths (orig the
/// `d = (sym-0x120) >> 4; val = d ? (1<<d)+read(d) : 1` sequence).
fn decode_distance_index(sym: u32, br: &mut BitReader) -> u32 {
    let d = (sym - 0x120) >> 4;
    if d == 0 {
        1
    } else {
        (1u32 << d) + br.read(d)
    }
}

/// Faithful port of `0x423140`: turn a decoded symbol `>= 0x108` into a
/// structured [`MatchCode`], consuming the appropriate number of extra bits.
pub fn decode_match(sym: u32, br: &mut BitReader) -> MatchCode {
    match sym & 0xff0 {
        0x100 => {
            // sym 0x108..=0x10F: literal symbol + 13 extra bits.
            let value = ((sym << 13).wrapping_add(br.read(13))) as u16;
            MatchCode::LiteralSymbol(value)
        }
        0x110 => {
            // sym 0x110..=0x11F: repeat-length using remembered distance.
            let len = decode_length_nibble(sym & 0xf, br);
            MatchCode::RepeatLength(len)
        }
        _ => {
            // sym >= 0x120.
            let nib = sym & 0xf;
            if nib == 0 {
                MatchCode::DistanceA(decode_distance_index(sym, br))
            } else if nib == 0xf {
                MatchCode::DistanceB(decode_distance_index(sym, br))
            } else {
                let len = decode_length_nibble(nib, br);
                let index = decode_distance_index(sym, br);
                MatchCode::Match { len, index }
            }
        }
    }
}

/// Decode the Huffman/LZ layer to the word-transform's 16-bit alphabet.
/// Returns the symbols and the number of bytes consumed including the end marker.
pub fn decode_symbols(data: &[u8], limit: usize) -> crate::error::Result<(Vec<u16>, usize)> {
    use crate::error::ArchiveError;
    let bad = || ArchiveError::decompression_failed("JAR", "Invalid Huffman/LZ stream");
    let mut br = BitReader::new(data);
    let mut out = Vec::<u16>::new();
    let mut special = Vec::<u16>::new();
    let mut matches = Vec::<(usize, usize)>::new();
    let mut last = 0usize;
    let mut remembered = 0usize;
    loop {
        let padding = (8 - br.bits_consumed() % 8) % 8;
        br.skip_bits(padding as u32);
        if br.exhausted() {
            return Err(bad());
        }
        let marker = br.read(8);
        if marker == 0 {
            return Ok((out, br.bits_consumed() / 8));
        }
        if marker != 1 {
            return Err(bad());
        }
        let TableRead::Filled(table) = read_code_length_table(&mut br, false) else {
            return Err(bad());
        };
        if !table.is_complete() {
            return Err(bad());
        }
        let tree = HuffmanTree::build(&table.weights).ok_or_else(bad)?;
        let count = br.read(16);
        if count == 0 {
            return Err(bad());
        }
        for _ in 0..count {
            let sym = tree.decode_symbol(&mut br).ok_or_else(bad)?;
            let mut literal = None;
            let mut copy = None;
            let mut next_last = last;
            let next_remembered = last;
            if sym < 0x108 {
                literal = Some(sym as u16);
            } else {
                let operation = decode_match(sym, &mut br);
                match operation {
                    MatchCode::LiteralSymbol(v) => literal = Some(v),
                    MatchCode::DistanceA(i) => {
                        let i = i as usize;
                        if i == 0 || i > 0x1010 || i > special.len() {
                            return Err(bad());
                        }
                        literal = Some(special[special.len() - i]);
                    }
                    MatchCode::DistanceB(i) => {
                        let i = i as usize;
                        if i == 0 || i > 0x1010 || i > matches.len() {
                            return Err(bad());
                        }
                        let (pos, len) = matches[matches.len() - i];
                        copy = Some((out.len() - pos, len));
                    }
                    MatchCode::RepeatLength(len) => {
                        copy = Some((remembered, len as usize));
                        next_last = remembered;
                    }
                    MatchCode::Match { len, index } => {
                        copy = Some((index as usize, len as usize));
                        next_last = index as usize;
                    }
                }
            }
            if let Some(v) = literal {
                if out.len() >= limit {
                    return Err(bad());
                }
                out.push(v);
                if v >= 0x108 {
                    special.push(v);
                }
            }
            if let Some((distance, len)) = copy {
                if distance == 0 || distance > out.len() || len == 0 || len > limit.saturating_sub(out.len()) {
                    return Err(ArchiveError::decompression_failed(
                        "JAR",
                        &format!("distance={distance} len={len} at {}", out.len()),
                    ));
                }
                let start = out.len();
                for _ in 0..len {
                    let v = out[out.len() - distance];
                    out.push(v);
                    if v >= 0x108 {
                        special.push(v);
                    }
                }
                // 0x40fd20: remember matches of >=4 symbols, or 2/3-symbol
                // matches containing a transformed symbol. Literal-only short
                // matches are deliberately excluded from this model.
                if len >= 4 || (len >= 2 && out[start..].iter().any(|&v| v >= 0x108)) {
                    matches.push((start, len));
                }
            }
            last = next_last;
            remembered = next_remembered;
            if br.bits_consumed() > data.len() * 8 {
                return Err(bad());
            }
        }
    }
}
