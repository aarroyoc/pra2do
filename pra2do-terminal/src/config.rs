use std::fs::File;
use std::io::Read;
use serde::{Deserialize};

const CONFIG_FILE_NAME: &'static str = "pra2do.toml";

#[derive(Deserialize)]
pub struct Config {
    pub address: String,
    pub exec: String,
}

pub fn load_config() -> Config {
    let mut config_file = File::open(CONFIG_FILE_NAME).expect(&format!("Configuration file '{CONFIG_FILE_NAME}' is missing"));
    let mut config_str = String::new();
    config_file.read_to_string(&mut config_str);
    toml::from_str(&config_str).expect("Invalid file")
}