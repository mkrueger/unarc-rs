//! Multi-volume (split) ZIP archive support
//!
//! This module provides support for reading ZIP archives that have been split
//! across multiple files (e.g., archive.001, archive.002, archive.003).
//!
//! Split archives are simply concatenated data, where the ZIP local file headers
//! and data span across multiple physical files. The End of Central Directory
//! is typically in the last volume.

use std::io::{Read, Seek, SeekFrom};
use std::sync::Arc;

use crate::unified::VolumeProvider;

/// A reader that concatenates multiple volumes into a single seekable stream.
///
/// This reader treats multiple volume files as a single contiguous byte stream,
/// allowing the ZIP reader to operate as if it were reading a single file.
pub struct MultiVolumeReader {
    /// The volume provider to open additional volumes
    volume_provider: Arc<dyn VolumeProvider>,
    /// Currently open volume reader
    current_reader: Option<Box<dyn Read + Send>>,
    /// Current volume number (0-based)
    current_volume: u32,
    /// Total number of volumes (if known)
    total_volumes: Option<u32>,
    /// Sizes of each volume (cached for seeking)
    volume_sizes: Vec<u64>,
    /// Current position within the entire concatenated stream
    position: u64,
    /// Total size of all volumes combined
    total_size: u64,
    /// Buffer for seeking backwards
    buffer: Vec<u8>,
}

impl std::fmt::Debug for MultiVolumeReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MultiVolumeReader")
            .field("current_volume", &self.current_volume)
            .field("total_volumes", &self.total_volumes)
            .field("volume_sizes", &self.volume_sizes)
            .field("position", &self.position)
            .field("total_size", &self.total_size)
            .finish()
    }
}

impl MultiVolumeReader {
    /// Create a new multi-volume reader.
    ///
    /// # Arguments
    /// * `volume_provider` - Provider to open each volume
    /// * `volume_sizes` - Sizes of each volume file in bytes
    pub fn new(volume_provider: Arc<dyn VolumeProvider>, volume_sizes: Vec<u64>) -> Self {
        let total_size = volume_sizes.iter().sum();
        let total_volumes = Some(volume_sizes.len() as u32);

        Self {
            volume_provider,
            current_reader: None,
            current_volume: 0,
            total_volumes,
            volume_sizes,
            position: 0,
            total_size,
            buffer: Vec::new(),
        }
    }

    /// Get the volume number and offset within that volume for a given position.
    fn volume_for_position(&self, pos: u64) -> Option<(u32, u64)> {
        let mut cumulative = 0u64;
        for (i, &size) in self.volume_sizes.iter().enumerate() {
            if pos < cumulative + size {
                return Some((i as u32, pos - cumulative));
            }
            cumulative += size;
        }
        None
    }

    /// Open a specific volume and seek to offset within it.
    fn open_volume_at(&mut self, volume: u32, offset: u64) -> std::io::Result<()> {
        // Open the volume
        let reader = self
            .volume_provider
            .open_volume(volume)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, format!("Cannot open volume {}", volume)))?;

        self.current_reader = Some(reader);
        self.current_volume = volume;

        // Skip to the offset by reading and discarding bytes
        if offset > 0 {
            let mut remaining = offset as usize;
            self.buffer.resize(8192.min(remaining), 0);
            while remaining > 0 {
                let to_read = self.buffer.len().min(remaining);
                let reader = self.current_reader.as_mut().unwrap();
                let n = reader.read(&mut self.buffer[..to_read])?;
                if n == 0 {
                    return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Unexpected end of volume while seeking"));
                }
                remaining -= n;
            }
        }

        Ok(())
    }
}

impl Read for MultiVolumeReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.is_empty() || self.position >= self.total_size {
            return Ok(0);
        }

        // Ensure we have an open reader
        if self.current_reader.is_none() {
            if let Some((vol, offset)) = self.volume_for_position(self.position) {
                self.open_volume_at(vol, offset)?;
            } else {
                return Ok(0);
            }
        }

        let reader = self.current_reader.as_mut().unwrap();
        match reader.read(buf) {
            Ok(0) => {
                // End of current volume, try next
                let next_volume = self.current_volume + 1;
                if let Some(total) = self.total_volumes {
                    if next_volume >= total {
                        return Ok(0);
                    }
                }

                // Open next volume
                self.current_reader = None;
                self.open_volume_at(next_volume, 0)?;

                // Try reading from the new volume
                let reader = self.current_reader.as_mut().unwrap();
                let n = reader.read(buf)?;
                self.position += n as u64;
                Ok(n)
            }
            Ok(n) => {
                self.position += n as u64;
                Ok(n)
            }
            Err(e) => Err(e),
        }
    }
}

impl Seek for MultiVolumeReader {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let new_pos = match pos {
            SeekFrom::Start(offset) => offset,
            SeekFrom::End(offset) => {
                if offset >= 0 {
                    self.total_size.saturating_add(offset as u64)
                } else {
                    self.total_size.saturating_sub((-offset) as u64)
                }
            }
            SeekFrom::Current(offset) => {
                if offset >= 0 {
                    self.position.saturating_add(offset as u64)
                } else {
                    self.position.saturating_sub((-offset) as u64)
                }
            }
        };

        // Clamp to valid range
        let new_pos = new_pos.min(self.total_size);

        // Find which volume contains this position
        if let Some((vol, offset)) = self.volume_for_position(new_pos) {
            // Need to reopen if we're seeking to a different volume or backwards
            let need_reopen = self.current_reader.is_none() || vol != self.current_volume || new_pos < self.position;

            if need_reopen {
                self.current_reader = None;
                self.open_volume_at(vol, offset)?;
            } else {
                // Seeking forward in the same volume
                let bytes_to_skip = new_pos - self.position;
                if bytes_to_skip > 0 {
                    let mut remaining = bytes_to_skip as usize;
                    self.buffer.resize(8192.min(remaining), 0);
                    while remaining > 0 {
                        let to_read = self.buffer.len().min(remaining);
                        let reader = self.current_reader.as_mut().unwrap();
                        let n = reader.read(&mut self.buffer[..to_read])?;
                        if n == 0 {
                            break;
                        }
                        remaining -= n;
                    }
                }
            }
            self.position = new_pos;
        } else if new_pos >= self.total_size {
            // Seeking to or past the end
            self.position = self.total_size;
            self.current_reader = None;
        }

        Ok(self.position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    struct TestVolumeProvider {
        volumes: Vec<Vec<u8>>,
    }

    impl VolumeProvider for TestVolumeProvider {
        fn open_volume(&self, volume_number: u32) -> Option<Box<dyn Read + Send>> {
            self.volumes
                .get(volume_number as usize)
                .map(|data| Box::new(Cursor::new(data.clone())) as Box<dyn Read + Send>)
        }
    }

    #[test]
    fn test_multi_volume_read_sequential() {
        let provider = TestVolumeProvider {
            volumes: vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10]],
        };
        let sizes = vec![4, 4, 2];
        let mut reader = MultiVolumeReader::new(Arc::new(provider), sizes);

        let mut buf = vec![0u8; 10];
        let n = reader.read(&mut buf).unwrap();
        assert_eq!(n, 4);
        assert_eq!(&buf[..4], &[1, 2, 3, 4]);

        let n = reader.read(&mut buf).unwrap();
        assert_eq!(n, 4);
        assert_eq!(&buf[..4], &[5, 6, 7, 8]);

        let n = reader.read(&mut buf).unwrap();
        assert_eq!(n, 2);
        assert_eq!(&buf[..2], &[9, 10]);

        let n = reader.read(&mut buf).unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn test_multi_volume_seek() {
        let provider = TestVolumeProvider {
            volumes: vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10]],
        };
        let sizes = vec![4, 4, 2];
        let mut reader = MultiVolumeReader::new(Arc::new(provider), sizes);

        // Seek to middle of second volume
        reader.seek(SeekFrom::Start(5)).unwrap();
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(buf[0], 6);

        // Seek to start of third volume
        reader.seek(SeekFrom::Start(8)).unwrap();
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(buf[0], 9);

        // Seek from end
        reader.seek(SeekFrom::End(-2)).unwrap();
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(buf[0], 9);
    }
}
