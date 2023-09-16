use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NekoConfig {
    pub pkg_limit: i32,

    pub use_colours: bool,

    pub show_id: bool,
    pub show_version: bool,
    pub show_maintainer: bool,
    pub show_votes: bool
}

pub fn get_neko_config(config_path: &str) -> NekoConfig {
    let config_string = fs::read_to_string(config_path).unwrap();
    serde_xml_rs::from_str(config_string.as_str()).unwrap()
}