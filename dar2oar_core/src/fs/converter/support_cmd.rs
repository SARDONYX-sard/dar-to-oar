use crate::error::{ConvertError, Result};
use crate::fs::path_changer;
use crate::ConvertedReport;
use async_walkdir::WalkDir;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio_stream::StreamExt;
use tracing::trace;

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
        if let Ok(parsed_path) =
            path_changer::parse_dar_path(path, Some("DynamicAnimationReplacer.mohidden"))
        {
            let dar_root = parsed_path.dar_root;
            let is_1st_person = parsed_path.is_1st_person;

            if restored_dar.is_none() && path.is_dir() {
                restored_dar = Some(dar_root);
                continue;
            }
            if restored_1st_dar.is_none() && path.is_dir() && is_1st_person {
                restored_1st_dar = Some(dar_root);
            }
        };
    }

    async fn rename_and_check(maybe_dar_root: Option<&PathBuf>) -> Result<()> {
        if let Some(dar_root) = maybe_dar_root {
            let dist = dar_root
                .as_os_str()
                .to_string_lossy()
                .replace(".mohidden", "");
            fs::rename(dar_root, dist).await?;
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

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use temp_dir::TempDir;
    use tokio::fs::create_dir_all;

    #[tokio::test]
    async fn unhide_dar_dirs() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let test_dir = Path::new(
            "TestMod/meshes/actors/character/animations/DynamicAnimationReplacer.mohidden",
        );
        let hidden_dar_path = temp_dir.path().join(test_dir);
        create_dir_all(&hidden_dar_path).await?;

        assert!(unhide_dar(temp_dir.path()).await.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn remove_oar_dir() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let test_dir = Path::new("TestMod/OpenAnimationReplacer");
        let oar_dir_path = temp_dir.path().join(test_dir);
        create_dir_all(&oar_dir_path).await?;

        assert!(remove_oar(temp_dir.path()).await.is_ok());
        Ok(())
    }
}
