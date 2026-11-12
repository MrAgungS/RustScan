use std::{fs::read, io::Error};

use sha2::{Digest, Sha256};

use crate::file_info::{FileInfo, HashedFile};

pub fn hash_files<'a>(files: &'a Vec<FileInfo>) -> Result<Vec<HashedFile<'a>>, Error> {
    let mut hashed_file: Vec<HashedFile<'a>> = Vec::new();

    for file_info in files {
        if file_info.is_dir {
            continue;
        }

        let contents = read(&file_info.name)?;

        let mut hasher = Sha256::new();
        hasher.update(&contents);
        let result = hasher.finalize();
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
