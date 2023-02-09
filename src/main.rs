use clap::Parser;
use colored::{ColoredString, Colorize};
use std::{fs, path::PathBuf};
use toml::{map::Map, Value};
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
        let icon = match nf.get(file.iconed(&config)) {
            Some(d) => d,
            None => nf.get("fa-file").unwrap(),
        };
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
    fn iconed<'a>(&'a self, config: &'a Map<String, Value>) -> &str;
}

impl Coloring for PathBuf {
    fn iconed<'a>(&'a self, config: &'a Map<String, Value>) -> &str {
        if self.is_dir() {
            return "fa-folder_open_o";
        }
        if self.is_symlink() {
            return "fa-link";
        }
        let default = "hidden";
        let extension = match self.extension() {
            Some(d) => d.to_str().unwrap(),
            None => default,
        };
        let icons = config.get("icons").unwrap().as_table().unwrap();
        let icon = match icons.get(extension) {
            Some(d) => d,
            None => icons.get(default).unwrap(),
        };

        icon.as_str().unwrap()
    }

    fn colored(&self) -> ColoredString {
        let path = &self.to_str().unwrap();
        if self.is_relative() {
            return path.truecolor(103, 0, 136);
        }
        return path.red();
    }
}
