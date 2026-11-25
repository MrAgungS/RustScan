use clap::Parser;
use std::path::PathBuf;

use crate::hasher::{find_duplicates, hash_files};

mod file_info;
mod hasher;
mod scanner;

/// Command-line arguments for rustscan.
#[derive(Parser)]
struct Args {
    /// The directory to scan.
    #[arg(short, long)]
    path: PathBuf,

    /// Names of files/folders to exclude from scanning (e.g. `.git`,
    /// `target`). Accepts zero or more values per flag invocation,
    /// thanks to `num_args = 0..` (as opposed to an earlier, overly
    /// strict fixed count like `num_args = 2`).
    #[arg(short, long, num_args = 0..)]
    ignored_folder: Vec<String>,

    /// When set, additionally computes a SHA-256 hash for every scanned
    /// file and reports which files are exact duplicates of each other
    /// (see `hasher::hash_files` and `hasher::find_duplicates`), instead
    /// of just printing raw scan results.
    #[arg(short, long)]
    check_duplicates: bool,
}

fn main() {
    let input = Args::parse();

    match scanner::scan_folder(&input.path, &input.ignored_folder) {
        Ok(success) => {
            // if the user opted into duplicate checking, run the
            // hashing pass, then group the results into duplicate
            // clusters; otherwise just print the raw FileInfo list
            // as-is.
            if input.check_duplicates {
                match hash_files(&success) {
                    Ok(duplicate_check) => {
                        // narrows the full hashed list down to only
                        // the groups that actually share a hash with
                        // at least one other file — i.e. real
                        // duplicates, not every file's individual hash.
                        let duplicates = find_duplicates(&duplicate_check);
                        println!("{:#?}", duplicates)
                    }

                    Err(err) => eprintln!("{:#?}", err),
                }
            } else {
                println!("{:#?}", success)
            }
        }
        Err(error) => eprintln!("got error {:#?}", error),
    }
}
