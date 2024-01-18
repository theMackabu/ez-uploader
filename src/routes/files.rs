use crate::{error, ok};
use colored::Colorize;
use global_placeholders::global;

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

pub fn upload(file_name: &String, domain: &Option<String>, random: &bool, invisible: &bool, emoji: &bool, amongus: &bool, custom: &bool) {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let user_agent = format!("[rust] EZ uploader v{}", env!("CARGO_PKG_VERSION"));
    let parse_header = |val: String| HeaderValue::from_str(&val).unwrap();

    let form = match Form::new().file("file", &file_name) {
        Ok(form) => form,
        Err(err) => {
            log::error!("{err}");
            error!("Unable to upload file, does it exist?");
        }
    };

    match std::fs::read_to_string(global!("ez.token")) {
        Ok(key) => headers.insert("key", parse_header(key)),
        Err(err) => {
            log::error!("{err}");
            error!("Unable to upload file, please add access key with 'ez login'");
        }
    };

    if let Some(domain) = domain {
        headers.insert("domain", parse_header(domain.clone()));
    }

    headers.insert("randomdomain", parse_header(random.to_string()));
    headers.insert("invisibleurl", parse_header(invisible.to_string()));
    headers.insert("EmojiURL", parse_header(emoji.to_string()));
    headers.insert("amongusUrl", parse_header(amongus.to_string()));
    headers.insert("customurl", parse_header(custom.to_string()));

    match client.post("https://api.e-z.host/files").multipart(form).header("user-agent", user_agent).headers(headers).send() {
        Ok(res) => match res.json::<Response>() {
            Ok(json) => {
                if !json.success {
                    error!("Unable to upload file, server returned failure!");
                } else {
                    ok!(format!("uploaded file `{}`\n", file_name));
                    println!("\x08{} {}", "ℹ".magenta(), format!("url: {}", json.imageUrl).bright_magenta());
                    println!("\x08{} {}", "ℹ".yellow(), format!("delete: {}", json.deletionUrl).bright_yellow());
                }
            }
            Err(err) => error!(format!("unable to upload file: {err}")),
        },
        Err(err) => error!(format!("unable to upload file: {err}")),
    };
}
