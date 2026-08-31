## Installation

### From Source

```bash
cargo install --path .
```

---

## Usage

### Interactive Mode

Simply run the binary without arguments:

```bash
hensuki
```

You will be prompted:

1. `[+] Please enter file you want to bulk download (e.g. file.txt): `
2. `[+] Please choose (1) Multiple posts or (2) Multiple pages: `
3. `[+] Select type image results (1) Original size or (2) Smaller size: `

### Non-Interactive Mode

Pass command-line arguments directly:

```bash
hensuki -f links.txt -m 1 -s 1
```

Options:

- `-f, --file <FILE>`: Path to `.txt` file containing URLs.
- `-m, --mode <MODE>`: `1` / `posts` for individual posts, or `2` / `pages` for gallery pages.
- `-s, --select-type <TYPE>`: `1` / `original` for full resolution, or `2` / `smaller` for compressed size.

---
