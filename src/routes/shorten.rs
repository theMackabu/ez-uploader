use crate::{error, helpers::USER_AGENT, ok};
use colored::Colorize;
use global_placeholders::global;

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};

#[derive(Debug, serde::Serialize)]
struct Request<'s> {
    url: &'s String,
}

#[derive(Debug, serde::Deserialize)]
#[allow(non_snake_case)]
struct Response {
    success: bool,
    shortendUrl: String,
    deletionUrl: String,
}

pub fn create(url: &String, domain: &Option<String>, longurl: &bool) {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let parse_header = |val: String| HeaderValue::from_str(&val).unwrap();

    match std::fs::read_to_string(global!("ez.token")) {
        Ok(key) => headers.insert("key", parse_header(key)),
        Err(err) => {
            log::error!("{err}");
            error!("Unable to shorten url, please add access key with 'ez login'");
        }
    };

    if let Some(domain) = domain {
        headers.insert("domain", parse_header(domain.clone()));
    }

    headers.insert("longurl", parse_header(longurl.to_string()));

    match client
        .post("https://api.e-z.host/shortener")
        .header("user-agent", *USER_AGENT)
        .headers(headers)
        .json(&Request { url })
        .send()
    {
        Ok(res) => match res.json::<Response>() {
            Ok(json) => {
                if !json.success {
                    error!("Unable to shorten url, server returned failure!");
                } else {
                    ok!(format!("shortened url `{url}`\n"));
                    println!("\x08{} {}", "ℹ".magenta(), format!("url: {}", json.shortendUrl).bright_magenta());
                    println!("\x08{} {}", "ℹ".yellow(), format!("delete: {}", json.deletionUrl).bright_yellow());
                }
            }
            Err(err) => error!(format!("Unable to shorten url: {err}")),
        },
        Err(err) => error!(format!("Unable to shorten url: {err}")),
    };
}
