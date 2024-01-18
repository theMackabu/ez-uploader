use crate::{error, helpers, routes};
use chrono::{DateTime, Utc};
use colored::Colorize;
use global_placeholders::global;
use inquire::Select;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FileInfo {
    pub(crate) from: String,
    pub(crate) to: String,
    pub(crate) name: NameInfo,
    pub(crate) delete_link: String,
    pub(crate) uploaded: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct NameInfo {
    pub(crate) local: String,
    pub(crate) server: String,
}

fn load() -> PickleDb { PickleDb::load(global!("ez.cache"), PickleDbDumpPolicy::AutoDump, SerializationMethod::Bin).unwrap() }

pub fn save(file: FileInfo) {
    log::info!("Saving {file:?}");

    let mut db = load();
    let all_keys = db.get_all();
    let current_size = db.total_keys();

    if current_size > 10 {
        let excess = current_size - 10;
        let keys_to_remove: Vec<_> = all_keys.iter().take(excess).collect();
        for key in keys_to_remove {
            db.rem(&key).unwrap();
        }
    }

    if let Err(err) = db.set(&file.name.local, &file) {
        log::error!("{err}");
        error!("Unable to cache file, database error!");
    };
}

pub fn delete(name: String) -> Result<FileInfo, anyhow::Error> {
    let mut db = load();

    match db.get::<FileInfo>(&name) {
        Some(data) => {
            db.rem(&name)?;
            Ok(data)
        }
        None => error!("Unable to find file, was it uploaded?"),
    }
}

pub fn list() {
    let db = load();

    let files = db
        .iter()
        .map(|item| {
            let key = item.get_key();
            let value = item.get_value::<FileInfo>().unwrap();

            format!("[{key}]: (from={}, to={}, uploaded={})", value.from, value.to, DateTime::format(&value.uploaded, "%d/%m/%y %H:%M %Z"))
        })
        .collect::<Vec<_>>();

    match Select::new("Select a file to delete:", files).prompt() {
        Ok(test) => {
            let key = helpers::trim_start_end(test.split(":").collect::<Vec<_>>()[0]);
            routes::files::delete(&Some(key.to_string()));
        }
        Err(_) => println!("{}", "Aborting...".white()),
    }
}
