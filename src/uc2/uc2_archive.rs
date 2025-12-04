use std::{
    arch::x86_64,
    io::{self, Read, Seek},
};

use super::ExtendedHeader;

pub struct Uc2Archive<T: Read + Seek> {
    archive_len: u32,
    is_damage_protected: bool,
    reader: T,
}

const HEADER_SIZE: usize = 4 + 8 + 1;
const ID: u32 = 0x1A324355; // UC2\x1A
const AMAG: u32 = 0x01B2C3D4;

impl<T: Read + Seek> Uc2Archive<T> {
    pub fn new(mut reader: T) -> io::Result<Self> {
        let (archive_len, is_damage_protected) = read_header(&mut reader)?;
        let x_head = ExtendedHeader::load_from(&mut reader)?;
        println!("c_dir_loc: {:?}", x_head.c_dir_loc);
        println!("made by: {:?}", x_head.made_by_version);
        println!("ver_need: {:?}", x_head.needed_version);
        Ok(Self {
            reader,
            archive_len,
            is_damage_protected,
        })
    }
}

fn read_header<T: Read + Seek>(reader: &mut T) -> Result<(u32, bool), io::Error> {
    let mut header_bytes = vec![0; HEADER_SIZE];
    reader.read_exact(&mut header_bytes)?;
    let mut header_bytes = header_bytes.as_slice();
    convert_u32!(id, header_bytes);
    if id != ID {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid UC2 archive",
        ));
    }
    convert_u32!(archive_len, header_bytes);
    convert_u32!(archive_len2, header_bytes);
    if archive_len.wrapping_add(AMAG) != archive_len2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "file header is damaged",
        ));
    }
    convert_u8!(damage_protected_flag, header_bytes);
    Ok((archive_len, damage_protected_flag != 0))
}
