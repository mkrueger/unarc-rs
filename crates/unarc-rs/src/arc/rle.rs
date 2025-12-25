const DLE: u8 = 0x90;

#[derive(Debug)]
enum State {
    Normal(u8),
    Count(u8),
}

/// Unpacks RLE compressed buffer
/// <char> DLE <count>
/// count == 0 -> DLE
pub fn unpack_rle(compressed_buffer: &[u8]) -> Vec<u8> {
    let mut res = Vec::new();
    unpack_rle_into(compressed_buffer, &mut res);
    res
}

/// Unpacks RLE compressed buffer into an existing Vec to avoid allocations.
pub fn unpack_rle_into(compressed_buffer: &[u8], res: &mut Vec<u8>) {
    res.clear();
    let mut state = State::Normal(0);
    for &c in compressed_buffer {
        match state {
            State::Normal(last) => {
                if c == DLE {
                    state = State::Count(last);
                } else {
                    res.push(c);
                    state = State::Normal(c);
                }
            }
            State::Count(last) => {
                if c == 0 {
                    res.push(DLE);
                    state = State::Normal(DLE);
                } else {
                    res.resize(res.len() + c as usize - 1, last);
                    state = State::Normal(last);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpack_dle() {
        let compressed_buffer = vec![0x01, 0x90, 0x00, 0x03];
        let res = unpack_rle(&compressed_buffer);
        assert_eq!(res, vec![0x01, 0x90, 0x03]);
    }

    #[test]
    fn test_unpack() {
        let compressed_buffer = vec![0x01, 0x90, 0x05, 0x02];
        let res = unpack_rle(&compressed_buffer);
        assert_eq!(res, vec![0x01, 0x01, 0x01, 0x01, 0x01, 0x02]);
    }
}
