use dar2oar_core::{convert_dar_to_oar, read_mapping_table};
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

#[tauri::command]
pub fn convert_dar2oar(
    dar_mod_folder: &str,
    oar_mod_folder: Option<&str>,
    mod_name: Option<&str>,
    mod_author: Option<&str>,
    mapping_path: Option<String>,
    mapping_1person_path: Option<String>,
    log_level: Option<String>,
) -> Result<(), String> {
    let dist = oar_mod_folder.and_then(|dist| match dist.is_empty() {
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

    log::debug!("src: {}", dar_mod_folder);
    log::debug!("dist: {:?}", dist);
    log::debug!("mod_name: {:?}", mod_name);
    log::debug!("mod_author: {:?}", mod_author);
    log::debug!("table path: {:?}", mapping_path.as_ref());
    log::debug!("1st person table path: {:?}", mapping_1person_path.as_ref());
    log::debug!("log level: {:?}", log_level);

    match convert_dar_to_oar(
        dar_mod_folder,
        dist,
        mod_name,
        mod_author,
        table,
        table_1person,
    ) {
        Ok(_) => Ok(()),
        Err(err) => bail!(err),
    }
}
