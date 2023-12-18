use clap::{arg, Parser};
use dar2oar_core::{convert_dar_to_oar, get_mapping_table, Closure, ConvertOptions};
use std::fs::File;
use std::str::FromStr;
use tokio::time::Instant;
use tracing::Level;

/// dar2oar --src "DAR path" --dist "OAR path"
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    /// DAR source dir path
    #[arg(long)]
    src: String,
    /// OAR destination dir path(If not, it is inferred from src)
    #[arg(long)]
    dist: Option<String>,
    /// mod name in config.json & directory name(If not, it is inferred from src)
    #[arg(long)]
    name: Option<String>,
    /// mod author in config.json
    #[arg(long)]
    author: Option<String>,
    /// path to section name table
    #[arg(long)]
    mapping_file: Option<String>,
    /// path to section name table(For _1st_person)
    #[arg(long)]
    mapping_1person_file: Option<String>,
    /// log_level trace | debug | info | warn | error
    #[arg(long, default_value = "error")]
    log_level: String,
    /// Output path of log file
    #[arg(long, default_value = "./convert.log")]
    log_path: String,
    /// use multi thread(Probably effective for those with long DAR syntax. Basically single-threaded is faster.)
    #[arg(long)]
    run_parallel: bool,
    #[arg(long)]
    /// After converting to OAR, add mohidden to the DAR directory before conversion to treat it as a hidden directory. (for MO2 users)
    hide_dar: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    let args = Args::parse();
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(File::create(&args.log_path)?)
        .with_max_level(Level::from_str(&args.log_level).unwrap_or(Level::ERROR))
        .init();

    let config = ConvertOptions {
        dar_dir: args.src,
        oar_dir: args.dist,
        mod_name: args.name.as_deref(),
        author: args.author.as_deref(),
        section_table: get_mapping_table(args.mapping_file).await,
        section_1person_table: get_mapping_table(args.mapping_1person_file).await,
        run_parallel: args.run_parallel,
        hide_dar: args.hide_dar,
    };

    match convert_dar_to_oar(config, Closure::default).await {
        Ok(msg) => {
            tracing::info!("{}", msg);
            let elapsed = start.elapsed();
            tracing::info!(
                "Conversion time: {}.{}secs.",
                elapsed.as_secs(),
                elapsed.subsec_millis()
            );
            Ok(())
        }
        Err(err) => {
            tracing::error!("{}", err);
            anyhow::bail!("{}", err)
        }
    }
}
