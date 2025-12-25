use crc16::{State, ARC};
use std::io::{Read, Seek};

use crate::arc::lzw;
use crate::error::{ArchiveError, Result};

use super::{
    local_file_header::{CompressionMethod, LocalFileHeader},
    rle::unpack_rle,
};

/// Decrypt data using ARC's simple XOR encryption.
///
/// ARC uses a repeating XOR with the password bytes.
/// Based on the original arccode.c by Thom Henderson.
fn decrypt(data: &mut [u8], password: &[u8]) {
    if password.is_empty() {
        return;
    }
    let mut key_pos = 0;
    for byte in data.iter_mut() {
        *byte ^= password[key_pos];
        key_pos += 1;
        if key_pos >= password.len() {
            key_pos = 0;
        }
    }
}

pub struct ArcArchive<T: Read + Seek> {
    reader: T,
    password: Option<Vec<u8>>,
}

impl<T: Read + Seek> ArcArchive<T> {
    pub fn new(reader: T) -> Result<Self> {
        Ok(Self {
            reader,
            password: None,
        })
    }

    /// Set the password for decrypting encrypted entries.
    ///
    /// Note: ARC has no encryption flag in headers - the user must know
    /// if a password is required. Wrong passwords result in CRC errors.
    pub fn set_password(&mut self, password: &str) {
        self.password = Some(password.as_bytes().to_vec());
    }

    /// Clear the current password.
    pub fn clear_password(&mut self) {
        self.password = None;
    }

    /// Check if a password is currently set.
    pub fn has_password(&self) -> bool {
        self.password.is_some()
    }

    pub fn skip(&mut self, header: &LocalFileHeader) -> Result<()> {
        self.reader
            .seek(std::io::SeekFrom::Current(header.compressed_size as i64))?;
        Ok(())
    }

    pub fn read(&mut self, header: &LocalFileHeader) -> Result<Vec<u8>> {
        let mut compressed_buffer = vec![0; header.compressed_size as usize];
        self.reader.read_exact(&mut compressed_buffer)?;

        // Decrypt if password is set
        if let Some(ref password) = self.password {
            decrypt(&mut compressed_buffer, password);
        }

        let uncompressed = match header.compression_method {
            CompressionMethod::Unpacked(_) => compressed_buffer,
            CompressionMethod::RLE90 => unpack_rle(&compressed_buffer),
            CompressionMethod::Squeezed => super::unsqueeze::unsqueeze(&compressed_buffer)?,
            CompressionMethod::Crunched(_level) => {
                let decompressed = lzw::Lzw::new().decomp(&compressed_buffer, true)?;
                unpack_rle(&decompressed)
            }
            CompressionMethod::Squashed => lzw::Lzw::new().decomp(&compressed_buffer, false)?,
            CompressionMethod::Crushed => {
                let decompressed = super::crushed::decompress(&compressed_buffer)?;
                unpack_rle(&decompressed)
            }
            CompressionMethod::Distilled => super::distilled::decompress(&compressed_buffer)?,
            CompressionMethod::Unknown(m) => {
                return Err(ArchiveError::unsupported_method(
                    "ARC",
                    format!("Unknown({})", m),
                ));
            }
        };
        let mut state = State::<ARC>::new();
        state.update(&uncompressed);
        if state.get() != header.crc16 {
            Err(ArchiveError::crc_mismatch(
                &header.name,
                header.crc16 as u32,
                state.get() as u32,
            ))
        } else {
            Ok(uncompressed)
        }
    }

    /// Read an entry with a specific password (for per-entry decryption)
    ///
    /// ARC/PAK has no reliable encryption flag in headers, so encryption is
    /// "best effort": if a password is set, the compressed data is XOR-decrypted
    /// before decompression and CRC verification.
    pub fn read_with_password(
        &mut self,
        header: &LocalFileHeader,
        password: Option<String>,
    ) -> Result<Vec<u8>> {
        let old_password = self.password.take();
        self.password = password.map(|p| p.into_bytes());
        let result = self.read(header);
        self.password = old_password;
        result
    }

    pub fn get_next_entry(&mut self) -> Result<Option<LocalFileHeader>> {
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

fn read_header<R: Read>(reader: &mut R) -> Result<Vec<u8>> {
    let mut u8_buf = [0];
    for _ in 0..MAX_SEARCH_SIZE {
        reader.read_exact(&mut u8_buf)?;
        if u8_buf[0] != ID {
            continue;
        }
        // Read compression method byte first to check for EOF marker
        reader.read_exact(&mut u8_buf)?;
        if u8_buf[0] == 0 {
            // EOF marker (0x1A 0x00) - no more entries
            return Ok(vec![0; HEADER_SIZE]);
        }
        // Read the rest of the header
        let mut header = vec![u8_buf[0]];
        header.resize(HEADER_SIZE, 0);
        reader.read_exact(&mut header[1..])?;
        return Ok(header);
    }
    Err(ArchiveError::invalid_header("ARC"))
}
