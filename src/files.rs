use crate::helpers::bool_to_str;
use colored::Colorize;
use reqwest::{
    blocking::{multipart::Form, Client},
    header::{HeaderMap, HeaderValue},
};

pub fn upload(file_name: &String, domain: &String, random: &bool, invisible: &bool, emoji: &bool, amongus: &bool, custom: &bool) {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let form = Form::new().file("file", &file_name).unwrap();
    let user_agent = format!("[rust] EZ uploader v{}", env!("CARGO_PKG_VERSION"));
    let parse_header = |val: &str| HeaderValue::from_str(val).unwrap();

    headers.insert("key", parse_header("<TODO: READ FILE>"));
    headers.insert("domain", parse_header(domain));
    headers.insert("randomdomain", parse_header(bool_to_str(random)));
    headers.insert("invisibleurl", parse_header(bool_to_str(invisible)));
    headers.insert("EmojiURL", parse_header(bool_to_str(emoji)));
    headers.insert("amongusUrl", parse_header(bool_to_str(amongus)));
    headers.insert("customurl", parse_header(bool_to_str(custom)));

    match client.post("https://api.e-z.host/files").multipart(form).header("user-agent", user_agent).headers(headers).send() {
        Ok(response) => println!("{}", &response.text().unwrap()),
        Err(err) => {
            eprintln!("\r{} {}\n", "✖".red(), format!("unable to upload file: {}", err.to_string()).bright_red());
            std::process::exit(1);
        }
    };
}
