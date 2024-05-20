use bitstream_io::{BitRead, BitReader, LittleEndian};
use std::io;

const NUMVALS: usize = 257;
const SPEOF: i16 = 256;

pub fn unsqz(mut buf: &[u8]) -> io::Result<Vec<u8>> {
    convert_u16!(numnodes, buf);
    println!("numnodes: {}", numnodes);
    let mut decoded = Vec::new();
    /*loop {
        i = dnode[i as usize][reader.read::<u8>(1)? as usize];
        if i < 0 {
            i = -(i + 1);
            if i == SPEOF {
                break;
            } else {
                decoded.push(i as u8);
                i = 0;
            }
        }
    }*/
    Ok(decoded)
}
