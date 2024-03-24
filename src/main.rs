mod cache;
mod globals;
mod helpers;
mod routes;

use clap::{Args, CommandFactory, Parser, Subcommand};
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
    /// Delete a file (last 10 cached)
    Delete {
        /// The file you want to delete
        #[command()]
        file: Option<String>,
    },
    /// Upload images, videos, gifs and audio
    Upload {
        /// The file you want to upload
        #[command()]
        file: String,
        #[command(flatten)]
        params: UploadParams,
    },
    /// Upload image from clipboard
    Clipboard {
        #[command(flatten)]
        params: UploadParams,
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

#[derive(Args)]
struct UploadParams {
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
}

fn main() {
    let cli = Cli::parse();
    let mut env = env_logger::Builder::new();
    let level = cli.verbose.log_level_filter();

    globals::init();
    env.filter_level(level).init();

    match &cli.command {
        // authentication
        Some(Commands::Login) => routes::auth::login(),
        Some(Commands::Logout) => routes::auth::logout(),

        // commands
        Some(Commands::Delete { file }) => routes::files::delete(file),
        Some(Commands::Shorten { url, domain, longurl }) => routes::shorten::create(url, domain, longurl),

        Some(Commands::Upload {
            file,
            params: UploadParams {
                domain,
                random,
                invisible,
                emoji,
                sus,
                custom,
            }
        }) => routes::files::upload(file, domain, random, invisible, emoji, sus, custom),

        Some(Commands::Clipboard {
            params: UploadParams {
                domain,
                random,
                invisible,
                emoji,
                sus,
                custom }
        }) => routes::files::upload_clipboard(domain, random, invisible, emoji, sus, custom),

        None => Cli::command().print_help().unwrap(),
    }
}
