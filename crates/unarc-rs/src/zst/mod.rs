//! ZST (Zstandard) single file format support
//!
//! Zstandard is a compression format for single files. Files with extension .zst
//! are zstd-compressed single files (as opposed to .tar.zst which contains a TAR archive).
//!
//! This module provides read-only access to .zst files. All frames are decoded one
//! after another and skippable frames are ignored, as `zstd -d` does.

use std::io::{self, Cursor, Read};

use ruzstd::decoding::{BlockDecodingStrategy, FrameDecoder};

use crate::error::{ArchiveError, Result};

/// Zstandard frame magic number (0xFD2FB528, little endian)
const ZSTD_MAGIC: [u8; 4] = [0x28, 0xB5, 0x2F, 0xFD];

/// Skippable frames use the magic numbers 0x184D2A50 to 0x184D2A5F
const SKIPPABLE_MAGIC: u32 = 0x184D_2A50;
const SKIPPABLE_MAGIC_MASK: u32 = 0xFFFF_FFF0;

/// Returns true if `magic` starts a Zstandard or a skippable frame
pub(crate) fn is_frame_magic(magic: [u8; 4]) -> bool {
    magic == ZSTD_MAGIC || u32::from_le_bytes(magic) & SKIPPABLE_MAGIC_MASK == SKIPPABLE_MAGIC
}

/// ZST archive reader for single compressed files
///
/// Note: The entire file is decompressed into memory on read.
pub struct ZstArchive<T: Read> {
    reader: Option<(T, [u8; 4])>,
}

impl<T: Read> ZstArchive<T> {
    /// Create a new ZST archive reader
    ///
    /// This validates the frame magic but doesn't decompress until read() is called.
    pub fn new(mut reader: T) -> Result<Self> {
        let mut header = [0u8; 4];
        reader.read_exact(&mut header)?;

        if !is_frame_magic(header) {
            return Err(ArchiveError::invalid_header("ZST"));
        }

        Ok(Self {
            reader: Some((reader, header)),
        })
    }

    /// Skip the file (ZST contains only one file)
    pub fn skip(&mut self) -> Result<()> {
        // Just one file in the archive, nothing to skip to
        Ok(())
    }

    /// Read and decompress the file
    pub fn read(&mut self) -> Result<Vec<u8>> {
        self.read_with_limit(None)
    }

    /// Read and decompress the file, failing if it decompresses to more than `limit` bytes
    pub fn read_with_limit(&mut self, limit: Option<u64>) -> Result<Vec<u8>> {
        let (reader, header) = self
            .reader
            .take()
            .ok_or_else(|| ArchiveError::io_error("ZST archive already read or in invalid state"))?;

        // Reconstruct with the header we already consumed
        let chained = Cursor::new(header).chain(reader);
        decompress(chained, limit, "ZST")
    }
}

/// Decompresses all frames in `reader`, failing once they produce more than `limit` bytes
///
/// The window size is read from each frame header, so it is checked against
/// [`crate::limits::window_limit`] before the decoder allocates it.
pub(crate) fn decompress<R: Read>(reader: R, limit: Option<u64>, entry: &str) -> Result<Vec<u8>> {
    let decoder = ZstdDecoder::new(reader, crate::limits::window_limit(limit));
    crate::limits::read_to_end_limited(decoder, limit, entry).map_err(|e| match e {
        ArchiveError::Io(e) => ArchiveError::io_error(format!("Failed to decompress zstd: {e}")),
        e => e,
    })
}

/// Reads the decompressed contents of every frame in a Zstandard stream
struct ZstdDecoder<R: Read> {
    source: R,
    decoder: FrameDecoder,
    in_frame: bool,
    started: bool,
}

impl<R: Read> ZstdDecoder<R> {
    fn new(source: R, max_window_size: u64) -> Self {
        let mut decoder = FrameDecoder::new();
        decoder.set_max_window_size(max_window_size);
        Self {
            source,
            decoder,
            in_frame: false,
            started: false,
        }
    }

    /// Starts decoding the next Zstandard frame, skipping skippable frames.
    /// Returns false at the end of the input.
    fn start_frame(&mut self) -> io::Result<bool> {
        loop {
            let mut magic = [0u8; 4];
            let read = read_up_to(&mut self.source, &mut magic)?;
            if read == 0 && self.started {
                return Ok(false);
            }
            if read < magic.len() {
                return Err(io::ErrorKind::UnexpectedEof.into());
            }
            self.started = true;

            if u32::from_le_bytes(magic) & SKIPPABLE_MAGIC_MASK == SKIPPABLE_MAGIC {
                let mut length = [0u8; 4];
                self.source.read_exact(&mut length)?;
                let length = u64::from(u32::from_le_bytes(length));
                if io::copy(&mut (&mut self.source).take(length), &mut io::sink())? < length {
                    return Err(io::ErrorKind::UnexpectedEof.into());
                }
                continue;
            }

            // The frame header follows the magic number we already consumed
            self.decoder.reset(Cursor::new(magic).chain(&mut self.source)).map_err(io::Error::other)?;
            self.in_frame = true;
            return Ok(true);
        }
    }

    /// Verifies the content checksum of a fully read frame, if it has one
    fn finish_frame(&mut self) -> io::Result<()> {
        self.in_frame = false;
        match self.decoder.get_checksum_from_data() {
            Some(expected) if self.decoder.get_calculated_checksum() != Some(expected) => {
                Err(io::Error::new(io::ErrorKind::InvalidData, "zstd frame checksum mismatch"))
            }
            _ => Ok(()),
        }
    }
}

impl<R: Read> Read for ZstdDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            if !self.in_frame && !self.start_frame()? {
                return Ok(0);
            }
            while self.decoder.can_collect() < buf.len() && !self.decoder.is_finished() {
                let needed = buf.len() - self.decoder.can_collect();
                self.decoder
                    .decode_blocks(&mut self.source, BlockDecodingStrategy::UptoBytes(needed))
                    .map_err(io::Error::other)?;
            }
            let read = self.decoder.read(buf)?;
            if read > 0 {
                return Ok(read);
            }
            self.finish_frame()?;
        }
    }
}

/// Reads until `buf` is full or the input ends, returning the number of bytes read
fn read_up_to<R: Read>(reader: &mut R, buf: &mut [u8]) -> io::Result<usize> {
    let mut filled = 0;
    while filled < buf.len() {
        match reader.read(&mut buf[filled..]) {
            Ok(0) => break,
            Ok(n) => filled += n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(filled)
}
