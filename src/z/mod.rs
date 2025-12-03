use std::io::{self, Read};
mod lzw;
pub struct ZArchive<T: Read> {
    block_mode: bool,
    max_bits: u8,
    reader: T,
}

const ID: [u8; 2] = [0x1F, 0x9D];
const BLOCK_MODE: u8 = 0x80;
const BIT_MASK: u8 = 0x1f;

impl<T: Read> ZArchive<T> {
    pub fn new(mut reader: T) -> io::Result<Self> {
        let mut header = [0; 3];
        reader.read_exact(&mut header)?;
        if header[0..2] != ID {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not a Z archive",
            ));
        }
        let block_mode = header[2] & BLOCK_MODE != 0;
        let max_bits = header[2] & BIT_MASK;
        Ok(Self {
            block_mode,
            max_bits,
            reader,
        })
    }

    pub fn skip(&mut self) -> io::Result<()> {
        // just 1 file in the archive
        Ok(())
    }

    pub fn read(&mut self) -> io::Result<Vec<u8>> {
        let mut compressed_buffer = Vec::new();
        self.reader.read_to_end(&mut compressed_buffer)?;
        let decompressed =
            lzw::Lzw::new(self.max_bits, self.block_mode).decomp(&compressed_buffer)?;
        Ok(decompressed)
    }
}
