//! Functions for writing `config.json`
use crate::conditions::{ConditionsConfig, MainConfig};
use crate::error::Result;
use std::path::Path;
use tokio::{fs, io::AsyncWriteExt};

/// Write json to a file.
async fn write_json_to<T>(target_path: impl AsRef<Path>, value: &T) -> Result<()>
where
    T: ?Sized + serde::Serialize,
{
    let mut config_file = fs::File::create(target_path).await?;
    let json = serde_json::to_string_pretty(value)?;
    config_file.write_all(json.as_bytes()).await?;
    Ok(())
}

/// Write config.json for a dir with each motion file with priority.
pub(crate) async fn write_section_config<P>(
    oar_dir: P,
    config_json: ConditionsConfig<'_>,
) -> Result<()>
where
    P: AsRef<Path>,
{
    write_json_to(oar_dir.as_ref().join("config.json"), &config_json).await
}

/// Write root config.json
///
/// If it exists, do nothing. (This behavior is intended to facilitate the creation of config files
/// for `1st_person` an`3rd_person`on.)
pub(crate) async fn write_name_space_config<P>(
    oar_name_space_path: P,
    mod_name: &str,
    author: Option<&str>,
) -> Result<()>
where
    P: AsRef<Path>,
{
    let target_file = oar_name_space_path.as_ref().join("config.json");
    if target_file.exists() {
        return Ok(());
    }

    let config_json = MainConfig {
        name: mod_name,
        author: author.unwrap_or_default(),
        ..Default::default()
    };
    fs::create_dir_all(&oar_name_space_path).await?;
    write_json_to(target_file, &config_json).await
}
