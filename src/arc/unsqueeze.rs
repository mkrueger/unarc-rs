use crate::arc::rle::unpack_rle;
use bitstream_io::{BitRead, BitReader, LittleEndian};
use std::io;

const NUMVALS: usize = 257;
const SPEOF: i16 = 256;

pub fn unsqueeze(mut buf: &[u8]) -> io::Result<Vec<u8>> {
    convert_u16!(numnodes, buf);
    if numnodes as usize >= NUMVALS {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid number of nodes {} (max {})", numnodes, NUMVALS - 1),
        ));
    }
    if numnodes == 0 {
        return Ok(Vec::new());
    }
    let mut dnode = Vec::with_capacity(numnodes as usize);
    for _ in 0..numnodes as usize {
        dnode.push([get_i16!(buf), get_i16!(buf)]);
    }
    let mut reader = BitReader::endian(buf, LittleEndian);
    let mut i: i16 = 0;
    let mut decoded = Vec::new();
    loop {
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
    }
    Ok(unpack_rle(&decoded))
}
