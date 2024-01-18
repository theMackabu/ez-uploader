use crate::error;
use crate::helpers::Exists;
use colored::Colorize;
use global_placeholders::init;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

pub fn init() {
    match home::home_dir() {
        Some(path) => {
            let path = path.display();

            if !Exists::check(&format!("{path}/.ez/")).folder() {
                std::fs::create_dir_all(format!("{path}/.ez/")).unwrap();
                log::info!("created e-z config directory");
            }

            if !Exists::check(&format!("{path}/.ez/cache.db")).file() {
                PickleDb::new(format!("{path}/.ez/cache.db"), PickleDbDumpPolicy::AutoDump, SerializationMethod::Bin);
                log::info!("created e-z cache database");
            }

            init!("ez.base", format!("{path}/.ez/"));
            init!("ez.token", format!("{path}/.ez/.token"));
            init!("ez.cache", format!("{path}/.ez/cache.db"));
        }
        None => error!("Impossible to get your home directory"),
    }
}
