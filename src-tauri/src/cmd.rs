use dar2oar_core::{convert_dar_to_oar, read_mapping_table};
use std::path::Path;

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
    let table = match mapping_path {
        Some(ref table_path) => {
            let mapping = match read_mapping_table(table_path) {
                Ok(table) => table,
                Err(err) => return Err(err.to_string()),
            };
            Some(mapping)
        }
        None => None,
    };
    let table_1person = match mapping_1person_path {
        Some(ref table_path) => {
            let mapping = match read_mapping_table(table_path) {
                Ok(table) => table,
                Err(err) => return Err(err.to_string()),
            };
            Some(mapping)
        }
        None => None,
    };

    let log_level = match log_level {
        Some(level) => match level.as_str() {
            "trace" | "debug" | "info" | "warn" | "error" => level,
            _ => "error".to_owned(),
        },
        None => "error".to_owned(),
    };

    log::debug!("src: {}", dar_mod_folder);
    log::debug!("dist: {:?}", dist);
    log::debug!("mod_name: {:?}", mod_name);
    log::debug!("mod_author: {:?}", mod_author);
    log::debug!("table path: {:?}", mapping_path.as_ref());
    log::debug!("1st person table path: {:?}", mapping_1person_path.as_ref());
    log::debug!("log level: {:?}", log_level.as_str());

    match convert_dar_to_oar(
        dar_mod_folder,
        dist,
        mod_name.as_deref(),
        mod_author.as_deref(),
        table,
        table_1person,
    ) {
        Ok(_) => Ok(()),
        Err(err) => return Err(err.to_string()),
    }
}
