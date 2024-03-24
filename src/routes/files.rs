use arboard::Clipboard;
use colored::Colorize;
use global_placeholders::global;
use image::RgbaImage;

use crate::{
    cache, error,
    helpers::{get_filename, USER_AGENT},
    ok,
};

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

#[derive(serde::Deserialize)]
struct DeleteResponse {
    success: bool,
}

pub fn upload(file_name: &String, domain: &Option<String>, random: &bool, invisible: &bool, emoji: &bool, amongus: &bool, custom: &bool) {
    let client = Client::new();
    let mut headers = HeaderMap::new();
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

    match client.post("https://api.e-z.host/files").multipart(form).header("user-agent", *USER_AGENT).headers(headers).send() {
        Ok(res) => match res.json::<Response>() {
            Ok(json) => {
                if !json.success {
                    error!("Unable to upload file, server returned failure!");
                } else {
                    let fmt_name = get_filename(file_name.clone());

                    cache::save(cache::FileInfo {
                        from: file_name.clone(),
                        to: json.imageUrl.clone(),
                        uploaded: chrono::Utc::now(),
                        delete_link: json.deletionUrl.to_string(),
                        name: cache::NameInfo {
                            local: fmt_name.clone(),
                            server: json.imageUrl.split('/').last().unwrap().to_string(),
                        },
                    });

                    ok!(format!("uploaded file `{}`\n", file_name));
                    println!("\x08{} {}", "ℹ".magenta(), format!("url: {}", json.imageUrl).bright_magenta());
                    println!("{}", format!("Delete with `e-z delete {fmt_name}`").white());
                }
            }
            Err(err) => error!(format!("unable to upload file: {err}")),
        },
        Err(err) => error!(format!("unable to upload file: {err}")),
    };
}

pub fn upload_clipboard(
    domain: &Option<String>,
    random: &bool,
    invisible: &bool,
    emoji: &bool,
    amongus: &bool,
    custom: &bool,
) {
    let mut clipboard = match Clipboard::new() {
        Ok(clipboard) => clipboard,
        Err(e) => error!(format!("Could not read from clipboard: {e}")),
    };

    let clipboard_image = match clipboard.get_image() {
        Ok(image) => image,
        Err(e) => error!(format!("Could not get image from clipboard: {e}")),
    };

    // This will break if someone has an image with more than 4 million pixels in height
    // but I don't particularly care
    let image_final = RgbaImage::from_raw(
        clipboard_image.width as u32,
        clipboard_image.height as u32,
        clipboard_image.bytes.into_owned(),
    )
    .expect("Buffer size invariant should be held");

    let image_file = match tempfile::Builder::new().suffix(".png").tempfile() {
        Ok(file) => file,
        Err(e) => error!(format!("Could not make file to store clipboard image: {e}")),
    };

    if let Err(e) = image_final.save(image_file.path()) {
        error!(format!("Could not save clipboard image to temporary file: {e}"));
    }

    upload(
        &image_file
            .path()
            .to_str()
            .expect("tempfile crate should only create valid utf-8 filenames")
            .to_owned(),
        domain,
        random,
        invisible,
        emoji,
        amongus,
        custom,
    )
}

pub fn delete(file_name: &Option<String>) {
    match file_name {
        Some(file_name) => {
            let client = Client::new();

            let data = match cache::delete(file_name.clone()) {
                Ok(data) => data,
                Err(err) => {
                    log::error!("{err}");
                    error!("Unable to delete file, was it uploaded?");
                }
            };

            match client.get(data.delete_link).header("user-agent", *USER_AGENT).send() {
                Ok(res) => match res.json::<DeleteResponse>() {
                    Ok(json) => {
                        if !json.success {
                            error!("Unable to delete file, server returned failure!");
                        } else {
                            ok!(format!("Deleted file `{}`", file_name));
                        }
                    }
                    Err(err) => error!(format!("unable to delete file: {err}")),
                },
                Err(err) => error!(format!("unable to delete file: {err}")),
            };
        }
        None => cache::list(),
    }
}
