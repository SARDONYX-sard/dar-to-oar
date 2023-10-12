use dar2oar_core::{
    convert_dar_to_oar,
    fs::{parallel, remove_oar, restore_dar, ConvertOptions},
    read_mapping_table,
};
use std::path::Path;

/// early return with Err() and write log error.
macro_rules! bail {
    ($err:expr) => {{
        log::error!("{}", $err);
        return Err($err.to_string());
    }};
}

macro_rules! try_get_mapping_table {
    ($mapping_path:ident) => {
        match $mapping_path {
            Some(ref table_path) => {
                let mapping = match read_mapping_table(table_path) {
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
pub(crate) fn convert_dar2oar(
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
                log::warn!("unknown log level {}. fallback to error", unknown_level);
                None
            }
        })
        .unwrap_or("error");

    log::debug!("src: {}", dar_dir);
    log::debug!("dist: {:?}", oar_dir);
    log::debug!("mod_name: {:?}", mod_name);
    log::debug!("mod_author: {:?}", mod_author);
    log::debug!("table path: {:?}", mapping_path.as_ref());
    log::debug!("1st person table path: {:?}", mapping_1person_path.as_ref());
    log::debug!("log level: {:?}", log_level);
    log::debug!("run parallel: {:?}", run_parallel);
    log::debug!("to hidden dar: {:?}", hide_dar);

    match run_parallel {
        Some(true) => match parallel::convert_dar_to_oar(ConvertOptions {
            dar_dir,
            oar_dir,
            mod_name,
            author: mod_author,
            section_table: table,
            section_1person_table: table_1person,
            hide_dar: hide_dar.unwrap_or(false),
        }) {
            Ok(complete_msg) => Ok(complete_msg),
            Err(err) => bail!(err),
        },
        Some(false) | None => match convert_dar_to_oar(ConvertOptions {
            dar_dir,
            oar_dir,
            mod_name,
            author: mod_author,
            section_table: table,
            section_1person_table: table_1person,
            hide_dar: hide_dar.unwrap_or(false),
        }) {
            Ok(complete_msg) => Ok(complete_msg),
            Err(err) => bail!(err),
        },
    }
}

#[tauri::command]
pub(crate) fn restore_dar_dir(dar_dir: &str) -> Result<String, String> {
    match restore_dar(dar_dir) {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub(crate) fn remove_oar_dir(path: &str) -> Result<(), String> {
    match remove_oar(path) {
        Ok(()) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}
