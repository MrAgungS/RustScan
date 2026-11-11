use clap::Parser;
use std::path::PathBuf;

use crate::hasher::hash_files;

mod file_info;
mod hasher;
mod scanner;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,

    #[arg(short, long, num_args = 0..)]
    ignored_folder: Vec<String>,

    #[arg(short, long)]
    check_duplicates: bool,
}

fn main() {
    let input = Args::parse();

    match scanner::scan_folder(&input.path, &input.ignored_folder) {
        // Ok(success) => println!("{:#?}", success),
        Ok(success) => {
            if input.check_duplicates {
                let duplicate_check = hash_files(&success);
                println!("{:#?}", duplicate_check)
            } else {
                println!("{:#?}", success)
            }
        }
        Err(error) => eprintln!("got error {:#?}", error),
    }
}
