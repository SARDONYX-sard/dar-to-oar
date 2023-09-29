// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmd;
mod runner;

use crate::runner::run_tauri;

fn main() -> std::io::Result<()> {
    match run_tauri() {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}
