use std::{fs::read, io::Error};

use sha2::{Digest, Sha256};

use crate::file_info::{FileInfo, HashedFile};

fn hash_files<'a>(files: &'a Vec<FileInfo>) -> Result<Vec<HashedFile<'a>>, Error> {
    let mut hashed_file: Vec<HashedFile> = Vec::new();

    for file_info in files {
        let contents = read(&file_info.name)?;

        let mut hasher = Sha256::new();
        hasher.update(contents);
        let result = hasher.finalize();
        let result_str = format!("{:x}", result);
    }
    Ok(_)
}
