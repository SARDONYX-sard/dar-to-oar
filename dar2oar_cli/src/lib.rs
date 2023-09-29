use clap::{arg, Parser};
use dar2oar_core::{convert_dar_to_oar, read_mapping_table};
use std::path::PathBuf;

/// dar2oar --src "DAR path" --src "OAR path" --name "" --author ""
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    /// DAR source dir path
    #[clap(long, value_parser)]
    src: String,
    /// OAR destination dir path(If not, it is inferred from src)
    #[clap(long, value_parser)]
    dist: Option<String>,
    /// mod name in config.json & folder name(If not, it is inferred from src)
    #[arg(long)]
    name: Option<String>,
    /// mod author in config.json
    #[arg(long)]
    author: Option<String>,
    /// path to section name table
    #[arg(long)]
    mapping_file: Option<String>,
}

pub fn run_cli(args: Args) -> anyhow::Result<()> {
    let dist: Option<PathBuf> = args.dist.map(|dist| PathBuf::from(&dist));

    let table = match args.mapping_file {
        Some(table_path) => {
            let mapping = read_mapping_table(table_path)?;
            Some(mapping)
        }
        None => None,
    };

    convert_dar_to_oar(
        args.src,
        dist,
        args.name.as_deref(),
        args.author.as_deref(),
        table,
    )?;
    Ok(())
}
