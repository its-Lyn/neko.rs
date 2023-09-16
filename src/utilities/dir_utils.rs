use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn dir_exists(dir_name: &str) -> bool {
    Path::new(dir_name).is_dir()
}

pub fn create_config(neko_config_path: &str) {
    fs::create_dir(neko_config_path)
        .expect("Failed to create config directory.");

    let mut config_file = File::create(format!("{}/config.xml", neko_config_path))
        .expect("Failed to create config file");

    let config_xml =
        r#"<?xml version="1.0" encoding="utf-8"?>
<NekoConfig>
    <pkg_limit>200</pkg_limit>

    <use_colours>true</use_colours>

    <show_id>false</show_id>
    <show_version>true</show_version>
    <show_maintainer>false</show_maintainer>
    <show_votes>true</show_votes>
</NekoConfig>"#;

    config_file.write_all(config_xml.as_bytes())
        .expect("Failed to write config file");
}