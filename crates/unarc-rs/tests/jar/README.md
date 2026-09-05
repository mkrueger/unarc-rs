# Legacy JAR regression fixtures

The pre-existing license_m1–m4 archives contain the repository's Apache LICENSE
(11357 bytes). The pre-existing test.j contains A=`a\r\n`, AA=`aa\r\n`,
B=`b\r\n`, BB=`bb\r\n`.

multi_m1–m4 were generated with the supplied JAR32.EXE 1.02 under Wine on
2026-09-05, using `a -r -m1` through `a -r -m4`, from an input directory
containing:

| Entry | Original bytes |
| --- | --- |
| SUB | Directory |
| EMPTY | Empty file |
| HELLO.TXT | `Hello, world!\r\nThis is a test.\r\n`, repeated 20 times |
| LARGE.TXT | Repository LICENSE, repeated 24 times |
| SUB/NESTED.TXT | `Nested file\n`, repeated 80 times |
| BINARY.DAT | Bytes 0..255, repeated 4 times |
| RANDOM.BIN | Python `random.Random(12345).randbytes(40000)`, saved as random.bin |
| DELTA.BIN | 4000 pairs of LE16 values `(2*i, 3*i)` |
| WIDE.TXT | `The quick brown fox jumps over the lazy dog.\r\n` repeated 60 times, UTF-16LE |

The Rust output was compared recursively against this input directory for
each preset. Tests reconstruct all expected bytes except random.bin directly;
no DOS emulator, Wine or Python is required to run them. JAR's stored CRC is
over file records including the trailer, not just file bytes.