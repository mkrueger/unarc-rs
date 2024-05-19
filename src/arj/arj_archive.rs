use std::io::{self, Read, Seek};

use delharc::decode::{Decoder, DecoderAny};

use crate::date_time::DosDateTime;

use super::{
    decode_fastest::decode_fastest,
    local_file_header::{CompressionMethod, LocalFileHeader},
    main_header::{HostOS, MainHeader},
};

pub struct ArjArchieve<T: Read + Seek> {
    reader: T,
    header: MainHeader,
}

impl<T: Read + Seek> ArjArchieve<T> {
    pub fn new(mut reader: T) -> io::Result<Self> {
        let header_bytes = read_header(&mut reader)?;
        if header_bytes.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "archive ends without any headers",
            ));
        }
        let header = MainHeader::load_from(&header_bytes);
        // Skip extended headers
        read_extended_headers(&mut reader)?;

        Ok(Self { header, reader })
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
            CompressionMethod::Stored => compressed_buffer,
            CompressionMethod::CompressedMost
            | CompressionMethod::Compressed
            | CompressionMethod::CompressedFaster => {
                let mut decoder = DecoderAny::new_from_compression(
                    delharc::CompressionMethod::Lh6,
                    compressed_buffer.as_slice(),
                );
                let mut decompressed_buffer = vec![0; header.original_size as usize];
                decoder.fill_buffer(&mut decompressed_buffer)?;
                decompressed_buffer
            }
            CompressionMethod::CompressedFastest => {
                decode_fastest(compressed_buffer.as_slice(), header.original_size as usize)?
            }
            CompressionMethod::NoDataNoCrc
            | CompressionMethod::NoData
            | CompressionMethod::Unknown(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "unsupported compression method {:?}",
                        header.compression_method
                    ),
                ))
            }
        };

        if uncompressed.len() != header.original_size as usize {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "decompressed size does not match the original size",
            ));
        }

        let checksum = crc32fast::hash(&uncompressed);
        if checksum != header.original_crc32 {
            Err(io::Error::new(io::ErrorKind::InvalidData, "CRC32 mismatch"))
        } else {
            Ok(uncompressed)
        }
    }

    pub fn get_next_entry(&mut self) -> io::Result<Option<LocalFileHeader>> {
        let header_bytes = read_header(&mut self.reader)?;
        if header_bytes.is_empty() {
            return Ok(None);
        }
        let current_local_file_header = LocalFileHeader::load_from(&header_bytes);
        if current_local_file_header.is_none() {
            return Ok(None);
        }
        read_extended_headers(&mut self.reader)?;
        Ok(current_local_file_header)
    }

    pub fn get_host_os(&self) -> HostOS {
        self.header.host_os
    }

    pub fn get_name(&self) -> &str {
        &self.header.name
    }

    pub fn get_comment(&self) -> &str {
        &self.header.comment
    }

    /// Returns the creation date and time of the archive in DOS format.
    pub fn get_creation_date_time(&self) -> DosDateTime {
        self.header.creation_date_time
    }

    pub fn get_compressed_size(&self) -> u32 {
        self.header.compr_size
    }

    pub fn get_archive_size(&self) -> u32 {
        self.header.archive_size
    }
}

const MAX_HEADER_SIZE: usize = 2600;
const ARJ_MAGIC_1: u8 = 0x60;
const ARJ_MAGIC_2: u8 = 0xEA;

fn read_header<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    let mut u8_buf = [0];
    loop {
        reader.read_exact(&mut u8_buf)?;
        if u8_buf[0] != ARJ_MAGIC_1 {
            continue;
        }
        reader.read_exact(&mut u8_buf)?;
        if u8_buf[0] == ARJ_MAGIC_2 {
            break;
        }
    }
    let mut u16_buf = [0, 0];
    reader.read_exact(&mut u16_buf)?;

    let header_size = u16_buf[0] as u16 | (u16_buf[1] as u16) << 8;
    if header_size == 0 {
        return Ok(Vec::new());
    }
    if header_size > MAX_HEADER_SIZE as u16 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "header size is too big",
        ));
    }
    let mut header_bytes = vec![0; header_size as usize];
    reader.read_exact(&mut header_bytes)?;
    let mut crc = [0, 0, 0, 0];
    reader.read_exact(&mut crc)?;
    let checksum = crc32fast::hash(&header_bytes);
    if checksum != u32::from_le_bytes(crc) {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "header checksum is invalid",
        ))
    } else {
        Ok(header_bytes)
    }
}

fn read_extended_headers<R: Read>(reader: &mut R) -> io::Result<Vec<Vec<u8>>> {
    let mut extended_header = Vec::new();
    let mut u16_buf = [0, 0];
    loop {
        reader.read_exact(&mut u16_buf)?;
        let ext_header_size = u16_buf[0] as u16 | (u16_buf[1] as u16) << 8;
        if ext_header_size == 0 {
            return Ok(extended_header);
        }
        let mut header = vec![0; ext_header_size as usize];
        reader.read_exact(&mut header)?;
        let mut crc = [0, 0, 0, 0];
        reader.read_exact(&mut crc)?;
        let checksum = crc32fast::hash(&header);
        if checksum != u32::from_le_bytes(crc) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "extended header checksum is invalid",
            ));
        }
        extended_header.push(header);
    }
}
