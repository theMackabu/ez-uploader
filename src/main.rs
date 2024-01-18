mod cache;
mod globals;
mod helpers;
mod routes;

use clap::{CommandFactory, Parser, Subcommand};
use clap_verbosity_flag::{LogLevel, Verbosity};

#[derive(Copy, Clone, Debug, Default)]
struct NoneLevel;
impl LogLevel for NoneLevel {
    fn default() -> Option<log::Level> { None }
}

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[clap(flatten)]
    verbose: Verbosity<NoneLevel>,
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
        file: String,
        /// Override domain to be used when uploading
        #[arg(short, long)]
        domain: Option<String>,
        /// Toggle the use of your selected random domains
        #[arg(short, long)]
        random: bool,
        /// Toggle the use of invisible characters in filenames
        #[arg(short, long)]
        invisible: bool,
        /// Toggle the use of emojis in filenames
        #[arg(short, long)]
        emoji: bool,
        /// Toggle the use of among us characters in filenames
        #[arg(short, long)]
        sus: bool,
        /// Toggle the use of custom characters in filenames
        #[arg(short, long)]
        custom: bool,
    },
    /// Shorten urls
    Shorten {
        /// The url you want to shorten
        #[command()]
        url: String,
        /// Override domain to be used when uploading
        #[arg(short, long)]
        domain: Option<String>,
        /// Toggle between 8 and 18 character URLs
        #[arg(short, long)]
        longurl: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut env = env_logger::Builder::new();
    let level = cli.verbose.log_level_filter();

    globals::init();
    env.filter_level(level).init();

    match &cli.command {
        Some(Commands::Upload {
            file,
            domain,
            random,
            invisible,
            emoji,
            sus,
            custom,
        }) => routes::files::upload(file, domain, random, invisible, emoji, sus, custom),

        Some(Commands::Shorten { url, domain, longurl }) => routes::shorten::create(url, domain, longurl),
        Some(Commands::Login) => routes::auth::login(),
        Some(Commands::Logout) => routes::auth::logout(),
        None => Cli::command().print_help().unwrap(),
    }
}
