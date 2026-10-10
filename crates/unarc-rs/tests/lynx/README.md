# Lynx fixtures

Both containers were written by a small script from the self-authored files
described in `tests/d64/README.md`, following the layout cbmconvert 2.1.5
writes (`cbmconvert -L`), including REL side sectors. Padding inside the
containers is `$EA` (file blocks), `$55` (side sectors) and `$00` (directory).

- `basic.lnx`: a self-authored BASIC loader (`10 PRINT"UNLYNX ME":END`), the
  signature `*LYNX XV  BY UNARC TESTS`, a 2-block directory and HELLO, DATA,
  EXACT, LONG and NOTE, with spaces around all numbers.
- `bare.lnx`: no BASIC loader, no spaces around numbers, a 1-block directory
  and a zero-length DEL separator (`----------------`), HELLO, a PRG named
  `Shift` (`$D3` + `HIFT`), the REL file RECS (record length 32, 27 blocks
  including one side sector), LONG and NOTE.

Cross-checks (2026-10-09): every file extracted by unarc-rs matches its source,
cbmconvert 2.1.5 (`cbmconvert -N -l`) for both containers, and VICE 3.7.1
(`c1541 -format x,xx d64 x.d64 -unlynx basic.lnx`, then `-extract`) for
`basic.lnx` (VICE needs the BASIC loader and does not support REL files).
