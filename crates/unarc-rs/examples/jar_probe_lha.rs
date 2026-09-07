//! Throwaway experiment: try LHA decoders on JAR m1 payload.
use delharc::decode::{Decoder, DecoderAny};

fn crc32(data: &[u8]) -> u32 {
    let mut h = crc32fast::Hasher::new();
    h.update(data);
    h.finalize()
}

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/jar/license_m1.j");
    let data = std::fs::read(path).unwrap();
    let target_crc = u32::from_le_bytes([data[0x18], data[0x19], data[0x1a], data[0x1b]]);
    let usize_field = u32::from_le_bytes([data[0x20], data[0x21], data[0x22], data[0x23]]) as usize;
    println!("target_crc=0x{target_crc:08X} usize_field={usize_field} file_len={}", data.len());

    let methods = [
        delharc::CompressionMethod::Lh1,
        delharc::CompressionMethod::Lh4,
        delharc::CompressionMethod::Lh5,
        delharc::CompressionMethod::Lh6,
        delharc::CompressionMethod::Lh7,
        delharc::CompressionMethod::Lhx,
    ];

    for start in [84usize, 85, 86, 64] {
        for m in methods {
            for outlen in [usize_field, 3262, 3260, 4096, 8192] {
                let payload = &data[start..];
                let mut decoder = DecoderAny::new_from_compression(m, payload);
                let mut out = vec![0u8; outlen];
                if decoder.fill_buffer(&mut out).is_ok() {
                    let c = crc32(&out);
                    let printable = out.iter().take(64).filter(|&&b| (32..127).contains(&b)).count();
                    if c == target_crc {
                        println!("*** MATCH start={start} method={m:?} outlen={outlen} CRC OK ***");
                    } else if printable > 50 {
                        let preview: String = out.iter().take(48).map(|&b| if (32..127).contains(&b) { b as char } else { '.' }).collect();
                        println!("start={start} {m:?} outlen={outlen} crc=0x{c:08X} preview: {preview}");
                    }
                }
            }
        }
    }
}
