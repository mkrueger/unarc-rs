# Self-authored ATR fixtures

Made with the `atr` tool from [jhallen/atari-tools](https://github.com/jhallen/atari-tools)
(commit 835d5a6), using only the repository `LICENSE` and `FILE_ID.DIZ` from
this directory. `atr mkfs` writes zero boot sectors, so no Atari DOS code is
included.

- `dos20s.atr`: Atari DOS 2.0S, 720 single-density sectors, with `LICENSE`,
  `FILE_ID.DIZ` and a deleted `GONE.TXT`.
- `dos25.atr`: Atari DOS 2.5, 1040 enhanced-density sectors. Seven copies of
  `LICENSE` (`PAD1.TXT` to `PAD7.TXT`) fill the sectors below 720, so `LICENSE`
  and `FILE_ID.DIZ` use sectors above 719 and are flagged `$03`.
- `dos20d.atr`: Atari DOS 2.0D, 720 double-density (256-byte) sectors with
  128-byte boot sectors, with `LICENSE` and `FILE_ID.DIZ`.

`atr <image> x -a` extracts every file byte for byte as the reader does.

## Reproduction

Run from the repository root with `atr` on PATH; `atr` names files in lower case
for `rm`.

```sh
D=crates/unarc-rs/tests/atr
atr dos20s.atr mkfs dos2.0s
atr dos20s.atr put LICENSE LICENSE
atr dos20s.atr put $D/FILE_ID.DIZ FILE_ID.DIZ
atr dos20s.atr put $D/FILE_ID.DIZ GONE.TXT
atr dos20s.atr rm gone.txt

atr dos25.atr mkfs dos2.5
for i in 1 2 3 4 5 6 7; do atr dos25.atr put LICENSE PAD$i.TXT; done
atr dos25.atr put LICENSE LICENSE
atr dos25.atr put $D/FILE_ID.DIZ FILE_ID.DIZ

atr dos20d.atr mkfs dos2.0d
atr dos20d.atr put LICENSE LICENSE
atr dos20d.atr put $D/FILE_ID.DIZ FILE_ID.DIZ
```

MyDOS and SpartaDOS disks, other boot sector layouts and corrupt images are
built by `tests/common/atr.rs`.
