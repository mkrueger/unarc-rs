use bitstream_io::{BitRead, BitReader, LittleEndian};

use crate::error::Result;

const BITS: usize = 12;
const INIT_BITS: usize = 9;
const FIRST: u32 = 257;
const CLEAR: u16 = 256;

pub struct Lzw {
    oldcode: u16,
    finchar: u8,
    n_bits: usize,
    maxcode: u32,
    prefix: [u16; 8191],
    suffix: [u8; 8191],
    clear_flg: bool,
    stack: Vec<u8>,
    free_ent: u32,
    maxcodemax: u32,
    max_bits: u8,
    block_mode: bool,
}

impl Lzw {
    pub fn new(max_bits: u8, block_mode: bool) -> Self {
        Lzw {
            max_bits,
            block_mode,
            oldcode: 0,
            finchar: 0,
            n_bits: 0,
            maxcode: 0,
            prefix: [0; 8191],
            suffix: [0; 8191],
            clear_flg: false,
            stack: Vec::new(),
            free_ent: FIRST,
            maxcodemax: 0,
        }
    }

    fn getcode(&mut self, reader: &mut BitReader<&[u8], LittleEndian>) -> Option<u16> {
        if self.clear_flg || self.free_ent > self.maxcode {
            if self.free_ent > self.maxcode {
                self.n_bits += 1;
                if self.n_bits == BITS {
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

        reader.read_var::<u16>(self.n_bits as u32).ok()
    }

    pub fn decomp(&mut self, input: &[u8]) -> Result<Vec<u8>> {
        let mut result = Vec::new();
        self.maxcodemax = 1 << self.max_bits;

        self.clear_flg = false;
        self.n_bits = INIT_BITS;
        self.maxcode = (1 << self.n_bits) - 1;
        for code in 0..256 {
            self.suffix[code] = code as u8;
        }

        let mut reader = BitReader::endian(input, LittleEndian);
        self.free_ent = if self.block_mode { FIRST } else { 256 };
        self.oldcode = if let Some(old) = self.getcode(&mut reader) {
            old
        } else {
            return Ok(result);
        };
        self.finchar = self.oldcode as u8;
        result.push(self.finchar);

        while let Some(mut code) = self.getcode(&mut reader) {
            if code == CLEAR && self.block_mode {
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
            if code >= self.free_ent as u16 {
                self.stack.push(self.finchar);
                code = self.oldcode;
            }
            while code >= 256 {
                self.stack.push(self.suffix[code as usize]);
                code = self.prefix[code as usize];
            }
            self.finchar = self.suffix[code as usize];
            self.stack.push(self.finchar);
            result.extend(self.stack.drain(..).rev());

            code = self.free_ent as u16;
            if (code as u32) < self.maxcodemax {
                self.prefix[code as usize] = self.oldcode;
                self.suffix[code as usize] = self.finchar;
                self.free_ent = code as u32 + 1;
            }
            self.oldcode = incode;
        }
        Ok(result)
    }
}
