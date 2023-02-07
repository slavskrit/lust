use clap::Parser;
use colored::Colorize;
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///Initial population size
    #[clap(short, long, default_value_t = String::from("."))]
    pub path: String,
}

fn main() {
    let args = Args::parse();
    let path = args.path;
    get_files(&path)
}

fn get_files(path: &String) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            println!("{}", path.to_str().unwrap().red());
        }
    }
}
