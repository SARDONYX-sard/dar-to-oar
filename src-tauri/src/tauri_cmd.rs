use std::path::Path;

use crate::converter::convert_dar_to_oar;

#[tauri::command]
pub fn convert_dar2oar(
    dar_mod_folder: &str,
    oar_mod_folder: &str,
    mod_name: Option<&str>,
    mod_author: Option<&str>,
) -> Result<(), String> {
    convert_dar_to_oar(
        &Path::new(dar_mod_folder),
        &Path::new(oar_mod_folder),
        mod_name,
        mod_author,
    )
    .map_err(|err| err.to_string())
}
