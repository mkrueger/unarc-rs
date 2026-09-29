use std::io::{Read, Seek};

use crc16::{State, ARC};
use delharc::decode::{Decoder, DecoderAny};
use salzweg::CodeSizeStrategy;

use crate::error::{ArchiveError, Result};

use super::{
    dirent::{CompressionMethod, DirectoryEntry, DIRENT_HEADER_SIZE},
    zoo_header::{ZooHeader, ZOO_HEADER_SIZE, ZOO_TAG},
};

/// End of the `next` field in a directory entry (tag: 4, type: 1, method: 1, next: 4)
const DIRENT_NEXT_END: usize = 10;

pub struct ZooArchive<T: Read + Seek> {
    pub header: ZooHeader,
    has_next: bool,
    reader: T,
}

impl<T: Read + Seek> ZooArchive<T> {
    pub fn new(mut reader: T) -> Result<Self> {
        let mut header_bytes = [0; ZOO_HEADER_SIZE];
        reader.read_exact(&mut header_bytes)?;
        let header = ZooHeader::load_from(&header_bytes)?;
        reader.seek(std::io::SeekFrom::Start(header.zoo_start as u64))?;

        Ok(Self {
            header,
            reader,
            has_next: true,
        })
    }

    pub fn skip(&mut self, header: &DirectoryEntry) -> Result<()> {
        if header.next == 0 {
            self.has_next = false;
            return Ok(());
        }
        self.reader.seek(std::io::SeekFrom::Start(header.next as u64))?;
        Ok(())
    }

    pub fn read(&mut self, header: &DirectoryEntry) -> Result<Vec<u8>> {
        self.reader.seek(std::io::SeekFrom::Start(header.offset as u64))?;
        let compressed_buffer = crate::limits::read_exact_vec(&mut self.reader, header.size_now as u64)?;

        if header.next == 0 {
            self.has_next = false;
        } else {
            self.reader.seek(std::io::SeekFrom::Start(header.next as u64))?;
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
                    return Err(ArchiveError::decompression_failed(&header.name, err.to_string()));
                }
                decompressed
            }

            CompressionMethod::CompressedLh5 => {
                let mut decoder = DecoderAny::new_from_compression(delharc::CompressionMethod::Lh5, compressed_buffer.as_slice());
                let mut decompressed_buffer = vec![0; header.org_size as usize];
                decoder.fill_buffer(&mut decompressed_buffer)?;
                decompressed_buffer
            }

            CompressionMethod::Unknown(m) => {
                return Err(ArchiveError::unsupported_method("ZOO", format!("Unknown({})", m)));
            }
        };
        let mut state = State::<ARC>::new();
        state.update(&uncompressed);
        if state.get() != header.file_crc16 {
            Err(ArchiveError::crc_mismatch(&header.name, header.file_crc16 as u32, state.get() as u32))
        } else {
            Ok(uncompressed)
        }
    }

    pub fn get_next_entry(&mut self) -> Result<Option<DirectoryEntry>> {
        if !self.has_next {
            return Ok(None);
        }
        let mut header_bytes = [0; DIRENT_HEADER_SIZE];
        // The directory chain ends with a dummy entry whose `next` is 0. It may be shorter than
        // a full entry, so check it before reading the rest of the header.
        self.reader.read_exact(&mut header_bytes[..DIRENT_NEXT_END])?;
        if u32::from_le_bytes([header_bytes[0], header_bytes[1], header_bytes[2], header_bytes[3]]) != ZOO_TAG {
            return Err(ArchiveError::invalid_header("ZOO"));
        }
        let next = u32::from_le_bytes([header_bytes[6], header_bytes[7], header_bytes[8], header_bytes[9]]);
        if next == 0 {
            self.has_next = false;
            return Ok(None);
        }
        self.reader.read_exact(&mut header_bytes[DIRENT_NEXT_END..])?;
        let entry = DirectoryEntry::load_from(&header_bytes)?;

        Ok(Some(entry))
    }
}
