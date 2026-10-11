//! Unsupported, corrupt and hostile Amiga LZX archives
use std::io::Cursor;

use unarc_rs::error::ArchiveError;
use unarc_rs::lzx::{LzxArchive, LzxEntry, PackMode};
use unarc_rs::unified::{ArchiveFormat, ArchiveOptions, UnifiedArchive};

const LICENSE: &[u8] = include_bytes!("../../../LICENSE");
const SINGLE: &[u8] = include_bytes!("lzx/license.lzx");
const MULTI: &[u8] = include_bytes!("lzx/multi.lzx");

// license.lzx: info header, one 31-byte entry header named "LICENSE", 4014 packed bytes
const HEADER: usize = 10;
const DATA: usize = HEADER + 31 + 7;
// multi.lzx: README.TXT (stored, with comment) and the header of docs/binary.dat, the last of the merged group
const README_HEADER: usize = 10;
const README_DATA: usize = README_HEADER + 31 + 10 + 27;
const BINARY_HEADER: usize = 288;

/// Recomputes the header CRC of the entry header at `offset`
fn fix_header_crc(data: &mut [u8], offset: usize) {
    let end = offset + 31 + usize::from(data[offset + 30]) + usize::from(data[offset + 14]);
    data[offset + 26..offset + 30].fill(0);
    let crc = crc32fast::hash(&data[offset..end]);
    data[offset + 26..offset + 30].copy_from_slice(&crc.to_le_bytes());
}

fn patched(archive: &[u8], header: usize, field: usize, bytes: &[u8]) -> Vec<u8> {
    let mut data = archive.to_vec();
    data[header + field..header + field + bytes.len()].copy_from_slice(bytes);
    fix_header_crc(&mut data, header);
    data
}

fn entries(archive: &mut LzxArchive<Cursor<Vec<u8>>>) -> (Vec<LzxEntry>, Option<ArchiveError>) {
    let mut entries = Vec::new();
    loop {
        match archive.get_next_entry() {
            Ok(Some(entry)) => entries.push(entry),
            Ok(None) => return (entries, None),
            Err(error) => return (entries, Some(error)),
        }
    }
}

fn read_first(data: Vec<u8>) -> unarc_rs::Result<Vec<u8>> {
    let mut archive = LzxArchive::new(Cursor::new(data))?;
    let entry = archive.get_next_entry()?.unwrap();
    archive.read(&entry)
}

#[test]
fn not_an_lzx_archive() {
    for data in [&b""[..], b"LZ", b"LZY\0\0\0\0\0\0\0", b"LZX\0"] {
        assert!(matches!(LzxArchive::new(Cursor::new(data.to_vec())), Err(ArchiveError::InvalidHeader { .. })));
    }
    // An info header alone is an empty archive.
    let mut archive = LzxArchive::new(Cursor::new(SINGLE[..HEADER].to_vec())).unwrap();
    assert!(archive.get_next_entry().unwrap().is_none());
}

#[test]
fn header_crc_mismatch() {
    let mut data = SINGLE.to_vec();
    data[HEADER + 2] ^= 1;
    assert!(matches!(LzxArchive::new(Cursor::new(data)), Err(ArchiveError::CorruptedEntry { .. })));

    // A damaged later header still lists the entries before it.
    let mut data = MULTI.to_vec();
    data[BINARY_HEADER + 40] ^= 1;
    let mut archive = LzxArchive::new(Cursor::new(data)).unwrap();
    let (list, error) = entries(&mut archive);
    assert_eq!(list.len(), 5);
    assert!(matches!(error, Some(ArchiveError::CorruptedEntry { reason, .. }) if reason == "header CRC mismatch"));
    assert_eq!(archive.read(&list[0]).unwrap(), b"Self-authored LZX test fixture for unarc-rs.\n");
    // The merged group never got its data.
    assert!(matches!(archive.read(&list[2]), Err(ArchiveError::CorruptedEntry { .. })));
    assert!(archive.read(&list[4]).unwrap().is_empty());
}

#[test]
fn data_crc_mismatch() {
    let mut data = MULTI.to_vec();
    data[README_DATA] ^= 0x20;
    assert!(matches!(read_first(data), Err(ArchiveError::CrcMismatch { .. })));

    let data = patched(SINGLE, HEADER, 22, &0x1234_5678u32.to_le_bytes());
    assert!(matches!(
        read_first(data),
        Err(ArchiveError::CrcMismatch {
            expected: 0x1234_5678,
            actual: 0x7B5D_04BC,
            ..
        })
    ));
}

#[test]
fn corrupt_compressed_data_never_panics() {
    // Every byte of the block header and trees, then a sample of the token data
    for position in (DATA..SINGLE.len()).filter(|p| p - DATA < 400 || p % 13 == 0) {
        for mask in [0x01, 0xFF] {
            let mut data = SINGLE.to_vec();
            data[position] ^= mask;
            match read_first(data) {
                Ok(output) => assert_eq!(output, LICENSE, "byte {position} ^ {mask:#x}"),
                Err(ArchiveError::CrcMismatch { .. } | ArchiveError::CorruptedEntry { .. }) => {}
                Err(error) => panic!("byte {position} ^ {mask:#x}: {error}"),
            }
        }
    }
}

#[test]
fn truncated_archives_never_panic() {
    for (archive, step) in [(SINGLE, 1), (MULTI, 61)] {
        for cut in 0..archive.len() {
            if cut % step != 0 && cut > 400 {
                continue;
            }
            let Ok(mut lzx) = LzxArchive::new(Cursor::new(archive[..cut].to_vec())) else {
                continue;
            };
            let (list, _) = entries(&mut lzx);
            for entry in list {
                let _ = lzx.read(&entry);
            }
        }
    }
    // Cut inside the packed data: listed, but not readable.
    let mut archive = LzxArchive::new(Cursor::new(SINGLE[..SINGLE.len() - 1].to_vec())).unwrap();
    let (list, error) = entries(&mut archive);
    assert_eq!(list.len(), 1);
    assert!(matches!(error, Some(ArchiveError::CorruptedEntry { reason, .. }) if reason == "entry data extends past the end of the archive"));
    assert!(matches!(archive.read(&list[0]), Err(ArchiveError::CorruptedEntry { reason, .. }) if reason == "the archive ends before the entry's data"));
}

#[test]
fn trailing_bytes() {
    // Too short for a header (e.g. XMODEM padding): ignored
    let mut data = SINGLE.to_vec();
    data.extend_from_slice(&[0x1A; 30]);
    let mut archive = LzxArchive::new(Cursor::new(data)).unwrap();
    let (list, error) = entries(&mut archive);
    assert_eq!(list.len(), 1);
    assert!(error.is_none());
    assert_eq!(archive.read(&list[0]).unwrap(), LICENSE);

    // Long enough for a header, but not one
    let mut data = SINGLE.to_vec();
    data.extend_from_slice(&[0x1A; 128]);
    let mut archive = LzxArchive::new(Cursor::new(data)).unwrap();
    let (list, error) = entries(&mut archive);
    assert_eq!(list.len(), 1);
    assert!(error.is_some());
    assert_eq!(archive.read(&list[0]).unwrap(), LICENSE);
}

#[test]
fn unknown_pack_mode() {
    let data = patched(SINGLE, HEADER, 11, &[1]);
    let mut archive = UnifiedArchive::open_with_format(Cursor::new(data), ArchiveFormat::Lzx).unwrap();
    let entry = archive.next_entry().unwrap().unwrap();
    assert_eq!(entry.compression_method(), "Unknown(1)");
    assert!(matches!(
        archive.read(&entry),
        Err(ArchiveError::UnsupportedMethod { method, .. }) if method == "pack mode 1"
    ));

    let mut archive = LzxArchive::new(Cursor::new(patched(SINGLE, HEADER, 11, &[3]))).unwrap();
    assert_eq!(archive.get_next_entry().unwrap().unwrap().pack_mode, PackMode::Unknown(3));
}

#[test]
fn stored_size_larger_than_its_data() {
    // Original size 46 with 45 stored bytes
    let data = patched(MULTI, README_HEADER, 2, &46u32.to_le_bytes());
    assert!(matches!(read_first(data), Err(ArchiveError::CorruptedEntry { reason, .. }) if reason == "stored data is shorter than the entry"));
}

#[test]
fn size_limits() {
    let options = ArchiveOptions::new().with_max_entry_size(Some(11356));
    let mut archive = UnifiedArchive::open_with_format_and_options(Cursor::new(SINGLE), ArchiveFormat::Lzx, options).unwrap();
    let entry = archive.next_entry().unwrap().unwrap();
    assert!(matches!(archive.read(&entry), Err(ArchiveError::SizeLimitExceeded { limit: 11356, .. })));

    // A forged size cannot make the decoder produce more than its input allows.
    let data = patched(SINGLE, HEADER, 2, &u32::MAX.to_le_bytes());
    let options = ArchiveOptions::new().with_max_entry_size(None);
    let mut archive = UnifiedArchive::open_with_format_and_options(Cursor::new(data), ArchiveFormat::Lzx, options).unwrap();
    let entry = archive.next_entry().unwrap().unwrap();
    assert!(matches!(archive.read(&entry), Err(ArchiveError::CorruptedEntry { .. })));
}

#[test]
fn entries_of_another_archive_are_rejected() {
    let mut multi = LzxArchive::new(Cursor::new(MULTI.to_vec())).unwrap();
    let (list, _) = entries(&mut multi);
    let mut single = LzxArchive::new(Cursor::new(SINGLE.to_vec())).unwrap();
    assert!(matches!(single.read(&list[5]), Err(ArchiveError::IndexMismatch(_))));
    // Same group index, but beyond the end of the group
    let (other, _) = entries(&mut single);
    let mut multi = LzxArchive::new(Cursor::new(MULTI.to_vec())).unwrap();
    assert!(matches!(multi.read(&other[0]), Err(ArchiveError::IndexMismatch(_))));
}

/// Writes bits least significant first into big-endian 16-bit words, like the LZX compressor
#[derive(Default)]
struct BitWriter {
    out: Vec<u8>,
    acc: u32,
    count: u32,
}

impl BitWriter {
    fn bits(&mut self, value: u32, n: u32) {
        self.acc |= value << self.count;
        self.count += n;
        while self.count >= 16 {
            self.out.extend_from_slice(&(self.acc as u16).to_be_bytes());
            self.acc >>= 16;
            self.count -= 16;
        }
    }

    fn finish(mut self) -> Vec<u8> {
        if self.count > 0 {
            self.bits(0, 16 - self.count);
        }
        self.out
    }
}

/// An LZX stream of `len` zero bytes: one literal, then 258-byte matches repeating offset 1,
/// 7 bits each. Literal 0 and main symbol 736 (length slot 15, position slot 0) have 1-bit codes.
fn zeros_stream(len: u32) -> Vec<u8> {
    let mut w = BitWriter::default();
    w.bits(2, 3);
    for shift in [16, 8, 0] {
        w.bits(len >> shift & 0xFF, 8);
    }
    // Each section's pretree: symbols 16 (new length 1) and 18 (zero run) with codes 0 and 1
    let pretree = |w: &mut BitWriter| (0..20).for_each(|symbol| w.bits(u32::from(symbol == 16 || symbol == 18), 4));
    pretree(&mut w);
    w.bits(0, 1);
    for _ in 0..5 {
        w.bits(1, 1);
        w.bits(31, 5); // 51 zeros
    }
    pretree(&mut w);
    for run in [63, 63, 63, 63, 63, 51] {
        w.bits(1, 1);
        w.bits(run, 6); // run + 19 zeros: 480 in all
    }
    w.bits(0, 1);
    w.bits(1, 1);
    w.bits(12, 6); // the last 31 zeros
    w.bits(0, 1);
    let matches = (len - 1) / 258;
    for _ in 0..matches {
        w.bits(1, 1);
        w.bits(63, 6);
    }
    for _ in 0..(len - 1) % 258 {
        w.bits(0, 1);
    }
    w.finish()
}

/// A merged group of `files` files of `size` zero bytes each; the stream may be cut to `packed` bytes.
fn zeros_group(files: u32, size: u32, packed: Option<usize>) -> Vec<u8> {
    let mut stream = zeros_stream(files * size);
    stream.truncate(packed.unwrap_or(stream.len()));
    let crc = crc32fast::hash(&vec![0; size as usize]);
    let mut archive = b"LZX\0\x0c\0\x0a\x04\0\0".to_vec();
    for index in 0..files {
        let last = index == files - 1;
        let name = format!("zero{index:02}");
        let mut header = [0u8; 31];
        header[0] = 0x0F;
        header[2..6].copy_from_slice(&size.to_le_bytes());
        header[6..10].copy_from_slice(&(if last { stream.len() as u32 } else { 0 }).to_le_bytes());
        header[10..16].copy_from_slice(&[10, 2, 1, 0, 0, 10]);
        header[22..26].copy_from_slice(&crc.to_le_bytes());
        header[30] = name.len() as u8;
        let offset = archive.len();
        archive.extend_from_slice(&header);
        archive.extend_from_slice(name.as_bytes());
        fix_header_crc(&mut archive, offset);
        if last {
            archive.extend_from_slice(&stream);
        }
    }
    archive
}

fn unified(data: Vec<u8>, options: ArchiveOptions) -> (UnifiedArchive<Cursor<Vec<u8>>>, Vec<unarc_rs::unified::ArchiveEntry>) {
    let mut archive = UnifiedArchive::open_with_format_and_options(Cursor::new(data), ArchiveFormat::Lzx, options).unwrap();
    let mut list = Vec::new();
    while let Some(entry) = archive.next_entry().unwrap() {
        list.push(entry);
    }
    (archive, list)
}

#[test]
fn skipped_merged_data_counts_toward_size_limits() {
    const FILES: u32 = 40;
    const SIZE: u32 = 256 * 1024;
    let data = zeros_group(FILES, SIZE, None);
    assert!(data.len() < 40_000);
    let group = u64::from(FILES * SIZE);

    // In order, the group is decoded once: exactly the total limit.
    let (mut archive, list) = unified(data.clone(), ArchiveOptions::new().with_max_total_size(Some(group)));
    for entry in &list {
        assert_eq!(archive.read(entry).unwrap().len(), SIZE as usize);
    }

    // Backwards, the first read decodes the whole group and leaves nothing for the next one,
    // which fails before decoding anything.
    let (mut archive, list) = unified(data.clone(), ArchiveOptions::new().with_max_total_size(Some(group)));
    assert_eq!(archive.read(&list[list.len() - 1]).unwrap().len(), SIZE as usize);
    for entry in list.iter().rev().skip(1) {
        assert!(matches!(archive.read(entry), Err(ArchiveError::SizeLimitExceeded { .. })));
    }

    // The entry limit bounds the data a single read decodes, including the skipped files.
    let (mut archive, list) = unified(data, ArchiveOptions::new().with_max_entry_size(Some(u64::from(2 * SIZE))));
    assert!(matches!(
        archive.read(&list[2]),
        Err(ArchiveError::SizeLimitExceeded { limit, .. }) if limit == u64::from(2 * SIZE)
    ));
    assert_eq!(archive.read(&list[1]).unwrap().len(), SIZE as usize);
    // The cursor stopped after file 1, so file 2 needs no skipping now.
    assert_eq!(archive.read(&list[2]).unwrap().len(), SIZE as usize);
    assert!(matches!(archive.read(&list[0]), Ok(data) if data.len() == SIZE as usize));
    assert!(matches!(archive.read(&list[3]), Err(ArchiveError::SizeLimitExceeded { .. })));
}

#[test]
fn failed_reads_leave_later_reads_of_the_group_working() {
    // A wrong data CRC on docs/words.txt (the second file of the merged group)
    const WORDS_HEADER: usize = 202;
    let data = patched(MULTI, WORDS_HEADER, 22, &0u32.to_le_bytes());
    let (mut archive, list) = unified(data, ArchiveOptions::new());
    assert_eq!(archive.read(&list[2]).unwrap(), LICENSE);
    assert!(matches!(archive.read(&list[3]), Err(ArchiveError::CrcMismatch { .. })));
    assert!(archive.read(&list[4]).unwrap().is_empty());
    assert_eq!(archive.read(&list[5]).unwrap().len(), 40960);

    // A size-limit failure in the middle of the group
    let (mut archive, list) = unified(MULTI.to_vec(), ArchiveOptions::new());
    assert_eq!(archive.read(&list[2]).unwrap(), LICENSE);
    let small = ArchiveOptions::new().with_max_entry_size(Some(1000));
    assert!(matches!(
        archive.read_with_options(&list[3], &small),
        Err(ArchiveError::SizeLimitExceeded { .. })
    ));
    assert_eq!(archive.read(&list[5]).unwrap().len(), 40960);

    // A stream that ends early fails its later files; reading starts over for the earlier ones.
    // The stream holds about 146 bytes; 100 cover the first three files.
    let data = zeros_group(8, 4096, Some(100));
    let (mut archive, list) = unified(data, ArchiveOptions::new());
    assert!(matches!(archive.read(&list[7]), Err(ArchiveError::CorruptedEntry { .. })));
    assert_eq!(archive.read(&list[0]).unwrap(), vec![0; 4096]);
    assert_eq!(archive.read(&list[1]).unwrap(), vec![0; 4096]);
    assert!(matches!(archive.read(&list[7]), Err(ArchiveError::CorruptedEntry { .. })));
}

#[test]
fn failed_decodes_charge_buffered_output_to_total_limit() {
    const SIZE: u32 = 4096;
    // The shorter stream fails before delivering a batch; the longer one delivers
    // a full batch before failing with more output still buffered.
    // These cuts leave the initial literal and 75 or 190 complete 258-byte matches.
    for (files, packed, decoded) in [(8, 100, 1 + 75 * 258), (16, 200, 1 + 190 * 258)] {
        for prime_cursor in [false, true] {
            let data = zeros_group(files, SIZE, Some(packed));
            let options = ArchiveOptions::new().with_max_total_size(Some(u64::from(files * SIZE)));
            let (mut archive, list) = unified(data, options);
            if prime_cursor {
                assert_eq!(archive.read(&list[0]).unwrap(), vec![0; SIZE as usize]);
            }
            assert!(matches!(archive.read(list.last().unwrap()), Err(ArchiveError::CorruptedEntry { .. })));

            let remaining = files * SIZE - decoded;
            for _ in 0..remaining / SIZE {
                assert_eq!(archive.read(&list[0]).unwrap(), vec![0; SIZE as usize]);
            }
            assert!(
                matches!(archive.read(&list[0]), Err(ArchiveError::SizeLimitExceeded { limit, .. }) if limit == u64::from(remaining % SIZE)),
                "failed output was not exactly charged: files={files}, prime_cursor={prime_cursor}"
            );
        }
    }
}

#[test]
fn detection_needs_a_valid_first_header() {
    assert_eq!(ArchiveFormat::detect_from_bytes(SINGLE), Some(ArchiveFormat::Lzx));
    assert_eq!(ArchiveFormat::detect_from_bytes(MULTI), Some(ArchiveFormat::Lzx));
    for data in [
        &b"LZX"[..],
        &SINGLE[..HEADER],
        &SINGLE[..HEADER + 31],
        &SINGLE[..HEADER + 37],
        b"LZX is a compression format used on the Amiga and in CAB files.",
    ] {
        assert_eq!(ArchiveFormat::detect_from_bytes(data), None);
    }
    let mut data = SINGLE.to_vec();
    data[HEADER + 2] ^= 1;
    assert_eq!(ArchiveFormat::detect_from_bytes(&data), None);
    // A name and comment that extend past the 512 bytes read for detection cannot be checked.
    let mut long = SINGLE[..HEADER + 31].to_vec();
    long[HEADER + 14] = 255;
    long[HEADER + 30] = 255;
    long.resize(512, b'x');
    assert_eq!(ArchiveFormat::detect_from_bytes(&long), Some(ArchiveFormat::Lzx));
    assert_eq!(ArchiveFormat::detect_from_bytes(&long[..511]), None);
    // The extension still selects the format.
    assert_eq!(
        ArchiveFormat::detect(&mut Cursor::new(&data), Some(std::path::Path::new("x.lzx"))).unwrap(),
        Some(ArchiveFormat::Lzx)
    );
}
