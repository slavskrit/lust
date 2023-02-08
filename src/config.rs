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
