use crate::file_info::FileInfo;
use std::{fs, io::Error, path::PathBuf};

pub fn scan_folder(path: &str, exclude_list: &Vec<String>) -> Result<Vec<FileInfo>, Error> {
    let entries = fs::read_dir(path)?;
    let mut results: Vec<FileInfo> = Vec::new();

    for entry in entries {
        let dir = entry?;
        let entry_path = &dir.path();

        if should_skip(entry_path, exclude_list) {
            continue;
        }

        let metadata = dir.metadata()?;

        let size = metadata.len();
        let modified = metadata.modified()?;
        let name = entry_path.clone();

        let file_info = FileInfo {
            name,
            modified,
            size,
        };

        results.push(file_info);
    }
    Ok(results)
}

fn should_skip(entry_path: &PathBuf, exclude_list: &Vec<String>) -> bool {
    let entry = entry_path.file_name();
    let Some(entry_string) = entry.and_then(|e| e.to_str()).map(|s| s.to_string()) else {
        return false;
    };

    exclude_list.contains(&entry_string)
}
