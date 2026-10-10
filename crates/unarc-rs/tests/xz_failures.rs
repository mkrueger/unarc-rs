use std::io::{self, Cursor, Read};

use unarc_rs::error::ArchiveError;
use unarc_rs::txz::TxzArchive;
use unarc_rs::unified::{ArchiveFormat, ArchiveOptions, UnifiedArchive};
use unarc_rs::xz::XzArchive;

const MIB: u64 = 1024 * 1024;

const LICENSE_XZ: &[u8] = include_bytes!("xz/LICENSE.xz");

fn read_xz(data: &[u8]) -> unarc_rs::Result<Vec<u8>> {
    XzArchive::new(Cursor::new(data.to_vec()))?.read()
}

fn read_unified(data: &[u8], format: ArchiveFormat, options: ArchiveOptions) -> unarc_rs::Result<Vec<u8>> {
    let mut archive = UnifiedArchive::open_with_format_and_options(Cursor::new(data.to_vec()), format, options)?;
    archive.set_single_file_name("data".to_string());
    let entry = archive.next_entry()?.expect("one entry");
    archive.read(&entry)
}

/// Sets the LZMA2 dictionary size property of the first block and fixes the block header CRC32
fn set_dictionary_property(xz: &mut [u8], property: u8) {
    const BLOCK: usize = 12; // the block header follows the 12 byte stream header
    let header_size = (usize::from(xz[BLOCK]) + 1) * 4;
    assert_eq!(xz[BLOCK + 1], 0, "expected one filter and no size fields");
    // Filter flags: ID 0x21 (LZMA2), one property byte, the dictionary size
    assert_eq!(&xz[BLOCK + 2..BLOCK + 4], &[0x21, 0x01]);
    xz[BLOCK + 4] = property;
    let crc = crc32fast::hash(&xz[BLOCK..BLOCK + header_size - 4]);
    xz[BLOCK + header_size - 4..BLOCK + header_size].copy_from_slice(&crc.to_le_bytes());
}

#[test]
fn invalid_magic() {
    assert!(matches!(
        XzArchive::new(Cursor::new(b"\x1f\x8b\x08\x00\x00\x00".to_vec())),
        Err(ArchiveError::InvalidHeader { .. })
    ));
}

#[test]
fn truncated_stream() {
    assert!(read_xz(&LICENSE_XZ[..LICENSE_XZ.len() / 2]).is_err());
    assert!(read_xz(&LICENSE_XZ[..LICENSE_XZ.len() - 1]).is_err());
}

#[test]
fn corrupted_data() {
    let mut data = LICENSE_XZ.to_vec();
    data[LICENSE_XZ.len() / 2] ^= 0x55;
    assert!(read_xz(&data).is_err());
}

#[test]
fn check_mismatch() {
    // The stream footer's backward size locates the index; the block's CRC64 precedes it
    let mut data = LICENSE_XZ.to_vec();
    let footer = data.len() - 12;
    let index_size = (u32::from_le_bytes(data[footer + 4..footer + 8].try_into().unwrap()) as usize + 1) * 4;
    data[footer - index_size - 1] ^= 0x01;
    let error = read_xz(&data).unwrap_err();
    assert!(error.to_string().contains("checksum"), "{error}");
}

#[test]
fn trailing_garbage() {
    let mut data = LICENSE_XZ.to_vec();
    data.extend_from_slice(b"garbage!");
    assert!(read_xz(&data).is_err());
}

struct ShortReads<R> {
    inner: R,
    chunk_size: usize,
    interrupt: bool,
}

impl<R: Read> Read for ShortReads<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !buf.is_empty() {
            self.interrupt = !self.interrupt;
            if self.interrupt {
                return Err(io::ErrorKind::Interrupted.into());
            }
        }
        let len = buf.len().min(self.chunk_size);
        self.inner.read(&mut buf[..len])
    }
}

#[test]
fn short_and_interrupted_reads() {
    for chunk_size in [1, 2, 3, 7] {
        for data in [LICENSE_XZ, &include_bytes!("xz/multi_stream.xz")[..]] {
            let reader = ShortReads {
                inner: Cursor::new(data),
                chunk_size,
                interrupt: false,
            };
            assert_eq!(XzArchive::new(reader).unwrap().read().unwrap(), include_bytes!("../../../LICENSE"));
        }
        let reader = ShortReads {
            inner: Cursor::new(include_bytes!("txz/license.tar.xz")),
            chunk_size,
            interrupt: false,
        };
        let mut archive = TxzArchive::new(reader).unwrap();
        let entry = archive.get_next_entry().unwrap().unwrap();
        assert_eq!(archive.read(&entry).unwrap(), include_bytes!("../../../LICENSE"));
    }
}

#[test]
fn stream_padding_must_be_a_multiple_of_four() {
    for padding in 0..=8 {
        let mut data = LICENSE_XZ.to_vec();
        data.extend(std::iter::repeat_n(0, padding));
        let result = read_xz(&data);
        if padding % 4 == 0 {
            assert_eq!(result.unwrap(), include_bytes!("../../../LICENSE"));
        } else {
            assert!(result.unwrap_err().to_string().contains("padding"));
        }

        data.extend_from_slice(LICENSE_XZ);
        let result = read_xz(&data);
        if padding % 4 == 0 {
            assert_eq!(result.unwrap(), include_bytes!("../../../LICENSE").repeat(2));
        } else {
            assert!(result.unwrap_err().to_string().contains("padding"));
        }
    }
}

#[test]
fn txz_rejects_invalid_trailing_padding() {
    let mut data = include_bytes!("txz/license.tar.xz").to_vec();
    data.push(0);
    let error = match TxzArchive::new(Cursor::new(&data)) {
        Err(error) => error,
        Ok(_) => panic!("accepted invalid trailing padding"),
    };
    assert!(error.to_string().contains("padding"), "{error}");
}

#[test]
fn bomb_is_stopped_by_entry_limit() {
    // 4 MiB of zeros
    let data = include_bytes!("xz/zeros.xz");

    let options = ArchiveOptions::new().with_max_entry_size(Some(MIB));
    match read_unified(data, ArchiveFormat::Xz, options) {
        Err(ArchiveError::SizeLimitExceeded { entry, limit }) => {
            assert_eq!(entry, "data");
            assert_eq!(limit, MIB);
        }
        other => panic!("expected size limit error, got {other:?}"),
    }

    assert_eq!(read_unified(data, ArchiveFormat::Xz, ArchiveOptions::new()).unwrap().len() as u64, 4 * MIB);
}

#[test]
fn limit_covers_all_concatenated_streams() {
    // The first stream holds 5000 bytes, both together 11357
    let data = include_bytes!("xz/multi_stream.xz");
    let options = ArchiveOptions::new().with_max_entry_size(Some(6000));
    assert!(matches!(
        read_unified(data, ArchiveFormat::Xz, options),
        Err(ArchiveError::SizeLimitExceeded { limit: 6000, .. })
    ));
}

#[test]
fn oversized_dictionary_is_rejected_before_allocation() {
    // Dictionary property 40 announces a 4 GiB dictionary
    let mut data = LICENSE_XZ.to_vec();
    set_dictionary_property(&mut data, 40);

    for options in [ArchiveOptions::new(), ArchiveOptions::new().with_max_entry_size(None)] {
        let error = read_unified(&data, ArchiveFormat::Xz, options).unwrap_err();
        assert!(error.to_string().contains("memory"), "{error}");
    }
    assert!(read_xz(&data).is_err());

    // The dictionary of `xz -9` (64 MiB) is accepted even with a small output limit
    let options = ArchiveOptions::new().with_max_entry_size(Some(MIB));
    assert_eq!(
        read_unified(LICENSE_XZ, ArchiveFormat::Xz, options).unwrap(),
        include_bytes!("../../../LICENSE")
    );
}

#[test]
fn txz_limit_applies_when_opening() {
    let data = include_bytes!("txz/license.tar.xz");

    let options = ArchiveOptions::new().with_max_entry_size(Some(16 * 1024));
    assert!(matches!(
        UnifiedArchive::open_with_format_and_options(Cursor::new(data), ArchiveFormat::Txz, options),
        Err(ArchiveError::SizeLimitExceeded { .. })
    ));
    assert!(TxzArchive::new_with_limit(Cursor::new(data), Some(16 * 1024)).is_err());
    assert!(TxzArchive::new_with_limit(Cursor::new(data), Some(MIB)).is_ok());
}

#[test]
fn txz_truncated() {
    let data = include_bytes!("txz/license.tar.xz");
    assert!(TxzArchive::new(Cursor::new(&data[..data.len() / 2])).is_err());
}
