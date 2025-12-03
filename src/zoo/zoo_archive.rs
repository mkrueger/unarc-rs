use std::io::{self, Read, Seek};

use crc16::{State, ARC};
use delharc::decode::{Decoder, DecoderAny};
use salzweg::CodeSizeStrategy;

use super::{
    dirent::{CompressionMethod, DirectoryEntry, DIRENT_HEADER_SIZE},
    zoo_header::{ZooHeader, ZOO_HEADER_SIZE},
};

pub struct ZooArchive<T: Read + Seek> {
    pub header: ZooHeader,
    has_next: bool,
    reader: T,
}

impl<T: Read + Seek> ZooArchive<T> {
    pub fn new(mut reader: T) -> io::Result<Self> {
        let mut header_bytes = [0; ZOO_HEADER_SIZE];
        reader.read_exact(&mut header_bytes)?;
        let header = ZooHeader::load_from(&header_bytes)?;
        reader.seek(io::SeekFrom::Start(header.zoo_start as u64))?;

        Ok(Self {
            header,
            reader,
            has_next: true,
        })
    }

    pub fn skip(&mut self, header: &DirectoryEntry) -> io::Result<()> {
        if header.next == 0 {
            self.has_next = false;
            return Ok(());
        }
        self.reader.seek(io::SeekFrom::Start(header.next as u64))?;
        Ok(())
    }

    pub fn read(&mut self, header: &DirectoryEntry) -> io::Result<Vec<u8>> {
        self.reader
            .seek(io::SeekFrom::Start(header.offset as u64))?;
        let mut compressed_buffer = vec![0; header.size_now as usize];
        self.reader.read_exact(&mut compressed_buffer)?;

        if header.next == 0 {
            self.has_next = false;
        } else {
            self.reader.seek(io::SeekFrom::Start(header.next as u64))?;
        }

        let uncompressed = match header.compression_method {
            CompressionMethod::Stored => compressed_buffer,
            CompressionMethod::Compressed => {
                let mut decompressed = vec![];
                if let Err(err) = salzweg::decoder::VariableDecoder::decode(
                    compressed_buffer.as_slice(),
                    &mut decompressed,
                    8,
                    salzweg::Endianness::LittleEndian,
                    CodeSizeStrategy::Default,
                ) {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, err));
                }
                decompressed
            }

            CompressionMethod::CompressedLh5 => {
                let mut decoder = DecoderAny::new_from_compression(
                    delharc::CompressionMethod::Lh5,
                    compressed_buffer.as_slice(),
                );
                let mut decompressed_buffer = vec![0; header.org_size as usize];
                decoder.fill_buffer(&mut decompressed_buffer)?;
                decompressed_buffer
            }

            CompressionMethod::Unknown(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "unsupported compression method {:?}",
                        header.compression_method
                    ),
                ))
            }
        };
        let mut state = State::<ARC>::new();
        state.update(&uncompressed);
        if state.get() != header.file_crc16 {
            Err(io::Error::new(io::ErrorKind::InvalidData, "CRC mismatch"))
        } else {
            Ok(uncompressed)
        }
    }

    pub fn get_next_entry(&mut self) -> io::Result<Option<DirectoryEntry>> {
        if !self.has_next {
            return Ok(None);
        }
        let mut header_bytes = [0; DIRENT_HEADER_SIZE];
        self.reader.read_exact(&mut header_bytes)?;
        let entry = DirectoryEntry::load_from(&header_bytes)?;

        // Mark as no more entries if this is the last one
        if entry.next == 0 {
            self.has_next = false;
        }

        Ok(Some(entry))
    }
}
