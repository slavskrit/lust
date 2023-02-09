use std::fs;
use toml::{map::Map, Table, Value};

pub fn read_file() -> Map<String, Value> {
    let filename = "src/config/config.toml";
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            panic!("Could not read file `{}`", filename)
        }
    };

    return match contents.parse::<Table>() {
        Ok(d) => d,
        Err(e) => {
            panic!("Unable to load data from `{}` {}", filename, e);
        }
    };
}
