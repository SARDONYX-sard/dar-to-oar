// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmd;
mod logging;
mod runner;

use crate::runner::run_tauri;

fn main() -> std::io::Result<()> {
    run_tauri().map_err(|err| {
        log::error!("Error: {}", err);
        std::process::exit(1);
    })
}
