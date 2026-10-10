# T64 fixtures

Both images were written by a small script from the self-authored HELLO, LONG
and DATA files described in `tests/d64/README.md` (DATA as a SEQ entry whose
start address is its first two bytes, `LI`):

- `test.t64`: signature `C64S tape image file`, 8 directory slots, 3 used,
  correct end addresses, data in directory order.
- `c3c6.t64`: signature `C64 tape image file`, 4 slots, every end address set
  to `$C3C6` as CONV64 did, and the data stored in the order DATA, HELLO, LONG.

Cross-checks (2026-10-09): every file extracted by unarc-rs matches its source
and the files written by VICE 3.7.1 (`c1541 -format x,xx d64 x.d64 -tape <image>`,
then `-extract`). `test.t64` also matches `cbmconvert -N -t` (cbmconvert trusts
the end addresses, so it cannot read `c3c6.t64`).
