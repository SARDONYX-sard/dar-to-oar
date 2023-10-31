use crate::logging::change_log_level;
use dar2oar_core::{
    convert_dar_to_oar,
    fs::{parallel, remove_oar, restore_dar, ConvertOptions},
    read_mapping_table,
};
use std::path::Path;

/// early return with Err() and write log error.
macro_rules! bail {
    ($err:expr) => {{
        tracing::error!("{}", $err);
        return Err($err.to_string());
    }};
}

macro_rules! try_get_mapping_table {
    ($mapping_path:ident) => {
        match $mapping_path {
            Some(ref table_path) => {
                let mapping = match read_mapping_table(table_path).await {
                    Ok(table) => table,
                    Err(err) => bail!(err),
                };
                Some(mapping)
            }
            None => None,
        }
    };
}

#[allow(clippy::too_many_arguments)]
#[tauri::command]
pub(crate) async fn convert_dar2oar(
    dar_dir: &str,
    oar_dir: Option<&str>,
    mod_name: Option<&str>,
    mod_author: Option<&str>,
    mapping_path: Option<String>,
    mapping_1person_path: Option<String>,
    log_level: Option<String>,
    run_parallel: Option<bool>,
    hide_dar: Option<bool>,
) -> Result<String, String> {
    let oar_dir = oar_dir.and_then(|dist| match dist.is_empty() {
        true => None,
        false => Some(Path::new(dist).to_path_buf()),
    });

    let table = try_get_mapping_table!(mapping_path);
    let table_1person = try_get_mapping_table!(mapping_1person_path);

    let log_level = log_level
        .as_deref()
        .and_then(|level| match level {
            "trace" | "debug" | "info" | "warn" | "error" => Some(level),
            unknown_level => {
                tracing::warn!("unknown log level {}. fallback to error", unknown_level);
                None
            }
        })
        .unwrap_or("error");

    tracing::debug!("src: {}", dar_dir);
    tracing::debug!("dist: {:?}", oar_dir);
    tracing::debug!("mod_name: {:?}", mod_name);
    tracing::debug!("mod_author: {:?}", mod_author);
    tracing::debug!("table path: {:?}", mapping_path.as_ref());
    tracing::debug!("1st person table path: {:?}", mapping_1person_path.as_ref());
    tracing::debug!("log level: {:?}", log_level);
    tracing::debug!("run parallel: {:?}", run_parallel);
    tracing::debug!("to hidden dar: {:?}", hide_dar);

    change_log_level(log_level).map_err(|err| err.to_string())?;

    let config = ConvertOptions {
        dar_dir,
        oar_dir,
        mod_name,
        author: mod_author,
        section_table: table,
        section_1person_table: table_1person,
        hide_dar: hide_dar.unwrap_or(false),
        ..Default::default()
    };
    let res = match run_parallel {
        Some(true) => parallel::convert_dar_to_oar(config).await,
        Some(false) | None => convert_dar_to_oar(config).await,
    };

    match res {
        Ok(complete_msg) => Ok(complete_msg),
        Err(err) => bail!(err),
    }
}

#[tauri::command]
pub(crate) async fn restore_dar_dir(dar_dir: &str) -> Result<String, String> {
    restore_dar(dar_dir).await.map_err(|err| err.to_string())
}

#[tauri::command]
pub(crate) async fn remove_oar_dir(path: &str) -> Result<(), String> {
    remove_oar(path).await.map_err(|err| err.to_string())
}
