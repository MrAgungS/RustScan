use std::{collections::HashMap, fs::read, io::Error};

use sha2::{Digest, Sha256};

use crate::file_info::{FileInfo, HashedFile};

/// Computes a SHA-256 hash for every REGULAR FILE in `files`, skipping
/// directories (which have no byte content to hash).
///
/// This produces the raw, unfiltered pairing of every file with its
/// hash — identifying which files are actual DUPLICATES of each other
/// (i.e. share the same hash) is a separate step, handled by
/// `find_duplicates` below, which operates on this function's output.
///
/// The lifetime `'a` ties the returned `Vec<HashedFile<'a>>` to the
/// lifetime of the input `files` slice — each `HashedFile` borrows its
/// `FileInfo` rather than cloning it, so `files` must stay alive for as
/// long as the returned Vec is used.
pub fn hash_files<'a>(files: &'a Vec<FileInfo>) -> Result<Vec<HashedFile<'a>>, Error> {
    let mut hashed_file: Vec<HashedFile<'a>> = Vec::new();

    for file_info in files {
        // directories have no content to hash — skip them and move to
        // the next entry.
        if file_info.is_dir {
            continue;
        }

        // read the entire file into memory as a Vec<u8>. This can fail
        // (e.g. permission denied, file deleted after scanning but
        // before hashing), in which case `?` propagates the error and
        // aborts the whole hashing pass.
        let contents = read(&file_info.name)?;

        // SHA-256 hashing: create a fresh hasher per file, feed it the
        // file's full contents, then finalize to get the digest bytes.
        let mut hasher = Sha256::new();
        hasher.update(&contents);
        let result = hasher.finalize();

        // convert the raw hash bytes into a familiar hex string
        // representation (e.g. "a3f5...") — each byte becomes exactly
        // two hex characters via the `{:02x}` format specifier, and all
        // of them are concatenated into one continuous String.
        let result_str = result
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();

        hashed_file.push(HashedFile {
            file_info,
            hash: result_str,
        });
    }
    Ok(hashed_file)
}

/// Groups `hashed_files` by their hash, returning only the groups that
/// contain MORE THAN ONE file — i.e. actual duplicate sets. Files whose
/// hash is unique across the whole input (no match found) are dropped
/// entirely from the result, since a group of one isn't a duplicate.
///
/// The returned `Vec<Vec<&'a FileInfo>>` is a list of duplicate groups;
/// each inner Vec holds references to every `FileInfo` that shares an
/// identical SHA-256 hash with the others in that same group. The hash
/// value itself is discarded in the output, since the caller only
/// cares about which files are duplicates, not what their shared hash
/// value is.
pub fn find_duplicates<'a>(hashed_files: &[HashedFile<'a>]) -> Vec<Vec<&'a FileInfo>> {
    // keys borrow directly from each HashedFile's `hash` String (no
    // cloning needed) — values accumulate every FileInfo reference that
    // shares that hash.
    let mut groups: HashMap<&str, Vec<&'a FileInfo>> = HashMap::new();

    for hashed_file in hashed_files {
        // `.entry(...).or_default()` creates an empty Vec for this hash
        // the FIRST time it's seen, and simply returns the existing Vec
        // on every subsequent file that shares the same hash — avoiding
        // a manual "does this key already exist?" check.
        groups
            .entry(&hashed_file.hash)
            .or_default()
            .push(hashed_file.file_info);
    }

    // into_values() consumes the map, discarding the hash keys (they're
    // no longer needed) and yielding only the grouped Vec<&FileInfo>
    // values. filter() then drops any group with just one member —
    // those files are unique, not duplicates.
    groups
        .into_values()
        .filter(|files| files.len() > 1)
        .collect()
}
