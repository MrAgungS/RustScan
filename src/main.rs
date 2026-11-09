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

    match scanner::scan_folder(&input.path, &input.ignored_folder) {
        Ok(success) => println!("{:#?}", success),
        Err(error) => eprintln!("got error {:#?}", error),
    }
}
