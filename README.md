<p align="center"><img style="width: 150px;" src="https://cdn.e-z.host/e-zimagehosting/eztransparent.png"></p>

## Introduction

This is a CLI tool for interacting directly with the `e-z.host` API. (from the ✨*command line*✨)

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
login            | Save your access key
logout           | Remove your access key
ez upload <path> | Upload images, videos, gifs and audio
Options:
  -d, --domain <DOMAIN>  Domain to be used when uploading [default: i.e-z.host]
  -r, --random           Whether random domain is used
  -i, --invisible        Whether invisible url is used
  -e, --emoji            Whether emoji url is used
  -a, --amongus          Whether amongus url is used
  -c, --custom           Whether custom url is used
ez shorten <url> | Shorten urls
Options:
  -d, --domain <DOMAIN>  Domain to be used when shortened [default: astolfo.host]
  -l, --longurl          Whether long url is used
ez --help        | Displays the usage of this cli
```
