use crate::convert_option::GuiConverterOptions;
use dar2oar_core::{Closure, convert_dar_to_oar, remove_oar, unhide_dar};
use std::time::Instant;
use tauri::{Emitter as _, Window};

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

/// Cast the conversion options in the GUI and perform the conversion.
macro_rules! dar_to_oar {
    ($options:ident, $sender:expr) => {
        convert_dar_to_oar(
            GuiConverterOptions::to_convert_options($options)
                .await
                .or_else(|err| bail!(err))?,
            $sender,
        )
    };
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
    ($window:ident, $emit_name:literal) => {
        move |index: usize| {
            if let Err(err) = $window.emit($emit_name, Payload { index }) {
                tracing::error!("{}", err);
            };
        }
    };
}

#[tauri::command]
pub(crate) async fn convert_dar2oar(options: GuiConverterOptions) -> Result<(), String> {
    let start = Instant::now();

    let options = GuiConverterOptions::to_convert_options(options)
        .await
        .or_else(|err| bail!(err))?;
    let res = convert_dar_to_oar(options, Closure::default)
        .await
        .or_else(|err| bail!(err));

    let elapsed = start.elapsed();
    tracing::info!(
        "{} time: {}.{}secs.",
        "Conversion",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );
    res
}

#[tauri::command]
pub(crate) async fn convert_dar2oar_with_progress(
    window: Window,
    options: GuiConverterOptions,
) -> Result<(), String> {
    let sender = sender!(window, "/dar2oar/progress/converter");
    time!("Conversion with progress", dar_to_oar!(options, sender))
}

#[tauri::command]
pub(crate) async fn change_log_level(log_level: Option<&str>) -> Result<(), String> {
    tracing::debug!("Selected log level: {:?}", log_level);
    tracing_rotation::change_level(log_level.unwrap_or("error")).or_else(|err| bail!(err))
}

/// Define our own `writeTextFile` api for tauri,
/// because there was a bug that contents were not written properly
/// (there was a case that the order of some data in contents was switched).
#[tauri::command]
pub(crate) async fn write_file(path: &str, content: &str) -> Result<(), String> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent).or_else(|err| bail!(err))?;
    }
    std::fs::write(path, content).or_else(|err| bail!(err))
}

#[tauri::command]
pub(crate) async fn remove_oar_dir(window: Window, path: &str) -> Result<(), String> {
    let sender = sender!(window, "/dar2oar/progress/remove-oar");
    time!("remove_oar", remove_oar(path, sender))
}

#[tauri::command]
pub(crate) async fn unhide_dar_dir(window: Window, dar_dir: &str) -> Result<(), String> {
    let sender = sender!(window, "/dar2oar/progress/unhide-dar");
    time!("unhide_dar", unhide_dar(dar_dir, sender))
}

#[tauri::command]
pub(crate) async fn generate_mapping_table(
    path: &std::path::Path,
    strategy: mapping_table::builder::MappingStrategy,
) -> Result<mapping_table::MappingTable, String> {
    mapping_table::builder::generate_mapping_table(path, strategy).or_else(|err| bail!(err))
}

#[tauri::command]
pub(crate) async fn read_mapping_table(
    path: &std::path::Path,
) -> Result<mapping_table::MappingTable, String> {
    let content = std::fs::read_to_string(path).or_else(|err| bail!(err))?;
    match mapping_table::reader::parse_mapping_table(&content) {
        Ok(table) => Ok(table),
        Err(error) => {
            let error = format!(
                "Error reading mapping table from {}:\n{}",
                path.display(),
                error
            );
            tracing::error!("{error}");
            Err(error)
        }
    }
}
