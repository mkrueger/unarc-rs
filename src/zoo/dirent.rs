use std::io;

use crate::{date_time::DosDateTime, zoo::zoo_header::ZOO_TAG};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemId {
    Unix = 0,
    Dos = 1,
    Portable = 2,
    Unknown(u8),
}

impl From<u8> for SystemId {
    fn from(value: u8) -> Self {
        match value {
            0 => SystemId::Unix,
            1 => SystemId::Dos,
            2 => SystemId::Portable,
            _ => SystemId::Unknown(value),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMethod {
    Stored,
    Compressed,
    CompressedLh5,
    Unknown(u8),
}

impl From<u8> for CompressionMethod {
    fn from(value: u8) -> Self {
        match value {
            0 => CompressionMethod::Stored,
            1 => CompressionMethod::Compressed,
            2 => CompressionMethod::CompressedLh5,
            _ => CompressionMethod::Unknown(value),
        }
    }
}
pub struct DirectoryEntry {
    //unsigned long zoo_tag;     /* tag -- redundancy check */
    //uchar type;                 /* type of directory entry.  always 1 for now */
    pub compression_method: CompressionMethod,

    /// pos'n of next directory entry
    pub next: u32,
    /// position of this file
    pub offset: u32,

    /// DOS format date
    pub date_time: DosDateTime,

    pub file_crc16: u16,
    pub org_size: u32,
    pub size_now: u32,

    pub major_ver: u8,
    pub minor_ver: u8, /* minimum version needed to extract */
    pub deleted: bool, /* will be 1 if deleted, 0 if not */
    pub struc: u8,     /* file structure if any */
    pub comment: u32,  /* points to comment;  zero if none */
    pub cmt_size: u16, /* length of comment, 0 if none */
    pub name: String,  /* filename */

    pub var_dir_len: u8, /* length of variable part of dir entry */
    pub tz: u8,          /* timezone where file was archived */
    pub dir_crc: u32,    /* CRC of directory entry */

    /* fields for variable part of directory entry follow */
    pub namlen: u8,      /* length of long filename */
    pub dirlen: u8,      /* length of directory name */
    pub lfname: String,  /* long filename */
    pub dirname: String, /* directory name */
    pub system_id: SystemId,
    pub fattr: u32,      /* File attributes -- 24 bits */
    pub vflag: u32,      /* version flag bits -- one byte in archive */
    pub version_no: u32, /* file version number if any */
}

/// Size of DOS filename
const FNAMESIZE: usize = 13;
pub const DIRENT_HEADER_SIZE: usize = 5 +  // tag + dir_type
    1 +  // comp method
    8 +  // offsets
    24 +  // up to incl. cmt_size
    FNAMESIZE +
    8;
/// archive header type
const _H_TYPE: u8 = 1;

impl DirectoryEntry {
    pub fn load_from(mut header_bytes: &[u8]) -> io::Result<Self> {
        convert_u32!(zoo_tag, header_bytes);
        if zoo_tag != ZOO_TAG {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid archive tag",
            ));
        }
        convert_u8!(_dir_type, header_bytes); // type of directory entry.  always 1 for now

        convert_u8!(compression_method, header_bytes);

        convert_u32!(next, header_bytes);
        convert_u32!(offset, header_bytes);

        convert_u32!(date_time2, header_bytes);
        convert_u16!(file_crc, header_bytes);
        convert_u32!(org_size, header_bytes);
        convert_u32!(size_now, header_bytes);
        convert_u8!(major_ver, header_bytes);
        convert_u8!(minor_ver, header_bytes);
        convert_u8!(deleted, header_bytes);
        convert_u8!(struc, header_bytes);
        convert_u32!(comment, header_bytes);
        convert_u16!(cmt_size, header_bytes);

        let idx = header_bytes
            .iter()
            .position(|&x| x == 0)
            .unwrap_or(FNAMESIZE);
        let fname = String::from_utf8_lossy(&header_bytes[0..idx]).to_string();
        header_bytes = &header_bytes[FNAMESIZE..];

        convert_u8!(var_dir_len, header_bytes);
        convert_u8!(tz, header_bytes);
        convert_u32!(dir_crc, header_bytes);

        convert_u8!(namlen, header_bytes);
        convert_u8!(dirlen, header_bytes);

        Ok(DirectoryEntry {
            compression_method: compression_method.into(),
            next,
            offset,
            date_time: DosDateTime::new(date_time2),
            file_crc16: file_crc,
            org_size,
            size_now,
            major_ver,
            minor_ver,
            deleted: deleted != 0,
            struc,
            comment,
            cmt_size,
            name: fname,
            var_dir_len,
            tz,
            dir_crc,
            namlen,
            dirlen,
            lfname: String::new(),
            dirname: String::new(),
            system_id: SystemId::Unknown(0),
            fattr: 0,
            vflag: 0,
            version_no: 0,
        })
    }
}
