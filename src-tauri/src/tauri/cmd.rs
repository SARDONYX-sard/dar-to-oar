use crate::converter::{convert_dar_to_oar, read_mapping_table};
use std::path::Path;

#[tauri::command]
pub fn convert_dar2oar(
    dar_mod_folder: &str,
    oar_mod_folder: Option<&str>,
    mod_name: Option<&str>,
    mod_author: Option<&str>,
    mapping_path: Option<String>,
) -> Result<(), String> {
    let dist = match oar_mod_folder {
        Some(dist) => Some(Path::new(dist).to_path_buf()),
        None => None,
    };
    let table = match mapping_path {
        Some(table_path) => {
            let mapping = match read_mapping_table(table_path) {
                Ok(table) => table,
                Err(err) => return Err(err.to_string()),
            };
            Some(mapping)
        }
        None => None,
    };

    match convert_dar_to_oar(
        dar_mod_folder,
        dist,
        mod_name.as_deref(),
        mod_author.as_deref(),
        table,
    ) {
        Ok(_) => Ok(()),
        Err(err) => return Err(err.to_string()),
    }
}
