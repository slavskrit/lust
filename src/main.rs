use clap::Parser;
use colored::{ColoredString, Colorize};
use std::{fs, path::PathBuf};
extern crate nerd_fonts;
use nerd_fonts::NerdFonts;
mod config;

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
    let files = get_files_in_directory(&path);
    print_files(files);
}

fn print_files(files: impl Iterator<Item = std::path::PathBuf>) {
    let config = config::read_file();
    let nf = NerdFonts {
        nf: NerdFonts::load(),
    };
    for file in files {
        let icon = nf.get(file.iconed()).unwrap();
        let colored_name = file.colored();
        println!("{} {}", icon, colored_name);
    }
}

fn get_files_in_directory(path: &String) -> impl Iterator<Item = std::path::PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
}

trait Coloring {
    fn colored(&self) -> ColoredString;
    fn iconed(&self) -> &str;
}

impl Coloring for PathBuf {
    fn iconed(&self) -> &str {
        if self.is_dir() {
            return "fa-folder_open_o";
        }
        return "fa-file";
    }

    fn colored(&self) -> ColoredString {
        let path = &self.to_str().unwrap();
        if self.is_relative() {
            return path.truecolor(103, 0, 136);
        }
        return path.red();
    }
}
