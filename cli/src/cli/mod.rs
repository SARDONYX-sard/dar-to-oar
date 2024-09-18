#[cfg(feature = "color")]
mod color;
mod convert;

use crate::logger::LogLevel;
use convert::dar2oar;
use dar2oar_core::{remove_oar, unhide_dar, Closure};
use std::path::PathBuf;

pub(crate) async fn run_cli(args: Cli) -> anyhow::Result<()> {
    crate::logger::init(args.log_file, args.log_level, args.stdout)?;

    match args.command {
        Commands::Convert(args) => dar2oar(args).await?,
        Commands::UnhideDar(args) => unhide_dar(args.dar_dir, Closure::default).await?,
        Commands::RemoveOar(args) => remove_oar(args.target_path, Closure::default).await?,
    }

    Ok(())
}

/// Converter CLI version
#[derive(Debug, clap::Parser)]
#[clap(name = "dar2oar", about)]
#[cfg_attr(feature = "color", command(styles=color::get_styles()))]
pub(crate) struct Cli {
    #[clap(subcommand)]
    command: Commands,

    // --logger (Global options)
    #[clap(global = true, long, display_order = 100)]
    /// Enable standard output of the log
    pub stdout: bool,

    #[clap(global = true, long, display_order = 101)]
    #[clap(ignore_case = true, default_value = "error")]
    /// Log level to be recorded in logger
    pub log_level: LogLevel,

    #[clap(global = true, long, display_order = 102)]
    #[clap(default_value = "./convert.log")]
    /// Output path of log file
    pub log_file: Option<PathBuf>,
}

#[derive(Debug, clap::Parser)]
#[clap(version, about)]
enum Commands {
    /// Convert DAR to OAR
    #[clap(arg_required_else_help = true)]
    Convert(convert::Convert),

    #[clap(arg_required_else_help = true)]
    /// Unhide all files in the `DynamicAnimationReplacer` directory
    /// by removing the `mohidden` extension
    UnhideDar(UnhideDarOption),

    #[clap(arg_required_else_help = true)]
    /// Find and delete `OpenAnimationReplacer` directory
    RemoveOar(RemoveOarOption),
}

#[derive(Debug, clap::Args)]
struct UnhideDarOption {
    #[clap(value_parser)]
    /// DAR directory containing files with ".mohidden" extension
    pub dar_dir: String,
}

#[derive(Debug, clap::Args)]
struct RemoveOarOption {
    #[clap(value_parser)]
    /// Path containing the "OpenAnimationReplacer" directory
    pub target_path: String,
}
