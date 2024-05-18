use bitstream_io::{BigEndian, BitRead, BitReader};
use std::io;

pub fn decode_val(r: &mut BitReader<&[u8], BigEndian>, from: u32, to: u32) -> io::Result<u16> {
    let mut res = 0;
    let mut add = 0;
    let mut exp = 1 << from;
    let mut bit = from;
    while bit < to {
        res = r.read::<u16>(1)?;
        if res == 0 {
            break;
        }
        add += exp;
        exp <<= 1;
        bit += 1;
    }
    if bit != 0 {
        res = r.read::<u16>(bit)?;
    }
    res += add;
    Ok(res)
}

const THRESHOLD: usize = 3;

pub fn decode_fastest(data: &[u8], original_size: usize) -> io::Result<Vec<u8>> {
    let mut res = Vec::with_capacity(original_size);
    let mut r = BitReader::endian(data, BigEndian);
    while res.len() < original_size {
        let len = decode_val(&mut r, 0, 7)?;
        if len == 0 {
            let next_char = r.read::<u8>(8)?;
            res.push(next_char);
        } else {
            let rep_count = len as usize + THRESHOLD - 1;
            let back_ptr = decode_val(&mut r, 9, 13)? as usize;
            if back_ptr > res.len() - 1 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid back_ptr",
                ));
            }
            let mut i = res.len() - 1 - back_ptr;
            for _ in 0..rep_count {
                res.push(res[i]);
                i += 1;
            }
        }
    }
    Ok(res)
}
