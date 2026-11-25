use std::{path::PathBuf, time::SystemTime};

/// Represents metadata for a single scanned file or directory entry.
///
/// This is the core data model produced by `scanner::scan_folder` — one
/// instance per entry found while walking a directory tree.
#[derive(Debug)]
pub struct FileInfo {
    /// Full path to the entry, as discovered during the directory walk.
    pub name: PathBuf,
    /// Last modification timestamp, as reported by the filesystem.
    /// Kept as `SystemTime` (not converted to a human-readable string)
    /// so that formatting/display concerns stay separate from the raw
    /// data — conversion happens later, only when actually displaying
    /// results, not at scan time.
    pub modified: SystemTime,
    /// Size of the entry in bytes, as reported by the filesystem.
    /// `u64` is used because file sizes are always non-negative and can
    /// exceed what a 32-bit integer could hold (e.g. large video files).
    pub size: u64,
    /// Whether this entry is a directory (`true`) or a regular file
    /// (`false`). This distinction matters downstream — e.g. hashing
    /// logic in `hasher.rs` skips directories entirely, since a
    /// directory has no byte content to hash.
    pub is_dir: bool,
}

/// Pairs a `FileInfo` with the SHA-256 hash of its file contents.
///
/// Produced by `hasher::hash_files` as an intermediate step toward
/// duplicate detection. Borrows the original `FileInfo` (`&'a FileInfo`)
/// rather than owning a copy, since the original `Vec<FileInfo>` from
/// scanning already exists and doesn't need to be duplicated in memory
/// just to attach a hash to each entry.
#[derive(Debug)]
pub struct HashedFile<'a> {
    pub file_info: &'a FileInfo,
    pub hash: String,
}
