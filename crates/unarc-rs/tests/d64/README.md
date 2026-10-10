# D64 fixtures

`test.d64` (35 tracks, 174848 bytes) holds only self-authored files whose
contents are rebuilt by `tests/c64_common/mod.rs`:

| Entry | Type | Bytes | Contents |
| --- | --- | --- | --- |
| HELLO | PRG | 22 | `10 PRINT"HELLO, C64"`, loading at `$0801` |
| DATA | SEQ | 690 | `LINE nnn OF A SEQ FILE\r` for nnn = 000..029 |
| EXACT | PRG | 508 | exactly two blocks, loading at `$C000` |
| LONG | PRG | 1502 | six blocks, loading at `$2000` |
| NOTE | USR | 17 | `USR FILE CONTENT\r` |
| EMPTY | SEQ | 0 | empty file (one block with link `0/1`) |
| Mixed | PRG | 22 | same as HELLO; name typed with shift (`$CD`) |
| RECS | REL | 6400 | 200 records of 32 bytes, one side sector |
| a/b | PRG | 22 | same as HELLO; name containing `/` |
| NINTH | SEQ | 690 | same as DATA; second directory sector |

It was created on 2026-10-09 with VICE 3.7.1 `c1541`:

```sh
c1541 -format "unarc test,ut" d64 test.d64 \
  -write hello.prg hello -write data.seq "data,s" -write exact.prg exact \
  -write long.prg long -write note.usr "note,u" -write /dev/null "empty,s" \
  -write hello.prg "Mixed" -write note.usr "gone,s" -write hello.prg "a/b" \
  -write data.seq "ninth,s" -delete gone
cbmconvert -n -D4 test.d64 recs.l20   # cbmconvert 2.1.5 adds the REL file
```

The REL file reuses the scratched `gone` slot. Every file extracted by
unarc-rs is byte-identical to its source, to `c1541 -extract` (which skips
REL files and returns 254 bytes for the empty file) and to `cbmconvert -N -d`.
Failure tests modify copies of this image in memory.
