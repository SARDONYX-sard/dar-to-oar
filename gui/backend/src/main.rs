// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod runner;
mod cmd;
mod convert_option;
mod logging;

fn main() -> std::io::Result<()> {
    crate::runner::run().map_err(|err| {
        tracing::error!("Error: {}", err);
        std::process::exit(1);
    })
}
