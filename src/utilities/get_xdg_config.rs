use std::env;

pub fn get_xdg_config(env_name: &str) -> String {
    let config_base: String;

    match env::var(env_name) {
        Ok(v) => {
            config_base = format!("{}/neko", v);
        }
        Err(_) => {
            match env::var("HOME") {
                Ok(home) => {
                    config_base = format!("{}/.config/neko", home);
                }
                Err(e) => {
                    panic!("$HOME does not exist! Please set $HOME environment variable and come back. ({})", e)
                }
            }
        }
    }

    config_base
}