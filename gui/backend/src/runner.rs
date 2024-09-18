use crate::logging::init_logger;
use anyhow::Context as _;
use tauri_plugin_window_state::StateFlags;

pub fn run() -> anyhow::Result<()> {
    let builder = tauri::Builder::default();

    #[allow(clippy::large_stack_frames)]
    builder
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(
            // https://github.com/tauri-apps/plugins-workspace/issues/344
            tauri_plugin_window_state::Builder::default()
                .with_state_flags(StateFlags::all() & !StateFlags::VISIBLE)
                .build(),
        )
        .setup(|app| Ok(init_logger(app)?))
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
