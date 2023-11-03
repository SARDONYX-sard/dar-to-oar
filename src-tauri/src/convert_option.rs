use dar2oar_core::{fs::ConvertOptions, get_mapping_table};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GuiConverterOptions<'a> {
    pub(crate) dar_dir: &'a str,
    pub(crate) oar_dir: Option<&'a str>,
    pub(crate) mod_name: Option<&'a str>,
    pub(crate) mod_author: Option<&'a str>,
    pub(crate) mapping_path: Option<&'a str>,
    pub(crate) mapping_1person_path: Option<&'a str>,
    pub(crate) log_level: Option<String>,
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
            log_level: _,
            run_parallel: _,
            hide_dar,
        } = options;

        let oar_dir = oar_dir.and_then(|dist| match dist.is_empty() {
            true => None,
            false => Some(Path::new(dist).to_path_buf()),
        });

        Self {
            dar_dir,
            oar_dir,
            mod_name,
            author,
            section_table: get_mapping_table(mapping_path).await,
            section_1person_table: get_mapping_table(mapping_1person_path).await,
            hide_dar: hide_dar.unwrap_or(false),
        }
    }
}
