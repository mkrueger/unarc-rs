//! Bit stream reader for ACE decompression
//!
//! ACE uses Intel-endian 32-bit-byte-swapped, MSB first bitstream.
//! Data is read in 4-byte little-endian chunks, then bits are extracted MSB first.

use std::io::Read;

use crate::error::{ArchiveError, Result};

/// Bit stream reader (ACE format: little-endian 32-bit words, MSB first bits)
pub struct BitStream<R: Read> {
    #[allow(dead_code)]
    reader: R,
    /// Buffer of 32-bit words (little-endian from file)
    buffer: Vec<u32>,
    /// Current position in bits
    pos: usize,
    /// Total length in bits
    len: usize,
}

impl<R: Read> BitStream<R> {
    /// Create a new bit stream from a reader with known size
    pub fn new(mut reader: R, size: usize) -> Self {
        // Read all data upfront for simplicity (ACE files are typically small)
        let mut data = vec![0u8; size];
        let _ = reader.read(&mut data);

        // Convert to 32-bit words (little-endian)
        let mut buffer = Vec::with_capacity(size.div_ceil(4));
        for chunk in data.chunks(4) {
            let mut bytes = [0u8; 4];
            bytes[..chunk.len()].copy_from_slice(chunk);
            buffer.push(u32::from_le_bytes(bytes));
        }

        let len = buffer.len() * 32;
        Self {
            reader,
            buffer,
            pos: 0,
            len,
        }
    }

    /// Get bits from a 32-bit value (MSB first)
    fn get_bits(value: u32, start: usize, length: usize) -> u32 {
        if length == 0 {
            return 0;
        }
        let mask = ((0xFFFFFFFFu64 << (32 - length)) & 0xFFFFFFFF) >> start;
        ((value as u64 & mask) >> (32 - length - start)) as u32
    }

    /// Peek at the next n bits without consuming them
    pub fn peek_bits(&mut self, n: u8) -> Result<u32> {
        if n == 0 {
            return Ok(0);
        }
        let bits = n as usize;

        // Check if we have enough bits (with padding)
        if self.pos + bits > self.len + 31 {
            return Err(ArchiveError::decompression_failed(
                "bitstream",
                "unexpected end of data",
            ));
        }

        // Ensure buffer has padding word
        while self.pos + bits > self.buffer.len() * 32 {
            self.buffer.push(0);
        }

        let word_idx = self.pos / 32;
        let bit_idx = self.pos % 32;

        // How many bits from first word
        let peeked = bits.min(32 - bit_idx);
        let mut res = Self::get_bits(self.buffer[word_idx], bit_idx, peeked) as u64;

        // Additional full words
        let mut total_peeked = peeked;
        while bits - total_peeked >= 32 {
            res <<= 32;
            res += self.buffer[(self.pos + total_peeked) / 32] as u64;
            total_peeked += 32;
        }

        // Remaining bits
        if bits > total_peeked {
            let remaining = bits - total_peeked;
            res <<= remaining;
            res += Self::get_bits(self.buffer[(self.pos + total_peeked) / 32], 0, remaining) as u64;
        }

        Ok(res as u32)
    }

    /// Skip n bits
    pub fn skip_bits(&mut self, n: u8) -> Result<()> {
        self.pos += n as usize;
        Ok(())
    }

    /// Read n bits and consume them
    pub fn read_bits(&mut self, n: u8) -> Result<u32> {
        let value = self.peek_bits(n)?;
        self.skip_bits(n)?;
        Ok(value)
    }

    /// Read a value with known bit width (MSB is always 1, not encoded)
    pub fn read_known_width_uint(&mut self, bits: u8) -> Result<u32> {
        if bits < 2 {
            return Ok(bits as u32);
        }
        let actual_bits = bits - 1;
        let value = self.read_bits(actual_bits)?;
        Ok(value + (1 << actual_bits))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    /// Test based on Python acefile doctest:
    /// >>> bs = BitStream(io.BytesIO(b'01234567'))
    /// >>> bs.peek_bits(31)
    /// 429463704
    #[test]
    fn test_python_compatibility() {
        let data = b"01234567";
        let mut bs = BitStream::new(Cursor::new(data.to_vec()), 8);

        // Python: peek_bits(31) = 429463704
        let result = bs.peek_bits(31).unwrap();
        println!("peek_bits(31) = {} (expected 429463704)", result);
        assert_eq!(result, 429463704);

        // Python: read_bits(31) = 429463704
        let result = bs.read_bits(31).unwrap();
        assert_eq!(result, 429463704);

        // Python: skip_bits(3)
        bs.skip_bits(3).unwrap();

        // Python: read_bits(5) = 27
        let result = bs.read_bits(5).unwrap();
        println!("read_bits(5) = {} (expected 27)", result);
        assert_eq!(result, 27);
    }
}
