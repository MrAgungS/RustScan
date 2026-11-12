# RustScan

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A command-line file system scanner built in Rust. RustScan scans a directory (recursively), collects file metadata, lets you exclude specific folders, and detects duplicate files via content hashing.

This project was built as a learning exercise to practice core Rust concepts: ownership, error handling, modules, lifetimes, and the standard library's filesystem APIs — alongside the `clap`, `walkdir`, and `sha2` crates.

## Features

- **Recursive directory scanning** — walks through a directory and all its subdirectories, collecting each file's name, size, and last modified date.
- **Folder exclusion** — skip specific folders (e.g. `.git`, `target`, `node_modules`) from being scanned at all, rather than just filtering them out afterward.
- **Duplicate detection** — optionally compute a SHA-256 hash for every scanned file, making it easy to spot files with identical content.
- **Installable as a CLI tool** — build once, install globally, and run `rustscan` from anywhere.

## Installation

Clone the repository and install it locally with Cargo:

```bash
git clone https://github.com/MrAgungS/rustscan.git
cd rustscan
cargo install --path .
```

This installs the `rustscan` binary to Cargo's bin directory (typically `~/.cargo/bin`), making it available as a global command as long as that directory is on your `$PATH`.

## Usage

### Basic scan

```bash
rustscan --path .
```

Scans the given directory recursively and prints metadata for every file found.

### Excluding folders

```bash
rustscan --path . --ignored-folder .git --ignored-folder target
```

Each `--ignored-folder` flag excludes one folder by name. Folders matching the exclusion list are skipped entirely — their contents are never walked into, so this works efficiently even on large directories like `.git`.

### Detecting duplicate files

```bash
rustscan --path . --check-duplicates
```

Computes a SHA-256 hash for every scanned file and includes it in the output, allowing you to identify files with identical content by comparing hashes.

### Combining options

```bash
rustscan --path /home/user/projects --ignored-folder .git --ignored-folder node_modules --check-duplicates
```

## CLI Options

| Flag | Description |
|---|---|
| `-p, --path <PATH>` | Directory to scan (required) |
| `-i, --ignored-folder <NAME>` | Folder name to exclude; can be repeated for multiple folders |
| `-c, --check-duplicates` | Compute file hashes and include them in the output |
| `-h, --help` | Print help |

## Tech Stack

- **Language:** Rust
- **Dependencies:**
  - [`clap`](https://crates.io/crates/clap) — command-line argument parsing
  - [`walkdir`](https://crates.io/crates/walkdir) — recursive directory traversal
  - [`sha2`](https://crates.io/crates/sha2) — SHA-256 file hashing


## Learning Context

This project was built incrementally, with an emphasis on understanding *why* each design decision is made rather than just getting code to compile. It progressed through:

1. Single-directory scanning with the standard library
2. CLI argument handling with `clap`
3. Efficient folder exclusion (checked before expensive filesystem operations)
4. Recursive scanning with `walkdir`
5. Duplicate detection via file hashing, including lifetime-annotated data structures to avoid unnecessary cloning

## License

Licensed under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0).
