//! ACE CRC implementation
//!
//! ACE uses a standard CRC-32 (polynomial 0xEDB88320, reflected),
//! but with inverted output. The CRC-16 is the lower 16 bits of this CRC-32.

/// CRC-32 lookup table (standard polynomial 0xEDB88320, reflected)
const CRC32_TABLE: [u32; 256] = {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut crc = i as u32;
        let mut j = 0;
        while j < 8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
};

/// Calculate ACE CRC-32 checksum
///
/// ACE CRC-32 uses standard CRC-32 polynomial (0xEDB88320, reflected)
/// with init=0xFFFFFFFF but NO final XOR (unlike standard CRC-32)
pub fn ace_crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFFFFFF;
    for &byte in data {
        crc = (crc >> 8) ^ CRC32_TABLE[((crc ^ byte as u32) & 0xFF) as usize];
    }
    crc // No final XOR for ACE
}

/// Calculate ACE CRC-16 checksum
///
/// ACE CRC-16 is the lower 16 bits of the ACE CRC-32
pub fn ace_crc16(data: &[u8]) -> u16 {
    (ace_crc32(data) & 0xFFFF) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc16() {
        // Test vector: "123456789" should give 50905
        let data = b"123456789";
        let crc = ace_crc16(data);
        assert_eq!(crc, 50905);
    }

    #[test]
    fn test_crc32() {
        // Test vector: "123456789" should give 873187033
        let data = b"123456789";
        let crc = ace_crc32(data);
        assert_eq!(crc, 873187033);
    }
}
