# unarc-cli

[![Crates.io](https://img.shields.io/crates/v/unarc-cli.svg)](https://crates.io/crates/unarc-cli)
[![License](https://img.shields.io/crates/l/unarc-cli.svg)](https://github.com/mkrueger/unarc-rs)

A command-line tool for listing and extracting files from various archive formats, with a focus on legacy/retro formats from the BBS era.

## Installation

```bash
cargo install unarc-cli
```

Or build from source:

```bash
git clone https://github.com/mkrueger/unarc-rs
cd unarc-rs
cargo build --release -p unarc-cli
```

The binary will be available at `target/release/unarc`.

## Usage

### List Archive Contents

```bash
unarc list archive.arj
unarc l archive.zip      # short alias
```

Example output:
```
Archive: test.arj (ARJ)

Name                                     Compressed     Original    Ratio Method       Encryption
---------------------------------------------------------------------------------------------------------
readme.txt                                      1234         5678   21.7% Method 4     
secret.doc                                       890         2345   37.9% Method 1     ARJ Garble
---------------------------------------------------------------------------------------------------------
2 file(s), 1 encrypted                          2124         8023   26.5%
```

### Extract Files

```bash
unarc extract archive.arj                    # Extract to current directory
unarc x archive.zip -o ./output              # Extract to specific directory
unarc extract -f archive.rar                 # Overwrite existing files
unarc extract -p secret encrypted.arj        # Decrypt with password
```

### Show Supported Formats

```bash
unarc formats
```

## Command Reference

### `unarc list` (alias: `l`)

List contents of an archive.

```
Usage: unarc list <ARCHIVE>

Arguments:
  <ARCHIVE>  Archive file to list
```

### `unarc extract` (alias: `x`)

Extract files from an archive.

```
Usage: unarc extract [OPTIONS] <ARCHIVE>

Arguments:
  <ARCHIVE>  Archive file to extract

Options:
  -o, --output <OUTPUT>      Output directory [default: .]
  -f, --force                Overwrite existing files
  -p, --password <PASSWORD>  Password for encrypted archives
  -h, --help                 Print help
```

### `unarc formats`

Show all supported archive formats with their extensions and magic bytes.

## Supported Formats

| Format | Extensions | Encryption Support |
|--------|------------|-------------------|
| **7z** | `.7z` | AES-256 |
| **ZIP** | `.zip` | ZipCrypto, AES |
| **RAR** | `.rar` | AES |
| **LHA/LZH** | `.lha`, `.lzh` | - |
| **TAR** | `.tar` | - |
| **ACE** | `.ace` | Blowfish |
| **ARJ** | `.arj` | Garble, GOST40 |
| **ARC/PAK** | `.arc`, `.pak` | - |
| **ZOO** | `.zoo` | - |
| **HA** | `.ha` | - |
| **UC2** | `.uc2` | - |
| **SQ** | `.sq`, `.qqq` | - |
| **SQZ** | `.sqz` | - |
| **HYP** | `.hyp` | - |
| **Z** | `.Z` | - |
| **GZ** | `.gz` | - |
| **BZ2** | `.bz2` | - |
| **Pack-Ice** | `.pi9` | - |
| **TGZ** | `.tgz`, `.tar.gz` | - |
| **TBZ** | `.tbz`, `.tar.bz2` | - |
| **TAR.Z** | `.tar.Z` | - |

## Examples

Extract an encrypted ARJ archive:
```bash
unarc extract -p mypassword encrypted.arj -o ./extracted/
```

List a 7z archive:
```bash
unarc list backup.7z
```

Extract all files from a ZOO archive, overwriting existing:
```bash
unarc x -f old_archive.zoo
```

## Related

- [unarc-rs](https://crates.io/crates/unarc-rs) - The library this CLI is built on

## License

MIT OR Apache-2.0
