//! Microsoft Cabinet (CAB) archive reader

use std::io::{BufReader, Read, Seek, SeekFrom};

use crate::error::{ArchiveError, Result};

use super::header::{
    CabFileHeader, CabFolder, CabHeader, CompressionMethod, IFOLD_CONTINUED_FROM_PREV, IFOLD_CONTINUED_PREV_AND_NEXT, IFOLD_CONTINUED_TO_NEXT,
};
use super::mszip::MsZipDecoder;

/// Largest uncompressed size of a data block
const MAX_BLOCK_SIZE: usize = 0x8000;
/// Largest compressed size of a data block (32 KiB plus the worst-case expansion allowed by the format)
const MAX_COMPRESSED_BLOCK_SIZE: usize = MAX_BLOCK_SIZE + 6144;

/// Microsoft Cabinet archive reader
///
/// Files are grouped into folders, each a single compressed stream. Reading a
/// file decodes its folder up to the end of the file; reading the files of a
/// folder in order continues where the previous read stopped, while reading an
/// earlier file restarts the folder from its first data block.
pub struct CabArchive<R: Read + Seek> {
    reader: R,
    /// Stream position of the `MSCF` signature; cabinet offsets are relative to it
    base: u64,
    header: CabHeader,
    folders: Vec<CabFolder>,
    files: Vec<CabFileHeader>,
    next_entry: usize,
    cursor: Option<FolderCursor>,
}

impl<R: Read + Seek> CabArchive<R> {
    /// Opens a cabinet and reads its folder and file tables
    pub fn new(mut reader: R) -> Result<Self> {
        let base = reader.stream_position()?;
        let (header, folders) = {
            let mut buffered = BufReader::new(&mut reader);
            let header = CabHeader::read(&mut buffered)?;
            let mut folders = Vec::new();
            for _ in 0..header.folder_count {
                folders.push(CabFolder::read(&mut buffered, header.folder_reserve)?);
            }
            (header, folders)
        };

        reader.seek(SeekFrom::Start(base + u64::from(header.files_offset)))?;
        let mut files = Vec::new();
        {
            let mut buffered = BufReader::new(&mut reader);
            for index in 0..usize::from(header.file_count) {
                let mut file = CabFileHeader::read(&mut buffered, index)?;
                let folder = match file.folder_index {
                    IFOLD_CONTINUED_FROM_PREV => folders.first(),
                    IFOLD_CONTINUED_TO_NEXT | IFOLD_CONTINUED_PREV_AND_NEXT => folders.last(),
                    index => folders.get(usize::from(index)),
                };
                let Some(folder) = folder else {
                    return Err(ArchiveError::corrupted_entry_named("CAB", &file.name, "folder index out of range"));
                };
                file.compression_method = folder.compression_method;
                files.push(file);
            }
        }

        Ok(Self {
            reader,
            base,
            header,
            folders,
            files,
            next_entry: 0,
            cursor: None,
        })
    }

    /// Returns the cabinet header
    pub const fn header(&self) -> &CabHeader {
        &self.header
    }

    /// Returns the folders (compressed streams) of the cabinet
    pub fn folders(&self) -> &[CabFolder] {
        &self.folders
    }

    /// Returns the next file entry, or `None` after the last one
    pub fn get_next_entry(&mut self) -> Result<Option<CabFileHeader>> {
        let entry = self.files.get(self.next_entry).cloned();
        if entry.is_some() {
            self.next_entry += 1;
        }
        Ok(entry)
    }

    /// Skips an entry; data is only decoded when an entry is read
    pub const fn skip(&mut self, _header: &CabFileHeader) -> Result<()> {
        Ok(())
    }

    /// Reads and decompresses an entry
    pub fn read(&mut self, header: &CabFileHeader) -> Result<Vec<u8>> {
        self.read_with_limit(header, None)
    }

    /// Reads and decompresses an entry, failing if it is larger than `limit` bytes
    ///
    /// The output never exceeds the size recorded for the entry, which is checked
    /// against `limit` before anything is decoded.
    pub fn read_with_limit(&mut self, header: &CabFileHeader, limit: Option<u64>) -> Result<Vec<u8>> {
        crate::limits::check_size(u64::from(header.original_size), limit, &header.name)?;
        if header.is_continued() {
            return Err(ArchiveError::unsupported_format(format!(
                "CAB entry '{}' continues in another cabinet; multi-cabinet sets are not supported",
                header.name
            )));
        }
        let folder_index = usize::from(header.folder_index);
        let folder = self
            .folders
            .get(folder_index)
            .ok_or_else(|| ArchiveError::corrupted_entry_named("CAB", &header.name, "folder index out of range"))?;
        let method = folder.compression_method;
        match method {
            CompressionMethod::None | CompressionMethod::MsZip => {}
            CompressionMethod::Lzx { window_bits } if (15..=21).contains(&window_bits) => {}
            _ => return Err(ArchiveError::unsupported_method("CAB", method.to_string())),
        }
        if header.original_size == 0 {
            return Ok(Vec::new());
        }

        let start = u64::from(header.folder_offset);
        let end = start + u64::from(header.original_size);
        // A folder cannot hold more than its block count allows; reject without decoding.
        if end > u64::from(folder.block_count) * MAX_BLOCK_SIZE as u64 {
            return Err(ArchiveError::corrupted_entry_named(
                "CAB",
                &header.name,
                "file extends past the end of its folder",
            ));
        }

        let mut data = Vec::with_capacity(crate::limits::capacity_hint(u64::from(header.original_size)));
        let result = self.decode_range(folder_index, start, end - start, &mut data);
        match result {
            Ok(()) => Ok(data),
            Err(reason) => {
                // The decoder state is unusable after an error; the next read starts over.
                self.cursor = None;
                Err(match reason {
                    CursorError::Io(e) => ArchiveError::Io(e),
                    CursorError::Corrupt(reason) => ArchiveError::corrupted_entry_named("CAB", &header.name, reason),
                    CursorError::Unsupported(reason) => ArchiveError::unsupported_format(format!("CAB entry '{}': {reason}", header.name)),
                })
            }
        }
    }

    /// Appends `len` bytes from offset `start` of a folder to `output`
    fn decode_range(&mut self, folder_index: usize, start: u64, len: u64, output: &mut Vec<u8>) -> std::result::Result<(), CursorError> {
        let cursor = match self.cursor.take() {
            Some(cursor) if cursor.folder == folder_index && cursor.position() <= start => cursor,
            _ => {
                let folder = self
                    .folders
                    .get(folder_index)
                    .ok_or_else(|| CursorError::Corrupt("folder index out of range".to_string()))?;
                FolderCursor::new(folder_index, folder, self.base, self.header.data_reserve)?
            }
        };
        let cursor = self.cursor.insert(cursor);
        cursor.skip_to(&mut self.reader, start)?;
        cursor.read_into(&mut self.reader, output, len)
    }
}

enum CursorError {
    Io(std::io::Error),
    Corrupt(String),
    Unsupported(&'static str),
}

impl From<std::io::Error> for CursorError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

enum Decoder {
    Stored,
    MsZip(Box<MsZipDecoder>),
    Lzx(Box<lzxd::Lzxd>),
}

/// Sequential decoding position within one folder
struct FolderCursor {
    folder: usize,
    decoder: Decoder,
    blocks_left: u16,
    next_block_offset: u64,
    data_reserve: u8,
    /// Uncompressed folder offset of `block[0]`
    block_start: u64,
    block: Vec<u8>,
    block_pos: usize,
    compressed: Vec<u8>,
}

impl FolderCursor {
    fn new(index: usize, folder: &CabFolder, base: u64, data_reserve: u8) -> std::result::Result<Self, CursorError> {
        let decoder = match folder.compression_method {
            CompressionMethod::None => Decoder::Stored,
            CompressionMethod::MsZip => Decoder::MsZip(Box::new(MsZipDecoder::new())),
            CompressionMethod::Lzx { window_bits } => Decoder::Lzx(Box::new(lzxd::Lzxd::new(lzx_window(window_bits)?))),
            method => return Err(CursorError::Corrupt(format!("unsupported compression method {method}"))),
        };
        Ok(Self {
            folder: index,
            decoder,
            blocks_left: folder.block_count,
            next_block_offset: base + u64::from(folder.data_offset),
            data_reserve,
            block_start: 0,
            block: Vec::new(),
            block_pos: 0,
            compressed: Vec::new(),
        })
    }

    const fn position(&self) -> u64 {
        self.block_start + self.block_pos as u64
    }

    /// Decodes and discards data up to the uncompressed folder offset `target`
    fn skip_to<R: Read + Seek>(&mut self, reader: &mut R, target: u64) -> std::result::Result<(), CursorError> {
        while self.position() < target {
            if self.block_pos == self.block.len() {
                self.next_block(reader)?;
            }
            let wanted = usize::try_from(target - self.position()).unwrap_or(usize::MAX);
            self.block_pos += wanted.min(self.block.len() - self.block_pos);
        }
        Ok(())
    }

    /// Appends the next `len` bytes of the folder to `output`
    fn read_into<R: Read + Seek>(&mut self, reader: &mut R, output: &mut Vec<u8>, len: u64) -> std::result::Result<(), CursorError> {
        let mut remaining = len;
        while remaining > 0 {
            if self.block_pos == self.block.len() {
                self.next_block(reader)?;
            }
            let available = self.block.len() - self.block_pos;
            let take = usize::try_from(remaining).unwrap_or(usize::MAX).min(available);
            output.extend_from_slice(&self.block[self.block_pos..self.block_pos + take]);
            self.block_pos += take;
            remaining -= take as u64;
        }
        Ok(())
    }

    /// Reads, verifies and decodes the next `CFDATA` block
    fn next_block<R: Read + Seek>(&mut self, reader: &mut R) -> std::result::Result<(), CursorError> {
        if self.blocks_left == 0 {
            return Err(CursorError::Corrupt("folder data ends before the end of the file".to_string()));
        }
        reader.seek(SeekFrom::Start(self.next_block_offset))?;
        let mut head = [0u8; 8];
        reader.read_exact(&mut head)?;
        let checksum = u32::from_le_bytes([head[0], head[1], head[2], head[3]]);
        let compressed_size = usize::from(u16::from_le_bytes([head[4], head[5]]));
        let uncompressed_size = usize::from(u16::from_le_bytes([head[6], head[7]]));
        if uncompressed_size == 0 {
            return Err(CursorError::Unsupported(
                "data block continues in another cabinet; multi-cabinet sets are not supported",
            ));
        }
        if uncompressed_size > MAX_BLOCK_SIZE || compressed_size > MAX_COMPRESSED_BLOCK_SIZE {
            return Err(CursorError::Corrupt("invalid data block size".to_string()));
        }

        let reserve = usize::from(self.data_reserve);
        self.compressed.resize(reserve + compressed_size, 0);
        reader.read_exact(&mut self.compressed)?;
        if checksum != 0 && !block_checksum_matches(checksum, &head[4..8], &self.compressed, reserve) {
            return Err(CursorError::Corrupt("data block checksum mismatch".to_string()));
        }
        self.next_block_offset += (head.len() + reserve + compressed_size) as u64;
        self.blocks_left -= 1;
        self.block_start += self.block.len() as u64;
        self.block_pos = 0;

        let input = &self.compressed[reserve..];
        match &mut self.decoder {
            Decoder::Stored => {
                if input.len() != uncompressed_size {
                    return Err(CursorError::Corrupt("stored data block sizes differ".to_string()));
                }
                self.block.clear();
                self.block.extend_from_slice(input);
            }
            Decoder::MsZip(decoder) => decoder.decode_block(input, &mut self.block, uncompressed_size).map_err(CursorError::Corrupt)?,
            Decoder::Lzx(decoder) => {
                let output = decoder
                    .decompress_next(input, uncompressed_size)
                    .map_err(|e| CursorError::Corrupt(format!("LZX: {e}")))?;
                self.block.clear();
                self.block.extend_from_slice(output);
            }
        }
        if self.block.len() != uncompressed_size {
            return Err(CursorError::Corrupt("data block decoded to the wrong size".to_string()));
        }
        Ok(())
    }
}

fn lzx_window(bits: u8) -> std::result::Result<lzxd::WindowSize, CursorError> {
    Ok(match bits {
        15 => lzxd::WindowSize::KB32,
        16 => lzxd::WindowSize::KB64,
        17 => lzxd::WindowSize::KB128,
        18 => lzxd::WindowSize::KB256,
        19 => lzxd::WindowSize::KB512,
        20 => lzxd::WindowSize::MB1,
        21 => lzxd::WindowSize::MB2,
        _ => return Err(CursorError::Corrupt(format!("invalid LZX window size {bits}"))),
    })
}

/// Checks a `CFDATA` checksum. `block` holds the reserved area (`reserve` bytes) followed by the data.
///
/// The specification covers the reserved area; libmspack does not, so both are accepted.
fn block_checksum_matches(expected: u32, sizes: &[u8], block: &[u8], reserve: usize) -> bool {
    let matches = |data: &[u8]| checksum(sizes, checksum(data, 0)) == expected;
    matches(block) || (reserve > 0 && matches(&block[reserve..]))
}

/// The cabinet checksum: XOR of little-endian 32-bit words, with a big-endian tail
fn checksum(data: &[u8], seed: u32) -> u32 {
    let (words, tail) = data.as_chunks::<4>();
    let sum = words.iter().fold(seed, |sum, word| sum ^ u32::from_le_bytes(*word));
    sum ^ tail.iter().fold(0, |acc, &b| (acc << 8) | u32::from(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_matches_reference_algorithm() {
        assert_eq!(checksum(&[], 0), 0);
        assert_eq!(checksum(&[1, 2, 3, 4], 0), 0x0403_0201);
        // Trailing bytes are combined most significant first.
        assert_eq!(checksum(&[1, 2, 3, 4, 5, 6, 7], 0), 0x0403_0201 ^ 0x0005_0607);
        assert_eq!(checksum(&[9], 0x10), 0x19);
    }
}
