#![allow(dead_code)]
use std::{fs, path::Path};

pub struct Exists<'p> {
    path: &'p str,
}

impl<'p> Exists<'p> {
    pub fn check(path: &'p str) -> Self { Self { path } }
    pub fn folder(&self) -> bool { Path::new(self.path).is_dir() }
    pub fn file(&self) -> bool { Path::new(self.path).exists() }
    pub fn empty(&self) -> bool { fs::metadata(Path::new(self.path)).map(|m| m.len() == 0).unwrap_or(true) }
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
