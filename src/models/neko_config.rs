use std::{env, fs};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NekoConfig {
    pub pkg_limit: i32,

    pub use_colours: bool,

    pub show_id: bool,
    pub show_version: bool,
    pub show_maintainer: bool,
    pub show_votes: bool
}

pub fn get_neko_config() -> NekoConfig {
    let mut config_path: String = String::new();
    match env::current_exe() {
        Ok(exe) => {
            let dir = exe.parent().unwrap().to_str().unwrap();
            config_path = format!("{}/config/config.xml", dir);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    let config_string = fs::read_to_string(config_path).unwrap();

    serde_xml_rs::from_str(config_string.as_str()).unwrap()
}