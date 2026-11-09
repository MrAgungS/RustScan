use walkdir::WalkDir;

use crate::file_info::FileInfo;
use std::{io::Error, path::Path};

pub fn scan_folder(path: &Path, exclude_list: &[String]) -> Result<Vec<FileInfo>, Error> {
    let entries = WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .filter_entry(|entry_path| !should_skip(entry_path.path(), exclude_list));
    let mut results: Vec<FileInfo> = Vec::new();

    for entry in entries {
        let dir = entry?;

        let entry_path = dir.path();
        let metadata = dir.metadata()?;

        let file_info = FileInfo {
            name: entry_path.to_path_buf(),
            modified: metadata.modified()?,
            size: metadata.len(),
        };

        results.push(file_info);
    }
    Ok(results)
}

fn should_skip(entry_path: &Path, exclude_list: &[String]) -> bool {
    let Some(entry) = entry_path.file_name().and_then(|e| e.to_str()) else {
        return false;
    };

    exclude_list.iter().any(|excluded| excluded == entry)
}
