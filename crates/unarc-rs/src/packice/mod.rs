//! Pack-Ice compressed file support
//!
//! Pack-Ice is a compression format developed for the Atari ST platform,
//! primarily used for compressing executable files and picture data
//! (e.g., Degas Elite `.PI?` formats).
//!
//! Pack-Ice uses a backward LZ77 variant with variable-length codes.
//! Three major versions exist:
//! - v0 (1.1–1.14): Footer-based with `Ice!` magic at end
//! - v1 (2.0–2.20): Header-based with `Ice!`, `TMM!`, `TSM!`, `SHE!`
//! - v2 (2.31+): Header-based with `ICE!`, byte-oriented bitstream

use crate::error::{ArchiveError, Result};

const FOURCC_ICE1: u32 = u32::from_be_bytes(*b"Ice!");
const FOURCC_ICE2: u32 = u32::from_be_bytes(*b"ICE!");
const FOURCC_TMM: u32 = u32::from_be_bytes(*b"TMM!");
const FOURCC_TSM: u32 = u32::from_be_bytes(*b"TSM!");
const FOURCC_SHE: u32 = u32::from_be_bytes(*b"SHE!");

/// Pack-Ice archive reader
pub struct PackIceArchive {
    #[allow(dead_code)]
    original_size: u32,
    data: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
struct Header {
    ver: u8,
    packed_end: usize,
    raw_size: u32,
}

fn be_u32(data: &[u8], off: usize) -> Result<u32> {
    if off + 4 > data.len() {
        return Err(ArchiveError::invalid_header("PackIce"));
    }
    Ok(u32::from_be_bytes(data[off..off + 4].try_into().unwrap()))
}

/// Check if the given data starts with a Pack-Ice magic signature
pub fn is_pack_ice(data: &[u8]) -> bool {
    if data.len() < 4 {
        return false;
    }
    let hdr = u32::from_be_bytes(data[0..4].try_into().unwrap());
    if matches!(hdr, FOURCC_ICE1 | FOURCC_ICE2 | FOURCC_TMM | FOURCC_TSM | FOURCC_SHE) {
        return true;
    }
    // Check footer for v0
    if data.len() >= 8 {
        let footer = u32::from_be_bytes(data[data.len() - 4..].try_into().unwrap());
        if footer == FOURCC_ICE1 {
            return true;
        }
    }
    false
}

fn parse_header(data: &[u8]) -> Result<Header> {
    if data.len() < 8 {
        return Err(ArchiveError::invalid_header("PackIce"));
    }

    let hdr = be_u32(data, 0)?;
    let footer = if data.len() >= 4 {
        u32::from_be_bytes(data[data.len() - 4..].try_into().unwrap())
    } else {
        0
    };

    // ver 0 is detected by footer == "Ice!" (packed size known only with full data).
    if footer == FOURCC_ICE1 {
        if data.len() < 8 {
            return Err(ArchiveError::invalid_header("PackIce"));
        }
        let raw_size = be_u32(data, data.len() - 8)?;
        if raw_size == 0 {
            return Err(ArchiveError::invalid_header("PackIce"));
        }
        return Ok(Header {
            ver: 0,
            packed_end: data.len() - 8,
            raw_size,
        });
    }

    // ver 1 / 2 are detected by 4-byte header.
    let is_v1 = matches!(hdr, FOURCC_ICE1 | FOURCC_TMM | FOURCC_TSM | FOURCC_SHE);
    let is_v2 = hdr == FOURCC_ICE2;
    if !is_v1 && !is_v2 {
        return Err(ArchiveError::invalid_header("PackIce"));
    }
    if data.len() < 12 {
        return Err(ArchiveError::invalid_header("PackIce"));
    }

    let packed_end = be_u32(data, 4)? as usize;
    if packed_end == 0 || packed_end > data.len() {
        return Err(ArchiveError::invalid_header("PackIce"));
    }
    let raw_size = be_u32(data, 8)?;
    if raw_size == 0 {
        return Err(ArchiveError::invalid_header("PackIce"));
    }

    Ok(Header {
        ver: if is_v2 { 2 } else { 1 },
        packed_end,
        raw_size,
    })
}

fn decompress(data: &[u8], header: &Header) -> Result<Vec<u8>> {
    let raw_size = header.raw_size as usize;
    let mut out = vec![0u8; raw_size];

    match header.ver {
        0 => decompress_internal(data, header, false, &mut out)?,
        1 => {
            // Mixed v1/v2 with identical IDs exists; try v1 bitstream first.
            if decompress_internal(data, header, false, &mut out).is_err() {
                out.fill(0);
                decompress_internal(data, header, true, &mut out)?;
            }
        }
        2 => decompress_internal(data, header, true, &mut out)?,
        _ => return Err(ArchiveError::invalid_header("PackIce")),
    }

    Ok(out)
}

struct BackwardInputStream<'a> {
    data: &'a [u8],
    low: usize,
    pos: usize,
}

impl<'a> BackwardInputStream<'a> {
    fn new(data: &'a [u8], start_offset: usize, end_offset: usize) -> Result<Self> {
        if start_offset > end_offset || end_offset > data.len() {
            return Err(ArchiveError::decompression_failed("PackIce", "invalid input bounds"));
        }
        Ok(Self {
            data,
            low: start_offset,
            pos: end_offset,
        })
    }

    fn read_u8(&mut self) -> Result<u8> {
        if self.pos <= self.low {
            return Err(ArchiveError::decompression_failed("PackIce", "unexpected end of input"));
        }
        self.pos -= 1;
        Ok(self.data[self.pos])
    }

    fn read_be32(&mut self) -> Result<u32> {
        let b0 = self.read_u8()? as u32;
        let b1 = self.read_u8()? as u32;
        let b2 = self.read_u8()? as u32;
        let b3 = self.read_u8()? as u32;
        Ok((b3 << 24) | (b2 << 16) | (b1 << 8) | b0)
    }

    fn eof(&self) -> bool {
        self.pos == self.low
    }

    fn available_bytes(&self) -> usize {
        self.pos.saturating_sub(self.low)
    }
}

struct MsbBitReader<'a> {
    input: BackwardInputStream<'a>,
    buf: u32,
    len: u8,
}

impl<'a> MsbBitReader<'a> {
    fn new(input: BackwardInputStream<'a>) -> Self {
        Self { input, buf: 0, len: 0 }
    }

    fn reset(&mut self, buf_content: u32, buf_len: u8) {
        self.buf = buf_content;
        self.len = buf_len;
    }

    fn read_bits_generic<F>(&mut self, mut count: u32, mut read_word: F) -> Result<u32>
    where
        F: FnMut(&mut BackwardInputStream<'a>) -> Result<(u32, u8)>,
    {
        if count > 32 {
            return Err(ArchiveError::decompression_failed("PackIce", "bit count too large"));
        }
        let mut ret: u32 = 0;
        while count > 0 {
            if self.len == 0 {
                let (content, len) = read_word(&mut self.input)?;
                self.buf = content;
                self.len = len;
            }
            let max_count = (count as u8).min(self.len);
            self.len -= max_count;
            let mask = if max_count == 32 { u32::MAX } else { (1u32 << max_count) - 1 };
            ret = (ret << max_count) | ((self.buf >> self.len) & mask);
            count -= max_count as u32;
        }
        Ok(ret)
    }

    fn read_bits8(&mut self, count: u32) -> Result<u32> {
        self.read_bits_generic(count, |inp| Ok((inp.read_u8()? as u32, 8)))
    }

    fn read_bits_be32(&mut self, count: u32) -> Result<u32> {
        self.read_bits_generic(count, |inp| Ok((inp.read_be32()?, 32)))
    }

    fn available_bits(&self) -> usize {
        self.input.available_bytes() * 8 + self.len as usize
    }

    fn input_eof(&self) -> bool {
        self.input.eof()
    }

    fn read_u8(&mut self) -> Result<u8> {
        self.input.read_u8()
    }

    fn read_be32(&mut self) -> Result<u32> {
        self.input.read_be32()
    }
}

struct BackwardOutputStream<'a> {
    out: &'a mut [u8],
    pos: usize,
}

impl<'a> BackwardOutputStream<'a> {
    fn new(out: &'a mut [u8]) -> Self {
        Self { pos: out.len(), out }
    }

    fn write_byte(&mut self, b: u8) -> Result<()> {
        if self.pos == 0 {
            return Err(ArchiveError::decompression_failed("PackIce", "output overflow"));
        }
        self.pos -= 1;
        self.out[self.pos] = b;
        Ok(())
    }

    fn eof(&self) -> bool {
        self.pos == 0
    }

    fn copy(&mut self, distance: usize, count: usize) -> Result<()> {
        if distance == 0 {
            return Err(ArchiveError::decompression_failed("PackIce", "invalid distance"));
        }
        for _ in 0..count {
            if self.pos == 0 {
                return Err(ArchiveError::decompression_failed("PackIce", "output overflow"));
            }
            let src = self
                .pos
                .checked_add(distance)
                .ok_or_else(|| ArchiveError::decompression_failed("PackIce", "distance overflow"))?;
            if src >= self.out.len() {
                return Err(ArchiveError::decompression_failed("PackIce", "invalid back-reference"));
            }
            let b = self.out[src];
            self.pos -= 1;
            self.out[self.pos] = b;
        }
        Ok(())
    }
}

#[derive(Clone)]
struct VariableLengthCodeDecoder {
    bit_lengths: Vec<u8>,
    offsets: Vec<u32>,
}

impl VariableLengthCodeDecoder {
    fn new(args: &[i32]) -> Self {
        let mut bit_lengths = Vec::with_capacity(args.len());
        let mut offsets = vec![0u32; args.len()];
        let mut length: u32 = 0;
        for (i, &v) in args.iter().enumerate() {
            let bl = if v < 0 { (-v) as u8 } else { v as u8 };
            bit_lengths.push(bl);
            if v < 0 {
                offsets[i] = 0;
                length = 1u32 << (-v);
            } else {
                offsets[i] = length;
                length = length.wrapping_add(1u32 << v);
            }
        }
        Self { bit_lengths, offsets }
    }

    fn decode(&self, br: &mut MsbBitReader<'_>, use_bytes: bool, base: u32) -> Result<u32> {
        let base = base as usize;
        if base >= self.bit_lengths.len() {
            return Err(ArchiveError::decompression_failed("PackIce", "bad VLC base"));
        }
        let bl = self.bit_lengths[base];
        Ok(self.offsets[base] + read_bits(br, use_bytes, bl as u32)?)
    }

    fn decode_cascade(&self, br: &mut MsbBitReader<'_>, use_bytes: bool) -> Result<u32> {
        for i in 0..self.bit_lengths.len() {
            let bl = self.bit_lengths[i];
            if bl == 0 {
                return Err(ArchiveError::decompression_failed("PackIce", "bad VLC"));
            }
            let tmp = read_bits(br, use_bytes, bl as u32)?;
            if i == self.bit_lengths.len() - 1 || tmp != (1u32 << bl) - 1 {
                return Ok(self.offsets[i] - (i as u32) + tmp);
            }
        }
        Err(ArchiveError::decompression_failed("PackIce", "bad VLC cascade"))
    }
}

fn read_bits(br: &mut MsbBitReader<'_>, use_bytes: bool, count: u32) -> Result<u32> {
    if use_bytes {
        br.read_bits8(count)
    } else {
        br.read_bits_be32(count)
    }
}

fn decompress_internal(data: &[u8], header: &Header, use_bytes: bool, out: &mut [u8]) -> Result<()> {
    let (start_offset, end_offset) = if header.ver == 0 {
        (0usize, header.packed_end)
    } else {
        (12usize, header.packed_end)
    };

    let input = BackwardInputStream::new(data, start_offset, end_offset)?;
    let mut br = MsbBitReader::new(input);

    // anchor-bit handling
    {
        let value = if use_bytes { br.read_u8()? as u32 } else { br.read_be32()? };
        let mut tmp = value;
        let mut count: u32 = 0;
        while tmp != 0 {
            tmp <<= 1;
            count += 1;
        }
        count = count.saturating_sub(1);
        if count > 0 {
            let buf_content = value >> (32 - count);
            let buf_len = count - if use_bytes { 24 } else { 0 };
            br.reset(buf_content, buf_len as u8);
        }
    }

    let mut os = BackwardOutputStream::new(out);

    let lit_vlc_old = VariableLengthCodeDecoder::new(&[1, 2, 2, 3, 10]);
    let lit_vlc_new = VariableLengthCodeDecoder::new(&[1, 2, 2, 3, 8, 15]);
    let count_base_dec = VariableLengthCodeDecoder::new(&[1, 1, 1, 1]);
    let count_dec = VariableLengthCodeDecoder::new(&[0, 0, 1, 2, 10]);
    let dist_base_dec = VariableLengthCodeDecoder::new(&[1, 1]);
    let dist_dec = VariableLengthCodeDecoder::new(&[5, 8, 12]);

    loop {
        if read_bits(&mut br, use_bytes, 1)? != 0 {
            let lit_len = if header.ver != 0 {
                lit_vlc_new.decode_cascade(&mut br, use_bytes)? + 1
            } else {
                lit_vlc_old.decode_cascade(&mut br, use_bytes)? + 1
            };
            for _ in 0..lit_len {
                os.write_byte(br.read_u8()?)?;
            }
        }

        if os.eof() {
            break;
        }

        let count_base = count_base_dec.decode_cascade(&mut br, use_bytes)?;
        let count = count_dec.decode(&mut br, use_bytes, count_base)? + 2;

        let distance: u32;
        if count == 2 {
            let mut d = if read_bits(&mut br, use_bytes, 1)? != 0 {
                read_bits(&mut br, use_bytes, 9)? + 0x40
            } else {
                read_bits(&mut br, use_bytes, 6)?
            };
            d += count - if use_bytes { 1 } else { 0 };
            distance = d;
        } else {
            let mut dist_base = dist_base_dec.decode_cascade(&mut br, use_bytes)?;
            if dist_base < 2 {
                dist_base ^= 1;
            }
            let mut d = dist_dec.decode(&mut br, use_bytes, dist_base)?;
            if use_bytes {
                if d != 0 {
                    d += count - 1;
                } else {
                    d = 1;
                }
            } else {
                d += count;
            }
            distance = d;
        }

        os.copy(distance as usize, count as usize)?;
    }

    // picture mode
    if header.ver != 0 && br.available_bits() > 0 && read_bits(&mut br, use_bytes, 1)? != 0 {
        let mut picture_size: u32 = 32000;
        if header.ver == 2 {
            // Format changes between versions.
            if br.available_bits() >= 17 && read_bits(&mut br, use_bytes, 1)? != 0 {
                picture_size = read_bits(&mut br, use_bytes, 16)? * 8 + 8;
            }
        }

        let picture_size = picture_size as usize;
        if out.len() < picture_size {
            return Err(ArchiveError::decompression_failed("PackIce", "bad picture size"));
        }

        let start = out.len() - picture_size;
        for i in (start..out.len()).step_by(8) {
            let mut values: [u16; 4] = [0; 4];
            for j in (0..8).step_by(2) {
                let idx = i + 6 - j;
                let tmp = u16::from_be_bytes([out[idx], out[idx + 1]]);
                let mut t = tmp;
                for k in 0..16 {
                    let vi = (k & 3) as usize;
                    values[vi] = (values[vi] << 1) | ((t >> 15) & 1);
                    t <<= 1;
                }
            }
            for j in 0..4 {
                out[i + j * 2] = (values[j] >> 8) as u8;
                out[i + j * 2 + 1] = values[j] as u8;
            }
        }
    }

    if !br.input_eof() {
        return Err(ArchiveError::decompression_failed("PackIce", "trailing input"));
    }

    Ok(())
}

impl PackIceArchive {
    /// Returns the original (uncompressed) size of the file
    pub fn original_size(&self) -> u32 {
        self.original_size
    }

    /// Create a new Pack-Ice archive reader from raw data
    pub fn new(data: &[u8]) -> Result<Self> {
        let header = parse_header(data)?;
        let decompressed = decompress(data, &header)?;
        Ok(Self {
            original_size: header.raw_size,
            data: decompressed,
        })
    }

    /// Create a new Pack-Ice archive reader from a reader
    pub fn from_reader<R: std::io::Read>(mut reader: R) -> Result<Self> {
        let mut file_data = Vec::new();
        reader.read_to_end(&mut file_data)?;
        Self::new(&file_data)
    }

    /// Skip the current entry (Pack-Ice files only contain one file)
    pub fn skip(&mut self) -> Result<()> {
        // Pack-Ice archives contain only one file, nothing to skip
        Ok(())
    }

    /// Read the decompressed data
    pub fn read(&mut self) -> Result<Vec<u8>> {
        Ok(std::mem::take(&mut self.data))
    }
}
