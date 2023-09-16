use colored::Colorize;
use crate::models::neko_config::NekoConfig;
use crate::models::package_results::PackageResults;

pub struct NekoConsole {
    config: NekoConfig,
    packages: PackageResults
}

impl NekoConsole {
    pub fn new(config: NekoConfig, packages: PackageResults) -> Self {
        NekoConsole {
            config,
            packages
        }
    }

    pub fn write(&self) {
        let config = &self.config;
        let packages = &self.packages;

        if config.use_colours {
            println!("{}", format!("{} results found!", packages.result_count).green());

            if packages.result_count > config.pkg_limit {
                println!("{}", format!("Package limit exceeded! Only showing {} results.\n", config.pkg_limit).bright_red());
            }

            print!("\n");
        }

        println!("{} results found!", packages.result_count);
        if packages.result_count > config.pkg_limit {
            println!("Package limit exceeded! Only showing {} results.\n", config.pkg_limit);
        } else {
            print!("\n");
        }

        let mut shown: i32 = 0;
        for package in packages.results.iter() {
            if shown > config.pkg_limit {
                break;
            }

            if config.use_colours {
                print!("{}", format!("aur/").magenta());
                println!("{}{}{}{}    {}  {}{}{}",
                    format!("{}", package.name).cyan(),
                    if config.show_version {format!(" ({})", package.version)} else {"".to_string()},
                    if config.show_id {format!(" - #{}", package.id).bright_black()} else {"".bright_black()},
                    if config.show_votes {format!("  [+{}]", package.num_votes).green()} else {"".green()},
                    if package.out_of_date != None {"(Out Of Date)".bright_red()} else {"".red()},
                    if package.maintainer == None {"(No Maintainer)".bright_red()} else {"".red()},
                    if package.description != None {format!("\n    {}", package.description.as_ref().unwrap()).bright_white()} else {"".bright_white()},
                    if config.show_maintainer && package.maintainer != None {format!("\n\nMaintainer: {}", package.maintainer.as_ref().unwrap())} else {"".to_string()}
                );
            } else {
                println!("aur/{}{}{}{}    {}  {}{}{}",
                    package.name,
                    if config.show_version {format!(" ({})", package.version)} else {"".to_string()},
                    if config.show_id {format!(" - #{}", package.id)} else {"".to_string()},
                    if config.show_votes {format!("  [+{}]", package.num_votes)} else {"".to_string()},
                    if package.out_of_date != None {"(Out Of Date)"} else {""},
                    if package.maintainer == None {"(No Maintainer)"} else {""},
                    if package.description != None {format!("\n    {}", package.description.as_ref().unwrap())} else {"".to_string()},
                    if config.show_maintainer && package.maintainer != None {format!("\n\nMaintainer: {}", package.maintainer.as_ref().unwrap())} else {"".to_string()}
                );
            }

            print!("\n");
            shown += 1;
        }
    }
}
