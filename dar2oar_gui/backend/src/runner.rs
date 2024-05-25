use crate::logging::init_logger;
use anyhow::Context as _;
use tauri::{
    window::{Color, Effect, EffectState, EffectsBuilder},
    Manager,
};

pub fn run_tauri() -> anyhow::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                window.set_effects(
                    EffectsBuilder::new()
                        .radius(5.)
                        .color(Color(255, 255, 0, 0))
                        .build(),
                )?;
            }
            Ok(init_logger(app)?)
        })
        .invoke_handler(tauri::generate_handler![
            crate::cmd::change_log_level,
            crate::cmd::convert_dar2oar,
            crate::cmd::convert_dar2oar_with_progress,
            crate::cmd::write_file,
            crate::cmd::remove_oar_dir,
            crate::cmd::unhide_dar_dir,
        ])
        .run(tauri::generate_context!())
        .context("Failed to execute tauri")
}
