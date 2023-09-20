use crate::converter::convert_dar_to_oar;
use clap::{arg, command, Parser, Subcommand};
use std::path::Path;

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
    /// source dir path
    #[clap(value_parser)]
    dar_mod_folder: String,
    /// destination dir path
    #[clap(value_parser)]
    oar_mod_folder: String,
    /// mod name
    #[arg(long)]
    mod_name: Option<String>,
    /// Keeps local symbols (e.g., those starting with `.L`
    #[arg(long)]
    mod_author: Option<String>,
}

pub(crate) fn run_cli(args: Commands) {
    match args {
        Commands::Cli(sub_args) => {
            convert_dar_to_oar(
                &Path::new(&sub_args.dar_mod_folder),
                &Path::new(&sub_args.oar_mod_folder),
                sub_args.mod_name.as_deref(),
                sub_args.mod_author.as_deref(),
            )
            .unwrap_or_else(|err| eprintln!("{err}"));
        }
    }
}
