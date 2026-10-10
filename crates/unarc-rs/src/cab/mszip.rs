//! MSZIP decoder.
//!
//! Every MSZIP data block is `"CK"` followed by a complete raw Deflate stream
//! that may refer back into the last 32 KiB produced by the previous blocks of
//! the same folder. flate2 offers no preset dictionary for raw Deflate with all
//! of its backends, so the history is replayed as a stored Deflate block before
//! each data block.

use flate2::{Decompress, FlushDecompress, Status};

/// Deflate window size
const HISTORY_SIZE: usize = 32 * 1024;

pub struct MsZipDecoder {
    inflater: Decompress,
    history: Vec<u8>,
    primer: Vec<u8>,
    discard: Vec<u8>,
}

impl MsZipDecoder {
    pub fn new() -> Self {
        Self {
            inflater: Decompress::new(false),
            history: Vec::with_capacity(HISTORY_SIZE),
            primer: Vec::new(),
            discard: Vec::new(),
        }
    }

    /// Decodes one data block into `output`, which receives exactly `size` bytes
    pub fn decode_block(&mut self, input: &[u8], output: &mut Vec<u8>, size: usize) -> Result<(), String> {
        let data = input.strip_prefix(b"CK").ok_or("missing MSZIP block signature")?;

        self.inflater.reset(false);
        if !self.history.is_empty() {
            // Non-final stored block holding the history; the length fits as it is at most 32 KiB.
            let len = u16::try_from(self.history.len()).map_err(|_| "MSZIP history too large")?;
            let mut primer = std::mem::take(&mut self.primer);
            let mut discard = std::mem::take(&mut self.discard);
            primer.clear();
            primer.push(0);
            primer.extend_from_slice(&len.to_le_bytes());
            primer.extend_from_slice(&(!len).to_le_bytes());
            primer.extend_from_slice(&self.history);
            discard.resize(self.history.len(), 0);
            let produced = self.inflate(&primer, &mut discard, FlushDecompress::Sync);
            self.primer = primer;
            self.discard = discard;
            if produced?.0 != self.history.len() {
                return Err("MSZIP history could not be restored".to_string());
            }
        }

        // One spare byte detects blocks that produce more than they declare.
        output.clear();
        output.resize(size + 1, 0);
        let (produced, status) = self.inflate(data, output, FlushDecompress::Finish)?;
        if produced != size {
            return Err(format!("MSZIP block decoded to {produced} bytes, expected {size}"));
        }
        if status != Status::StreamEnd {
            return Err("MSZIP block contains an unfinished Deflate stream".to_string());
        }
        output.truncate(size);

        if output.len() >= HISTORY_SIZE {
            self.history.clear();
            self.history.extend_from_slice(&output[output.len() - HISTORY_SIZE..]);
        } else {
            self.history.extend_from_slice(output);
            let excess = self.history.len().saturating_sub(HISTORY_SIZE);
            self.history.drain(..excess);
        }
        Ok(())
    }

    /// Inflates until the stream ends, the output is full or no progress is made
    fn inflate(&mut self, input: &[u8], output: &mut [u8], flush: FlushDecompress) -> Result<(usize, Status), String> {
        let (mut consumed, mut produced) = (0usize, 0usize);
        loop {
            let (in_before, out_before) = (self.inflater.total_in(), self.inflater.total_out());
            let status = self
                .inflater
                .decompress(&input[consumed..], &mut output[produced..], flush)
                .map_err(|e| format!("MSZIP: {e}"))?;
            let read = usize::try_from(self.inflater.total_in() - in_before).map_err(|_| "MSZIP: input overflow")?;
            let written = usize::try_from(self.inflater.total_out() - out_before).map_err(|_| "MSZIP: output overflow")?;
            consumed += read;
            produced += written;
            if status == Status::StreamEnd || produced == output.len() || (read == 0 && written == 0) {
                return Ok((produced, status));
            }
        }
    }
}
