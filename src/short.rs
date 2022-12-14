use crate::helpers::{bool_to_str, string_to_str};
use colored::Colorize;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};

#[derive(Debug, serde::Deserialize)]
#[allow(non_snake_case)]
struct Response {
    success: bool,
    shortendUrl: String,
    deletionUrl: String,
}

pub fn create_link(url: &String, domain: &String, longurl: &bool) {
    match home::home_dir() {
        Some(path) => {
            let client = Client::new();
            let mut headers = HeaderMap::new();
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
            headers.insert("longurl", parse_header(bool_to_str(longurl)));

            match client
                .post("https://api.e-z.host/shortener")
                .header("user-agent", user_agent)
                .header(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"))
                .headers(headers)
                .body(format!("{{\"url\":\"{url}\"}}"))
                .send()
            {
                Ok(response) => match serde_json::from_str::<Response>(&response.text().unwrap()) {
                    Ok(json) => {
                        if !json.success {
                            println!("\x08{} {}", "✖".red(), format!("unable to shorten url").bright_red());
                        } else {
                            println!("\x08{} {}\n", "✔".green(), format!("shortened url `{}`", url).bright_green());
                            println!("\x08{} {}", "ℹ".magenta(), format!("url: {}", json.shortendUrl).bright_magenta());
                            println!("\x08{} {}", "ℹ".yellow(), format!("delete: {}", json.deletionUrl).bright_yellow());
                        }
                    }
                    Err(err) => {
                        eprint!("\r{} {}\n", "✖".red(), format!("unable to shorten url: {}", err.to_string()).bright_red());
                        std::process::exit(1);
                    }
                },
                Err(err) => {
                    eprintln!("\r{} {}\n", "✖".red(), format!("unable to shorten url: {}", err.to_string()).bright_red());
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
