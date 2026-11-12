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
        Ok(success) => {
            if input.check_duplicates {
                match hash_files(&success) {
                    Ok(duplicate_check) => println!("{:#?}", duplicate_check),
                    Err(err) => eprintln!("{:#?}", err)
                }
            } else {
                println!("{:#?}", success)
            }
        }
        Err(error) => eprintln!("got error {:#?}", error),
    }
}
