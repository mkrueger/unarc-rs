//! Benchmarks for the native (pure Rust, in-crate) decompressors.
//!
//! Run with `cargo bench -p unarc-rs`, or filter e.g. `cargo bench -p unarc-rs -- uc2`.

use std::hint::black_box;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use unarc_rs::unified::ArchiveFormat;

/// Test fixtures (relative to `tests/`) grouped by format
const FIXTURES: &[(&str, &[&str])] = &[
    ("uc2", &["uc2/fast.uc2", "uc2/normal.uc2", "uc2/tight.uc2"]),
    ("ha", &["ha/asc.ha", "ha/hsc.ha"]),
    ("jar", &["jar/multi_m1.j", "jar/multi_m2.j", "jar/multi_m3.j", "jar/multi_m4.j"]),
    ("arj", &["arj/method1.arj", "arj/method2.arj", "arj/method3.arj", "arj/method4.arj"]),
    ("ace", &["ace/license1.ace"]),
    (
        "arc",
        &[
            "arc/crunch.arc",
            "arc/crunch2.arc",
            "arc/squashed.arc",
            "pak/license_crunched.pak",
            "pak/license_crushed.pak",
            "pak/license.pak",
        ],
    ),
    ("zoo", &["zoo/default.zoo", "zoo/high_per.zoo"]),
    ("sqz", &["sqz/license_m1.sqz", "sqz/license_m2.sqz", "sqz/license_m3.sqz", "sqz/license_m4.sqz"]),
    ("hyp", &["hyp/license.hyp"]),
    ("sq", &["qqq/license.sq", "qqq/license.sq2"]),
    ("z", &["Z/LICENSE.Z"]),
    ("ice", &["ice/license_lha.ice"]),
    ("pack_ice", &["pi9/KOERPER.PI9", "pi9/MAGGIE.PI9"]),
];

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join(name)
}

/// Decompresses every file entry of an in-memory archive and returns the total number of bytes produced
fn extract_all(format: ArchiveFormat, data: &[u8]) -> u64 {
    let mut archive = format.open(Cursor::new(data)).expect("open archive");
    let mut total = 0;
    while let Some(entry) = archive.next_entry().expect("read entry header") {
        if entry.is_directory() {
            continue;
        }
        total += archive.read(&entry).expect("decompress entry").len() as u64;
    }
    total
}

fn bench_decoders(c: &mut Criterion) {
    for (group_name, files) in FIXTURES {
        let mut group = c.benchmark_group(*group_name);
        for file in *files {
            let path = fixture_path(file);
            let format = ArchiveFormat::from_path(&path).unwrap_or_else(|| panic!("unknown format for {file}"));
            let data = std::fs::read(&path).unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
            let unpacked = extract_all(format, &data);

            group.throughput(Throughput::Bytes(unpacked));
            group.bench_function(file.rsplit('/').next().unwrap_or(file), |b| b.iter(|| extract_all(format, black_box(&data))));
        }
        group.finish();
    }
}

criterion_group!(benches, bench_decoders);
criterion_main!(benches);
