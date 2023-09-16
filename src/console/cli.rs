use clap::{arg, Command};

pub fn create_cli() -> Command {
    Command::new("neko")
        .about("A pretty and simple to use AUR helper.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("search")
                .alias("s")
                .about("Search the AUR for a package")
                .arg(arg!(<PACKAGE> "The package to search for"))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("install")
                .alias("i")
                .about("Install a package")
                .arg(arg!(<PACKAGE> "The package to install"))
                .arg_required_else_help(true)
        )
}