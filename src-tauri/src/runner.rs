use crate::logging::{init_logger, INSTANCE};
use anyhow::Context as _;

pub fn run_tauri() -> anyhow::Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            if INSTANCE.set(init_logger(app)?).is_err() {
                Err(anyhow::anyhow!("Couldn't init logger"))?
            };
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::cmd::convert_dar2oar,
            crate::cmd::remove_oar_dir,
            crate::cmd::restore_dar_dir,
        ])
        .run(tauri::generate_context!())
        .context("Failed to execute tauri")
}
