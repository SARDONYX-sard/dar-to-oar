use crate::converter::convert_dar_to_oar;
use clap::{arg, command, Parser, Subcommand};
use std::path::Path;

/// dar2oar --src "DAR path" --src "OAR path" --name "" --author ""
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    #[command(about = "run with cli mode")]
    Cli(SubArgs),
}

#[derive(Debug, Parser)]
pub(crate) struct SubArgs {
    /// DAR source dir path
    #[clap(long, value_parser)]
    src: String,
    /// OAR destination dir path
    #[clap(long, value_parser)]
    dist: String,
    /// mod name in config.json & folder name
    /// - If not, it is extracted from the mod name in src.
    #[arg(long)]
    name: Option<String>,
    /// mod author in config.json
    #[arg(long)]
    author: Option<String>,
}

pub(crate) fn run_cli(args: Commands) {
    match args {
        Commands::Cli(sub_args) => {
            convert_dar_to_oar(
                &Path::new(&sub_args.src),
                &Path::new(&sub_args.dist),
                sub_args.name.as_deref(),
                sub_args.author.as_deref(),
            )
            .unwrap_or_else(|err| eprintln!("{err}"));
        }
    }
}
