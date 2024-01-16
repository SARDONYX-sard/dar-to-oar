use crate::convert::Convert;
use clap::Args;

#[derive(Debug, clap::Parser)]
#[clap(version, about)]
pub(crate) enum Commands {
    /// Convert DAR to OAR
    #[clap(arg_required_else_help = true)]
    Convert(Convert),

    #[clap(arg_required_else_help = true)]
    /// Unhide all files in the `DynamicAnimationReplacer` directory
    /// by removing the `mohidden` extension
    UnhideDar(UnhideDarOption),

    #[clap(arg_required_else_help = true)]
    /// Find and delete `OpenAnimationReplacer` directory
    RemoveOar(RemoveOarOption),
}

#[derive(Debug, Args)]
pub(super) struct UnhideDarOption {
    #[clap(value_parser)]
    /// DAR directory containing files with ".mohidden" extension
    pub dar_dir: String,

    // ---logger
    #[clap(long)]
    /// Log output to stdout as well
    pub stdout: bool,
    #[clap(long, default_value = "error")]
    /// Log level
    ///
    /// trace | debug | info | warn | error
    pub log_level: String,
    #[clap(long, default_value = "./convert.log")]
    /// Output path of log file
    pub log_path: String,
}

#[derive(Debug, Args)]
pub(super) struct RemoveOarOption {
    #[clap(value_parser)]
    /// Path containing the "OpenAnimationReplacer" directory
    pub target_path: String,

    // ---logger
    #[clap(long)]
    /// Log output to stdout as well
    pub stdout: bool,
    #[clap(long, default_value = "error")]
    /// Log level
    ///
    /// trace | debug | info | warn | error
    pub log_level: String,
    #[clap(long, default_value = "./convert.log")]
    /// Output path of log file
    pub log_path: String,
}
