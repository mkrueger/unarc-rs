use bitstream_io::{BigEndian, BitRead, BitReader};

use crate::error::{ArchiveError, Result};

pub fn decode_val(r: &mut BitReader<&[u8], BigEndian>, from: u32, to: u32) -> Result<u16> {
    let mut res = 0;
    let mut add = 0;
    let mut exp = 1 << from;
    let mut bit = from;
    while bit < to {
        res = r.read::<1, u16>()?;
        if res == 0 {
            break;
        }
        add += exp;
        exp <<= 1;
        bit += 1;
    }
    if bit != 0 {
        res = r.read_var::<u16>(bit)?;
    }
    res += add;
    Ok(res)
}

const THRESHOLD: usize = 3;

pub fn decode_fastest(data: &[u8], original_size: usize) -> Result<Vec<u8>> {
    let mut res = Vec::with_capacity(original_size);
    let mut r = BitReader::endian(data, BigEndian);
    while res.len() < original_size {
        let len = decode_val(&mut r, 0, 7)?;
        if len == 0 {
            let next_char = r.read::<8, u8>()?;
            res.push(next_char);
        } else {
            let rep_count = len as usize + THRESHOLD - 1;
            let back_ptr = decode_val(&mut r, 9, 13)? as usize;
            if back_ptr >= res.len() {
                return Err(ArchiveError::decompression_failed("ARJ", "invalid back pointer in LZ77 stream"));
            }
            if rep_count > original_size - res.len() {
                return Err(ArchiveError::decompression_failed("ARJ", "LZ77 match exceeds original size"));
            }
            let start = res.len() - 1 - back_ptr;
            for i in start..start + rep_count {
                res.push(res[i]);
            }
        }
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_match_before_first_literal() {
        assert!(decode_fastest(&[0x80, 0], 1).is_err());
    }

    #[test]
    fn rejects_truncated_literal_and_match() {
        for data in [&[][..], &[0][..], &[0x80][..]] {
            assert!(decode_fastest(data, 1).is_err());
        }
    }

    #[test]
    fn overlapping_match_respects_output_size() {
        // 0 + 'A' (8 bits), length 1 (100), distance 0 (ten zero bits).
        // The match copies three bytes at distance one, producing AAAA.
        let data = [0x20, 0xc0, 0];
        assert_eq!(decode_fastest(&data, 4).unwrap(), b"AAAA");
        assert!(decode_fastest(&data, 2).is_err());
        assert!(decode_fastest(&data, 3).is_err());
    }
}
