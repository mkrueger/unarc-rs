use std::io::{self, Read};

pub mod uc2_archive;

#[derive(Debug)]
struct Location {
    pub volume: u32,
    pub offset: u32,
}

struct ExtendedHeader {
    // start of CDIR
    pub c_dir_loc: Location,
    // fletcher checksum of CDIR
    pub fletch: u16,
    pub is_busy: bool,
    pub made_by_version: u16,
    pub needed_version: u16,
}

const EXT_HEADER_SIZE: usize = 8 + 2 + 1 + 4 + 1;
impl ExtendedHeader {
    pub fn load_from<T: Read>(reader: &mut T) -> io::Result<Self> {
        let mut header_bytes = vec![0; EXT_HEADER_SIZE];
        reader.read_exact(&mut header_bytes)?;
        let mut header_bytes = header_bytes.as_slice();
        convert_u32!(c_dir_loc_volume, header_bytes);
        convert_u32!(c_dir_loc_offset, header_bytes);
        convert_u16!(fletch, header_bytes);
        convert_u8!(is_busy, header_bytes);
        convert_u16!(made_by_version, header_bytes);
        convert_u16!(needed_version, header_bytes);
        // last byte is reserved
        debug_assert!(header_bytes.len() == 1);
        Ok(Self {
            c_dir_loc: Location {
                volume: c_dir_loc_volume,
                offset: c_dir_loc_offset,
            },
            fletch,
            is_busy: is_busy != 0,
            made_by_version,
            needed_version,
        })
    }
}
