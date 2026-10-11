//! Decoder for the compressed ("normal", pack mode 2) Amiga LZX stream
//!
//! Written from the format description in `doc/lzx.md`. The stream is an LZ77
//! stream over a 64 KiB window, coded with canonical Huffman codes in blocks:
//!
//! - Bits are read from big-endian 16-bit words, least significant bit first.
//!   Huffman codes are canonical and their first bit is the most significant one.
//! - A block header holds a 3-bit type (1 = reuse the previous tree, 2 = verbatim,
//!   3 = aligned offsets), for type 3 eight 3-bit aligned-offset code lengths, and
//!   the number of bytes the block decodes to as three bytes, most significant first.
//!   Types 2 and 3 then send the 768-symbol main tree as deltas against the previous
//!   lengths, in two sections (literals, then matches), each with its own pretree.
//! - Main symbols 0-255 are literals. Symbol `256 + length_slot * 32 + position_slot`
//!   is a match; position slot 0 repeats the last match offset.
//!
//! The code lengths, the last offset and the window persist from block to block and,
//! for merged groups, from file to file.

use std::io::{self, Read, Seek, SeekFrom};

const WINDOW_SIZE: usize = 1 << 16;
const WINDOW_MASK: usize = WINDOW_SIZE - 1;
const MAIN_SYMBOLS: usize = 768;
const LITERALS: usize = 256;
const ALIGNED_SYMBOLS: usize = 8;
const PRETREE_SYMBOLS: usize = 20;
const MAX_CODE_LENGTH: usize = 16;
const MAIN_TABLE_BITS: u32 = 12;
const ALIGNED_TABLE_BITS: u32 = 7;
const PRETREE_TABLE_BITS: u32 = 6;
const MIN_MATCH: u32 = 3;
/// Compressed bytes read from the archive at a time
const INPUT_CHUNK: usize = 32 * 1024;
/// Decoded bytes collected before they are handed out; with the longest match this stays well below the window size
const OUTPUT_BATCH: u64 = 32 * 1024;

/// Footer bits of each position or length slot
const FOOTER_BITS: [u8; 32] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14,
];
/// Smallest value of each position or length slot
const SLOT_BASE: [u32; 32] = [
    0, 1, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64, 96, 128, 192, 256, 384, 512, 768, 1024, 1536, 2048, 3072, 4096, 6144, 8192, 12288, 16384, 24576, 32768, 49152,
];

/// Why decoding stopped
#[derive(Debug)]
pub(super) enum DecodeError {
    Io(io::Error),
    Corrupt(&'static str),
}

impl From<io::Error> for DecodeError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

type DecodeResult<T> = std::result::Result<T, DecodeError>;

/// Reads the packed data of a group from the archive in chunks
struct BitReader {
    chunk: Vec<u8>,
    pos: usize,
    /// Archive offset of the first packed byte not yet in `chunk`
    next_offset: u64,
    /// Packed bytes not yet read into `chunk`
    unread: u64,
    /// Buffered bits; the next bit is bit 0
    bits: u64,
    count: u32,
    consumed: u64,
    /// Bits in the packed data, rounded up to whole words
    limit: u64,
}

impl BitReader {
    fn new(offset: u64, size: u64) -> Self {
        Self {
            chunk: Vec::new(),
            pos: 0,
            next_offset: offset,
            unread: size,
            bits: 0,
            count: 0,
            consumed: 0,
            limit: size.div_ceil(2).saturating_mul(16),
        }
    }

    /// Makes at least `n` (at most 32) bits available. Past the end of the data the
    /// buffer is padded with zeros; consuming them is an error.
    fn ensure<R: Read + Seek>(&mut self, reader: &mut R, n: u32) -> DecodeResult<()> {
        while self.count < n {
            if self.chunk.len() - self.pos < 2 && self.unread > 0 {
                self.load(reader)?;
            }
            let word = match self.chunk.get(self.pos..) {
                Some([high, low, ..]) => {
                    self.pos += 2;
                    u16::from_be_bytes([*high, *low])
                }
                // An odd trailing byte is the high half of a word.
                Some([high]) => {
                    self.pos += 1;
                    u16::from(*high) << 8
                }
                _ => 0,
            };
            self.bits |= u64::from(word) << self.count;
            self.count += 16;
        }
        Ok(())
    }

    fn load<R: Read + Seek>(&mut self, reader: &mut R) -> DecodeResult<()> {
        self.chunk.drain(..self.pos);
        self.pos = 0;
        let want = usize::try_from(self.unread).unwrap_or(usize::MAX).min(INPUT_CHUNK);
        reader.seek(SeekFrom::Start(self.next_offset))?;
        let start = self.chunk.len();
        self.chunk.resize(start + want, 0);
        reader.read_exact(&mut self.chunk[start..])?;
        self.next_offset += want as u64;
        self.unread -= want as u64;
        Ok(())
    }

    fn peek(&self) -> u32 {
        self.bits as u32
    }

    fn consume(&mut self, n: u32) -> DecodeResult<()> {
        self.consumed += u64::from(n);
        if self.consumed > self.limit {
            return Err(DecodeError::Corrupt("compressed data ends early"));
        }
        self.bits >>= n;
        self.count -= n;
        Ok(())
    }

    fn read<R: Read + Seek>(&mut self, reader: &mut R, n: u32) -> DecodeResult<u32> {
        if n == 0 {
            return Ok(0);
        }
        self.ensure(reader, n)?;
        let value = self.peek() & ((1 << n) - 1);
        self.consume(n)?;
        Ok(value)
    }
}

/// Canonical Huffman decoding table
struct Huffman {
    table_bits: u32,
    /// Indexed by the next `table_bits` input bits: `length << 10 | symbol`, or 0 for longer codes
    table: Vec<u16>,
    /// Number of codes of each length
    counts: [u16; MAX_CODE_LENGTH + 1],
    /// Symbols ordered by code length, then symbol value
    symbols: Vec<u16>,
}

impl Huffman {
    /// Builds the table from code lengths. Over-subscribed lengths are rejected; incomplete
    /// ones are accepted, and decoding fails only if an unassigned code is read.
    fn new(lengths: &[u8], table_bits: u32) -> DecodeResult<Self> {
        let mut counts = [0u16; MAX_CODE_LENGTH + 1];
        for &length in lengths {
            counts[usize::from(length)] += 1;
        }
        counts[0] = 0;
        let mut left = 1i32;
        for &count in &counts[1..] {
            left = left * 2 - i32::from(count);
            if left < 0 {
                return Err(DecodeError::Corrupt("invalid Huffman code lengths"));
            }
        }

        let mut next_code = [0u32; MAX_CODE_LENGTH + 1];
        let mut offsets = [0usize; MAX_CODE_LENGTH + 2];
        let mut code = 0u32;
        for length in 1..=MAX_CODE_LENGTH {
            code = (code + u32::from(counts[length - 1])) << 1;
            next_code[length] = code;
            offsets[length + 1] = offsets[length] + usize::from(counts[length]);
        }

        let mut symbols = vec![0u16; offsets[MAX_CODE_LENGTH + 1]];
        let mut table = vec![0u16; 1 << table_bits];
        for (symbol, &length) in lengths.iter().enumerate() {
            let length = usize::from(length);
            if length == 0 {
                continue;
            }
            symbols[offsets[length]] = symbol as u16;
            offsets[length] += 1;
            let code = next_code[length];
            next_code[length] += 1;
            if length as u32 <= table_bits {
                // The first input bit is the code's most significant bit.
                let reversed = (code.reverse_bits() >> (32 - length)) as usize;
                let entry = (length as u16) << 10 | symbol as u16;
                for index in (reversed..table.len()).step_by(1 << length) {
                    table[index] = entry;
                }
            }
        }
        Ok(Self {
            table_bits,
            table,
            counts,
            symbols,
        })
    }

    fn decode<R: Read + Seek>(&self, bits: &mut BitReader, reader: &mut R) -> DecodeResult<u16> {
        bits.ensure(reader, MAX_CODE_LENGTH as u32)?;
        let input = bits.peek();
        let entry = self.table[(input & ((1 << self.table_bits) - 1)) as usize];
        if entry != 0 {
            bits.consume(u32::from(entry >> 10))?;
            return Ok(entry & 0x3FF);
        }
        // Longer code: walk the canonical code one bit at a time.
        let (mut code, mut first, mut index) = (0u32, 0u32, 0usize);
        for length in 1..=MAX_CODE_LENGTH {
            code |= (input >> (length - 1)) & 1;
            let count = u32::from(self.counts[length]);
            if code < first + count {
                bits.consume(length as u32)?;
                return Ok(self.symbols[index + (code - first) as usize]);
            }
            index += count as usize;
            first = (first + count) << 1;
            code <<= 1;
        }
        Err(DecodeError::Corrupt("invalid Huffman code"))
    }
}

/// Parameters for one section of the main tree's code lengths
struct Section {
    range: std::ops::Range<usize>,
    /// Smallest run for symbols 17 (zeros, 4 extra bits) and 19 (same length, 1 extra bit)
    short_run: u32,
    /// Extra bits and smallest run for symbol 18 (zeros)
    long_bits: u32,
    long_run: u32,
}

const LITERAL_SECTION: Section = Section {
    range: 0..LITERALS,
    short_run: 4,
    long_bits: 5,
    long_run: 20,
};

const MATCH_SECTION: Section = Section {
    range: LITERALS..MAIN_SYMBOLS,
    short_run: 3,
    long_bits: 6,
    long_run: 19,
};

/// Decoding state of one compressed stream (a file, or a merged group of files)
pub(super) struct Decoder {
    bits: BitReader,
    window: Vec<u8>,
    /// Bytes decoded so far
    produced: u64,
    /// Bytes handed out so far; the rest of `produced` is still in the window
    delivered: u64,
    main_lengths: [u8; MAIN_SYMBOLS],
    main: Option<Huffman>,
    aligned: Option<Huffman>,
    block_type: u32,
    block_remaining: u32,
    last_offset: u32,
}

impl Decoder {
    /// Starts decoding `size` packed bytes at archive offset `offset`
    pub(super) fn new(offset: u64, size: u64) -> Self {
        Self {
            bits: BitReader::new(offset, size),
            window: vec![0; WINDOW_SIZE],
            produced: 0,
            delivered: 0,
            main_lengths: [0; MAIN_SYMBOLS],
            main: None,
            aligned: None,
            block_type: 0,
            block_remaining: 0,
            last_offset: 1,
        }
    }

    /// Number of decoded bytes handed out so far
    pub(super) const fn position(&self) -> u64 {
        self.delivered
    }

    /// Decoded bytes still buffered, not yet handed out
    pub(super) const fn buffered_bytes(&self) -> u64 {
        self.produced - self.delivered
    }

    /// Decodes the next `len` bytes and passes them to `sink`
    pub(super) fn read<R: Read + Seek>(&mut self, reader: &mut R, len: u64, sink: &mut dyn FnMut(&[u8])) -> DecodeResult<()> {
        let mut left = len;
        loop {
            let pending = (self.produced - self.delivered).min(left);
            if pending > 0 {
                let start = (self.delivered as usize) & WINDOW_MASK;
                let end = start + pending as usize;
                if end <= WINDOW_SIZE {
                    sink(&self.window[start..end]);
                } else {
                    sink(&self.window[start..]);
                    sink(&self.window[..end - WINDOW_SIZE]);
                }
                self.delivered += pending;
                left -= pending;
            }
            if left == 0 {
                return Ok(());
            }
            let target = left.min(OUTPUT_BATCH);
            while self.produced - self.delivered < target {
                self.decode_token(reader)?;
            }
        }
    }

    fn decode_token<R: Read + Seek>(&mut self, reader: &mut R) -> DecodeResult<()> {
        while self.block_remaining == 0 {
            self.read_block_header(reader)?;
        }
        let main = self.main.as_ref().ok_or(DecodeError::Corrupt("block reuses a tree that was never sent"))?;
        let symbol = usize::from(main.decode(&mut self.bits, reader)?);
        if symbol < LITERALS {
            self.window[(self.produced as usize) & WINDOW_MASK] = symbol as u8;
            self.produced += 1;
            self.block_remaining -= 1;
            return Ok(());
        }

        let position_slot = (symbol - LITERALS) & 31;
        let length_slot = (symbol - LITERALS) >> 5;
        let footer = u32::from(FOOTER_BITS[position_slot]);
        let mut offset = SLOT_BASE[position_slot];
        if self.block_type == 3 && footer >= 3 {
            offset += self.bits.read(reader, footer - 3)? << 3;
            let aligned = self.aligned.as_ref().ok_or(DecodeError::Corrupt("missing aligned offset tree"))?;
            offset += u32::from(aligned.decode(&mut self.bits, reader)?);
        } else {
            offset += self.bits.read(reader, footer)?;
        }
        // Position slot 0 repeats the previous offset.
        if offset != 0 {
            self.last_offset = offset;
        }
        let length = SLOT_BASE[length_slot] + self.bits.read(reader, u32::from(FOOTER_BITS[length_slot]))? + MIN_MATCH;

        let distance = self.last_offset as usize;
        let mut dest = (self.produced as usize) & WINDOW_MASK;
        for _ in 0..length {
            self.window[dest] = self.window[dest.wrapping_sub(distance) & WINDOW_MASK];
            dest = (dest + 1) & WINDOW_MASK;
        }
        self.produced += u64::from(length);
        // A match may end past the block; the next block starts after it.
        self.block_remaining = self.block_remaining.saturating_sub(length);
        Ok(())
    }

    fn read_block_header<R: Read + Seek>(&mut self, reader: &mut R) -> DecodeResult<()> {
        let block_type = self.bits.read(reader, 3)?;
        if !(1..=3).contains(&block_type) {
            return Err(DecodeError::Corrupt("invalid block type"));
        }
        if block_type == 3 {
            let mut lengths = [0u8; ALIGNED_SYMBOLS];
            for length in &mut lengths {
                *length = self.bits.read(reader, 3)? as u8;
            }
            self.aligned = Some(Huffman::new(&lengths, ALIGNED_TABLE_BITS)?);
        }
        let mut length = 0;
        for _ in 0..3 {
            length = length << 8 | self.bits.read(reader, 8)?;
        }
        if block_type != 1 {
            self.read_lengths(reader, &LITERAL_SECTION)?;
            self.read_lengths(reader, &MATCH_SECTION)?;
            self.main = Some(Huffman::new(&self.main_lengths, MAIN_TABLE_BITS)?);
        }
        self.block_type = block_type;
        self.block_remaining = length;
        Ok(())
    }

    /// Updates one section of the main tree's code lengths
    fn read_lengths<R: Read + Seek>(&mut self, reader: &mut R, section: &Section) -> DecodeResult<()> {
        let mut pretree_lengths = [0u8; PRETREE_SYMBOLS];
        for length in &mut pretree_lengths {
            *length = self.bits.read(reader, 4)? as u8;
        }
        let pretree = Huffman::new(&pretree_lengths, PRETREE_TABLE_BITS)?;
        // New length from a pretree symbol 0-16: the previous length minus the symbol, modulo 17
        let delta = |previous: u8, symbol: u16| (previous + 17 - symbol as u8) % 17;

        let mut index = section.range.start;
        while index < section.range.end {
            let symbol = pretree.decode(&mut self.bits, reader)?;
            let (run, value) = match symbol {
                0..=16 => (1, delta(self.main_lengths[index], symbol)),
                17 => (self.bits.read(reader, 4)? + section.short_run, 0),
                18 => (self.bits.read(reader, section.long_bits)? + section.long_run, 0),
                _ => {
                    let run = self.bits.read(reader, 1)? + section.short_run;
                    let symbol = pretree.decode(&mut self.bits, reader)?;
                    if symbol > 16 {
                        return Err(DecodeError::Corrupt("invalid code length delta"));
                    }
                    (run, delta(self.main_lengths[index], symbol))
                }
            };
            let end = index + run as usize;
            if end > section.range.end {
                return Err(DecodeError::Corrupt("code length run overruns its table"));
            }
            self.main_lengths[index..end].fill(value);
            index = end;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    /// Writes bits least significant first into big-endian 16-bit words, like the LZX compressor
    #[derive(Default)]
    struct BitWriter {
        out: Vec<u8>,
        acc: u32,
        count: u32,
    }

    impl BitWriter {
        fn bits(&mut self, value: u32, n: u32) {
            self.acc |= value << self.count;
            self.count += n;
            while self.count >= 16 {
                self.out.extend_from_slice(&(self.acc as u16).to_be_bytes());
                self.acc >>= 16;
                self.count -= 16;
            }
        }

        /// Writes a canonical Huffman code, most significant bit first
        fn code(&mut self, code: u32, length: u32) {
            self.bits(code.reverse_bits() >> (32 - length), length);
        }

        fn finish(mut self) -> Vec<u8> {
            if self.count > 0 {
                self.bits(0, 16 - self.count);
            }
            self.out
        }
    }

    #[derive(Clone, Copy)]
    enum Token {
        Literal(u8),
        /// Offset 0 repeats the previous offset
        Match(u32, u32),
    }

    impl Token {
        const fn len(self) -> u32 {
            match self {
                Self::Literal(_) => 1,
                Self::Match(_, length) => length,
            }
        }
    }

    /// Expands tokens the way the format defines them: a zero-filled history and a repeat offset starting at 1
    fn expand(tokens: &[Token]) -> Vec<u8> {
        let mut out = Vec::new();
        let mut last_offset = 1;
        for &token in tokens {
            match token {
                Token::Literal(byte) => out.push(byte),
                Token::Match(offset, length) => {
                    if offset != 0 {
                        last_offset = offset as usize;
                    }
                    for _ in 0..length {
                        let byte = out.len().checked_sub(last_offset).map_or(0, |index| out[index]);
                        out.push(byte);
                    }
                }
            }
        }
        out
    }

    /// `delta` (0-16), 17, 18 and 19 get the 2-bit codes 00, 01, 10 and 11.
    fn pretree(w: &mut BitWriter, delta: u32) {
        for symbol in 0..20 {
            w.bits(if symbol == delta || symbol >= 17 { 2 } else { 0 }, 4);
        }
    }

    /// Sends 9-bit codes for the literals and main symbols 256-511 (length slots 0-7, lengths up to 18), so symbol `s` has code `s`.
    /// Delta symbol 8 turns a length of 0 into 9; delta 0 keeps the lengths of the previous tree.
    fn main_tree(w: &mut BitWriter, delta: u32) {
        pretree(w, delta);
        // 51 runs of five (symbol 19, extra bit 1), then one single length
        for _ in 0..51 {
            w.code(3, 2);
            w.bits(1, 1);
            w.code(0, 2);
        }
        w.code(0, 2);
        pretree(w, delta);
        // 64 runs of four, then 256 zeros as three runs of 82 (symbol 18) and one of 10 (symbol 17)
        for _ in 0..64 {
            w.code(3, 2);
            w.bits(1, 1);
            w.code(0, 2);
        }
        for _ in 0..3 {
            w.code(2, 2);
            w.bits(63, 6);
        }
        w.code(1, 2);
        w.bits(7, 4);
    }

    fn slot(value: u32) -> usize {
        SLOT_BASE.iter().rposition(|&base| base <= value).unwrap()
    }

    fn block(w: &mut BitWriter, block_type: u32, delta: u32, tokens: &[Token]) {
        w.bits(block_type, 3);
        if block_type == 3 {
            for _ in 0..ALIGNED_SYMBOLS {
                w.bits(3, 3);
            }
        }
        let length: u32 = tokens.iter().map(|t| t.len()).sum();
        for shift in [16, 8, 0] {
            w.bits(length >> shift & 0xFF, 8);
        }
        if block_type != 1 {
            main_tree(w, delta);
        }
        for &token in tokens {
            match token {
                Token::Literal(byte) => w.code(u32::from(byte), 9),
                Token::Match(offset, length) => {
                    let (position_slot, length_slot) = (slot(offset), slot(length - MIN_MATCH));
                    assert!(length_slot < 8);
                    w.code((LITERALS + length_slot * 32 + position_slot) as u32, 9);
                    let footer = u32::from(FOOTER_BITS[position_slot]);
                    let extra = offset - SLOT_BASE[position_slot];
                    if block_type == 3 && footer >= 3 {
                        w.bits(extra >> 3, footer - 3);
                        w.code(extra & 7, 3);
                    } else {
                        w.bits(extra, footer);
                    }
                    w.bits(length - MIN_MATCH - SLOT_BASE[length_slot], u32::from(FOOTER_BITS[length_slot]));
                }
            }
        }
    }

    fn decode_in(stream: &[u8], pieces: &[u64]) -> DecodeResult<Vec<u8>> {
        let mut reader = Cursor::new(stream);
        let mut decoder = Decoder::new(0, stream.len() as u64);
        let mut out = Vec::new();
        for &len in pieces {
            decoder.read(&mut reader, len, &mut |bytes| out.extend_from_slice(bytes))?;
        }
        assert_eq!(decoder.position(), out.len() as u64);
        Ok(out)
    }

    fn decode(stream: &[u8], len: u64) -> DecodeResult<Vec<u8>> {
        decode_in(stream, &[len])
    }

    fn sample_tokens() -> (Vec<Token>, Vec<Token>) {
        let mut first: Vec<Token> = b"LZX test: the quick brown fox jumps over".iter().map(|&b| Token::Literal(b)).collect();
        first.extend([
            Token::Match(32, 10),
            Token::Match(0, 5),
            Token::Literal(b'!'),
            Token::Match(1, 18),
            Token::Match(100, 18),
            Token::Match(49, 4),
        ]);
        let second = vec![
            Token::Match(17, 4),
            Token::Match(48, 9),
            Token::Literal(b'z'),
            Token::Match(0, 3),
            Token::Match(150, 17),
        ];
        (first, second)
    }

    #[test]
    fn aligned_block_then_block_reusing_its_tree() {
        let (first, second) = sample_tokens();
        let mut w = BitWriter::default();
        block(&mut w, 3, 8, &first);
        block(&mut w, 1, 0, &second);
        let stream = w.finish();
        let expected = expand(&[first, second].concat());
        assert_eq!(decode(&stream, expected.len() as u64).unwrap(), expected);
        // Reading in pieces hands out the same bytes.
        let pieces = [7, 0, 50, 1, expected.len() as u64 - 58];
        assert_eq!(decode_in(&stream, &pieces).unwrap(), expected);
    }

    #[test]
    fn verbatim_blocks_send_trees_as_deltas() {
        let (first, second) = sample_tokens();
        let mut w = BitWriter::default();
        block(&mut w, 2, 8, &first);
        block(&mut w, 2, 0, &second);
        let expected = expand(&[first.clone(), second.clone()].concat());
        assert_eq!(decode(&w.finish(), expected.len() as u64).unwrap(), expected);

        // Lengths persist: sending delta 8 again turns every 9 into 1.
        let mut w = BitWriter::default();
        block(&mut w, 2, 8, &first);
        block(&mut w, 2, 8, &second);
        assert_eq!(corrupt(&w.finish(), expected.len() as u64), "invalid Huffman code lengths");
    }

    #[test]
    fn matches_before_the_start_read_zeros() {
        let tokens = [Token::Literal(b'a'), Token::Match(5, 6), Token::Literal(b'b'), Token::Match(0, 3)];
        let mut w = BitWriter::default();
        block(&mut w, 2, 8, &tokens);
        assert_eq!(decode(&w.finish(), 11).unwrap(), b"a\0\0\0\0a\0b\0\0a");
    }

    #[test]
    fn matches_wrap_around_the_window() {
        let mut tokens = vec![Token::Literal(1), Token::Literal(2)];
        tokens.extend(std::iter::repeat_n(Token::Match(2, 18), 8000));
        tokens.push(Token::Match(65535, 18));
        let mut w = BitWriter::default();
        block(&mut w, 3, 8, &tokens);
        let expected = expand(&tokens);
        assert!(expected.len() > WINDOW_SIZE * 2);
        assert_eq!(decode(&w.finish(), expected.len() as u64).unwrap(), expected);
    }

    fn corrupt(stream: &[u8], len: u64) -> &'static str {
        match decode(stream, len) {
            Err(DecodeError::Corrupt(reason)) => reason,
            other => panic!("expected a corrupt stream, got {other:?}"),
        }
    }

    #[test]
    fn invalid_streams_are_rejected() {
        for block_type in [0, 4, 7] {
            let mut w = BitWriter::default();
            w.bits(block_type, 3);
            assert_eq!(corrupt(&w.finish(), 1), "invalid block type");
        }

        let mut w = BitWriter::default();
        block(&mut w, 1, 0, &[Token::Literal(0)]);
        assert_eq!(corrupt(&w.finish(), 1), "block reuses a tree that was never sent");

        // Every aligned offset symbol with a 1-bit code
        let mut w = BitWriter::default();
        w.bits(3, 3);
        for _ in 0..ALIGNED_SYMBOLS {
            w.bits(1, 3);
        }
        assert_eq!(corrupt(&w.finish(), 1), "invalid Huffman code lengths");

        // A zero run (symbol 18, longest run 51) starting at literal 240
        let mut w = BitWriter::default();
        w.bits(2, 3);
        w.bits(1, 24);
        pretree(&mut w, 8);
        for _ in 0..48 {
            w.code(3, 2);
            w.bits(1, 1);
            w.code(0, 2);
        }
        w.code(2, 2);
        w.bits(31, 5);
        assert_eq!(corrupt(&w.finish(), 1), "code length run overruns its table");

        // Delta symbol after symbol 19 must be 0-16
        let mut w = BitWriter::default();
        w.bits(2, 3);
        w.bits(1, 24);
        pretree(&mut w, 8);
        w.code(3, 2);
        w.bits(1, 1);
        w.code(3, 2);
        assert_eq!(corrupt(&w.finish(), 1), "invalid code length delta");
    }

    #[test]
    fn truncated_streams_fail_without_reading_padding() {
        let (first, _) = sample_tokens();
        let mut w = BitWriter::default();
        block(&mut w, 3, 8, &first);
        let stream = w.finish();
        let len = expand(&first).len() as u64;
        for cut in 0..stream.len() - 2 {
            assert!(decode(&stream[..cut], len).is_err(), "cut at {cut}");
        }
        // More output than the stream holds: the zero padding of the last word is no valid block header
        assert_eq!(corrupt(&stream, len + 1000), "invalid block type");
        // Without padding, the stream simply ends.
        let mut w = BitWriter::default();
        block(&mut w, 3, 8, &first);
        w.bits(2, 16 - w.count);
        assert_eq!(corrupt(&w.finish(), len + 1000), "compressed data ends early");
    }

    #[test]
    fn incomplete_trees_fail_only_on_unused_codes() {
        // Only literal 'A' has a code (length 1); the code 1 is unassigned.
        let lengths: Vec<u8> = (0..MAIN_SYMBOLS).map(|s| u8::from(s == usize::from(b'A'))).collect();
        let huffman = Huffman::new(&lengths, MAIN_TABLE_BITS).unwrap();
        let mut reader = Cursor::new([0u8, 0b10]);
        let mut bits = BitReader::new(0, 2);
        assert_eq!(huffman.decode(&mut bits, &mut reader).unwrap(), u16::from(b'A'));
        assert!(matches!(
            huffman.decode(&mut bits, &mut reader),
            Err(DecodeError::Corrupt("invalid Huffman code"))
        ));
    }

    #[test]
    fn long_codes_use_the_slow_path() {
        // Lengths 1, 2, ..., 15, 16, 16: codes longer than the 12-bit table
        let mut lengths = [0u8; 17];
        for (symbol, length) in lengths.iter_mut().enumerate() {
            *length = (symbol + 1).min(16) as u8;
        }
        let huffman = Huffman::new(&lengths, MAIN_TABLE_BITS).unwrap();
        for symbol in [0u16, 11, 12, 15, 16] {
            // Canonical code for symbol s < 16: s ones then a zero; the last two are all ones and 15 ones plus zero.
            let length = u32::from(lengths[usize::from(symbol)]);
            let code = if symbol == 16 { 0xFFFF } else { ((1u32 << length) - 1) & !1 };
            let mut w = BitWriter::default();
            w.code(code, length);
            let stream = w.finish();
            let mut reader = Cursor::new(&stream);
            let mut bits = BitReader::new(0, stream.len() as u64);
            assert_eq!(huffman.decode(&mut bits, &mut reader).unwrap(), symbol, "symbol {symbol}");
        }
    }
}
