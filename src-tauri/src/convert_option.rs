use std::{collections::HashMap, path::Path};

use dar2oar_core::{fs::ConvertOptions, read_mapping_table};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GuiConverterOptions<'a> {
    pub(crate) dar_dir: &'a str,
    pub(crate) oar_dir: Option<&'a str>,
    pub(crate) mod_name: Option<&'a str>,
    pub(crate) mod_author: Option<&'a str>,
    pub(crate) mapping_path: Option<String>,
    pub(crate) mapping_1person_path: Option<String>,
    pub(crate) log_level: Option<String>,
    pub(crate) run_parallel: Option<bool>,
    pub(crate) hide_dar: Option<bool>,
}

async fn try_get_mapping_table(mapping_path: Option<&str>) -> Option<HashMap<String, String>> {
    match mapping_path {
        Some(table_path) => read_mapping_table(table_path).await.ok(),
        None => None,
    }
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

        let section_table = try_get_mapping_table(mapping_path.as_deref()).await;
        let section_1person_table = try_get_mapping_table(mapping_1person_path.as_deref()).await;

        Self {
            dar_dir,
            oar_dir,
            mod_name,
            author,
            section_table,
            section_1person_table,
            hide_dar: hide_dar.unwrap_or(false),
        }
    }
}
