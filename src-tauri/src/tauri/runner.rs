use anyhow::Context;
use tauri_plugin_log::LogTarget;

pub fn run_tauri() -> anyhow::Result<()> {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![crate::tauri::cmd::convert_dar2oar])
        .run(tauri::generate_context!())
        .context("error while running tauri application")
}
