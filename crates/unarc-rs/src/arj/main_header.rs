use crate::date_time::DosDateTime;

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HostOS {
    MsDos = 0,
    PrimOS = 1,
    Unix = 2,
    Amiga = 3,
    MacOs = 4,
    OS2 = 5,
    AppleGS = 6,
    AtariST = 7,
    NeXT = 8,
    VaxVMS = 9,
    Win95 = 10,
    Win32 = 11,

    Unknown(u8),
}

impl From<u8> for HostOS {
    fn from(value: u8) -> Self {
        match value {
            0 => HostOS::MsDos,
            1 => HostOS::PrimOS,
            2 => HostOS::Unix,
            3 => HostOS::Amiga,
            4 => HostOS::MacOs,
            5 => HostOS::OS2,
            6 => HostOS::AppleGS,
            7 => HostOS::AtariST,
            8 => HostOS::NeXT,
            9 => HostOS::VaxVMS,
            10 => HostOS::Win95,
            11 => HostOS::Win32,
            _ => HostOS::Unknown(value),
        }
    }
}

pub struct MainHeader {
    pub archiver_version_number: u8,
    pub min_version_to_extract: u8,
    pub host_os: HostOS,
    pub flags: u8,
    pub security_version: u8,
    pub file_type: u8,

    pub creation_date_time: DosDateTime,
    pub compr_size: u32,
    pub archive_size: u32,
    /// file position
    pub security_envelope: u32,
    pub file_spec_position: u16,
    pub security_envelope_length: u16,
    pub encryption_version: u8,
    pub last_chapter: u8,
    pub arj_protection_factor: u8,
    pub flags2: u8,
    pub name: String,
    pub comment: String,
}
const FIRST_HDR_SIZE: u8 = 34;
impl MainHeader {
    pub fn load_from(mut header_bytes: &[u8]) -> Self {
        convert_u8!(header_size, header_bytes);
        convert_u8!(archiver_version_number, header_bytes);
        convert_u8!(min_version_to_extract, header_bytes);
        convert_u8!(host_os, header_bytes);
        convert_u8!(flags, header_bytes);
        convert_u8!(security_version, header_bytes);
        convert_u8!(file_type, header_bytes);
        skip!(header_bytes, 1);
        convert_u32!(creation_date_time, header_bytes);
        convert_u32!(compr_size, header_bytes);
        convert_u32!(archive_size, header_bytes);
        convert_u32!(security_envelope, header_bytes);
        convert_u16!(file_spec_position, header_bytes);
        convert_u16!(security_envelope_length, header_bytes);

        convert_u8!(encryption_version, header_bytes);
        convert_u8!(last_chapter, header_bytes);

        let mut arj_protection_factor = 0;
        let mut flags2 = 0;

        if header_size >= FIRST_HDR_SIZE {
            convert_u8!(arj_protection_factor2, header_bytes);
            convert_u8!(arj_flags22, header_bytes);
            arj_protection_factor = arj_protection_factor2;
            flags2 = arj_flags22;
            skip!(header_bytes, 2);
        }

        convert_string!(name, header_bytes);
        convert_string!(comment, header_bytes);
        Self {
            archiver_version_number,
            min_version_to_extract,
            host_os: host_os.into(),
            flags,
            security_version,
            file_type,
            creation_date_time: DosDateTime::new(creation_date_time),
            compr_size,
            archive_size,
            security_envelope,
            file_spec_position,
            security_envelope_length,
            encryption_version,
            last_chapter,
            arj_protection_factor,
            flags2,
            name,
            comment,
        }
    }

    pub fn is_gabled(&self) -> bool {
        self.flags & 0x01 != 0
    }

    pub fn is_ansi_page(&self) -> bool {
        self.flags & 0x02 != 0
    }

    pub fn is_volume(&self) -> bool {
        self.flags & 0x04 != 0
    }

    pub fn is_arj_protected(&self) -> bool {
        self.flags & 0x08 != 0
    }

    pub fn is_path_sym(&self) -> bool {
        self.flags & 0x10 != 0
    }

    pub fn is_backup(&self) -> bool {
        self.flags & 0x20 != 0
    }

    pub fn is_secured(&self) -> bool {
        self.flags & 0x40 != 0
    }

    pub fn is_altname(&self) -> bool {
        self.flags & 0x80 != 0
    }
}
