use crate::file_info::FileInfo;
use std::{fs, io::Error};

pub fn scan_folder(path: &str) -> Result<Vec<FileInfo>, Error> {
    let entries = fs::read_dir(path)?;
    let name_fileinfo: Vec<FileInfo> = Vec::new();

    for entry in entries {
        let dir = entry?;
        println!("{:?}", dir.path());
    }
    Ok(name_fileinfo)
}
