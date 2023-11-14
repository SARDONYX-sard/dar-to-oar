use dar2oar_core::{get_mapping_table, ConvertOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GuiConverterOptions<'a> {
    pub(crate) dar_dir: &'a str,
    pub(crate) oar_dir: Option<&'a str>,
    pub(crate) mod_name: Option<&'a str>,
    pub(crate) mod_author: Option<&'a str>,
    pub(crate) mapping_path: Option<&'a str>,
    pub(crate) mapping_1person_path: Option<&'a str>,
    pub(crate) run_parallel: Option<bool>,
    pub(crate) hide_dar: Option<bool>,
}

#[async_trait::async_trait]
pub(crate) trait AsyncFrom<T> {
    async fn async_from(options: T) -> Self;
}

#[async_trait::async_trait]
impl<'a> AsyncFrom<GuiConverterOptions<'a>> for ConvertOptions<'a, &'a str> {
    async fn async_from(options: GuiConverterOptions<'a>) -> Self {
        let GuiConverterOptions {
            dar_dir,
            oar_dir,
            mod_name,
            mod_author: author,
            mapping_path,
            mapping_1person_path,
            run_parallel,
            hide_dar,
        } = options;

        Self {
            dar_dir,
            oar_dir,
            mod_name,
            author,
            section_table: get_mapping_table(mapping_path).await,
            section_1person_table: get_mapping_table(mapping_1person_path).await,
            run_parallel: run_parallel.unwrap_or(false),
            hide_dar: hide_dar.unwrap_or(false),
        }
    }
}
