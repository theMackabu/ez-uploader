use crate::{error, helpers, routes};
use chrono::{DateTime, Utc};
use colored::Colorize;
use global_placeholders::global;
use inquire::Select;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::str::from_utf8;

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

lazy_static! {
    static ref DB: sled::Db = {
        match sled::open(global!("ez.cache")) {
            Ok(db) => db,
            Err(err) => {
                log::error!("{err}");
                error!("Unable to open database, does it exist?");
            }
        }
    };
}

pub fn save(file: FileInfo) {
    log::info!("Saving {file:?}");

    let current_size = DB.len();
    if current_size > 10 {
        let excess = current_size - 10;
        let keys_to_remove: Vec<_> = DB.iter().map(|item| item.unwrap().0.to_vec()).take(excess).collect();
        for key in keys_to_remove {
            DB.remove(&key).unwrap();
        }
    }

    let json = match to_string(&file) {
        Ok(json) => json,
        Err(err) => {
            log::error!("{err}");
            error!("Unable to cache file, serde error!");
        }
    };

    if let Err(err) = DB.insert(file.name.local, json.as_str()) {
        log::error!("{err}");
        error!("Unable to cache file, database error!");
    };
}

pub fn load(name: String) -> Result<FileInfo, anyhow::Error> {
    match DB.get(&name) {
        Ok(data) => match data {
            Some(data) => {
                DB.remove(&name)?;
                Ok(from_str(from_utf8(&data)?)?)
            }
            None => error!("Unable to find file, was it uploaded?"),
        },
        Err(err) => {
            log::error!("{err}");
            error!("Unable to find file, was it uploaded?");
        }
    }
}

pub fn list() {
    let options = DB
        .range(""..)
        .map(|item| {
            let (key, value) = item.unwrap();
            let json = from_str::<FileInfo>(from_utf8(&value).unwrap()).unwrap();
            format!(
                "[{}]: (from={}, to={}, uploaded={})",
                from_utf8(&key).unwrap(),
                json.from,
                json.to,
                DateTime::format(&json.uploaded, "%d/%m/%y %H:%M %Z")
            )
        })
        .collect::<Vec<_>>();

    match Select::new("Select a file to delete:", options).prompt() {
        Ok(test) => {
            let key = helpers::trim_start_end(test.split(":").collect::<Vec<_>>()[0]);
            routes::files::delete(&Some(key.to_string()));
        }
        Err(_) => println!("{}", "Aborting...".white()),
    }
}
