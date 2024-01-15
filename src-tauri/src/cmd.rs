use crate::convert_option::{AsyncFrom, GuiConverterOptions};
use dar2oar_core::{convert_dar_to_oar, remove_oar, unhide_dar, Closure, ConvertOptions};
use std::time::Instant;
use tauri::Window;

/// Early return with Err() and write log error.
macro_rules! bail {
    ($err:expr) => {{
        tracing::error!("{}", $err);
        return Err($err.to_string());
    }};
}

/// Measure the elapsed time and return the result of the given asynchronous function.
macro_rules! time {
    ($name:literal, $expr:expr) => {{
        let start = Instant::now();
        let res = $expr.await.or_else(|err| bail!(err));
        let elapsed = start.elapsed();
        tracing::info!(
            "{} time: {}.{}secs.",
            $name,
            elapsed.as_secs(),
            elapsed.subsec_millis()
        );
        res
    }};
}

/// # Progress report for progress bar
///
/// - First: number of files/dirs explored
/// - After: working index
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Payload {
    /// - First: number of files/dirs explored
    /// - After: working index
    index: usize,
}

/// Closure that reports the number of files
macro_rules! sender {
    ($window:ident) => {
        move |index: usize| {
            if let Err(err) = $window.emit("show-progress", Payload { index }) {
                tracing::error!("{}", err);
            };
        }
    };
}

#[tauri::command]
pub(crate) async fn convert_dar2oar(options: GuiConverterOptions) -> Result<(), String> {
    let config = ConvertOptions::async_from(options).await;
    time!("Conversion", convert_dar_to_oar(config, Closure::default))
}

#[tauri::command]
pub(crate) async fn convert_dar2oar_with_progress(
    window: Window,
    options: GuiConverterOptions,
) -> Result<(), String> {
    let config = ConvertOptions::async_from(options).await;
    time!(
        "Conversion with progress",
        convert_dar_to_oar(config, sender!(window))
    )
}

#[tauri::command]
pub(crate) async fn change_log_level(log_level: Option<&str>) -> Result<(), String> {
    tracing::debug!("Selected log level: {:?}", log_level);
    crate::logging::change_log_level(log_level.unwrap_or("error")).or_else(|err| bail!(err))
}

#[tauri::command]
pub(crate) async fn unhide_dar_dir(window: Window, dar_dir: &str) -> Result<(), String> {
    time!("unhide_dar", unhide_dar(dar_dir, sender!(window)))
}

#[tauri::command]
pub(crate) async fn remove_oar_dir(window: Window, path: &str) -> Result<(), String> {
    time!("remove_oar", remove_oar(path, sender!(window)))
}
