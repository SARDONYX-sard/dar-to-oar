use clap::{arg, Parser};
use dar2oar_core::{
    convert_dar_to_oar,
    fs::{async_closure::AsyncClosure, parallel, ConvertOptions},
    read_mapping_table,
};
use std::fs::File;
use std::{path::PathBuf, str::FromStr};
use tracing::Level;

/// dar2oar --src "DAR path" --dist "OAR path"
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    /// DAR source dir path
    #[clap(long, value_parser)]
    src: String,
    /// OAR destination dir path(If not, it is inferred from src)
    #[clap(long, value_parser)]
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

pub async fn run_cli(args: Args) -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(File::create(&args.log_path)?)
        .with_max_level(Level::from_str(&args.log_level).unwrap_or(Level::ERROR))
        .init();

    macro_rules! read_table {
        ($path:expr) => {
            match $path {
                Some(table_path) => {
                    let mapping = read_mapping_table(table_path).await?;
                    Some(mapping)
                }
                None => None,
            }
        };
    }

    let config = ConvertOptions {
        dar_dir: args.src,
        oar_dir: args.dist.map(|dist| PathBuf::from(&dist)),
        mod_name: args.name.as_deref(),
        author: args.author.as_deref(),
        section_table: read_table!(args.mapping_file),
        section_1person_table: read_table!(args.mapping_1person_file),
        hide_dar: args.hide_dar,
    };

    let res = match args.run_parallel {
        true => parallel::convert_dar_to_oar(config, AsyncClosure::default).await,
        false => convert_dar_to_oar(config, AsyncClosure::default).await,
    };

    match res {
        Ok(msg) => {
            tracing::info!("{}", msg);
            Ok(())
        }
        Err(err) => {
            tracing::error!("{}", err);
            anyhow::bail!("{}", err)
        }
    }
}
