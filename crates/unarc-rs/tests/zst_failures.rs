use std::io::Cursor;

use unarc_rs::error::ArchiveError;
use unarc_rs::tzst::TzstArchive;
use unarc_rs::unified::{ArchiveFormat, ArchiveOptions, UnifiedArchive};
use unarc_rs::zst::ZstArchive;

const MIB: u64 = 1024 * 1024;

const LICENSE_ZST: &[u8] = include_bytes!("zst/LICENSE.zst");

fn read_zst(data: &[u8]) -> unarc_rs::Result<Vec<u8>> {
    ZstArchive::new(Cursor::new(data.to_vec()))?.read()
}

fn read_unified(data: &[u8], format: ArchiveFormat, options: ArchiveOptions) -> unarc_rs::Result<Vec<u8>> {
    let mut archive = UnifiedArchive::open_with_format_and_options(Cursor::new(data.to_vec()), format, options)?;
    archive.set_single_file_name("data".to_string());
    let entry = archive.next_entry()?.expect("one entry");
    archive.read(&entry)
}

/// An empty frame whose header announces a window of 2^`window_log` bytes
fn empty_frame(window_log: u8) -> Vec<u8> {
    let mut frame = vec![0x28, 0xB5, 0x2F, 0xFD];
    // Frame header descriptor: no content size, no checksum, no dictionary, not single segment
    frame.push(0x00);
    // Window descriptor: exponent in the upper five bits, no mantissa
    frame.push((window_log - 10) << 3);
    // Last block, raw, zero bytes
    frame.extend_from_slice(&[0x01, 0x00, 0x00]);
    frame
}

#[test]
fn invalid_magic() {
    assert!(matches!(
        ZstArchive::new(Cursor::new(b"\x1f\x8b\x08\x00".to_vec())),
        Err(ArchiveError::InvalidHeader { .. })
    ));
}

#[test]
fn truncated_stream() {
    assert!(read_zst(&LICENSE_ZST[..LICENSE_ZST.len() / 2]).is_err());
    assert!(read_zst(&LICENSE_ZST[..LICENSE_ZST.len() - 1]).is_err());
    // A skippable frame that is shorter than announced
    assert!(read_zst(b"\x50\x2a\x4d\x18\x08\x00\x00\x00skip").is_err());
}

#[test]
fn corrupted_data() {
    let mut data = LICENSE_ZST.to_vec();
    data[LICENSE_ZST.len() / 2] ^= 0x55;
    assert!(read_zst(&data).is_err());
}

#[test]
fn checksum_mismatch() {
    // `zstd` ends each frame with the low 32 bits of the content's XXH64
    let mut data = LICENSE_ZST.to_vec();
    *data.last_mut().unwrap() ^= 0x01;
    let error = read_zst(&data).unwrap_err();
    assert!(error.to_string().contains("checksum"), "{error}");
}

#[test]
fn trailing_garbage() {
    let mut data = LICENSE_ZST.to_vec();
    data.extend_from_slice(b"garbage!");
    assert!(read_zst(&data).is_err());
}

#[test]
fn bomb_is_stopped_by_entry_limit() {
    // 4 MiB of zeros
    let data = include_bytes!("zst/zeros.zst");

    let options = ArchiveOptions::new().with_max_entry_size(Some(MIB));
    match read_unified(data, ArchiveFormat::Zst, options) {
        Err(ArchiveError::SizeLimitExceeded { entry, limit }) => {
            assert_eq!(entry, "data");
            assert_eq!(limit, MIB);
        }
        other => panic!("expected size limit error, got {other:?}"),
    }

    assert_eq!(read_unified(data, ArchiveFormat::Zst, ArchiveOptions::new()).unwrap().len() as u64, 4 * MIB);
}

#[test]
fn limit_covers_all_frames() {
    // The first frame holds 5000 bytes, both together 11357
    let data = include_bytes!("zst/multi_frame.zst");
    let options = ArchiveOptions::new().with_max_entry_size(Some(6000));
    assert!(matches!(
        read_unified(data, ArchiveFormat::Zst, options),
        Err(ArchiveError::SizeLimitExceeded { limit: 6000, .. })
    ));
}

#[test]
fn oversized_window_is_rejected_before_allocation() {
    // 2 TiB, larger than any encoder writes
    for options in [ArchiveOptions::new(), ArchiveOptions::new().with_max_entry_size(None)] {
        let error = read_unified(&empty_frame(41), ArchiveFormat::Zst, options).unwrap_err();
        assert!(error.to_string().contains("window_size is too big"), "{error}");
    }
    assert!(read_zst(&empty_frame(41)).is_err());

    // A 256 MiB window is refused for a 1 MiB output limit, but fine for the default 1 GiB
    let options = ArchiveOptions::new().with_max_entry_size(Some(MIB));
    let error = read_unified(&empty_frame(28), ArchiveFormat::Zst, options).unwrap_err();
    assert!(error.to_string().contains("window_size is too big"), "{error}");
    assert_eq!(read_unified(&empty_frame(28), ArchiveFormat::Zst, ArchiveOptions::new()).unwrap(), b"");
}

#[test]
fn tzst_limit_applies_when_opening() {
    let data = include_bytes!("tzst/license.tar.zst");

    let options = ArchiveOptions::new().with_max_entry_size(Some(16 * 1024));
    assert!(matches!(
        UnifiedArchive::open_with_format_and_options(Cursor::new(data), ArchiveFormat::Tzst, options),
        Err(ArchiveError::SizeLimitExceeded { .. })
    ));
    assert!(TzstArchive::new_with_limit(Cursor::new(data), Some(16 * 1024)).is_err());
    assert!(TzstArchive::new_with_limit(Cursor::new(data), Some(MIB)).is_ok());
}

#[test]
fn tzst_truncated() {
    let data = include_bytes!("tzst/license.tar.zst");
    assert!(TzstArchive::new(Cursor::new(&data[..data.len() / 2])).is_err());
}
