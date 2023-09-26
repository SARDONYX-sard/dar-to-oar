// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli;
mod converter;
mod tauri;

use crate::cli::{run_cli, Cli};
use clap::Parser as _;
use tauri::runner::run_tauri;

fn main() -> anyhow::Result<()> {
    match Cli::parse().command {
        Some(args) => run_cli(args),
        None => run_tauri(),
    }
}
