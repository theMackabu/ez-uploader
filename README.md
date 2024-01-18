<p align="center"><img style="width: 150px;" src="https://r2.e-z.host/21c2dfbb-4d82-4c2a-a45e-3dee4795515c/hedta24p.png"></p>

## Introduction

This is a CLI tool for interacting directly with the [e-z.host](https://ez.gg) API. (from the ✨*command line*✨)

### Building

- Clone the project `git clone https://git.themackabu.dev/ez-uploader`
- Open a terminal in the project folder
- Check if you have cargo (Rust's package manager) installed, just type in `cargo`
- If cargo is installed, run `cargo build --release`
- Put the executable into one of your PATH entries
  - Linux: usually /bin/ or /usr/bin/
  - Windows: C:\Windows\System32 is good for it but don't use windows

## Usage

```
Commands:
  login    Save your access key
  logout   Remove your access key
  upload   Upload images, videos, gifs and audio
  shorten  Shorten urls
  help     Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
  -V, --version     Print version
```

### Upload images, videos, gifs and audio

```
Usage: ez upload [OPTIONS] <FILE>

Arguments:
  <FILE>  The file you want to upload

Options:
  -d, --domain <DOMAIN>  Override domain to be used when uploading
  -r, --random           Toggle the use of your selected random domains
  -i, --invisible        Toggle the use of invisible characters in filenames
  -e, --emoji            Toggle the use of emojis in filenames
  -s, --sus              Toggle the use of among us characters in filenames
  -c, --custom           Toggle the use of custom characters in filenames
```

### Shorten urls

```
Usage: ez shorten [OPTIONS] <URL>

Arguments:
  <URL>  The url you want to shorten

Options:
  -d, --domain <DOMAIN>  Override domain to be used when uploading
  -l, --longurl          Toggle between 8 and 18 character URLs
```
