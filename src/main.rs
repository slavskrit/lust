use clap::Parser;
use colored::{ColoredString, Colorize};
use std::{fs, path::PathBuf};
extern crate nerd_fonts;
use nerd_fonts::NerdFonts;
use std::process::exit;
use toml;
mod config;
use config::Colors;

use crate::config::Config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(index=1, default_value_t = String::from("."))]
    pub path: String,

    #[clap(short, long, default_value_t = true)]
    pub colorify: bool,
}

fn main() {
    let filename = "src/config/config.toml";
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };

    dbg!(&contents);

    let data: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Unable to load data from `{}` {}", filename, e);
            exit(1);
        }
    };

    println!("{}", data.colors.unrecognized_file);
    let args = Args::parse();
    let path = args.path;
    get_files(&path)
}

fn get_files(path: &String) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let nf = NerdFonts {
            nf: NerdFonts::load(),
        };
        let nf_custom_c = nf.get("fa-folder_open_o").unwrap(); // '\u{e61e}'
        println!("{} {}", nf_custom_c, path.prinbtable());
    }
}

trait Coloring {
    fn prinbtable(&self) -> ColoredString;
}

impl Coloring for PathBuf {
    fn prinbtable(&self) -> ColoredString {
        let path = &self.to_str().unwrap();
        if self.is_relative() {
            return path.green();
        }
        return path.red();
    }
}
