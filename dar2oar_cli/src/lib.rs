use clap::{arg, Parser};
use dar2oar_core::{convert_dar_to_oar, fs::parallel, read_mapping_table};
use std::path::PathBuf;

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
    /// mod name in config.json & folder name(If not, it is inferred from src)
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
    run_parallel: Option<bool>,
}

pub fn run_cli(args: Args) -> anyhow::Result<()> {
    let config = simple_log::LogConfigBuilder::builder()
        .path(args.log_path)
        .size(100)
        .roll_count(10)
        .level(args.log_level)
        .output_file()
        .output_console()
        .build();
    simple_log::new(config).unwrap();

    let dist: Option<PathBuf> = args.dist.map(|dist| PathBuf::from(&dist));

    let table = match args.mapping_file {
        Some(table_path) => {
            let mapping = read_mapping_table(table_path)?;
            Some(mapping)
        }
        None => None,
    };

    let table_1person = match args.mapping_1person_file {
        Some(table_path) => {
            let mapping = read_mapping_table(table_path)?;
            Some(mapping)
        }
        None => None,
    };

    match args.run_parallel {
        Some(true) => parallel::convert_dar_to_oar(
            args.src,
            dist,
            args.name.as_deref(),
            args.author.as_deref(),
            table,
            table_1person,
        ),
        Some(false) | None => convert_dar_to_oar(
            args.src,
            dist,
            args.name.as_deref(),
            args.author.as_deref(),
            table,
            table_1person,
        ),
    }
}
