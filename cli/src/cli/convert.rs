use dar2oar_core::{convert_dar_to_oar, error::Result, get_mapping_table, Closure, ConvertOptions};
use std::path::PathBuf;

pub(crate) async fn dar2oar(args: Convert) -> Result<()> {
    let config = ConvertOptions {
        dar_dir: args.source,
        oar_dir: args.destination,
        mod_name: args.name,
        author: args.author,
        section_table: get_mapping_table(args.mapping_file).await,
        section_1person_table: get_mapping_table(args.mapping_1person_file).await,
        run_parallel: args.run_parallel,
        hide_dar: args.hide_dar,
    };
    convert_dar_to_oar(config, Closure::default).await
}

#[derive(Debug, clap::Args)]
pub(crate) struct Convert {
    #[clap(value_parser)]
    /// Path containing the "DynamicAnimationReplacer" directory
    source: String,
    #[clap(long)]
    /// "OpenAnimationReplacer" directory output destination (if none, inferred from DAR path)
    destination: Option<String>,
    #[clap(long)]
    /// Mod name in config.json & directory name (if none, inferred from DAR path)
    name: Option<String>,
    #[clap(long)]
    /// Mod author in config.json
    author: Option<String>,
    #[clap(long)]
    /// Path to section name table
    ///
    /// - See more details
    ///   `https://github.com/SARDONYX-sard/dar-to-oar/wiki#what-is-the-mapping-file`
    mapping_file: Option<PathBuf>,
    #[clap(long)]
    /// Path to section name table(For _1st_person)
    mapping_1person_file: Option<PathBuf>,
    #[clap(long)]
    /// Use multi thread
    ///
    /// # Note
    ///
    /// More than twice the processing speed can be expected,
    /// but the concurrent processing results in thread termination timings being out of order,
    /// so log writes will be out of order as well, greatly reducing readability of the logs.
    run_parallel: bool,
    #[clap(long)]
    /// After conversion, add ".mohidden" to all DAR files to hide them(For MO2 user)
    hide_dar: bool,
}
