use std::io::{self, Read, Seek};

use crc16::{State, ARC};

use super::{local_file_header::{CompressionMethod, LocalFileHeader}, rle::unpack_rle};


pub struct ArcArchieve<T: Read + Seek> {
    reader: T,
}

impl<T: Read + Seek> ArcArchieve<T> {
    pub fn new(reader: T) -> io::Result<Self> {
        Ok(Self { reader })
    }

    pub fn skip(&mut self, header: &LocalFileHeader) -> io::Result<()> {
        self.reader
            .seek(io::SeekFrom::Current(header.compressed_size as i64))?;
        Ok(())
    }

    pub fn read(&mut self, header: &LocalFileHeader) -> io::Result<Vec<u8>> {
        let mut compressed_buffer = vec![0; header.compressed_size as usize];
        self.reader.read_exact(&mut compressed_buffer)?;

        let uncompressed = match header.compression_method {
            CompressionMethod::Unpacked(_) => compressed_buffer,
            CompressionMethod::RLE => unpack_rle(&compressed_buffer, header.original_size as usize),
            CompressionMethod::Squeezed => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Squeeze not implemented",
                ))
            }
            CompressionMethod::Crunched(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Crunch not implemented",
                ))
            }
            CompressionMethod::Squashed => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Squash not implemented",
                ))
            }
            CompressionMethod::Crushed => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Crushed not implemented",
                ))
            }
            CompressionMethod::Distilled => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Distilled not implemented",
                ))
            }
            CompressionMethod::Unknown(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Unsupported compression method {:?}",
                        header.compression_method
                    ),
                ))
            }
        };
        let mut state = State::<ARC>::new();
        state.update(&uncompressed);
        if state.get() != header.crc16 {
            Err(io::Error::new(io::ErrorKind::InvalidData, "CRC mismatch"))
        } else {
            Ok(uncompressed)
        }
    }

    pub fn get_next_entry(&mut self) -> io::Result<Option<LocalFileHeader>> {
        let header_bytes = read_header(self.reader.by_ref())?;
        let current_local_file_header = LocalFileHeader::load_from(&header_bytes);
        if current_local_file_header.is_none() {
            return Ok(None);
        }
        Ok(current_local_file_header)
    }
}

const HEADER_SIZE: usize = 28;
const MAX_SEARCH_SIZE: usize = u16::MAX as usize;
const ID: u8 = 0x1A;

fn read_header<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    let mut u8_buf = [0];
    for _ in 0..MAX_SEARCH_SIZE {
        reader.read_exact(&mut u8_buf)?;
        if u8_buf[0] != ID {
            continue;
        }
        let mut header = [0; HEADER_SIZE];
        reader.read_exact(&mut header)?;
        return Ok(header.to_vec());
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "no arc header found",
    ))
}
