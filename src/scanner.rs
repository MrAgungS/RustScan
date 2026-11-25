use walkdir::WalkDir;

use crate::file_info::FileInfo;
use std::{io::Error, path::Path};

/// Recursively scans a directory tree starting at `path`, collecting
/// metadata for every file and subdirectory encountered, while skipping
/// any entry whose name matches something in `exclude_list`.
///
/// Unlike the earlier single-level implementation (which used
/// `std::fs::read_dir`), this version uses the `walkdir` crate to walk
/// into subdirectories as well — this is what makes exclude_list genuinely
/// useful: without it, scanning something like a project root would also
/// walk into `.git` or `target`, which could mean scanning thousands of
/// irrelevant files.
pub fn scan_folder(path: &Path, exclude_list: &[String]) -> Result<Vec<FileInfo>, Error> {
    let entries = WalkDir::new(path)
        // min_depth(1) excludes the root path itself from the results —
        // we only want entries *inside* `path`, not `path` itself as an
        // entry (which would otherwise show up as its own FileInfo with
        // is_dir: true).
        .min_depth(1)
        .into_iter()
        // filter_entry is the KEY efficiency choice here: unlike
        // filtering the final results after the fact, filter_entry
        // tells the walker not to descend into a directory at all once
        // it's identified as excluded — so an excluded folder's entire
        // contents are never even visited, not just hidden from the
        // output. This is checked BEFORE metadata is ever fetched for
        // an entry, avoiding wasted filesystem calls on things we're
        // going to throw away anyway.
        .filter_entry(|entry_path| !should_skip(entry_path.path(), exclude_list));

    let mut results: Vec<FileInfo> = Vec::new();

    for entry in entries {
        // each entry from the WalkDir iterator is wrapped in a Result
        // (walking can fail, e.g. due to permission errors) — propagate
        // any such error with `?` rather than silently skipping it.
        let dir = entry?;

        let entry_path = dir.path();
        // fetching metadata can also fail — propagate with `?` as well.
        let metadata = dir.metadata()?;

        let file_info = FileInfo {
            // to_path_buf() clones the borrowed Path into an owned
            // PathBuf, since FileInfo needs to own its data rather than
            // borrow from the (soon-to-be-dropped) directory entry.
            name: entry_path.to_path_buf(),
            modified: metadata.modified()?,
            size: metadata.len(),
            is_dir: metadata.is_dir(),
        };

        results.push(file_info);
    }
    Ok(results)
}

/// Determines whether a given path's final component (its file or
/// folder name) matches any entry in `exclude_list`.
///
/// Returns `false` (don't skip) whenever the name can't be determined
/// at all — e.g. if `file_name()` returns `None`, or the name isn't
/// valid UTF-8 — rather than panicking, so a single oddly-named entry
/// never crashes the whole scan.
fn should_skip(entry_path: &Path, exclude_list: &[String]) -> bool {
    // file_name() returns Option<&OsStr>; and_then + to_str() chains
    // through both the "might not have a file name" case and the
    // "might not be valid UTF-8" case, falling through to `false` via
    // the let-else if either step fails.
    let Some(entry) = entry_path.file_name().and_then(|e| e.to_str()) else {
        return false;
    };

    // .any() short-circuits as soon as a match is found — no need to
    // check every remaining entry in exclude_list once one matches.
    exclude_list.iter().any(|excluded| excluded == entry)
}
