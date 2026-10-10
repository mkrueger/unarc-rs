// Based on dearc.pas

use bitstream_io::{BitRead, BitReader, LittleEndian};

use crate::error::{ArchiveError, Result};

const BITS: usize = 12;
const CRUNCH_BITS: usize = 12;
const SQUASH_BITS: usize = 13;
const INIT_BITS: usize = 9;
const FIRST: u16 = 257;
const CLEAR: u16 = 256;
/// Codes 0..8191: the largest table, squashing's 13 bits.
const TABLE_SIZE: usize = 1 << SQUASH_BITS;
/// The encoder writes codes in groups of eight (`n_bits` bytes), like compress.
const GROUP_CODES: u32 = 8;

pub struct Lzw {
    oldcode: u16,
    finchar: u8,
    n_bits: usize,
    maxcode: u16,
    prefix: [u16; TABLE_SIZE],
    suffix: [u8; TABLE_SIZE],
    clear_flg: bool,
    stack: Vec<u8>,
    free_ent: u16,
    maxcodemax: u16,
    /// Code width limit of the current method: 12 when crunched, 13 when squashed.
    max_bits: usize,
    /// Codes read from the current group of eight.
    group_codes: u32,
}

impl Default for Lzw {
    fn default() -> Self {
        Self::new()
    }
}

impl Lzw {
    pub fn new() -> Self {
        Lzw {
            oldcode: 0,
            finchar: 0,
            n_bits: 0,
            maxcode: 0,
            prefix: [0; TABLE_SIZE],
            suffix: [0; TABLE_SIZE],
            clear_flg: false,
            stack: Vec::new(),
            free_ent: FIRST,
            maxcodemax: 0,
            max_bits: CRUNCH_BITS,
            group_codes: 0,
        }
    }

    fn getcode(&mut self, reader: &mut BitReader<&[u8], LittleEndian>) -> Option<u16> {
        if self.clear_flg || self.free_ent > self.maxcode {
            // A new code width starts a new group: the encoder pads the current one to
            // eight codes. Width increases happen to fall on group boundaries, but a
            // CLEAR can come anywhere, so skip the padding as z/lzw.rs does.
            if self.group_codes != 0 {
                let padding = (GROUP_CODES - self.group_codes) * self.n_bits as u32;
                self.group_codes = 0;
                if reader.skip(padding).is_err() {
                    return None;
                }
            }
            if self.free_ent > self.maxcode {
                self.n_bits += 1;
                if self.n_bits == self.max_bits {
                    self.maxcode = self.maxcodemax;
                } else {
                    self.maxcode = (1 << self.n_bits) - 1;
                }
            }
            if self.clear_flg {
                self.clear_flg = false;
                self.n_bits = INIT_BITS;
                self.maxcode = (1 << self.n_bits) - 1;
            }
        }

        let code = reader.read_var::<u16>(self.n_bits as u32).ok()?;
        self.group_codes = (self.group_codes + 1) % GROUP_CODES;
        Some(code)
    }

    pub fn decomp(&mut self, input: &[u8], use_crunched: bool) -> Result<Vec<u8>> {
        let mut result = Vec::new();
        self.decomp_into(input, use_crunched, &mut result)?;
        Ok(result)
    }

    /// Decompress into an existing buffer to avoid allocations.
    pub fn decomp_into(&mut self, mut input: &[u8], use_crunched: bool, result: &mut Vec<u8>) -> Result<()> {
        result.clear();
        let bits = if use_crunched {
            let b = input[0];
            input = &input[1..];
            if b as usize != BITS {
                return Err(ArchiveError::decompression_failed(
                    "LZW",
                    format!("file packed with {} bits, expected {}", b, BITS),
                ));
            }
            CRUNCH_BITS
        } else {
            SQUASH_BITS
        };
        self.maxcodemax = 1 << bits;
        self.max_bits = bits;
        self.group_codes = 0;
        self.stack.clear();

        self.clear_flg = false;
        self.n_bits = INIT_BITS;
        self.maxcode = (1 << self.n_bits) - 1;
        for code in 0..256 {
            self.suffix[code] = code as u8;
        }

        let mut reader = BitReader::endian(input, LittleEndian);
        self.free_ent = FIRST;
        self.oldcode = if let Some(old) = self.getcode(&mut reader) {
            old
        } else {
            return Ok(());
        };
        self.finchar = self.oldcode as u8;
        result.push(self.finchar);

        while let Some(mut code) = self.getcode(&mut reader) {
            if code == CLEAR {
                self.prefix.fill(0);
                self.clear_flg = true;
                self.free_ent = FIRST - 1;
                if let Some(c) = self.getcode(&mut reader) {
                    code = c;
                } else {
                    break;
                }
            }
            let incode = code;
            if code >= self.free_ent {
                self.stack.push(self.finchar);
                code = self.oldcode;
            }
            // Safety limit to prevent infinite loops on corrupt data
            let mut iterations = 0usize;
            const MAX_ITERATIONS: usize = 65536;
            while code >= 256 {
                iterations += 1;
                if iterations > MAX_ITERATIONS {
                    return Err(ArchiveError::decompression_failed(
                        "LZW",
                        "infinite loop detected (corrupt data or wrong password)",
                    ));
                }
                self.stack.push(self.suffix[code as usize]);
                code = self.prefix[code as usize];
            }
            self.finchar = self.suffix[code as usize];
            self.stack.push(self.finchar);
            result.extend(self.stack.drain(..).rev());
            code = self.free_ent;
            if code < self.maxcodemax {
                self.prefix[code as usize] = self.oldcode;
                self.suffix[code as usize] = self.finchar;
                self.free_ent = code + 1;
            }
            self.oldcode = incode;
        }
        Ok(())
    }
}
