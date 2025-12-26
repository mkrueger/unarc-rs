use std::io::{Read, Seek};
use std::sync::Arc;

use delharc::decode::{Decoder, DecoderAny};

use crate::date_time::DosDateTime;
use crate::encryption::ArjEncryption;
use crate::error::{ArchiveError, Result};
use crate::unified::VolumeProvider;

use super::{
    crypto::decrypt_arj_data,
    decode_fastest::decode_fastest,
    local_file_header::{CompressionMethod, LocalFileHeader},
    main_header::{HostOS, MainHeader},
    password_verifier::ArjPasswordVerifier,
};

pub struct ArjArchive<T: Read + Seek> {
    reader: T,
    header: MainHeader,
    password: Option<String>,
    volume_provider: Option<Arc<dyn VolumeProvider>>,
    current_volume: u32,
}

impl<T: Read + Seek> ArjArchive<T> {
    pub fn new(mut reader: T) -> Result<Self> {
        let header_bytes = read_header(&mut reader)?;
        if header_bytes.is_empty() {
            return Err(ArchiveError::invalid_header("ARJ"));
        }
        let header = MainHeader::load_from(&header_bytes);
        // Skip extended headers
        read_extended_headers(&mut reader)?;

        Ok(Self {
            header,
            reader,
            password: None,
            volume_provider: None,
            current_volume: 0,
        })
    }

    /// Set the volume provider for multi-volume archives
    pub fn set_volume_provider(&mut self, provider: Arc<dyn VolumeProvider>) {
        self.volume_provider = Some(provider);
    }

    /// Set the password for decrypting garbled entries
    pub fn set_password(&mut self, password: &str) {
        self.password = Some(password.to_string());
    }

    /// Clear the password
    pub fn clear_password(&mut self) {
        self.password = None;
    }

    /// Get the encryption type for the archive
    pub fn get_encryption_type(&self) -> Option<ArjEncryption> {
        ArjEncryption::from_version(self.header.encryption_version, self.header.is_gabled())
    }

    pub fn skip(&mut self, header: &LocalFileHeader) -> Result<()> {
        self.reader
            .seek(std::io::SeekFrom::Current(header.compressed_size as i64))?;
        Ok(())
    }

    pub fn read(&mut self, header: &LocalFileHeader) -> Result<Vec<u8>> {
        // Check if file is encrypted and we have no password
        if header.is_garbled() && self.password.is_none() {
            // Skip the data and return error
            self.skip(header)?;
            return Err(ArchiveError::encryption_required(&header.name, "ARJ"));
        }

        // Some ARJ encryption modes are detected but cannot be decrypted.
        // Fail early with a clear error instead of later CRC/decompression errors.
        let encryption_type = if header.is_garbled() {
            let enc = self.get_encryption_type();
            match enc {
                Some(ArjEncryption::Gost256) => {
                    self.skip(header)?;
                    return Err(ArchiveError::unsupported_method(
                        "ARJ",
                        "GOST-256 encryption (requires ARJCRYPT; decrypt externally first)",
                    ));
                }
                Some(ArjEncryption::Unknown) => {
                    self.skip(header)?;
                    return Err(ArchiveError::unsupported_method(
                        "ARJ",
                        "unknown encryption method",
                    ));
                }
                _ => enc,
            }
        } else {
            None
        };

        // For multi-volume files, we need to decompress each chunk separately
        // and then concatenate the decompressed data
        if header.is_volume() {
            return self.read_multi_volume(header, encryption_type);
        }

        // Single volume: read and decompress normally
        let mut compressed_buffer = vec![0; header.compressed_size as usize];
        self.reader.read_exact(&mut compressed_buffer)?;

        // Decrypt if needed
        if header.is_garbled() {
            if let Some(ref password) = self.password {
                let ftime: u32 = header.date_time_modified.into();
                decrypt_arj_data(
                    &mut compressed_buffer,
                    encryption_type,
                    password,
                    header.password_modifier,
                    ftime,
                );
            }
        }

        let uncompressed = self.decompress_chunk(
            &compressed_buffer,
            header.original_size as usize,
            &header.compression_method,
        )?;

        if uncompressed.len() != header.original_size as usize {
            return Err(ArchiveError::decompression_failed(
                &header.name,
                format!(
                    "size mismatch: expected {}, got {}",
                    header.original_size,
                    uncompressed.len()
                ),
            ));
        }

        let checksum = crc32fast::hash(&uncompressed);
        if checksum != header.original_crc32 {
            Err(ArchiveError::crc_mismatch(
                &header.name,
                header.original_crc32,
                checksum,
            ))
        } else {
            Ok(uncompressed)
        }
    }

    /// Decompress a single chunk of data
    fn decompress_chunk(
        &self,
        compressed: &[u8],
        original_size: usize,
        method: &CompressionMethod,
    ) -> Result<Vec<u8>> {
        match method {
            CompressionMethod::Stored => Ok(compressed.to_vec()),
            CompressionMethod::CompressedMost
            | CompressionMethod::Compressed
            | CompressionMethod::CompressedFaster => {
                let mut decoder =
                    DecoderAny::new_from_compression(delharc::CompressionMethod::Lh6, compressed);
                let mut decompressed_buffer = vec![0; original_size];
                decoder.fill_buffer(&mut decompressed_buffer)?;
                Ok(decompressed_buffer)
            }
            CompressionMethod::CompressedFastest => decode_fastest(compressed, original_size),
            CompressionMethod::NoDataNoCrc
            | CompressionMethod::NoData
            | CompressionMethod::Unknown(_) => Err(ArchiveError::unsupported_method(
                "ARJ",
                format!("{:?}", method),
            )),
        }
    }

    /// Read and decompress a multi-volume file
    /// Each volume chunk is decompressed separately, CRC-checked, then concatenated
    fn read_multi_volume(
        &mut self,
        header: &LocalFileHeader,
        encryption_type: Option<ArjEncryption>,
    ) -> Result<Vec<u8>> {
        let volume_provider = match &self.volume_provider {
            Some(provider) => provider.clone(),
            None => {
                self.skip(header)?;
                return Err(ArchiveError::io_error(
                    "Multi-volume archive requires a volume provider",
                ));
            }
        };

        // Total expected size
        let total_size = if header.original_size_even_for_volumes > 0 {
            header.original_size_even_for_volumes as usize
        } else {
            header.original_size as usize
        };

        let mut result = Vec::with_capacity(total_size);
        let filename = header.name.clone();
        let compression_method = header.compression_method;

        // Read and decompress first chunk from current volume
        let mut compressed_buffer = vec![0; header.compressed_size as usize];
        self.reader.read_exact(&mut compressed_buffer)?;

        // Decrypt if needed
        if header.is_garbled() {
            if let Some(ref password) = self.password {
                let ftime: u32 = header.date_time_modified.into();
                decrypt_arj_data(
                    &mut compressed_buffer,
                    encryption_type,
                    password,
                    header.password_modifier,
                    ftime,
                );
            }
        }

        let decompressed = self.decompress_chunk(
            &compressed_buffer,
            header.original_size as usize,
            &compression_method,
        )?;

        // Verify CRC of first chunk
        let checksum = crc32fast::hash(&decompressed);
        if checksum != header.original_crc32 {
            return Err(ArchiveError::crc_mismatch(
                &filename,
                header.original_crc32,
                checksum,
            ));
        }

        result.extend_from_slice(&decompressed);

        // Read continuation chunks from subsequent volumes
        let mut continues = header.is_volume();
        while continues {
            self.current_volume += 1;

            // Open next volume
            let mut next_volume = match volume_provider.open_volume(self.current_volume) {
                Some(reader) => reader,
                None => {
                    return Err(ArchiveError::io_error(format!(
                        "Cannot open volume {} for multi-volume archive",
                        self.current_volume
                    )));
                }
            };

            // Read and skip main header
            let main_header_bytes = read_header(&mut next_volume)?;
            if main_header_bytes.is_empty() {
                return Err(ArchiveError::corrupted_entry_named(
                    "ARJ",
                    &filename,
                    "unexpected end of volume header",
                ));
            }
            read_extended_headers(&mut next_volume)?;

            // Read the file header (should be continuation of our file)
            let file_header_bytes = read_header(&mut next_volume)?;
            if file_header_bytes.is_empty() {
                return Err(ArchiveError::corrupted_entry_named(
                    "ARJ",
                    &filename,
                    "unexpected end of file header in volume",
                ));
            }

            let continuation_header =
                LocalFileHeader::load_from(&file_header_bytes).ok_or_else(|| {
                    ArchiveError::corrupted_entry_named(
                        "ARJ",
                        &filename,
                        "invalid continuation header",
                    )
                })?;

            // Verify this is a continuation of the same file
            if !continuation_header.is_ext_file() {
                return Err(ArchiveError::corrupted_entry_named(
                    "ARJ",
                    &filename,
                    "expected extended file (continuation) header",
                ));
            }

            if continuation_header.name != filename {
                return Err(ArchiveError::corrupted_entry_named(
                    "ARJ",
                    &filename,
                    format!(
                        "filename mismatch in continuation: expected '{}', got '{}'",
                        filename, continuation_header.name
                    ),
                ));
            }

            read_extended_headers(&mut next_volume)?;

            // Read the compressed data from this volume
            let mut chunk = vec![0; continuation_header.compressed_size as usize];
            next_volume.read_exact(&mut chunk)?;

            // Decrypt if needed (continuation chunks use their own header data)
            if continuation_header.is_garbled() {
                if let Some(ref password) = self.password {
                    let ftime: u32 = continuation_header.date_time_modified.into();
                    decrypt_arj_data(
                        &mut chunk,
                        encryption_type,
                        password,
                        continuation_header.password_modifier,
                        ftime,
                    );
                }
            }

            // Decompress this chunk
            let decompressed = self.decompress_chunk(
                &chunk,
                continuation_header.original_size as usize,
                &continuation_header.compression_method,
            )?;

            // Verify CRC of this chunk
            let checksum = crc32fast::hash(&decompressed);
            if checksum != continuation_header.original_crc32 {
                return Err(ArchiveError::crc_mismatch(
                    format!("{} (volume {})", filename, self.current_volume),
                    continuation_header.original_crc32,
                    checksum,
                ));
            }

            result.extend_from_slice(&decompressed);

            // Check if there are more volumes
            continues = continuation_header.is_volume();
        }

        // Verify total size
        if result.len() != total_size {
            return Err(ArchiveError::decompression_failed(
                &filename,
                format!(
                    "multi-volume size mismatch: expected {}, got {}",
                    total_size,
                    result.len()
                ),
            ));
        }

        Ok(result)
    }

    pub fn get_next_entry(&mut self) -> Result<Option<LocalFileHeader>> {
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

    /// Read an entry with a specific password (for per-entry decryption)
    ///
    /// This temporarily sets the password for this read operation only,
    /// then restores the previous password state.
    pub fn read_with_password(
        &mut self,
        header: &LocalFileHeader,
        password: Option<String>,
    ) -> Result<Vec<u8>> {
        let old_password = self.password.take();
        self.password = password;
        let result = self.read(header);
        self.password = old_password;
        result
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

    /// Create a standalone password verifier for the given entry.
    ///
    /// This reads the compressed data from the archive and creates a verifier
    /// that can be used independently (and in parallel) to test passwords.
    ///
    /// The verifier is `Send + Sync` and can be safely used with rayon.
    ///
    /// Returns `None` if the entry is not encrypted or uses unsupported encryption.
    pub fn create_password_verifier(
        &mut self,
        header: &LocalFileHeader,
    ) -> Result<Option<ArjPasswordVerifier>> {
        // Check if encrypted
        if !header.is_garbled() {
            return Ok(None);
        }

        // Check encryption type
        let encryption_type = self.get_encryption_type();
        match encryption_type {
            Some(ArjEncryption::Gost256) => {
                return Err(ArchiveError::unsupported_method(
                    "ARJ",
                    "GOST-256 encryption (requires ARJCRYPT; decrypt externally first)",
                ));
            }
            Some(ArjEncryption::Unknown) => {
                return Err(ArchiveError::unsupported_method(
                    "ARJ",
                    "unknown encryption method",
                ));
            }
            _ => {}
        }

        // Read compressed data
        let mut compressed_data = vec![0; header.compressed_size as usize];
        self.reader.read_exact(&mut compressed_data)?;

        let file_time: u32 = header.date_time_modified.into();

        Ok(Some(ArjPasswordVerifier::new(
            compressed_data,
            header.compression_method,
            header.original_crc32,
            header.original_size,
            header.name.clone(),
            encryption_type,
            header.password_modifier,
            file_time,
        )))
    }
}

const MAX_HEADER_SIZE: usize = 2600;
const ARJ_MAGIC_1: u8 = 0x60;
const ARJ_MAGIC_2: u8 = 0xEA;

fn read_header<R: Read>(reader: &mut R) -> Result<Vec<u8>> {
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
        return Err(ArchiveError::corrupted_entry_named(
            "ARJ",
            "header",
            format!(
                "header size {} exceeds maximum {}",
                header_size, MAX_HEADER_SIZE
            ),
        ));
    }
    let mut header_bytes = vec![0; header_size as usize];
    reader.read_exact(&mut header_bytes)?;
    let mut crc = [0, 0, 0, 0];
    reader.read_exact(&mut crc)?;
    let checksum = crc32fast::hash(&header_bytes);
    let expected = u32::from_le_bytes(crc);
    if checksum != expected {
        Err(ArchiveError::crc_mismatch("ARJ header", expected, checksum))
    } else {
        Ok(header_bytes)
    }
}

fn read_extended_headers<R: Read>(reader: &mut R) -> Result<Vec<Vec<u8>>> {
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
        let expected = u32::from_le_bytes(crc);
        if checksum != expected {
            return Err(ArchiveError::crc_mismatch(
                "ARJ extended header",
                expected,
                checksum,
            ));
        }
        extended_header.push(header);
    }
}
