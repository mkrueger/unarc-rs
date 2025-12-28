//! Encryption method detection for various archive formats.
//!
//! This module provides a unified way to represent encryption methods
//! used by different archive formats.

/// ZIP encryption methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZipEncryption {
    /// Traditional PKWARE encryption (weak, easily cracked)
    ZipCrypto,
    /// WinZip AES-128 encryption
    Aes128,
    /// WinZip AES-192 encryption
    Aes192,
    /// WinZip AES-256 encryption
    Aes256,
    /// Encrypted but method unknown
    Unknown,
}

impl std::fmt::Display for ZipEncryption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZipEncryption::ZipCrypto => write!(f, "ZipCrypto"),
            ZipEncryption::Aes128 => write!(f, "AES-128"),
            ZipEncryption::Aes192 => write!(f, "AES-192"),
            ZipEncryption::Aes256 => write!(f, "AES-256"),
            ZipEncryption::Unknown => write!(f, "Unknown"),
        }
    }
}

/// RAR encryption methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RarEncryption {
    /// RAR 2.x proprietary encryption (weak)
    Legacy,
    /// RAR 3.x/4.x AES-128 encryption
    Aes128,
    /// RAR 5.x AES-256 with PBKDF2 key derivation
    Aes256,
    /// Encrypted but method unknown
    Unknown,
}

impl std::fmt::Display for RarEncryption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RarEncryption::Legacy => write!(f, "RAR Legacy"),
            RarEncryption::Aes128 => write!(f, "AES-128"),
            RarEncryption::Aes256 => write!(f, "AES-256"),
            RarEncryption::Unknown => write!(f, "Unknown"),
        }
    }
}

/// 7z encryption methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SevenZEncryption {
    /// AES-256 with PBKDF2 key derivation
    Aes256,
    /// Encrypted but method unknown
    Unknown,
}

impl std::fmt::Display for SevenZEncryption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SevenZEncryption::Aes256 => write!(f, "AES-256"),
            SevenZEncryption::Unknown => write!(f, "Unknown"),
        }
    }
}

/// ARJ encryption methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArjEncryption {
    /// Standard "garble" encryption (XOR-based, very weak)
    Garble,
    /// GOST 28147-89 with 40-bit key (export-restricted, weak)
    Gost40,
    /// GOST 28147-89 with 256-bit key (requires ARJCRYPT module)
    Gost256,
    /// Encrypted but method unknown
    Unknown,
}

impl std::fmt::Display for ArjEncryption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArjEncryption::Garble => write!(f, "Garble (XOR)"),
            ArjEncryption::Gost40 => write!(f, "GOST40"),
            ArjEncryption::Gost256 => write!(f, "GOST-256"),
            ArjEncryption::Unknown => write!(f, "Unknown"),
        }
    }
}

impl ArjEncryption {
    /// Get encryption type from ARJ encryption version byte
    pub fn from_version(version: u8, is_garbled: bool) -> Option<Self> {
        if !is_garbled {
            return None;
        }
        Some(match version {
            0 | 1 => ArjEncryption::Garble,
            2 => ArjEncryption::Gost40,
            v if v >= 3 => ArjEncryption::Gost256,
            _ => ArjEncryption::Unknown,
        })
    }
}

/// Unified encryption method enum
///
/// Represents the encryption method used by an archive entry.
/// Each format has its own sub-enum for format-specific methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EncryptionMethod {
    /// Not encrypted
    #[default]
    None,
    /// ZIP encryption
    Zip(ZipEncryption),
    /// RAR encryption
    Rar(RarEncryption),
    /// 7z encryption
    SevenZ(SevenZEncryption),
    /// ACE encryption (Blowfish-based)
    Ace,
    /// ARJ encryption
    Arj(ArjEncryption),
    /// Unknown encryption method (encrypted but method not determined)
    Unknown,
}

impl EncryptionMethod {
    /// Returns true if the entry is encrypted
    pub fn is_encrypted(&self) -> bool {
        !matches!(self, EncryptionMethod::None)
    }

    /// Returns true if the encryption is considered weak/insecure
    pub fn is_weak(&self) -> bool {
        matches!(
            self,
            EncryptionMethod::Zip(ZipEncryption::ZipCrypto)
                | EncryptionMethod::Rar(RarEncryption::Legacy)
                | EncryptionMethod::Arj(ArjEncryption::Garble)
                | EncryptionMethod::Arj(ArjEncryption::Gost40)
        )
    }

    /// Returns a human-readable description of the encryption method
    pub fn description(&self) -> &'static str {
        match self {
            EncryptionMethod::None => "None",
            EncryptionMethod::Zip(ZipEncryption::ZipCrypto) => "ZIP Traditional (weak)",
            EncryptionMethod::Zip(ZipEncryption::Aes128) => "ZIP AES-128",
            EncryptionMethod::Zip(ZipEncryption::Aes192) => "ZIP AES-192",
            EncryptionMethod::Zip(ZipEncryption::Aes256) => "ZIP AES-256",
            EncryptionMethod::Zip(ZipEncryption::Unknown) => "ZIP Encrypted",
            EncryptionMethod::Rar(RarEncryption::Legacy) => "RAR Legacy (weak)",
            EncryptionMethod::Rar(RarEncryption::Aes128) => "RAR AES-128",
            EncryptionMethod::Rar(RarEncryption::Aes256) => "RAR AES-256",
            EncryptionMethod::Rar(RarEncryption::Unknown) => "RAR Encrypted",
            EncryptionMethod::SevenZ(SevenZEncryption::Aes256) => "7z AES-256",
            EncryptionMethod::SevenZ(SevenZEncryption::Unknown) => "7z Encrypted",
            EncryptionMethod::Ace => "ACE Blowfish",
            EncryptionMethod::Arj(ArjEncryption::Garble) => "ARJ Garble (weak)",
            EncryptionMethod::Arj(ArjEncryption::Gost40) => "ARJ GOST40 (weak)",
            EncryptionMethod::Arj(ArjEncryption::Gost256) => "ARJ GOST-256",
            EncryptionMethod::Arj(ArjEncryption::Unknown) => "ARJ Encrypted",
            EncryptionMethod::Unknown => "Unknown",
        }
    }
}

impl std::fmt::Display for EncryptionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_encrypted() {
        assert!(!EncryptionMethod::None.is_encrypted());
        assert!(EncryptionMethod::Zip(ZipEncryption::Aes256).is_encrypted());
        assert!(EncryptionMethod::Ace.is_encrypted());
        assert!(EncryptionMethod::Unknown.is_encrypted());
    }

    #[test]
    fn test_is_weak() {
        assert!(!EncryptionMethod::None.is_weak());
        assert!(EncryptionMethod::Zip(ZipEncryption::ZipCrypto).is_weak());
        assert!(!EncryptionMethod::Zip(ZipEncryption::Aes256).is_weak());
        assert!(EncryptionMethod::Arj(ArjEncryption::Garble).is_weak());
        assert!(EncryptionMethod::Arj(ArjEncryption::Gost40).is_weak());
        assert!(!EncryptionMethod::Arj(ArjEncryption::Gost256).is_weak());
        assert!(EncryptionMethod::Rar(RarEncryption::Legacy).is_weak());
        assert!(!EncryptionMethod::Rar(RarEncryption::Aes256).is_weak());
    }

    #[test]
    fn test_default() {
        assert_eq!(EncryptionMethod::default(), EncryptionMethod::None);
    }

    #[test]
    fn test_display() {
        assert_eq!(EncryptionMethod::Zip(ZipEncryption::Aes256).to_string(), "ZIP AES-256");
        assert_eq!(EncryptionMethod::Ace.to_string(), "ACE Blowfish");
    }
}
