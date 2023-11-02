mod mapping_table;
mod sequential;

pub mod async_closure;
pub mod parallel;
pub mod path_changer;

use crate::conditions::{ConditionsConfig, MainConfig};
use crate::error::{ConvertError, Result};
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
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ConvertedReport {
    #[error("Conversion Completed.")]
    Complete,

    #[error("Converted & Renamed 1st, 3rd person DAR")]
    Renamed1rdAnd3rdPersonDar,
    #[error("Converted & Renamed 1rd person DAR")]
    Renamed1rdPersonDar,
    #[error("Converted & Renamed 3rd person DAR")]
    Renamed3rdPersonDar,

    #[error("Unhide 1st & 3rd person")]
    Unhide1rdAnd3rdPerson,
    #[error("Unhide 1rd person")]
    Unhide1rdPerson,
    #[error("Unhide 3rd person")]
    Unhide3rdPerson,
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

async fn write_json_to<T>(target_path: impl AsRef<Path>, value: &T) -> Result<()>
where
    T: ?Sized + serde::Serialize,
{
    let mut config_file = fs::File::create(target_path).await?;
    let json = serde_json::to_string_pretty(value)?;
    config_file.write_all(json.as_bytes()).await?;
    Ok(())
}

async fn write_section_config<P>(oar_dir: P, config_json: ConditionsConfig) -> Result<()>
where
    P: AsRef<Path>,
{
    write_json_to(oar_dir.as_ref().join("config.json"), &config_json).await
}

/// If it exists, do nothing. (This behavior is intended to facilitate the creation of config files
/// for 1st_person and 3rd_person.)
async fn write_name_space_config<P>(
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

/// # Returns
/// Report which dirs have been shown
///
/// # NOTE
/// It is currently used only in GUI, but is implemented in Core as an API.
pub async fn unhide_dar(dar_dir: impl AsRef<Path>) -> Result<ConvertedReport> {
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

    async fn rename_and_check(maybe_dar_root: Option<&PathBuf>) -> Result<()> {
        if let Some(dar_root) = maybe_dar_root {
            let dist = dar_root
                .as_os_str()
                .to_string_lossy()
                .replace(".mohidden", "");
            fs::rename(dar_root.clone(), dist).await?;
        }
        Ok(())
    }

    let _ = tokio::join!(
        rename_and_check(restored_dar.as_ref()),
        rename_and_check(restored_1st_dar.as_ref())
    );

    match (restored_dar, restored_1st_dar) {
        (Some(_), Some(_)) => Ok(ConvertedReport::Unhide1rdAnd3rdPerson),
        (Some(_), None) => Ok(ConvertedReport::Unhide3rdPerson),
        (None, Some(_)) => Ok(ConvertedReport::Unhide1rdPerson),
        _ => Err(ConvertError::NotFoundUnhideTarget),
    }
}

/// # NOTE
/// It is currently used only in GUI, but is implemented in Core as an API.
pub async fn remove_oar(search_dir: impl AsRef<Path>) -> Result<()> {
    let mut removed_once = false;
    let mut entries = WalkDir::new(search_dir);
    while let Some(entry) = entries.next().await {
        let path = entry?.path();
        if path.is_dir() {
            let path = path.to_str();
            if let Some(path) = path {
                if path.ends_with("OpenAnimationReplacer") {
                    trace!("Try to remove oar dir: {:?}", &path);
                    fs::remove_dir_all(path).await?;
                    removed_once = true;
                }
            }
        }
    }

    match removed_once {
        true => Ok(()),
        false => Err(ConvertError::NotFoundOarDir),
    }
}

#[ignore]
#[tokio::test]
async fn remove_oar_dir() -> Result<()> {
    remove_oar("../test/data/UNDERDOG Animations").await
}
