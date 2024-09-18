mod cli;
mod logger;

use crate::cli::{run_cli, Cli};
use clap::Parser;
use std::process::exit;
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    let start = Instant::now();

    #[allow(clippy::print_stderr)]
    match run_cli(Cli::parse()).await {
        Ok(()) => {
            let elapsed = start.elapsed();
            let time = (elapsed.as_secs(), elapsed.subsec_millis());
            tracing::info!("Elapsed time: {}.{}secs.", time.0, time.1);
            exit(0);
        }
        Err(err) => {
            tracing::error!("{err}");
            eprintln!("{err}");
            exit(1);
        }
    }
}
