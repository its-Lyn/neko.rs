use std::env;
use crate::client::requests::get_generic;
use crate::console::neko_console::NekoConsole;
use crate::models::neko_config::{get_neko_config, NekoConfig};
use crate::models::package_results::PackageResults;
use crate::utilities::dir_utils::{create_config, dir_exists};
use crate::utilities::get_xdg_config::get_xdg_config;

mod client;
mod models;
mod console;
mod utilities;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: neko <package_name>");
        return;
    }

    let package_name = args[1].as_str();
    let aur_url = format!("https://aur.archlinux.org/rpc/?v=5&type=search&arg={}", package_name);

    let json_str: String = get_generic(aur_url.as_str()).await.unwrap();
    let json: PackageResults = serde_json::from_str(json_str.as_str()).unwrap();

    let neko_config_path = get_xdg_config("XDG_CONFIG_HOME");
    if !dir_exists(neko_config_path.as_str())
    {
        create_config(neko_config_path.as_str());
    }

    let config: NekoConfig = get_neko_config(format!("{}/config.xml", neko_config_path).as_str());
    let n_console: NekoConsole = NekoConsole::new(config, json);
    n_console.write()
}
