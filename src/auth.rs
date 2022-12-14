use crate::helpers;
use colored::Colorize;
use inquire::{Password, PasswordDisplayMode};
use std::io::Write;

pub fn login() {
    match home::home_dir() {
        Some(path) => {
            if !std::path::Path::new(helpers::string_to_str(format!("{}/.ez", path.display()))).is_dir() {
                std::fs::create_dir_all(format!("{}/.ez", path.display())).unwrap();
                println!("created {}/.ez", path.display());
            }

            println!("(please copy your API key here)");

            let key = Password::new("api key:")
                .with_display_toggle_enabled()
                .with_display_mode(PasswordDisplayMode::Masked)
                .without_confirmation()
                .prompt();

            match key {
                Ok(value) => {
                    let mut file = std::fs::File::create(format!("{}/.ez/.token", path.display())).unwrap();
                    file.write_all(value.as_bytes()).unwrap();
                    println!("\x08{} {}", "✔".green(), "added api access key".bright_green());
                }
                Err(_) => {
                    eprint!("\r{} {}\n", "✖".red(), "unable to add api key".bright_red());
                    std::process::exit(1);
                }
            };
        }
        None => {
            eprintln!("{} {}", "✖".red(), "unable to find your home directory".bright_red());
            std::process::exit(1);
        }
    }
}

pub fn logout() {
    match home::home_dir() {
        Some(path) => {
            if let Err(_) = std::fs::remove_file(format!("{}/.ez/.token", path.display())) {
                eprintln!("{} {}", "unable to remove api access key".red(), "(does it exist?)".bright_red());
            } else {
                println!("{} {}", "✔".green(), "removed api access key".bright_green())
            }
        }
        None => {
            eprintln!("{} {}", "✖".red(), "unable to find your home directory".bright_red());
            std::process::exit(1);
        }
    }
}
