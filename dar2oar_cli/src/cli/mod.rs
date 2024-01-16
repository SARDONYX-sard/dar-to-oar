mod commands;

use crate::cli::commands::Commands;
use crate::convert::dar2oar;
use crate::init_tracing;
use dar2oar_core::{remove_oar, unhide_dar, Closure};
use std::str::FromStr;
use tracing::Level;

/// Converter CLI version
#[derive(Debug, clap::Parser)]
#[clap(name = "dar2oar", about)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

macro_rules! init_logger {
    ($args:ident) => {
        init_tracing(
            &$args.log_path,
            Level::from_str(&$args.log_level).unwrap_or(Level::ERROR),
            $args.stdout,
        )?;
    };
}

pub(crate) async fn run_cli(args: Cli) -> anyhow::Result<()> {
    match args.command {
        Commands::Convert(args) => {
            init_logger!(args);
            dar2oar(args).await?;
        }
        Commands::UnhideDar(args) => {
            init_logger!(args);
            unhide_dar(args.dar_dir, Closure::default).await?;
        }
        Commands::RemoveOar(args) => {
            init_logger!(args);
            remove_oar(args.target_path, Closure::default).await?;
        }
    }

    Ok(())
}
