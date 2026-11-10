use std::{path::PathBuf, time::SystemTime};

#[derive(Debug)]
pub struct FileInfo {
    pub name: PathBuf,
    pub modified: SystemTime,
    pub size: u64,
}

#[derive(Debug)]
pub struct HashedFile<'a> {
    pub file_info: &'a FileInfo,
    pub hash: String,
}
