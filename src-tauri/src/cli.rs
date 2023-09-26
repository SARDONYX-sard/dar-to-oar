use crate::converter::{convert_dar_to_oar, read_mapping_table};
use clap::{arg, command, Parser, Subcommand};
use std::path::PathBuf;

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
    dist: Option<String>,
    /// mod name in config.json & folder name
    /// - If not, it is extracted from the mod name in src.
    #[arg(long)]
    name: Option<String>,
    /// mod author in config.json
    #[arg(long)]
    author: Option<String>,
    /// path to section name table
    #[arg(long)]
    mapping_file: Option<String>,
}

pub(crate) fn run_cli(args: Commands) -> anyhow::Result<()> {
    match args {
        Commands::Cli(sub_args) => {
            let dist: Option<PathBuf> = sub_args.dist.map(|dist| PathBuf::from(&dist));

            let table = match sub_args.mapping_file {
                Some(table_path) => {
                    let mapping = read_mapping_table(table_path)?;
                    Some(mapping)
                }
                None => None,
            };

            convert_dar_to_oar(
                sub_args.src,
                dist,
                sub_args.name.as_deref(),
                sub_args.author.as_deref(),
                table,
            )?;
            Ok(())
        }
    }
}
