use std::io;
use std::io::Write;
use std::process::{Command, Stdio};
use crate::client::get_aur_packages::get_aur_packages;

pub async fn download(package: &str) {
    let remote_aur_url: String = format!("https://aur.archlinux.org/{}.git", package);

    if get_aur_packages(package).await.result_count < 1 {
        println!("No packages found by that name!");
        return;
    }

    print!("You're about to download {}. Is this correct? (y/n) ", package);
    io::stdout().flush().expect("Error flushing stdout");
    let mut user_answer = String::new();
    io::stdin().read_line(&mut user_answer).expect("Error reading line");

    if user_answer.trim().to_lowercase() != "y" &&
       user_answer.trim() != "" {
        return;
    }

    let git_command = Command::new("git")
        .args(["clone", remote_aur_url.as_str()])
        .current_dir("/tmp/")
        .stdout(Stdio::inherit())
        .status()
        .expect("Failed to clone into remote");

    if !git_command.success() {
        eprintln!("Git command failed with exit code {:?}", git_command.code());
        return;
    }

    println!("Running makepkg...");
    let makepkg_command = Command::new("makepkg")
        .arg("-si")
        .current_dir(format!("/tmp/{}", package))
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .status()
        .expect("Failed to make package");

    if !makepkg_command.success() {
        eprintln!("Makepkg command failed with exit code {:?}", makepkg_command.code());
        return;
    }
}