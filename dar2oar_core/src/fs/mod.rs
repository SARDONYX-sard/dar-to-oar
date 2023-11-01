mod mapping_table;
mod sequential;

pub mod parallel;
pub mod path_changer;

use crate::conditions::{ConditionsConfig, MainConfig};
use anyhow::{bail, Context as _};
use async_walkdir::WalkDir;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio_stream::StreamExt;
use tracing::trace;

pub use mapping_table::read_mapping_table;
pub use sequential::convert_dar_to_oar;

#[derive(Debug, Default)]
pub struct ConvertOptions<'a, P: AsRef<Path>> {
    /// DAR source dir path
    pub dar_dir: P,
    /// OAR destination dir path(If not, it is inferred from src)
    pub oar_dir: Option<PathBuf>,
    /// mod name in config.json & directory name(If not, it is inferred from src)
    pub mod_name: Option<&'a str>,
    /// mod author in config.json
    pub author: Option<&'a str>,
    /// path to section name table
    pub section_table: Option<HashMap<String, String>>,
    /// path to section name table(For _1st_person)
    pub section_1person_table: Option<HashMap<String, String>>,
    /// After converting to OAR, add mohidden to the DAR directory before conversion to treat it as a hidden directory. (for MO2 users)
    pub hide_dar: bool,

    pub sender: Option<tokio::sync::mpsc::Sender<usize>>,
}

async fn read_file<P>(file_path: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = fs::File::open(file_path).await?;
    let mut content = String::new();
    file.read_to_string(&mut content).await?;
    Ok(content)
}

async fn write_section_config<P>(oar_dir: P, config_json: ConditionsConfig) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let target_path = oar_dir.as_ref().join("config.json");
    let mut config_file = fs::File::create(&target_path).await.with_context(|| {
        let msg = format!("writing section config target: {:?}", target_path);
        tracing::error!("{}", msg);
        msg
    })?;
    let json = serde_json::to_string_pretty(&config_json)?;
    config_file.write_all(json.as_bytes()).await?;
    Ok(())
}

/// If there is no name_space_config file, create one.
/// If it exists, do nothing. (This behavior is intended to facilitate the creation of config files
/// for 1st_person and 3rd_person.)
async fn write_name_space_config<P>(
    oar_name_space_path: P,
    mod_name: &str,
    author: Option<&str>,
) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let target_file = oar_name_space_path.as_ref().join("config.json");
    if target_file.exists() {
        return Ok(());
    }

    let config_json = MainConfig {
        name: mod_name.into(),
        author: author.unwrap_or_default().into(),
        ..Default::default()
    };
    fs::create_dir_all(&oar_name_space_path).await?;
    let mut config_file = fs::File::create(target_file).await?;
    let json = serde_json::to_string_pretty(&config_json)?;
    config_file.write_all(json.as_bytes()).await?;
    Ok(())
}

/// # Returns
/// Report which dirs have been restored
///
/// # NOTE
/// It is currently used only in GUI, but is implemented in Core as an API.
pub async fn restore_dar(dar_dir: impl AsRef<Path>) -> anyhow::Result<String> {
    let mut restored_dar = None;
    let mut restored_1st_dar = None;
    let mut entries = WalkDir::new(dar_dir);
    while let Some(entry) = entries.next().await {
        let path = entry?.path();
        let path = path.as_path();
        let (dar_root, _, is_1st_person, _, _, _) =
            match path_changer::parse_dar_path(path, Some("DynamicAnimationReplacer.mohidden")) {
                Ok(data) => data,
                Err(_) => continue, // NOTE: The first search is skipped because it does not yet lead to the DAR file.
            };

        if restored_dar.is_none() && path.is_dir() {
            restored_dar = Some(dar_root);
            continue;
        }
        if restored_1st_dar.is_none() && path.is_dir() && is_1st_person {
            restored_1st_dar = Some(dar_root);
        }
    }

    let mut msg = String::new();
    if let Some(dar_root) = restored_dar.as_ref() {
        let dist = dar_root
            .as_os_str()
            .to_string_lossy()
            .replace(".mohidden", "");
        fs::rename(dar_root.clone(), dist).await?;
        msg = format!("{}- Restored 3rd_person", msg);
    }
    if let Some(dar_root) = restored_1st_dar.as_ref() {
        let dist = dar_root
            .as_os_str()
            .to_string_lossy()
            .replace(".mohidden", "");
        fs::rename(dar_root.clone(), dist).await?;
        msg = format!("{}\n- Restored 1rd_person", msg);
    }

    if restored_dar.is_none() && restored_1st_dar.is_none() {
        anyhow::bail!("Neither 1st or 3rd person DynamicAnimationReplacer.mohidden found.")
    } else {
        Ok(msg)
    }
}

/// # NOTE
/// It is currently used only in GUI, but is implemented in Core as an API.
pub async fn remove_oar(dar_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let mut remove_target = None;
    let mut removed_target_1st = None;
    let mut entries = WalkDir::new(dar_dir);
    while let Some(entry) = entries.next().await {
        let path = entry?.path();
        let path = path.as_path();
        // NOTE: The OAR root obtained by parse fn is calculated and not guaranteed to exist.
        let (_, oar_name_space_path, is_1st_person, _, _, _) =
            match path_changer::parse_dar_path(path, Some("DynamicAnimationReplacer.mohidden")) {
                Ok(data) => data,
                Err(_) => {
                    match path_changer::parse_dar_path(path, Some("DynamicAnimationReplacer")) {
                        Ok(data) => data,
                        Err(_) => continue, // NOTE: The first search is skipped because it does not yet lead to the DAR file.
                    }
                } // NOTE: The first search is skipped because it does not yet lead to the DAR file.
            };

        if remove_target.is_none() && path.is_dir() && !is_1st_person {
            remove_target = Some(oar_name_space_path);
            continue;
        }
        if removed_target_1st.is_none() && path.is_dir() && is_1st_person {
            removed_target_1st = Some(oar_name_space_path);
        }
    }

    if remove_target.is_none() && removed_target_1st.is_none() {
        bail!("Not found OAR directory.")
    }

    if let Some(oar_root) = remove_target {
        if oar_root.exists() {
            trace!("Remove oar dir: {:?}", &oar_root);
            fs::remove_dir_all(oar_root).await?;
        }
    }
    if let Some(oar_root) = removed_target_1st {
        if oar_root.exists() {
            trace!("Remove oar dir: {:?}", &oar_root);
            fs::remove_dir_all(oar_root).await?;
        }
    }

    Ok(())
}
