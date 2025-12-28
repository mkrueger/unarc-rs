//! Arithmetic Decoder for HA archive decompression
//!
//! Implements a 16-bit range coder with E3 (underflow) scaling.
//! Based on the standard arithmetic coding algorithm as described in:
//! - Witten, Neal, Cleary: "Arithmetic Coding for Data Compression" (1987)
//! - Moffat, Neal, Witten: "Arithmetic Coding Revisited" (1998)

use std::io::{BufReader, Read};

use crate::error::Result;

/// Full 16-bit range for interval arithmetic
const RANGE_MAX: u16 = 0xFFFF;

/// MSB mask for convergence check
const MSB_MASK: u16 = 0x8000;

/// Second-MSB mask for underflow (E3) check
const UNDERFLOW_MASK: u16 = 0x4000;

/// Arithmetic decoder using 16-bit precision with E3 underflow prevention.
///
/// The decoder maintains an interval [low, high] and a code value read from
/// the bitstream. As symbols are decoded, the interval narrows and bits are
/// shifted out when the MSBs converge.
pub struct ArithmeticDecoder<R: Read> {
    /// Buffered input stream
    input: BufReader<R>,
    /// Upper bound of current interval
    high: u16,
    /// Lower bound of current interval
    low: u16,
    /// Current code value from bitstream
    code: u16,
    /// Buffered byte for bit extraction
    byte_buffer: u8,
    /// Remaining bits in buffer (0-8)
    bits_remaining: u8,
}

impl<R: Read> ArithmeticDecoder<R> {
    /// Initialize decoder by reading the first 16 bits of encoded data.
    #[inline]
    pub fn new(reader: R) -> Result<Self> {
        let mut input = BufReader::new(reader);

        // Bootstrap: read initial 16-bit code value (big-endian)
        let mut initial_bytes = [0u8; 2];
        input.read_exact(&mut initial_bytes)?;
        let code = u16::from_be_bytes(initial_bytes);

        Ok(Self {
            input,
            high: RANGE_MAX,
            low: 0,
            code,
            byte_buffer: 0,
            bits_remaining: 0,
        })
    }

    /// Extract next bit from input stream (MSB first within each byte).
    #[inline(always)]
    fn read_bit(&mut self) -> u16 {
        if self.bits_remaining == 0 {
            let mut byte = [0u8; 1];
            // On EOF, feed zeros (standard practice for trailing bits)
            if self.input.read_exact(&mut byte).is_ok() {
                self.byte_buffer = byte[0];
            } else {
                self.byte_buffer = 0;
            }
            self.bits_remaining = 8;
        }

        self.bits_remaining -= 1;
        ((self.byte_buffer >> self.bits_remaining) & 1) as u16
    }

    /// Compute threshold for symbol lookup given total frequency.
    ///
    /// Returns a value in [0, total-1] indicating where the current code
    /// falls within the frequency distribution.
    #[inline]
    pub fn threshold_val(&self, total: u16) -> u16 {
        // Interval width = high - low + 1
        let range = (self.high - self.low) as u32 + 1;
        // Position of code within interval, scaled to [0, total)
        let offset = (self.code - self.low) as u32 + 1;
        ((offset * total as u32 - 1) / range) as u16
    }

    /// Narrow interval after decoding a symbol and renormalize.
    ///
    /// - `cum_low`: cumulative frequency of all symbols before this one
    /// - `cum_high`: cumulative frequency up to and including this symbol
    /// - `total`: sum of all symbol frequencies
    #[inline]
    pub fn decode_update(&mut self, cum_low: u16, cum_high: u16, total: u16) -> Result<()> {
        let range = (self.high - self.low) as u32 + 1;
        let scale = total as u32;

        // Narrow interval to [new_low, new_high]
        let new_high = self.low.wrapping_add(((range * cum_high as u32 / scale) - 1) as u16);
        let new_low = self.low.wrapping_add((range * cum_low as u32 / scale) as u16);

        self.high = new_high;
        self.low = new_low;

        // Renormalization loop
        self.renormalize();

        Ok(())
    }

    /// Shift out converged bits and handle E3 underflow scaling.
    #[inline(always)]
    fn renormalize(&mut self) {
        loop {
            if (self.high ^ self.low) & MSB_MASK == 0 {
                // MSBs match: shift out and read new bit
                self.shift_out_msb();
            } else if (self.low & UNDERFLOW_MASK) != 0 && (self.high & UNDERFLOW_MASK) == 0 {
                // E3 underflow: low = 01..., high = 10...
                // Expand interval around midpoint
                self.handle_underflow();
            } else {
                // Interval is normalized
                break;
            }
        }
    }

    /// Shift out matching MSB and read next bit into code.
    #[inline(always)]
    fn shift_out_msb(&mut self) {
        self.low <<= 1;
        self.high = (self.high << 1) | 1;
        self.code = (self.code << 1) | self.read_bit();
    }

    /// Handle E3 scaling by flipping MSB and shifting.
    #[inline(always)]
    fn handle_underflow(&mut self) {
        // Clear bit 14 of low, set bit 15 and bit 0 of high
        self.low = (self.low << 1) & 0x7FFF;
        self.high = (self.high << 1) | 0x8001;
        // Flip MSB of code and shift in new bit
        self.code = ((self.code << 1) ^ MSB_MASK) | self.read_bit();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_initialization() {
        let data = vec![0xAB, 0xCD];
        let decoder = ArithmeticDecoder::new(Cursor::new(data)).unwrap();

        assert_eq!(decoder.low, 0);
        assert_eq!(decoder.high, 0xFFFF);
        assert_eq!(decoder.code, 0xABCD);
    }

    #[test]
    fn test_threshold_midpoint() {
        // Code at exact midpoint of full range
        let data = vec![0x80, 0x00];
        let decoder = ArithmeticDecoder::new(Cursor::new(data)).unwrap();

        // threshold = ((0x8000 + 1) * 256 - 1) / 0x10000 = 128
        assert_eq!(decoder.threshold_val(256), 128);
    }

    #[test]
    fn test_threshold_boundaries() {
        // Code at minimum
        let data = vec![0x00, 0x00];
        let decoder = ArithmeticDecoder::new(Cursor::new(data)).unwrap();
        assert_eq!(decoder.threshold_val(100), 0);

        // Code at maximum
        let data = vec![0xFF, 0xFF];
        let decoder = ArithmeticDecoder::new(Cursor::new(data)).unwrap();
        assert_eq!(decoder.threshold_val(100), 99);
    }
}
