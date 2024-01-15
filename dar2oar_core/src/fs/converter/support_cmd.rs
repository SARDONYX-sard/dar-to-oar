use crate::error::{ConvertError, Result};
use crate::fs::converter::parallel::{get_dar_files, get_oar, is_contain_oar};
use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::fs;

/// # Returns
/// Report which dirs have been shown
///
/// # NOTE
/// It is currently used only in GUI, but is implemented in Core as an API.
pub async fn unhide_dar(
    dar_dir: impl AsRef<Path>,
    mut progress_fn: impl FnMut(usize),
) -> Result<()> {
    let walk_len = get_dar_files(&dar_dir).into_iter().count();
    tracing::debug!("Parallel unhide DAR dir & file counts: {}", walk_len);
    progress_fn(walk_len);

    let mut task_handles: Vec<tokio::task::JoinHandle<Result<()>>> = Vec::new();
    let rename_once = Arc::new(AtomicBool::new(false));

    let entires = get_dar_files(dar_dir).into_iter();
    for (idx, entry) in entires.enumerate() {
        let path = Arc::new(entry.map_err(|_| ConvertError::NotFoundEntry)?.path());

        if path.extension() != Some(OsStr::new("mohidden")) {
            continue;
        };

        tracing::debug!("{:?}", &path);
        task_handles.push(tokio::spawn({
            let rename_once = Arc::clone(&rename_once);
            let path = Arc::clone(&path);
            async move {
                let mut no_hidden_path = path.as_path().to_owned();
                no_hidden_path.set_extension(""); // Remove .mohidden extension
                tracing::debug!("Rename {idx}th:\nfrom: {path:?}\nto: {no_hidden_path:?}");
                fs::rename(path.as_path(), no_hidden_path).await?;

                let _ =
                    rename_once.compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed);
                Ok(())
            }
        }));
    }

    for (idx, task_handle) in task_handles.into_iter().enumerate() {
        task_handle.await??;
        progress_fn(idx);
    }

    match rename_once.load(Ordering::Relaxed) {
        true => Ok(()),
        false => Err(ConvertError::NotFoundUnhideTarget),
    }
}

/// # NOTE
/// It is currently used only in GUI, but is implemented in Core as an API.
pub async fn remove_oar(
    search_dir: impl AsRef<Path>,
    mut progress_fn: impl FnMut(usize),
) -> Result<()> {
    let mut task_handles: Vec<tokio::task::JoinHandle<Result<()>>> = Vec::new();
    let found_once = Arc::new(AtomicBool::new(false));
    let mut prev_dir = OsString::new();

    for entry in get_oar(search_dir).into_iter() {
        let path = Arc::new(entry.map_err(|_| ConvertError::NotFoundEntry)?.path());

        if path.is_dir() {
            if let Some(idx) = is_contain_oar(path.as_ref()) {
                let paths: Vec<&OsStr> = path.iter().collect();
                if let Some(oar_dir) = paths.get(0..idx + 1).map(|path| path.join(OsStr::new("/")))
                {
                    if prev_dir == oar_dir {
                        continue;
                    }
                    prev_dir = oar_dir.clone();

                    task_handles.push(tokio::spawn({
                        let found_once = Arc::clone(&found_once);

                        async move {
                            tracing::debug!("Try to remove oar dir: {:?}", &oar_dir);
                            fs::remove_dir_all(oar_dir).await?;
                            let _ = found_once.compare_exchange(
                                false,
                                true,
                                Ordering::AcqRel,
                                Ordering::Relaxed,
                            );
                            Ok(())
                        }
                    }));
                };
            }
        };
    }

    for (idx, task_handle) in task_handles.into_iter().enumerate() {
        task_handle.await??;
        progress_fn(idx);
    }

    match found_once.load(Ordering::Relaxed) {
        true => Ok(()),
        false => Err(ConvertError::NotFoundOarDir),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{test_helper::init_tracing, Closure};
    use anyhow::Result;
    use temp_dir::TempDir;
    use tokio::fs::{create_dir_all, File};

    #[tokio::test]
    async fn should_unhide_dar_files() -> Result<()> {
        let _guard = init_tracing("unhide_dar", tracing::Level::DEBUG)?;

        let temp_dir = TempDir::new()?;
        let test_dir = temp_dir
            .path()
            .join("TestMod/meshes/actors/character/animations/DynamicAnimationReplacer/100");
        create_dir_all(test_dir.as_path()).await?;
        File::create(test_dir.join("_condition.txt.mohidden")).await?;

        assert!(unhide_dar(temp_dir.path(), Closure::default).await.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn should_remove_oar_dir() -> Result<()> {
        let _guard = init_tracing("remove_oar", tracing::Level::DEBUG)?;

        let temp_dir = TempDir::new()?;
        let test_dir = temp_dir
            .path()
            .join("TestMod/meshes/actors/character/animations/OpenAnimationReplacer/1000");
        let oar_dir_path = temp_dir.path().join(test_dir);
        create_dir_all(&oar_dir_path).await?;

        assert!(remove_oar(temp_dir.path(), Closure::default).await.is_ok());
        Ok(())
    }
}
