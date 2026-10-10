//! Helpers shared by the Commodore 64 container formats (D64, T64, Lynx).
//!
//! Commodore DOS stores file names in PETSCII and gives every file one of a
//! few CBM file types. Extracted entries are named `NAME.ext`, where `NAME`
//! is the PETSCII name converted to Unicode and `ext` comes from the file type.

/// Commodore DOS file type of a container entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CbmFileType {
    /// Deleted file (`DEL`); usually a zero-length directory separator.
    Del,
    /// Sequential data file (`SEQ`).
    Seq,
    /// Program file (`PRG`); the first two bytes are the load address.
    Prg,
    /// User file (`USR`).
    Usr,
    /// Relative (record-oriented) file (`REL`).
    Rel,
    /// A type value that Commodore DOS does not define for this medium.
    Other(u8),
}

impl CbmFileType {
    /// Decodes the low bits of a CBM DOS directory type byte.
    pub fn from_dos_type(value: u8) -> Self {
        match value & 0x07 {
            0 => Self::Del,
            1 => Self::Seq,
            2 => Self::Prg,
            3 => Self::Usr,
            4 => Self::Rel,
            other => Self::Other(other),
        }
    }

    /// File name extension used for extracted entries of this type.
    pub fn extension(self) -> &'static str {
        match self {
            Self::Del => "del",
            Self::Seq => "seq",
            Self::Prg => "prg",
            Self::Usr => "usr",
            Self::Rel => "rel",
            Self::Other(_) => "cbm",
        }
    }
}

/// Converts a PETSCII file name to a readable string.
///
/// The name ends at the first shifted space (`0xA0`), as in Commodore DOS.
/// Letters are converted as the C64 displays them: a name containing shifted
/// letters (`0x61`-`0x7A`, `0xC1`-`0xDA`) was typed in the lower/upper case
/// character set, so `0x41`-`0x5A` become lower case and the shifted letters
/// upper case; otherwise `0x41`-`0x5A` are upper case. `£`, `↑` and `←` keep
/// their Unicode equivalents. Graphics and control characters, and the path
/// separator `/`, become `_`, so the result never contains `/` or `\`.
pub fn petscii_to_string(name: &[u8]) -> String {
    let name = name.split(|&b| b == 0xA0).next().unwrap_or_default();
    let shifted = name.iter().any(|&b| matches!(b, 0x61..=0x7A | 0xC1..=0xDA));
    let converted: String = name
        .iter()
        .map(|&b| match b {
            b'/' => '_',
            0x41..=0x5A if shifted => char::from(b.to_ascii_lowercase()),
            0x20..=0x5B | b']' => char::from(b),
            0x61..=0x7A => char::from(b.to_ascii_uppercase()),
            0xC1..=0xDA => char::from(b - 0x80),
            0x5C => '£',
            0x5E => '↑',
            0x5F => '←',
            _ => '_',
        })
        .collect();
    if converted.is_empty() {
        "_".to_string()
    } else {
        converted
    }
}

/// Builds the name of an extracted entry: the converted PETSCII name plus the type extension.
pub fn entry_name(petscii_name: &[u8], file_type: CbmFileType) -> String {
    format!("{}.{}", petscii_to_string(petscii_name), file_type.extension())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unshifted_names_are_upper_case() {
        assert_eq!(petscii_to_string(b"HELLO WORLD\xA0\xA0\xA0\xA0\xA0"), "HELLO WORLD");
        assert_eq!(petscii_to_string(b"GAME V1.2"), "GAME V1.2");
    }

    #[test]
    fn shifted_names_keep_their_case() {
        // "Hello" typed in the lower/upper case character set
        assert_eq!(petscii_to_string(&[0xC8, 0x45, 0x4C, 0x4C, 0x4F]), "Hello");
        assert_eq!(petscii_to_string(&[0x61, 0x42]), "Ab");
    }

    #[test]
    fn name_ends_at_first_shifted_space() {
        assert_eq!(petscii_to_string(b"DEMO\xA0,8,1"), "DEMO");
    }

    #[test]
    fn separators_and_graphics_are_replaced() {
        assert_eq!(petscii_to_string(b"A/B"), "A_B");
        assert_eq!(petscii_to_string(&[0x5C, 0x5E, 0x5F, 0x12, 0xB0]), "£↑←__");
        assert_eq!(petscii_to_string(b""), "_");
        assert_eq!(petscii_to_string(b"\xA0\xA0"), "_");
        for b in 0..=255u8 {
            let s = petscii_to_string(&[b]);
            assert!(!s.contains('/') && !s.contains('\\'), "byte {b:#04x}");
        }
    }

    #[test]
    fn entry_names_carry_the_type_extension() {
        assert_eq!(entry_name(b"HELLO", CbmFileType::Prg), "HELLO.prg");
        assert_eq!(entry_name(b"DATA", CbmFileType::from_dos_type(0x81)), "DATA.seq");
        assert_eq!(CbmFileType::from_dos_type(0x84).extension(), "rel");
        assert_eq!(CbmFileType::from_dos_type(0x85), CbmFileType::Other(5));
    }
}
