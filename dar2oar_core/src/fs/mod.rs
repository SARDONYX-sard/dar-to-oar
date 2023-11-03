mod mapping_table;
mod section_writer;
mod sequential;

pub mod async_closure;
pub mod parallel;
pub mod path_changer;

use crate::error::{ConvertError, Result};
use async_walkdir::WalkDir;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio_stream::StreamExt;
use tracing::trace;

pub use mapping_table::{get_mapping_table, read_mapping_table};
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
