const DLE: u8 = 0x90;

#[derive(Debug)]
enum State {
    Normal(u8),
    Count(u8),
}

/// Unpacks RLE compressed buffer
/// <char> DLE <count>
/// count == 0 -> DLE
pub fn unpack_rle(compressed_buffer: &[u8], _original_size: usize) -> Vec<u8> {
    let mut res = Vec::new();
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
                    res.extend(std::iter::repeat(last).take(c as usize - 1));
                    state = State::Normal(last);
                }
            }
        }
    }
    res
}
