use std::env;
use crate::client::requests::get_generic;
use crate::console::neko_console::NekoConsole;
use crate::models::neko_config::{get_neko_config, NekoConfig};
use crate::models::package_results::PackageResults;

mod client;
mod models;
mod console;

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

    let config: NekoConfig = get_neko_config();
    let n_console: NekoConsole = NekoConsole::new(config, json);
    n_console.write()
}
