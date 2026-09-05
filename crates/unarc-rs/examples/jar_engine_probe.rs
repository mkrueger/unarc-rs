//! Probe: locate the JAR method-1 bitstream entry point and validate the
//! Huffman front-end.
//!
//! Strategy: for every plausible (byte start, bit skip, marker) entry point we
//! try to read a complete code-length table (`sum(weights) == 0x100000`), build
//! the decode tree, then decode the following symbol stream. We *score* each
//! candidate by how many of the first decoded literal symbols fall in printable
//! ASCII — the real bitstream start should produce mostly readable text (the
//! LICENSE file), whereas a wrong alignment produces garbage / early match
//! codes. The best-scoring candidates are printed with an ASCII preview.

use unarc_rs::jar::engine::{
    decode_match, read_code_length_table, BitReader, HuffmanTree, TableRead, NSYM,
};

struct Candidate {
    start: usize,
    skip: u32,
    marker: bool,
    skip_count: bool,
    live: usize,
    score: usize,
    decoded: usize,
    preview: String,
}

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/jar/license_m1.j");
    let data = std::fs::read(path).unwrap();

    // The solid stream's true start is unknown, so scan the first 64 payload
    // bytes rather than assuming a fixed local-header size.
    let payload = &data[0x40..];
    println!(
        "payload @file 0x40, first bytes: {:02x?}",
        &payload[..16.min(payload.len())]
    );

    let mut cands: Vec<Candidate> = Vec::new();

    for start in 0..64usize {
        if start >= payload.len() {
            break;
        }
        for skip in 0..8u32 {
            for marker in [false, true] {
                let mut br = BitReader::with_offset(payload, start);
                br.skip_bits(skip);
                let TableRead::Filled(tbl) = read_code_length_table(&mut br, marker) else {
                    continue;
                };
                if !tbl.is_complete() {
                    continue;
                }
                let Some(tree) = HuffmanTree::build(&tbl.weights) else {
                    continue;
                };
                let live = tbl.weights.iter().filter(|&&w| w != 0).count();

                for skip_count in [false, true] {
                    let mut dbr = BitReader::with_offset(payload, start);
                    dbr.skip_bits(skip);
                    let _ = read_code_length_table(&mut dbr, marker);
                    if skip_count {
                        let _ = dbr.read(16);
                    }
                    let (score, decoded, preview) = score_literals(&tree, &mut dbr, 200);
                    cands.push(Candidate {
                        start,
                        skip,
                        marker,
                        skip_count,
                        live,
                        score,
                        decoded,
                        preview,
                    });
                }
            }
        }
    }

    if cands.is_empty() {
        println!("no complete table found in scanned window — entry point still unknown");
        return;
    }

    cands.sort_by_key(|c| std::cmp::Reverse(c.score));
    println!("{} candidate(s); top by ASCII score:", cands.len());
    for c in cands.iter().take(10) {
        println!(
            "  start={:>2} skip={} marker={:<5} count16={:<5} live={:<3} ascii={:>3}/{:<3}  {:?}",
            c.start, c.skip, c.marker, c.skip_count, c.live, c.score, c.decoded, c.preview
        );
    }
}

/// Decode up to `max` symbols; count printable-ASCII literals and build a short
/// preview. Literal bytes (<0x100) render directly; for symbols `>= 0x108` the
/// match-code extra bits are consumed via [`decode_match`] so bit alignment is
/// preserved across matches (rendered as `~`). Stops on decode failure or
/// buffer exhaustion.
fn score_literals(tree: &HuffmanTree, br: &mut BitReader, max: usize) -> (usize, usize, String) {
    let mut printable = 0usize;
    let mut decoded = 0usize;
    let mut preview = String::new();
    for _ in 0..max {
        if br.exhausted() {
            break;
        }
        let Some(sym) = tree.decode_symbol(br) else {
            break;
        };
        decoded += 1;
        if (sym as usize) < 0x100 {
            let b = sym as u8;
            let readable = b == b'\n' || b == b'\r' || b == b'\t' || (0x20..0x7f).contains(&b);
            if readable {
                printable += 1;
            }
            if preview.len() < 78 {
                preview.push(if (0x20..0x7f).contains(&b) {
                    b as char
                } else {
                    '.'
                });
            }
        } else if (sym as usize) < NSYM {
            // Consume the match-code's trailing extra bits to stay aligned.
            let _ = decode_match(sym, br);
            if preview.len() < 78 {
                preview.push('~');
            }
        }
    }
    (printable, decoded, preview)
}
