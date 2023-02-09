use std::fs;

use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub colors: Colors,
}
#[derive(Deserialize)]
pub struct Colors {
    // Main Colors
    pub unrecognized_file: String,
}

pub fn read_file() -> Config {
    let filename = "src/config/config.toml";
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            panic!("Could not read file `{}`", filename)
        }
    };

    return match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            panic!("Unable to load data from `{}` {}", filename, e);
        }
    };
}
