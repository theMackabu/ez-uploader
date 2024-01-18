use crate::{error, ok};
use colored::Colorize;
use global_placeholders::global;
use inquire::{Password, PasswordDisplayMode};
use std::{fs::File, io::Write};

pub fn login() {
    println!("{}", "(please copy your API key here)".white());

    let key = Password::new("api key:")
        .with_display_toggle_enabled()
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt();

    match key {
        Ok(value) => {
            let mut file = File::create(global!("ez.token")).unwrap();
            file.write_all(value.as_bytes()).unwrap();
            ok!("Added API access key.");
        }
        Err(err) => {
            log::error!("{err}");
            error!("Unable to add API key!");
        }
    };
}

pub fn logout() {
    match std::fs::remove_file(global!("ez.token")) {
        Ok(_) => ok!("Removed API access key!"),
        Err(err) => {
            log::error!("{err}");
            error!("Unable to remove api access key, does it exist?");
        }
    }
}
