//! DMS track decompressors: RLE, QUICK, MEDIUM, DEEP and HEAVY1/HEAVY2.
//!
//! All modes share one 16 KiB history buffer. A track whose flags have bit 0 set
//! leaves the decoder state (buffer, positions, DEEP and HEAVY tables) to the next
//! track; otherwise the state is reset after the track. The behaviour follows the
//! public domain xDMS 1.3.2 by Andre Rodrigues de la Rocha, so images decode byte
//! for byte as `xdms u` writes them.

/// History buffer size; every mode masks its positions to at most 14 bits.
const WINDOW: usize = 0x4000;
/// A reset clears this much of the history buffer, as xDMS does.
const RESET_CLEARED: usize = 0x3fc8;
/// Escape byte of the run-length encoding.
const RLE_ESCAPE: u8 = 0x90;

/// Number of 6-bit position prefixes per code length (3 to 8 bits) in the MEDIUM and
/// DEEP position code, shortest first. Prefix 0 has the 3-bit code `000`.
const POSITION_CODE_GROUPS: [(usize, u8); 6] = [(1, 3), (3, 4), (8, 5), (12, 6), (24, 7), (16, 8)];

/// Maps the next eight bits of a position code to (6-bit prefix, code length).
const POSITION_CODES: [(u8, u8); 256] = {
    let mut table = [(0, 0); 256];
    let mut index = 0;
    let mut prefix = 0;
    let mut group = 0;
    while group < POSITION_CODE_GROUPS.len() {
        let (count, len) = POSITION_CODE_GROUPS[group];
        let mut n = 0;
        while n < count {
            let mut slot = 0;
            while slot < 1 << (8 - len) {
                table[index] = (prefix, len);
                index += 1;
                slot += 1;
            }
            prefix += 1;
            n += 1;
        }
        group += 1;
    }
    table
};

pub(crate) type UnpackResult<T> = std::result::Result<T, &'static str>;

/// MSB-first bit reader that keeps at least 16 bits buffered. Past the end of the
/// input it reads zero bytes: valid streams only look ahead there, which
/// [`Bits::check_end`] verifies once a track is decoded.
struct Bits<'a> {
    data: &'a [u8],
    pos: usize,
    buf: u32,
    count: u32,
}

impl<'a> Bits<'a> {
    fn new(data: &'a [u8]) -> Self {
        let mut bits = Self {
            data,
            pos: 0,
            buf: 0,
            count: 0,
        };
        bits.fill();
        bits
    }

    fn fill(&mut self) {
        while self.count < 16 {
            self.buf = (self.buf << 8) | u32::from(self.data.get(self.pos).copied().unwrap_or(0));
            self.pos += 1;
            self.count += 8;
        }
    }

    /// Returns the next `n` (at most 16) bits without consuming them.
    fn peek(&self, n: u32) -> u16 {
        (self.buf >> (self.count - n)) as u16
    }

    fn drop_bits(&mut self, n: u32) {
        self.count -= n;
        self.buf &= (1 << self.count) - 1;
        self.fill();
    }

    /// Fails if more bits were consumed than the input holds.
    fn check_end(&self) -> UnpackResult<()> {
        if self.pos * 8 - self.count as usize > self.data.len() * 8 {
            return Err("packed data ends early");
        }
        Ok(())
    }

    fn get(&mut self, n: u32) -> u16 {
        let value = self.peek(n);
        self.drop_bits(n);
        value
    }

    /// Reads a MEDIUM/DEEP position prefix code; returns the prefix and the eight bits after it.
    fn position_code(&mut self) -> (u16, u16) {
        let lead = self.get(8);
        let (prefix, len) = POSITION_CODES[usize::from(lead)];
        let len = u32::from(len);
        let low = ((lead << len) | self.get(len)) & 0xff;
        (u16::from(prefix), low)
    }
}

/// Undoes the run-length stage: `0x90 0x00` is a literal `0x90`, `0x90 n b` repeats
/// `b` n times, and `0x90 0xff b hi lo` repeats `b` a 16-bit number of times.
pub(crate) fn unpack_rle(input: &[u8], size: usize) -> UnpackResult<Vec<u8>> {
    let mut output = Vec::with_capacity(size);
    let mut bytes = input.iter().copied();
    let mut next = || bytes.next().ok_or("RLE data ends early");
    while output.len() < size {
        let byte = next()?;
        if byte != RLE_ESCAPE {
            output.push(byte);
            continue;
        }
        let count = next()?;
        if count == 0 {
            output.push(RLE_ESCAPE);
            continue;
        }
        let value = next()?;
        let count = if count == 0xff {
            usize::from(u16::from_be_bytes([next()?, next()?]))
        } else {
            usize::from(count)
        };
        if output.len() + count > size {
            return Err("RLE run exceeds the track size");
        }
        output.resize(output.len() + count, value);
    }
    Ok(output)
}

/// A canonical Huffman code: symbols ordered by code length, then by value.
#[derive(Clone)]
enum Huffman {
    /// A single symbol that takes no bits (transmitted when the code has one symbol).
    Constant(u16),
    Canonical {
        /// Number of codes of each length, indexed by length.
        counts: [u16; 32],
        symbols: Vec<u16>,
    },
}

impl Huffman {
    /// Builds a code from bit lengths (0 = unused); the lengths must form a complete code.
    fn from_lengths(lengths: &[u8]) -> UnpackResult<Self> {
        let mut counts = [0u16; 32];
        for &len in lengths {
            counts[usize::from(len)] += 1;
        }
        counts[0] = 0;
        // Kraft sum in units of 2^-31: a complete code fills the whole space.
        let space: u64 = (1..32).map(|len| u64::from(counts[len]) << (31 - len)).sum();
        if space != 1 << 31 {
            return Err("incomplete or oversubscribed Huffman code");
        }
        let mut symbols = Vec::with_capacity(lengths.len());
        for len in 1..32u8 {
            symbols.extend((0..lengths.len()).filter(|&s| lengths[s] == len).map(|s| s as u16));
        }
        Ok(Self::Canonical { counts, symbols })
    }

    fn decode(&self, bits: &mut Bits<'_>) -> u16 {
        match self {
            Self::Constant(symbol) => *symbol,
            Self::Canonical { counts, symbols } => {
                // Walks the canonical code one bit at a time; the code is complete, so it ends.
                let (mut code, mut first, mut index) = (0usize, 0usize, 0usize);
                for &count in &counts[1..] {
                    code |= usize::from(bits.get(1));
                    let count = usize::from(count);
                    let offset = code.wrapping_sub(first);
                    if offset < count {
                        return symbols[index + offset];
                    }
                    index += count;
                    first = (first + count) << 1;
                    code <<= 1;
                }
                // Not reached: `from_lengths` only accepts complete codes.
                0
            }
        }
    }
}

/// Literal/length symbols of the HEAVY modes: 256 literals and match lengths 3-256.
const HEAVY_SYMBOLS: usize = 510;
/// HEAVY match lengths are `symbol - 253`.
const HEAVY_LENGTH_BIAS: u16 = 253;
/// Upper bound on transmitted HEAVY position code lengths.
const HEAVY_MAX_POSITION_LENGTHS: usize = 20;

/// Adaptive Huffman tree of the DEEP mode: 314 symbols (256 literals, lengths 3-60).
struct DeepTree {
    freq: [u16; DEEP_NODES + 1],
    parent: [u16; DEEP_NODES + DEEP_SYMBOLS],
    child: [u16; DEEP_NODES],
}

const DEEP_SYMBOLS: usize = 314;
const DEEP_NODES: usize = DEEP_SYMBOLS * 2 - 1;
const DEEP_ROOT: usize = DEEP_NODES - 1;
const DEEP_MAX_FREQ: u16 = 0x8000;

impl DeepTree {
    fn new() -> Box<Self> {
        let mut tree = Box::new(Self {
            freq: [0; DEEP_NODES + 1],
            parent: [0; DEEP_NODES + DEEP_SYMBOLS],
            child: [0; DEEP_NODES],
        });
        for symbol in 0..DEEP_SYMBOLS {
            tree.freq[symbol] = 1;
            tree.child[symbol] = (symbol + DEEP_NODES) as u16;
            tree.parent[symbol + DEEP_NODES] = symbol as u16;
        }
        let mut node = DEEP_SYMBOLS;
        let mut pair = 0;
        while node <= DEEP_ROOT {
            tree.freq[node] = tree.freq[pair].wrapping_add(tree.freq[pair + 1]);
            tree.child[node] = pair as u16;
            tree.parent[pair] = node as u16;
            tree.parent[pair + 1] = node as u16;
            pair += 2;
            node += 1;
        }
        tree.freq[DEEP_NODES] = 0xffff;
        tree.parent[DEEP_ROOT] = 0;
        tree
    }

    fn decode(&mut self, bits: &mut Bits<'_>) -> u16 {
        let mut node = usize::from(self.child[DEEP_ROOT]);
        while node < DEEP_NODES {
            node = usize::from(self.child[node + usize::from(bits.get(1))]);
        }
        let symbol = node - DEEP_NODES;
        self.update(symbol);
        symbol as u16
    }

    /// Halves all leaf frequencies and rebuilds the tree.
    fn rebuild(&mut self) {
        let mut leaf = 0;
        for node in 0..DEEP_NODES {
            if usize::from(self.child[node]) >= DEEP_NODES {
                self.freq[leaf] = u32::from(self.freq[node]).div_ceil(2) as u16;
                self.child[leaf] = self.child[node];
                leaf += 1;
            }
        }
        let mut pair = 0;
        for node in DEEP_SYMBOLS..DEEP_NODES {
            let freq = self.freq[pair].wrapping_add(self.freq[pair + 1]);
            self.freq[node] = freq;
            let mut at = node - 1;
            while at > 0 && freq < self.freq[at] {
                at -= 1;
            }
            if freq >= self.freq[at] {
                at += 1;
            }
            self.freq.copy_within(at..node, at + 1);
            self.freq[at] = freq;
            self.child.copy_within(at..node, at + 1);
            self.child[at] = pair as u16;
            pair += 2;
        }
        for node in 0..DEEP_NODES {
            let child = usize::from(self.child[node]);
            self.parent[child] = node as u16;
            if child < DEEP_NODES {
                self.parent[child + 1] = node as u16;
            }
        }
    }

    /// Counts one more occurrence of `symbol`, keeping nodes ordered by frequency.
    fn update(&mut self, symbol: usize) {
        if self.freq[DEEP_ROOT] == DEEP_MAX_FREQ {
            self.rebuild();
        }
        let mut node = usize::from(self.parent[symbol + DEEP_NODES]);
        loop {
            self.freq[node] = self.freq[node].wrapping_add(1);
            let freq = self.freq[node];
            if freq > self.freq[node + 1] {
                // Swap with the last node of lower frequency (the sentinel ends the scan).
                let mut swap = node + 1;
                while freq > self.freq[swap + 1] {
                    swap += 1;
                }
                self.freq[node] = self.freq[swap];
                self.freq[swap] = freq;

                let child = usize::from(self.child[node]);
                self.parent[child] = swap as u16;
                if child < DEEP_NODES {
                    self.parent[child + 1] = swap as u16;
                }
                let other = usize::from(self.child[swap]);
                self.child[swap] = child as u16;
                self.parent[other] = node as u16;
                if other < DEEP_NODES {
                    self.parent[other + 1] = node as u16;
                }
                self.child[node] = other as u16;
                node = swap;
            }
            node = usize::from(self.parent[node]);
            if node == 0 {
                break;
            }
        }
    }
}

/// Decoder state carried between tracks.
pub(crate) struct Unpacker {
    text: Vec<u8>,
    quick_pos: u16,
    medium_pos: u16,
    deep_pos: u16,
    heavy_pos: u16,
    /// `None` until the first DEEP track after a reset.
    deep: Option<Box<DeepTree>>,
    // HEAVY code tables survive resets; a track without new tables reuses the last ones.
    heavy_literals: Huffman,
    heavy_positions: Huffman,
    heavy_last_position: u16,
}

impl Unpacker {
    pub(crate) fn new() -> Self {
        let mut unpacker = Self {
            text: vec![0; WINDOW],
            quick_pos: 0,
            medium_pos: 0,
            deep_pos: 0,
            heavy_pos: 0,
            deep: None,
            heavy_literals: Huffman::Constant(0),
            heavy_positions: Huffman::Constant(0),
            heavy_last_position: 0,
        };
        unpacker.reset();
        unpacker
    }

    fn reset(&mut self) {
        self.quick_pos = 251;
        self.medium_pos = 0x3fbe;
        self.heavy_pos = 0;
        self.deep_pos = 0x3fc4;
        self.deep = None;
        self.text[..RESET_CLEARED].fill(0);
    }

    /// Decodes one track. `packed_len` is the size after the first stage (before RLE).
    pub(crate) fn unpack(&mut self, data: &[u8], mode: u8, flags: u8, packed_len: usize, size: usize) -> UnpackResult<Vec<u8>> {
        let mut output = match mode {
            0 => data.get(..size).ok_or("NOCOMP data is shorter than the track")?.to_vec(),
            1 => unpack_rle(data, size)?,
            2 => unpack_rle(&self.quick(data, packed_len)?, size)?,
            3 => unpack_rle(&self.medium(data, packed_len)?, size)?,
            4 => unpack_rle(&self.deep(data, packed_len)?, size)?,
            5 | 6 => {
                let heavy = self.heavy(data, flags, mode == 6, packed_len)?;
                if flags & 4 != 0 {
                    unpack_rle(&heavy, size)?
                } else {
                    heavy
                }
            }
            _ => return Err("unknown compression mode"),
        };
        // Only a HEAVY track without the RLE stage can come out short; never pad it.
        if output.len() < size {
            return Err("decoded data is shorter than the track");
        }
        output.truncate(size);
        if flags & 1 == 0 {
            self.reset();
        }
        Ok(output)
    }

    /// Copies a match from the history. Like xDMS, a match may run past `size`; the
    /// extra bytes stay in the history and are dropped by the next stage.
    fn copy_match(text: &mut [u8], pos: &mut u16, mask: u16, from: u16, len: u16, output: &mut Vec<u8>) {
        let mut from = from;
        for _ in 0..len {
            let byte = text[usize::from(from & mask)];
            text[usize::from(*pos & mask)] = byte;
            output.push(byte);
            *pos = pos.wrapping_add(1);
            from = from.wrapping_add(1);
        }
    }

    fn literal(text: &mut [u8], pos: &mut u16, mask: u16, byte: u8, output: &mut Vec<u8>) {
        text[usize::from(*pos & mask)] = byte;
        output.push(byte);
        *pos = pos.wrapping_add(1);
    }

    /// QUICK: 256-byte history, literals or 2-5 byte matches with 8-bit offsets.
    fn quick(&mut self, data: &[u8], size: usize) -> UnpackResult<Vec<u8>> {
        const MASK: u16 = 0xff;
        let mut bits = Bits::new(data);
        let mut output = Vec::with_capacity(size + 8);
        while output.len() < size {
            if bits.get(1) != 0 {
                let byte = bits.get(8) as u8;
                Self::literal(&mut self.text, &mut self.quick_pos, MASK, byte, &mut output);
            } else {
                let len = bits.get(2) + 2;
                let from = self.quick_pos.wrapping_sub(bits.get(8)).wrapping_sub(1);
                Self::copy_match(&mut self.text, &mut self.quick_pos, MASK, from, len, &mut output);
            }
        }
        self.quick_pos = self.quick_pos.wrapping_add(5) & MASK;
        bits.check_end()?;
        Ok(output)
    }

    /// MEDIUM: 16 KiB history, 3-66 byte matches; length and offset use the position code.
    fn medium(&mut self, data: &[u8], size: usize) -> UnpackResult<Vec<u8>> {
        const MASK: u16 = 0x3fff;
        let mut bits = Bits::new(data);
        let mut output = Vec::with_capacity(size + 80);
        while output.len() < size {
            if bits.get(1) != 0 {
                let byte = bits.get(8) as u8;
                Self::literal(&mut self.text, &mut self.medium_pos, MASK, byte, &mut output);
            } else {
                // The length code is followed by the offset code; the eight bits read after
                // the length prefix are the first eight bits of the offset code.
                let lead = bits.get(8);
                let (prefix, len) = POSITION_CODES[usize::from(lead)];
                let length = u16::from(prefix) + 3;
                let len = u32::from(len);
                let next = ((lead << len) | bits.get(len)) & 0xff;
                let (high, len) = POSITION_CODES[usize::from(next)];
                let len = u32::from(len);
                let offset = (u16::from(high) << 8) | (((next << len) | bits.get(len)) & 0xff);
                let from = self.medium_pos.wrapping_sub(offset).wrapping_sub(1);
                Self::copy_match(&mut self.text, &mut self.medium_pos, MASK, from, length, &mut output);
            }
        }
        self.medium_pos = self.medium_pos.wrapping_add(66) & MASK;
        bits.check_end()?;
        Ok(output)
    }

    /// DEEP: adaptive Huffman literals and 3-60 byte lengths, 16 KiB history.
    fn deep(&mut self, data: &[u8], size: usize) -> UnpackResult<Vec<u8>> {
        const MASK: u16 = 0x3fff;
        let mut bits = Bits::new(data);
        let mut output = Vec::with_capacity(size + 64);
        let tree = self.deep.get_or_insert_with(DeepTree::new);
        while output.len() < size {
            let symbol = tree.decode(&mut bits);
            if symbol < 256 {
                Self::literal(&mut self.text, &mut self.deep_pos, MASK, symbol as u8, &mut output);
            } else {
                let length = symbol - 253;
                let (high, low) = bits.position_code();
                let from = self.deep_pos.wrapping_sub((high << 8) | low).wrapping_sub(1);
                Self::copy_match(&mut self.text, &mut self.deep_pos, MASK, from, length, &mut output);
            }
        }
        self.deep_pos = self.deep_pos.wrapping_add(60) & MASK;
        bits.check_end()?;
        Ok(output)
    }

    /// HEAVY1/HEAVY2: static Huffman codes (sent when flag bit 1 is set), 4/8 KiB history,
    /// and a position symbol that repeats the previous offset.
    fn heavy(&mut self, data: &[u8], flags: u8, heavy2: bool, size: usize) -> UnpackResult<Vec<u8>> {
        let (positions, mask): (u16, u16) = if heavy2 { (15, 0x1fff) } else { (14, 0x0fff) };
        let mut bits = Bits::new(data);
        if flags & 2 != 0 {
            self.heavy_literals = Self::read_code(&mut bits, 9, 5, HEAVY_SYMBOLS, HEAVY_SYMBOLS)?;
            self.heavy_positions = Self::read_code(&mut bits, 5, 4, usize::from(positions), HEAVY_MAX_POSITION_LENGTHS)?;
        }
        if let Huffman::Constant(symbol) = self.heavy_positions {
            if symbol >= positions {
                return Err("invalid HEAVY position code");
            }
        }
        let mut output = Vec::with_capacity(size + 256);
        while output.len() < size {
            let symbol = self.heavy_literals.decode(&mut bits);
            if symbol < 256 {
                Self::literal(&mut self.text, &mut self.heavy_pos, mask, symbol as u8, &mut output);
            } else {
                let length = symbol - HEAVY_LENGTH_BIAS;
                let code = self.heavy_positions.decode(&mut bits);
                if code != positions - 1 {
                    self.heavy_last_position = if code > 0 {
                        let extra = u32::from(code - 1);
                        bits.get(extra) | (1 << extra)
                    } else {
                        0
                    };
                }
                let from = self.heavy_pos.wrapping_sub(self.heavy_last_position).wrapping_sub(1);
                Self::copy_match(&mut self.text, &mut self.heavy_pos, mask, from, length, &mut output);
            }
        }
        bits.check_end()?;
        Ok(output)
    }

    /// Reads a HEAVY code: a count, then that many bit lengths, or a zero count followed
    /// by the single symbol.
    fn read_code(bits: &mut Bits<'_>, count_bits: u32, length_bits: u32, symbols: usize, max_count: usize) -> UnpackResult<Huffman> {
        let count = usize::from(bits.get(count_bits));
        if count == 0 {
            let symbol = bits.get(count_bits);
            if usize::from(symbol) >= symbols {
                return Err("invalid HEAVY single-symbol code");
            }
            return Ok(Huffman::Constant(symbol));
        }
        if count > max_count {
            return Err("too many HEAVY code lengths");
        }
        let mut lengths: Vec<u8> = (0..count).map(|_| bits.get(length_bits) as u8).collect();
        lengths.resize(symbols.max(count), 0);
        lengths.truncate(symbols);
        Huffman::from_lengths(&lengths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_codes_cover_all_prefixes() {
        assert_eq!(POSITION_CODES[0], (0, 3));
        assert_eq!(POSITION_CODES[31], (0, 3));
        assert_eq!(POSITION_CODES[32], (1, 4));
        assert_eq!(POSITION_CODES[0xbf], (23, 6));
        assert_eq!(POSITION_CODES[0xc0], (24, 7));
        assert_eq!(POSITION_CODES[0xef], (47, 7));
        assert_eq!(POSITION_CODES[0xf0], (48, 8));
        assert_eq!(POSITION_CODES[0xff], (63, 8));
    }

    #[test]
    fn rle_escapes_and_runs() {
        assert_eq!(unpack_rle(&[1, 0x90, 0, 2, 0x90, 3, 7], 6).unwrap(), [1, 0x90, 2, 7, 7, 7]);
        assert_eq!(unpack_rle(&[0x90, 0xff, 9, 0x01, 0x00], 256).unwrap(), vec![9; 256]);
        assert!(unpack_rle(&[0x90, 5, 1], 4).is_err());
        assert!(unpack_rle(&[0x90], 4).is_err());
    }

    #[test]
    fn canonical_codes_must_be_complete() {
        assert!(Huffman::from_lengths(&[1, 1]).is_ok());
        assert!(Huffman::from_lengths(&[1, 2, 2]).is_ok());
        assert!(Huffman::from_lengths(&[1, 0]).is_err());
        assert!(Huffman::from_lengths(&[1, 1, 1]).is_err());
        assert!(Huffman::from_lengths(&[0, 0]).is_err());
    }
}
