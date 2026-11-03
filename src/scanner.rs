use crate::file_info::FileInfo;
use std::{fs, io::Error};

pub fn scan_folder(path: &str) -> Result<Vec<FileInfo>, Error> {
    let entries = fs::read_dir(path)?;
    let mut name_fileinfo: Vec<FileInfo> = Vec::new();

    for entry in entries {
        let dir = entry?;
        let metadata = dir.metadata()?;

        let size = metadata.len();
        let modified = metadata.modified()?;
        let name = dir.path();

        let file_info = FileInfo {
            name,
            modified,
            size,
        };

        name_fileinfo.push(file_info);
    }
    Ok(name_fileinfo)
}
