# unarc-cli

[![Crates.io](https://img.shields.io/crates/v/unarc-cli.svg)](https://crates.io/crates/unarc-cli)
[![License](https://img.shields.io/crates/l/unarc-cli.svg)](https://github.com/mkrueger/unarc-rs)

A command-line tool for listing and extracting files from various archive formats, with a focus on legacy/retro formats from the BBS era.
No other command line clients required. Everything in an all in one package. List/Extract/Retrieve lost passwords - everything possible with unarc.

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
unarc list --json archive.lzh   # for scripts and other programs
```

Example output:

```text
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
unarc x archive.zip FILE_ID.DIZ docs/read.me # Extract only these entries

# Multi-volume archives (auto-detects all volumes)
unarc extract archive.zip.001 -o ./output    # Split ZIP (.001, .002, ...)
unarc extract archive.7z.001 -o ./output     # Split 7z (.7z.001, .7z.002, ...)
unarc extract archive.z01 -o ./output        # WinZip style (.z01, .z02, .zip)
```

### Show Supported Formats

```bash
unarc formats
```

### Try Passwords (Password Cracking)

Test passwords from a file, directory, or stdin against an encrypted archive:

```bash
# Try passwords from a file
unarc try-passwords archive.arj -f passwords.txt

# Try passwords from all .txt files in a directory (recursive)
unarc tp encrypted.7z -d ~/wordlists/

# Try passwords from stdin (pipe from another tool)
crunch 4 6 abc123 | unarc tp archive.zip --stdin

# Select a specific small file for faster testing
unarc tp large_archive.rar -f passwords.txt -e "small_file.txt"
```

## Command Reference

### `unarc list` (alias: `l`)

List contents of an archive.

```text
Usage: unarc list [OPTIONS] <ARCHIVE>

Arguments:
  <ARCHIVE>  Archive file to list

Options:
      --json  Print the listing as JSON, for scripts and other programs
  -h, --help  Print help
```

The table shortens long names to fit. `--json` prints every name in full, as stored
in the archive, with nothing else on stdout:

```json
{
  "archive": "release.zip",
  "format": "ZIP",
  "entries": [
    {
      "name": "FILE_ID.DIZ",
      "kind": "file",
      "size": 384,
      "compressed_size": 39,
      "method": "Deflated",
      "modified": "1994-03-12T10:22:30",
      "crc": 2330302245,
      "encrypted": false,
      "encryption": null,
      "link_target": null
    }
  ]
}
```

`kind` is one of `file`, `directory`, `symlink`, `hardlink`, `special` or `unknown`.
`modified` is the time as recorded, with no time zone, or `null` when the format
has none. Names are not sanitized: check them before using one as a path. New
fields may be added; existing ones keep their names.

### `unarc extract` (alias: `x`)

Extract files from an archive.

```text
Usage: unarc extract [OPTIONS] <ARCHIVE> [FILES]...

Arguments:
  <ARCHIVE>   Archive file to extract
  [FILES]...  Entries to extract, named exactly as `list` shows them (default: all)

Options:
  -o, --output <OUTPUT>      Output directory [default: .]
  -f, --force                Overwrite existing files
  -p, --password <PASSWORD>  Password for encrypted archives
  -h, --help                 Print help
```

Named entries keep their paths below the output directory. If any named entry is
not in the archive, the others are still extracted and `unarc` exits with an error.
Repeated requested names are treated as a single request. Solid ACE predecessors
are decoded and discarded when necessary, using `--password` for encrypted entries;
they are not written unless requested.

### `unarc formats`

Show all supported archive formats with their extensions and magic bytes.

### `unarc try-passwords` (alias: `tp`)

Test passwords against an encrypted archive using parallel processing.

```text
Usage: unarc try-passwords [OPTIONS] <ARCHIVE>

Arguments:
  <ARCHIVE>  Archive file to test

Options:
  -f, --password-file <PASSWORD_FILE>  File containing passwords (one per line)
  -d, --password-dir <PASSWORD_DIR>    Directory with password files (*.txt, recursive)
      --stdin                          Read passwords from stdin
  -v, --verbose-interval <N>           Show progress every N passwords [default: 1000]
  -e, --entry <ENTRY>                  Specific entry to test against (for faster testing)
  -h, --help                           Print help
```

**Features:**

- **Parallel processing** using all CPU cores (via rayon)
- **Multiple input sources**: single file, directory of wordlists, or stdin
- **Entry selection**: Use `-e` to pick a small file for faster password testing
- **Progress reporting**: See passwords/sec and current progress
- **Smart verification**: Uses CRC32 + size validation to avoid false positives

**Supported formats for password testing:** ARC, ARJ, ACE, ZIP, RAR, 7z

#### Tutorial: Using SecLists for Password Testing

[SecLists](https://github.com/danielmiessler/SecLists) is a collection of common wordlists for security testing, including password lists.

```bash
# 1. Clone SecLists (or download specific files)
git clone --depth 1 https://github.com/danielmiessler/SecLists.git ~/SecLists

# 2. Use a single password list
unarc tp encrypted.zip -f ~/SecLists/Passwords/Common-Credentials/10k-most-common.txt

# 3. Or use an entire directory of password lists
unarc tp encrypted.arj -d ~/SecLists/Passwords/

# 4. For faster testing, pick a small file in the archive
unarc tp large_archive.7z -d ~/SecLists/Passwords/ -e "readme.txt"
```

**Performance** varies by format due to encryption complexity:

- ARC: ~3.5 million passwords/sec (simple XOR)
- ARJ: ~2,000 passwords/sec (Garble)
- ZIP: ~50,000 passwords/sec (ZipCrypto)
- 7z: ~16 passwords/sec (AES-256, by design slow)

## Supported Formats

| Format | Extensions | Encryption | Multi-Volume |
| ------ | ---------- | ---------- | ------------ |
| **7z** | `.7z`, `.7z.001` | AES-256 | ✓ |
| **ZIP** | `.zip`, `.zip.001`, `.z01` | ZipCrypto, AES | ✓ |
| **RAR** | `.rar` | AES | — |
| **LHA/LZH** | `.lha`, `.lzh` | — | — |
| **TAR** | `.tar` | — | — |
| **ACE** | `.ace` | Blowfish | ✓ |
| **ARJ** | `.arj` | Garble, GOST40 | ✓ |
| **ARC/PAK** | `.arc`, `.pak` | XOR | — |
| **ZOO** | `.zoo` | — | — |
| **HA** | `.ha` | — | — |
| **UC2** | `.uc2` | — | — |
| **SQ/SQ2** | `.sq`, `.sq2`, `.qqq`, `?q?` | — | — |
| **SQZ** | `.sqz` | — | — |
| **HYP** | `.hyp` | — | — |
| **Z** | `.Z` | — | — |
| **GZ** | `.gz` | — | — |
| **BZ2** | `.bz2` | — | — |
| **XZ** | `.xz` | — | — |
| **ZST** | `.zst` | — | — |
| **Pack-Ice** | `.pi9` | — | — |
| **TGZ** | `.tgz`, `.tar.gz` | — | — |
| **TBZ** | `.tbz`, `.tar.bz2` | — | — |
| **TAR.Z** | `.tar.Z` | — | — |
| **TXZ** | `.txz`, `.tar.xz` | — | — |
| **TZST** | `.tzst`, `.tar.zst` | — | — |

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

Crack a password-protected archive:

```bash
# First, see which encrypted files are available
unarc tp archive.arj --stdin -e "?"  # Shows list of encrypted entries
# Error: Entry '?' not found. Encrypted entries:
#   - LICENSE (11357 bytes)
#   - bigfile.dat (50000000 bytes)

# Pick the smallest file for faster testing
unarc tp archive.arj -f rockyou.txt -e "LICENSE"
```

## Related

- [unarc-rs](https://crates.io/crates/unarc-rs) - The library this CLI is built on

## License

MIT OR Apache-2.0
