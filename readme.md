# RustScan

A command-line file system scanner built in Rust, created as a learning project to practice core Rust concepts: ownership, error handling, modules, and the standard library's filesystem APIs.

> **Status: Work in Progress** 
> This project is actively being developed as part of a personal Rust learning journey. Features, APIs, and project structure are subject to change.

## Overview

RustScan scans a given directory and collects metadata about its contents — file name, size, and last modified date. The long-term goal is to grow this into a more capable CLI tool with recursive scanning, file hashing (for duplicate detection), and possibly real-time file watching.

## Planned Features

- [x] Define core `FileInfo` data structure
- [x] Scan a single directory and collect file metadata
- [x] CLI argument parsing (via `clap`)
- [ ] Recursive directory scanning (via `walkdir`)
- [ ] File hashing for duplicate detection (via `sha2`)
- [ ] Optional file watcher for real-time scanning

## Tech Stack

- **Language:** Rust
- **Planned dependencies:**
  - [`clap`](https://crates.io/crates/clap) — command-line argument parsing
  - [`walkdir`](https://crates.io/crates/walkdir) — recursive directory traversal
  - [`sha2`](https://crates.io/crates/sha2) — file hashing

## Getting Started

This project is not yet ready for use. Instructions for building and running will be added once the core scanning functionality is complete.

```bash
cargo build
cargo run
```

## Learning Context

This project is being built incrementally, with a focus on understanding *why* each design decision is made rather than just getting code to compile. It follows a progression from:

1. Single-directory scanning with the standard library
2. CLI argument handling
3. Recursive scanning and additional features (hashing, filtering, etc.)

## License

Not yet decided.
