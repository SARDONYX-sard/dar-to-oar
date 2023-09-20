// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::cli::{run_cli, Cli};
use clap::Parser;
use tauri_plugin_log::LogTarget;

mod cli;
mod converter;
mod tauri_cmd;

fn main() {
    match Cli::parse().command {
        Some(args) => run_cli(args),
        None => run_tauri(),
    }
}

fn run_tauri() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![tauri_cmd::convert_dar2oar])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
