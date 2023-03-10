use crate::helpers::{bool_to_str, string_to_str};
use colored::Colorize;
use reqwest::{
    blocking::{multipart::Form, Client},
    header::{HeaderMap, HeaderValue},
};

#[derive(Debug, serde::Deserialize)]
#[allow(non_snake_case)]
struct Response {
    success: bool,
    imageUrl: String,
    deletionUrl: String,
}

pub fn upload(file_name: &String, domain: &String, random: &bool, invisible: &bool, emoji: &bool, amongus: &bool, custom: &bool) {
    match home::home_dir() {
        Some(path) => {
            let client = Client::new();
            let mut headers = HeaderMap::new();
            let form = Form::new().file("file", &file_name).unwrap();
            let user_agent = format!("[rust] EZ uploader v{}", env!("CARGO_PKG_VERSION"));
            let parse_header = |val: &str| HeaderValue::from_str(val).unwrap();

            match std::fs::read_to_string(format!("{}/.ez/.token", path.display())) {
                Ok(key) => headers.insert("key", parse_header(string_to_str(key))),
                Err(_) => {
                    eprintln!("{} {}", "✖".red(), "unable to upload file, please add access key with 'ez login'".bright_red());
                    std::process::exit(1);
                }
            };

            headers.insert("domain", parse_header(domain));
            headers.insert("randomdomain", parse_header(bool_to_str(random)));
            headers.insert("invisibleurl", parse_header(bool_to_str(invisible)));
            headers.insert("EmojiURL", parse_header(bool_to_str(emoji)));
            headers.insert("amongusUrl", parse_header(bool_to_str(amongus)));
            headers.insert("customurl", parse_header(bool_to_str(custom)));

            match client.post("https://api.e-z.host/files").multipart(form).header("user-agent", user_agent).headers(headers).send() {
                Ok(response) => match serde_json::from_str::<Response>(&response.text().unwrap()) {
                    Ok(json) => {
                        if !json.success {
                            println!("\x08{} {}", "✖".red(), format!("unable to upload file").bright_red());
                        } else {
                            println!("\x08{} {}\n", "✔".green(), format!("uploaded file `{}`", file_name).bright_green());
                            println!("\x08{} {}", "ℹ".magenta(), format!("url: {}", json.imageUrl).bright_magenta());
                            println!("\x08{} {}", "ℹ".yellow(), format!("delete: {}", json.deletionUrl).bright_yellow());
                        }
                    }
                    Err(err) => {
                        eprint!("\r{} {}\n", "✖".red(), format!("unable to upload file: {}", err.to_string()).bright_red());
                        std::process::exit(1);
                    }
                },
                Err(err) => {
                    eprintln!("\r{} {}\n", "✖".red(), format!("unable to upload file: {}", err.to_string()).bright_red());
                    std::process::exit(1);
                }
            };
        }
        None => {
            eprintln!("{} {}", "✖".red(), format!("unable to find home directory").bright_red());
            std::process::exit(1);
        }
    }
}
