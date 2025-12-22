use std::io::{self, Read};

mod consts;
mod decompress;
mod supermaster_decompressed;
pub mod uc2_archive;

/// Location in archive (volume + offset)
#[derive(Debug, Clone, Copy, Default)]
pub struct Location {
    pub volume: u32,
    pub offset: u32,
}

pub(crate) struct ExtendedHeader {
    /// Start of CDIR
    pub c_dir_loc: Location,
}

/// Extended header size: volume(4) + offset(4) + fletch(2) + is_busy(1) + versions(4) + reserved(1)
const EXT_HEADER_SIZE: usize = 16;

impl ExtendedHeader {
    pub fn load_from<T: Read>(reader: &mut T) -> io::Result<Self> {
        let mut header_bytes = [0u8; EXT_HEADER_SIZE];
        reader.read_exact(&mut header_bytes)?;
        let c_dir_loc_volume = u32::from_le_bytes([
            header_bytes[0],
            header_bytes[1],
            header_bytes[2],
            header_bytes[3],
        ]);
        let c_dir_loc_offset = u32::from_le_bytes([
            header_bytes[4],
            header_bytes[5],
            header_bytes[6],
            header_bytes[7],
        ]);
        // Remaining bytes (fletch, is_busy, versions, reserved) are unused
        Ok(Self {
            c_dir_loc: Location {
                volume: c_dir_loc_volume,
                offset: c_dir_loc_offset,
            },
        })
    }
}

/// Compression info for a file or master
#[derive(Debug, Clone, Default)]
pub struct CompressInfo {
    pub compressed_length: u32,
    pub method: u16,
    pub master_prefix: u32,
}

impl CompressInfo {
    pub fn load_from<T: Read>(reader: &mut T) -> io::Result<Self> {
        let mut buf = [0u8; 10];
        reader.read_exact(&mut buf)?;
        let mut data = buf.as_slice();
        convert_u32!(compressed_length, data);
        convert_u16!(method, data);
        convert_u32!(master_prefix, data);
        Ok(Self {
            compressed_length,
            method,
            master_prefix,
        })
    }
}

/// Entry types in CDIR
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EntryType {
    DirEntry = 1,
    FileEntry = 2,
    MasterEntry = 3,
    EndOfCdir = 4,
}

impl TryFrom<u8> for EntryType {
    type Error = io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(EntryType::DirEntry),
            2 => Ok(EntryType::FileEntry),
            3 => Ok(EntryType::MasterEntry),
            4 => Ok(EntryType::EndOfCdir),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid entry type: {}", value),
            )),
        }
    }
}

/// File attributes (MS-DOS style)
#[derive(Debug, Clone, Copy, Default)]
pub struct FileAttributes(u8);

impl FileAttributes {
    pub fn is_readonly(&self) -> bool {
        self.0 & 0x01 != 0
    }
    pub fn is_hidden(&self) -> bool {
        self.0 & 0x02 != 0
    }
    pub fn is_system(&self) -> bool {
        self.0 & 0x04 != 0
    }
    pub fn is_directory(&self) -> bool {
        self.0 & 0x10 != 0
    }
    pub fn is_archive(&self) -> bool {
        self.0 & 0x20 != 0
    }
}

impl From<u8> for FileAttributes {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

/// UC2 archive entry (file or directory)
#[derive(Debug, Clone)]
pub struct Uc2Entry {
    /// Parent directory index (0 = root)
    pub parent_dir: u32,
    /// DOS file attributes
    pub attributes: FileAttributes,
    /// DOS time
    pub dos_time: u32,
    /// File name
    pub name: String,
}
