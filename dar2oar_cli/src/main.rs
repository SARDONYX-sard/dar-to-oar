use clap::Parser as _;
use dar2oar_cli::{run_cli, Args};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run_cli(Args::parse()).await
}
