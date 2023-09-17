use nix::libc::getuid;
use crate::client::get_aur_packages::get_aur_packages;
use crate::commands::download::download;
use crate::console::cli::create_cli;
use crate::console::neko_console::NekoConsole;
use crate::models::neko_config::{get_neko_config, NekoConfig};
use crate::models::package_results::PackageResults;
use crate::utilities::dir_utils::{create_config, dir_exists};
use crate::utilities::get_xdg_config::get_xdg_config;

mod client;
mod models;
mod console;
mod utilities;
mod commands;

#[tokio::main]
async fn main() {
    unsafe {
        if getuid() == 0 {
            println!("Please do not run neko as root.");
            return;
        }
    }

    let neko_config_path = get_xdg_config("XDG_CONFIG_HOME");
    if !dir_exists(neko_config_path.as_str())
    {
        create_config(neko_config_path.as_str());
    }

    let arg_matches = create_cli().get_matches();
    match arg_matches.subcommand() {
        Some(("search", sub_matches)) => {
            let package_name = sub_matches.get_one::<String>("PACKAGE").expect("Expected package");
            let config: NekoConfig = get_neko_config(format!("{}/config.xml", neko_config_path).as_str());
            let packages: PackageResults = get_aur_packages(package_name).await;

            let n_console: NekoConsole = NekoConsole::new(config, packages);
            n_console.write()
        },
        Some(("install", sub_matches)) => {
            let package_name = sub_matches.get_one::<String>("PACKAGE").expect("Expected package");
            download(package_name.as_str()).await;
        },
        _ => {
            println!("Invalid argument. Aborting.");
            return;
        }
    }
}
