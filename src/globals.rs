use crate::error;
use crate::helpers::Exists;
use colored::Colorize;
use global_placeholders::init;

pub fn init() {
    match home::home_dir() {
        Some(path) => {
            let path = path.display();

            if !Exists::check(&format!("{path}/.ez/")).folder() {
                std::fs::create_dir_all(format!("{path}/.ez/")).unwrap();
                log::info!("created e-z config directory");
            }

            init!("ez.base", format!("{path}/.ez/"));
            init!("ez.token", format!("{path}/.ez/.token"));
        }
        None => error!("Impossible to get your home directory"),
    }
}
