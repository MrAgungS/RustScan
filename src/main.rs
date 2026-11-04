use clap::Parser;
use std::path::PathBuf;

mod file_info;
mod scanner;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
}

fn main() {
    let input = Args::parse();
    let path_str = input
        .path
        .to_str()
        .expect("Path contains invalid UTF-8 characters");

    match scanner::scan_folder(path_str) {
        Ok(success) => println!("{:#?}", success),
        Err(error) => eprintln!("got error {:#?}", error),
    }
}
