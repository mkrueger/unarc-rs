//! Unsupported, corrupt and hostile Microsoft Cabinet archives
use std::io::{Cursor, Write};

use unarc_rs::cab::CabArchive;
use unarc_rs::error::ArchiveError;
use unarc_rs::unified::{ArchiveFormat, ArchiveOptions, UnifiedArchive};

const LICENSE: &[u8] = include_bytes!("../../../LICENSE");
const STORED: &[u8] = include_bytes!("cab/license_none.cab");
const MIB: usize = 1024 * 1024;

// license_none.cab layout: CFHEADER (36 bytes), one CFFOLDER, one CFFILE, one CFDATA.
const FOLDER_TYPE: usize = 36 + 6;
const FILE_SIZE: usize = 0x2C;
const FILE_FOLDER: usize = 0x2C + 8;
const DATA: usize = 0x44;

fn patched(offset: usize, bytes: &[u8]) -> Vec<u8> {
    let mut data = STORED.to_vec();
    data[offset..offset + bytes.len()].copy_from_slice(bytes);
    data
}

fn open(data: Vec<u8>, options: ArchiveOptions) -> UnifiedArchive<Cursor<Vec<u8>>> {
    UnifiedArchive::open_with_format_and_options(Cursor::new(data), ArchiveFormat::Cab, options).unwrap()
}

fn read_first(data: Vec<u8>, options: ArchiveOptions) -> unarc_rs::Result<Vec<u8>> {
    let mut archive = open(data, options);
    let entry = archive.next_entry().unwrap().unwrap();
    archive.read(&entry)
}

/// Builds a single-folder cabinet: `blocks` are (uncompressed size, CFDATA payload) pairs.
fn cabinet(type_compress: u16, files: &[(&str, u32, u32)], blocks: &[(u16, Vec<u8>)]) -> Vec<u8> {
    let files_offset = 36 + 8;
    let file_table: Vec<u8> = files
        .iter()
        .flat_map(|(name, size, offset)| {
            let mut entry = Vec::new();
            entry.extend_from_slice(&size.to_le_bytes());
            entry.extend_from_slice(&offset.to_le_bytes());
            entry.extend_from_slice(&[0, 0, 0x49, 0x5D, 0, 0, 0x20, 0]);
            entry.extend_from_slice(name.as_bytes());
            entry.push(0);
            entry
        })
        .collect();
    let data_offset = files_offset + file_table.len();
    let mut data = Vec::new();
    for (size, payload) in blocks {
        data.extend_from_slice(&0u32.to_le_bytes()); // no checksum
        data.extend_from_slice(&u16::try_from(payload.len()).unwrap().to_le_bytes());
        data.extend_from_slice(&size.to_le_bytes());
        data.extend_from_slice(payload);
    }
    let mut cab = Vec::new();
    cab.extend_from_slice(b"MSCF");
    cab.extend_from_slice(&0u32.to_le_bytes());
    cab.extend_from_slice(&u32::try_from(data_offset + data.len()).unwrap().to_le_bytes());
    cab.extend_from_slice(&0u32.to_le_bytes());
    cab.extend_from_slice(&u32::try_from(files_offset).unwrap().to_le_bytes());
    cab.extend_from_slice(&0u32.to_le_bytes());
    cab.extend_from_slice(&[3, 1]);
    cab.extend_from_slice(&1u16.to_le_bytes());
    cab.extend_from_slice(&u16::try_from(files.len()).unwrap().to_le_bytes());
    cab.extend_from_slice(&[0; 6]); // flags, set ID, cabinet index
    cab.extend_from_slice(&u32::try_from(data_offset).unwrap().to_le_bytes());
    cab.extend_from_slice(&u16::try_from(blocks.len()).unwrap().to_le_bytes());
    cab.extend_from_slice(&type_compress.to_le_bytes());
    cab.extend_from_slice(&file_table);
    cab.extend_from_slice(&data);
    cab
}

/// An MSZIP block (independent of earlier blocks) holding `size` zero bytes
fn zero_mszip_block(size: usize) -> Vec<u8> {
    let mut encoder = flate2::write::DeflateEncoder::new(b"CK".to_vec(), flate2::Compression::best());
    encoder.write_all(&vec![0u8; size]).unwrap();
    encoder.finish().unwrap()
}

#[test]
fn quantum_is_unsupported() {
    let data = patched(FOLDER_TYPE, &0x1472u16.to_le_bytes());
    let mut archive = open(data, ArchiveOptions::new());
    let entry = archive.next_entry().unwrap().unwrap();
    assert_eq!(entry.compression_method(), "Quantum:20");
    match archive.read(&entry) {
        Err(ArchiveError::UnsupportedMethod { format, method }) => assert_eq!((format.as_str(), method.as_str()), ("CAB", "Quantum:20")),
        other => panic!("expected unsupported method, got {other:?}"),
    }
}

#[test]
fn unknown_method_and_invalid_lzx_window_are_unsupported() {
    for (type_compress, name) in [(0x000F, "Unknown(0x000f)"), (0x1903, "LZX:25"), (0x0E03, "LZX:14")] {
        let result = read_first(patched(FOLDER_TYPE, &u16::to_le_bytes(type_compress)), ArchiveOptions::new());
        assert!(
            matches!(&result, Err(ArchiveError::UnsupportedMethod { method, .. }) if method == name),
            "{type_compress:#x}: {result:?}"
        );
    }
}

#[test]
fn files_continued_across_cabinets_are_rejected() {
    for folder in [0xFFFDu16, 0xFFFE, 0xFFFF] {
        let mut archive = open(patched(FILE_FOLDER, &folder.to_le_bytes()), ArchiveOptions::new());
        let entry = archive.next_entry().unwrap().unwrap();
        assert_eq!(entry.name(), "LICENSE");
        match archive.read(&entry) {
            Err(ArchiveError::UnsupportedFormat(message)) => assert!(message.contains("multi-cabinet"), "{message}"),
            other => panic!("expected unsupported format, got {other:?}"),
        }
    }
}

#[test]
fn data_block_continued_in_next_cabinet_is_rejected() {
    // cbUncomp = 0 marks a block whose remainder is stored in the next cabinet.
    let result = read_first(patched(DATA + 6, &[0, 0]), ArchiveOptions::new());
    assert!(
        matches!(&result, Err(ArchiveError::UnsupportedFormat(m)) if m.contains("multi-cabinet")),
        "{result:?}"
    );
}

#[test]
fn folder_index_out_of_range_fails_to_open() {
    let result = CabArchive::new(Cursor::new(patched(FILE_FOLDER, &[1, 0])));
    assert!(matches!(result, Err(ArchiveError::CorruptedEntry { .. })));
}

#[test]
fn checksum_mismatch_is_detected() {
    let mut data = STORED.to_vec();
    data[DATA + 8 + 100] ^= 0x20;
    let result = read_first(data, ArchiveOptions::new());
    assert!(
        matches!(&result, Err(ArchiveError::CorruptedEntry { reason, .. }) if reason.contains("checksum")),
        "{result:?}"
    );
}

#[test]
fn bad_signature_and_truncation() {
    assert!(CabArchive::new(Cursor::new(b"MSCX".to_vec())).is_err());
    for len in [0, 4, 35, 36, 50, FILE_SIZE + 10] {
        assert!(CabArchive::new(Cursor::new(STORED[..len].to_vec())).is_err(), "{len}");
    }
    // Headers intact, data block cut short.
    let result = read_first(STORED[..STORED.len() - 1].to_vec(), ArchiveOptions::new());
    assert!(matches!(result, Err(ArchiveError::Io(_))), "{result:?}");
}

#[test]
fn lying_sizes_are_bounded() {
    // Larger than the default limit: rejected before decoding.
    let huge = patched(FILE_SIZE, &0x7FFF_0000u32.to_le_bytes());
    assert!(matches!(
        read_first(huge.clone(), ArchiveOptions::new()),
        Err(ArchiveError::SizeLimitExceeded { .. })
    ));
    // Without a limit: one data block cannot hold that much, so nothing is decoded either.
    let result = read_first(huge, ArchiveOptions::new().with_max_entry_size(None));
    assert!(
        matches!(&result, Err(ArchiveError::CorruptedEntry { reason, .. }) if reason.contains("past the end")),
        "{result:?}"
    );
    // Fits the block count but not the actual data.
    let result = read_first(patched(FILE_SIZE, &20_000u32.to_le_bytes()), ArchiveOptions::new());
    assert!(
        matches!(&result, Err(ArchiveError::CorruptedEntry { reason, .. }) if reason.contains("ends before")),
        "{result:?}"
    );
    // Smaller than the data: only the recorded size is returned.
    assert_eq!(
        read_first(patched(FILE_SIZE, &100u32.to_le_bytes()), ArchiveOptions::new()).unwrap(),
        &LICENSE[..100]
    );
}

#[test]
fn decompression_bomb_hits_the_limit() {
    let blocks: Vec<_> = (0..64).map(|_| (0x8000u16, zero_mszip_block(0x8000))).collect();
    let data = cabinet(1, &[("bomb", 2 * MIB as u32, 0)], &blocks);
    assert!(data.len() < 16 * 1024);

    let options = ArchiveOptions::new().with_max_entry_size(Some(MIB as u64));
    match read_first(data.clone(), options) {
        Err(ArchiveError::SizeLimitExceeded { entry, limit }) => assert_eq!((entry.as_str(), limit), ("bomb", MIB as u64)),
        other => panic!("expected size limit error, got {other:?}"),
    }
    let options = ArchiveOptions::new().with_max_total_size(Some(MIB as u64));
    assert!(matches!(read_first(data.clone(), options), Err(ArchiveError::SizeLimitExceeded { .. })));
    assert_eq!(read_first(data, ArchiveOptions::new()).unwrap().len(), 2 * MIB);

    // Two 1 MiB files from one folder: the second exceeds a 1.5 MiB total.
    let data = cabinet(1, &[("a", MIB as u32, 0), ("b", MIB as u32, MIB as u32)], &blocks);
    let mut archive = open(data, ArchiveOptions::new().with_max_total_size(Some(MIB as u64 * 3 / 2)));
    let first = archive.next_entry().unwrap().unwrap();
    assert_eq!(archive.read(&first).unwrap().len(), MIB);
    let second = archive.next_entry().unwrap().unwrap();
    assert!(matches!(archive.read(&second), Err(ArchiveError::SizeLimitExceeded { .. })));
}

#[test]
fn block_decoding_to_more_than_declared_is_corrupt() {
    let data = cabinet(1, &[("x", 100, 0)], &[(100, zero_mszip_block(0x8000))]);
    let result = read_first(data, ArchiveOptions::new());
    assert!(matches!(result, Err(ArchiveError::CorruptedEntry { .. })), "{result:?}");

    let data = cabinet(1, &[("x", 100, 0)], &[(0x8001, zero_mszip_block(0x8001))]);
    let result = read_first(data, ArchiveOptions::new());
    assert!(
        matches!(&result, Err(ArchiveError::CorruptedEntry { reason, .. }) if reason.contains("block size")),
        "{result:?}"
    );
}

#[test]
fn mszip_requires_a_complete_deflate_stream() {
    let non_final = b"CK\x00\x03\x00\xfc\xffabc";
    let terminator = b"\x01\x00\x00\xff\xff";
    for with_history in [false, true] {
        for terminator_len in 0..=terminator.len() {
            let mut payload = non_final.to_vec();
            payload.extend_from_slice(&terminator[..terminator_len]);
            let mut blocks = Vec::new();
            if with_history {
                blocks.push((3, b"CK\x01\x03\x00\xfc\xffabc".to_vec()));
            }
            blocks.push((3, payload));
            let offset = if with_history { 3 } else { 0 };
            let data = cabinet(1, &[("x", 3, offset)], &blocks);
            let mut archive = CabArchive::new(Cursor::new(&data)).unwrap();
            let entry = archive.get_next_entry().unwrap().unwrap();
            let direct = archive.read(&entry);
            let unified = read_first(data, ArchiveOptions::new());
            for result in [direct, unified] {
                if terminator_len == terminator.len() {
                    assert_eq!(result.unwrap(), b"abc", "history={with_history}");
                } else {
                    assert!(
                        matches!(&result, Err(ArchiveError::CorruptedEntry { .. })),
                        "history={with_history}, terminator_len={terminator_len}: {result:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn corrupt_compressed_data_does_not_panic() {
    let data = cabinet(1, &[("x", 100, 0)], &[(100, b"CK\xff\xff\xff\xff".to_vec())]);
    assert!(read_first(data, ArchiveOptions::new()).is_err());
    let data = cabinet(0x1503, &[("x", 100, 0)], &[(100, vec![0xFF; 64])]);
    assert!(read_first(data, ArchiveOptions::new()).is_err());
}

/// Clears every `CFDATA` checksum so that mutated data reaches the decoders
fn strip_checksums(mut data: Vec<u8>) -> Vec<u8> {
    let u16_at = |data: &[u8], pos: usize| usize::from(u16::from_le_bytes([data[pos], data[pos + 1]]));
    for folder in 0..u16_at(&data, 26) {
        let entry = 36 + folder * 8;
        let mut pos = u32::from_le_bytes(data[entry..entry + 4].try_into().unwrap()) as usize;
        for _ in 0..u16_at(&data, entry + 4) {
            data[pos..pos + 4].fill(0);
            pos += 8 + u16_at(&data, pos + 4);
        }
    }
    data
}

/// Random byte mutations of every fixture: errors are fine, panics are not.
#[test]
fn mutated_fixtures_do_not_panic() {
    let fixtures: [&[u8]; 7] = [
        include_bytes!("cab/license_none.cab"),
        include_bytes!("cab/license_mszip.cab"),
        include_bytes!("cab/big_mszip.cab"),
        include_bytes!("cab/license_lzx15.cab"),
        include_bytes!("cab/license_lzx18.cab"),
        include_bytes!("cab/license_lzx21.cab"),
        include_bytes!("cab/multi.cab"),
    ];
    let mut state = 0x2545_F491_4F6C_DD1Du64;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for fixture in fixtures {
        let fixture = strip_checksums(fixture.to_vec());
        for _ in 0..150 {
            let mut data = fixture.clone();
            for _ in 0..=next() % 4 {
                let pos = (next() % data.len() as u64) as usize;
                data[pos] = next() as u8;
            }
            let options = ArchiveOptions::new().with_max_entry_size(Some(MIB as u64));
            let Ok(mut archive) = UnifiedArchive::open_with_format_and_options(Cursor::new(data), ArchiveFormat::Cab, options) else {
                continue;
            };
            while let Ok(Some(entry)) = archive.next_entry() {
                let _ = archive.read(&entry);
            }
        }
    }
}
