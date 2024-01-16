use dar2oar_core::error::Result;
use dar2oar_core::{read_mapping_table, ConvertOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GuiConverterOptions {
    pub(crate) dar_dir: String,
    pub(crate) oar_dir: Option<String>,
    pub(crate) mod_name: Option<String>,
    pub(crate) mod_author: Option<String>,
    pub(crate) mapping_path: Option<String>,
    pub(crate) mapping_1person_path: Option<String>,
    pub(crate) run_parallel: Option<bool>,
    pub(crate) hide_dar: Option<bool>,
}

impl GuiConverterOptions {
    /// Cast to [`ConvertOptions`]
    pub(crate) async fn to_convert_options(options: Self) -> Result<ConvertOptions> {
        let Self {
            dar_dir,
            oar_dir,
            mod_name,
            mod_author: author,
            mapping_path,
            mapping_1person_path,
            run_parallel,
            hide_dar,
        } = options;

        let section_table = match mapping_path {
            Some(path) => Some(read_mapping_table(path).await?),
            None => None,
        };
        let section_1person_table = match mapping_1person_path {
            Some(path) => Some(read_mapping_table(path).await?),
            None => None,
        };

        Ok(ConvertOptions {
            dar_dir,
            oar_dir,
            mod_name,
            author,
            section_table,
            section_1person_table,
            run_parallel: run_parallel.unwrap_or(false),
            hide_dar: hide_dar.unwrap_or(false),
        })
    }
}
