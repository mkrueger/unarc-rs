# CAB regression fixtures

All fixtures are self-authored and contain only the repository's LICENSE
(11357 bytes) and the synthetic files listed below. Every fixture was
extracted with cabextract 1.11 (libmspack), which reported no errors, and its
output was compared byte for byte with the input files.

| Fixture | Created with | Contents |
| --- | --- | --- |
| license_none.cab | gcab 1.6 `-c` | LICENSE, stored |
| license_mszip.cab | gcab 1.6 `-c -z` | LICENSE, MSZIP |
| big_mszip.cab | gcab 1.6 `-c -z` | BIG.TXT, MSZIP, 3 data blocks |
| license_lzx15.cab | script below | LICENSE, LZX window 2^15 |
| license_lzx18.cab | script below | LICENSE, LZX window 2^18 |
| license_lzx21.cab | script below | LICENSE, LZX window 2^21 |
| multi.cab | script below | Three folders, see below |

BIG.TXT is LICENSE repeated 8 times (90856 bytes). gcab stores the file's
modification time in UTC (2026-10-09 18:34:56); the script uses
2026-10-09 12:34:56.

No common tool writes LZX cabinets, so the LZX fixtures and multi.cab were
written by a small Python script that lays out CFHEADER, CFFOLDER, CFFILE and
CFDATA records (with checksums) as described in doc/cab.md. LZX frames were
produced by the [lzxc](https://crates.io/crates/lzxc) 0.1.0 encoder, one 32 KiB
frame per data block; MSZIP blocks by Python's zlib with the previous 32 KiB
as preset dictionary, so blocks refer back into earlier ones.

multi.cab:

| Folder | Method | Entry | Contents | Attributes |
| --- | --- | --- | --- | --- |
| 0 | None | README.TXT | `Self-authored CAB test fixture for unarc-rs.\r\n` | 0x20 |
| 0 | None | EMPTY.TXT | Empty | 0x20 |
| 1 | MSZIP | docs\LICENSE | LICENSE | 0x20 |
| 1 | MSZIP | docs\BIG.TXT | BIG.TXT | 0x21 |
| 2 | LZX 2^16, E8 translation | data\BINARY.DAT | Bytes 0..255, repeated 160 times | 0x20 |
| 2 | LZX 2^16, E8 translation | données\naïve-ü.txt | `UTF-8 name: données/naïve-ü\n` | 0xA0 (UTF-8 name) |
| 2 | LZX 2^16, E8 translation | `caf\xE9.txt` | `Latin-1 name without the UTF-8 flag\n` | 0x20 |
| 2 | LZX 2^16, E8 translation | data\BIG.TXT | BIG.TXT | 0x20 |

Unsupported methods (Quantum), continued files and corrupt data are tested by
patching license_none.cab or building cabinets in cab_failures.rs.
