// Based on dearc.pas
use std::io;

use bitstream_io::{BitRead, BitReader, LittleEndian};

const BITS: usize = 12;
const CRUNCH_BITS: usize = 12;
const SQUASH_BITS: usize = 13;
const INIT_BITS: usize = 9;
const FIRST: u16 = 257;
const CLEAR: u16 = 256;

pub struct Lzw {
    oldcode: u16,
    finchar: u8,
    n_bits: usize,
    maxcode: u16,
    prefix: [u16; 8191],
    suffix: [u8; 8191],
    clear_flg: bool,
    stack: Vec<u8>,
    free_ent: u16,
    maxcodemax: u16,
}

impl Lzw {
    pub fn new() -> Self {
        Lzw {
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

        if let Ok(code) = reader.read::<u16>(self.n_bits as u32) {
            Some(code as u16)
        } else {
            None
        }
    }

    pub fn decomp(&mut self, mut input: &[u8], use_crunched: bool) -> io::Result<Vec<u8>> {
        let mut result = Vec::new();
        let bits = if use_crunched {
            let b = input[0];
            input = &input[1..];
            if b as usize != BITS {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("File packed with {}, I can only handle {}", b, BITS),
                ));
            }
            CRUNCH_BITS
        } else {
            SQUASH_BITS
        };
        self.maxcodemax = 1 << bits;

        self.clear_flg = false;
        self.n_bits = INIT_BITS;
        self.maxcode = (1 << self.n_bits) - 1;
        for code in 0..256 {
            self.suffix[code] = code as u8;
        }

        let mut reader: BitReader<&[u8], LittleEndian> = BitReader::endian(input, LittleEndian);
        self.free_ent = FIRST;
        self.oldcode = if let Some(old) = self.getcode(&mut reader) {
            old as u16
        } else {
            return Ok(result);
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
            if code >= self.free_ent as u16 {
                self.stack.push(self.finchar as u8);
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
            if code < self.maxcodemax {
                self.prefix[code as usize] = self.oldcode;
                self.suffix[code as usize] = self.finchar;
                self.free_ent = code + 1;
            }
            self.oldcode = incode;
        }
        Ok(result)
    }
}
