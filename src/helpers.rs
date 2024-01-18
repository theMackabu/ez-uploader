#![allow(dead_code)]
use macros_rs::fmtstr;
use once_cell::sync::Lazy;

use std::{
    fs,
    path::{Path, PathBuf},
};

pub const USER_AGENT: Lazy<&str> = Lazy::new(|| fmtstr!("[rust] EZ uploader v{}", env!("CARGO_PKG_VERSION")));

pub struct Exists<'p> {
    path: &'p str,
}

impl<'p> Exists<'p> {
    pub fn check(path: &'p str) -> Self { Self { path } }
    pub fn folder(&self) -> bool { Path::new(self.path).is_dir() }
    pub fn file(&self) -> bool { Path::new(self.path).exists() }
    pub fn empty(&self) -> bool { fs::metadata(Path::new(self.path)).map(|m| m.len() == 0).unwrap_or(true) }
}

pub fn get_filename(name: String) -> String {
    let path_buf = PathBuf::from(name);
    path_buf.file_name().map(|name| name.to_string_lossy().to_string()).unwrap()
}

pub fn trim_start_end(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

#[macro_export]
macro_rules! ok {
    ($string:expr) => {{
        print!("{} ", "✔".green());
        println!("{}", $string.bright_green());
    }};
}

#[macro_export]
macro_rules! error {
    ($string:expr) => {{
        print!("{} ", "✖".red());
        println!("{}", $string.bright_red());
        std::process::exit(1);
    }};
}
