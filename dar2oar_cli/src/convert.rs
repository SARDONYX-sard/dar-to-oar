use dar2oar_core::{convert_dar_to_oar, get_mapping_table, Closure, ConvertOptions};

pub(crate) async fn dar2oar(args: Convert) -> anyhow::Result<()> {
    let config = ConvertOptions {
        dar_dir: args.src,
        oar_dir: args.dist,
        mod_name: args.name,
        author: args.author,
        section_table: get_mapping_table(args.mapping_file).await,
        section_1person_table: get_mapping_table(args.mapping_1person_file).await,
        run_parallel: args.run_parallel,
        hide_dar: args.hide_dar,
    };
    convert_dar_to_oar(config, Closure::default).await?;
    Ok(())
}

#[derive(Debug, clap::Args)]
pub(crate) struct Convert {
    #[clap(value_parser)]
    /// DAR source dir path
    src: String,
    #[clap(long)]
    /// OAR destination dir path(If not, it is inferred from DAR path)
    dist: Option<String>,
    #[clap(long)]
    /// Mod name in config.json & directory name(If not, it is inferred from DAR path)
    name: Option<String>,
    #[clap(long)]
    /// Mod author in config.json
    author: Option<String>,
    #[clap(long)]
    /// Path to section name table
    ///
    /// - See more details
    /// https://github.com/SARDONYX-sard/dar-to-oar/wiki#what-is-the-mapping-file
    mapping_file: Option<String>,
    #[clap(long)]
    /// Path to section name table(For _1st_person)
    mapping_1person_file: Option<String>,
    #[clap(long)]
    /// Use multi thread
    ///
    /// [Note]
    /// More than twice the processing speed can be expected,
    /// but the concurrent processing results in thread termination timings being out of order,
    /// so log writes will be out of order as well, greatly reducing readability of the logs.
    run_parallel: bool,
    #[clap(long)]
    /// After conversion, add ".mohidden" to all DAR files to hide them(For MO2 user)
    hide_dar: bool,

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
