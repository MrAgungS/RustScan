use clap::Parser;
use std::path::PathBuf;

mod file_info;
mod scanner;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,

    #[arg(short, long, num_args = 0..)]
    ignored_folder: Vec<String>,
}

fn main() {
    let input = Args::parse();
    let path_str = input
        .path
        .to_str()
        .expect("Path contains invalid UTF-8 characters");

    let ignored_folder_string = &input.ignored_folder;

    match scanner::scan_folder(path_str, ignored_folder_string) {
        Ok(success) => println!("{:#?}", success),
        Err(error) => eprintln!("got error {:#?}", error),
    }
}
