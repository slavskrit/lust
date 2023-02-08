use clap::Parser;
use colored::{ColoredString, Colorize};
use std::{fs, path::PathBuf};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(index=1, default_value_t = String::from("."))]
    pub path: String,

    #[clap(short, long, default_value_t = true)]
    pub colorify: bool,
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

        println!("{}", path.color());
    }
}

trait Coloring {
    fn color(&self) -> ColoredString;
}

impl Coloring for PathBuf {
    fn color(&self) -> ColoredString {
        let path = &self.to_str().unwrap();
        if self.is_relative() {
            return path.green();
        }
        return path.red();
    }
}
