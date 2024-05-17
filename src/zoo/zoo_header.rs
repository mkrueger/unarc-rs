use std::io;

/// A random choice
pub const ZOO_TAG: u32 = 0xFDC4A7DC;
/// Header text for archive.
const TEXT: &[u8; 17] = b"ZOO 2.10 Archive.";
/// Size of header text
const SIZ_TEXT: usize = 20;

pub struct ZooHeader {
    /// where the archive's data starts
    pub zoo_start: u32,
    /// for consistency checking of zoo_start
    pub zoo_minus: u32,

    /// minimum version to extract all files
    pub major_ver: u8,
    /// minimum version to extract all files
    pub minor_ver: u8,
    /// position of archive comment
    pub cmt_pos: u32,
    /// length of archive comment
    pub cmt_len: u32,
    /// byte in archive;  data about versions
    pub vdata: u32,
}

pub const ZOO_HEADER_SIZE: usize = SIZ_TEXT + 4 + 22;
impl ZooHeader {
    pub fn load_from(mut header_bytes: &[u8]) -> io::Result<Self> {
        if !header_bytes.starts_with(TEXT) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid archive header",
            ));
        }
        header_bytes = &header_bytes[SIZ_TEXT..];
        convert_u32!(zoo_tag, header_bytes);
        if zoo_tag != ZOO_TAG {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid archive tag",
            ));
        }
        convert_u32!(zoo_start, header_bytes);
        convert_u32!(zoo_minus, header_bytes);
        convert_u8!(major_ver, header_bytes);
        convert_u8!(minor_ver, header_bytes);
        convert_u32!(cmt_pos, header_bytes);
        convert_u32!(cmt_len, header_bytes);
        convert_u32!(vadata, header_bytes);

        Ok(Self {
            zoo_start,
            zoo_minus,
            major_ver,
            minor_ver,
            cmt_pos,
            cmt_len,
            vdata: vadata,
        })
    }
}
