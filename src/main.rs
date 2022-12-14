mod auth;
mod files;
mod helpers;

use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Save your access key
    Login,
    /// Remove your access key
    Logout,
    /// Upload images, videos, gifs and audio
    Upload {
        /// The file you want to upload
        #[command()]
        file_name: String,
        /// Domain to be used when uploading
        #[arg(short, long, default_value_t = String::from("i.e-z.host"))]
        domain: String,
        /// Whether random domain is used
        #[arg(short, long, default_value_t = false)]
        random: bool,
        /// Whether invisible url is used
        #[arg(short, long, default_value_t = false)]
        invisible: bool,
        /// Whether emoji url is used
        #[arg(short, long, default_value_t = false)]
        emoji: bool,
        /// Whether amongus url is used
        #[arg(short, long, default_value_t = false)]
        amongus: bool,
        /// Whether custom url is used
        #[arg(short, long, default_value_t = false)]
        custom: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Upload {
            file_name,
            domain,
            random,
            invisible,
            emoji,
            amongus,
            custom,
        }) => files::upload(file_name, domain, random, invisible, emoji, amongus, custom),
        Some(Commands::Login) => auth::login(),
        Some(Commands::Logout) => auth::logout(),
        None => Cli::command().print_help().unwrap(),
    }
}
