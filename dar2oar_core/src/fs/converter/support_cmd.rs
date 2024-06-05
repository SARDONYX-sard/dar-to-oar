//! Auxiliary commands for smooth use of the converter
use crate::error::{ConvertError, Result};
use crate::fs::converter::parallel::{get_dar_files, get_oar, is_contain_oar};
use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::fs;

/// A parallel search will find the `DynamicAnimationReplacer` directory in the path passed as the argument
/// and remove only the `mohidden` extension names from the files in that directory.
///
/// # Errors
/// - Failed to find the `DynamicAnimationReplacer` directory
pub async fn unhide_dar(
    dar_dir: impl AsRef<Path>,
    mut progress_fn: impl FnMut(usize),
) -> Result<()> {
    let walk_len = get_dar_files(&dar_dir).into_iter().count();
    #[cfg(feature = "tracing")]
    tracing::debug!("Parallel unhide DAR dir & file counts: {}", walk_len);
    progress_fn(walk_len);

    let mut task_handles: Vec<tokio::task::JoinHandle<Result<()>>> = Vec::new();
    let rename_once = Arc::new(AtomicBool::new(false));

    let entires = get_dar_files(dar_dir).into_iter();
    for (idx, entry) in entires.enumerate() {
        let path = Arc::new(entry.map_err(|_err| ConvertError::NotFoundEntry)?.path());

        if path.extension() != Some(OsStr::new("mohidden")) {
            continue;
        };

        #[cfg(feature = "tracing")]
        tracing::debug!("{:?}", &path);
        task_handles.push(tokio::spawn({
            let rename_once = Arc::clone(&rename_once);
            let path = Arc::clone(&path);
            async move {
                let mut no_hidden_path = path.as_path().to_owned();
                let _ = no_hidden_path.set_extension(""); // Remove .mohidden extension
                #[cfg(feature = "tracing")]
                tracing::debug!("Rename {idx}th:\n- From: {path:?}\n-   To: {no_hidden_path:?}\n");
                fs::rename(path.as_path(), no_hidden_path).await?;

                // # Ordering validity:
                // Use `AcqRel` to `happened before relationship`(form a memory read/write order between threads) of cas(compare_and_swap),
                // so that other threads read after writing true to memory to prevent unnecessary file writing.
                // - In case of cas failure, use `Relaxed` because the order is unimportant.
                let _ =
                    rename_once.compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed);
                Ok(())
            }
        }));
        progress_fn(idx);
    }

    for task_handle in task_handles {
        task_handle.await??;
    }

    match rename_once.load(Ordering::Relaxed) {
        true => Ok(()),
        false => Err(ConvertError::NotFoundUnhideTarget),
    }
}

/// A parallel search will find and remove the `OpenAnimationReplacer` directory from the path passed as the argument.
///
/// # Errors
/// - Failed to find the `OpenAnimationReplacer` directory
pub async fn remove_oar(
    search_dir: impl AsRef<Path>,
    mut progress_fn: impl FnMut(usize),
) -> Result<()> {
    let walk_len = get_oar(&search_dir).into_iter().count();
    #[cfg(feature = "tracing")]
    tracing::debug!("Parallel remove OAR dir & file counts: {}", walk_len);
    progress_fn(walk_len);

    let mut task_handles: Vec<tokio::task::JoinHandle<Result<()>>> = Vec::new();
    let found_once = Arc::new(AtomicBool::new(false));
    let mut prev_dir = OsString::new();

    for (idx, entry) in get_oar(search_dir).into_iter().enumerate() {
        let path = Arc::new(entry.map_err(|_err| ConvertError::NotFoundEntry)?.path());
        if path.is_dir() {
            if let Some(oar_start_idx) = is_contain_oar(path.as_ref()) {
                let paths: Vec<&OsStr> = path.iter().collect();

                if let Some(oar_dir) = paths
                    .get(0..=oar_start_idx)
                    .map(|str_paths| str_paths.join(OsStr::new("/")))
                {
                    if prev_dir == oar_dir {
                        continue;
                    }
                    prev_dir.clone_from(&oar_dir);

                    task_handles.push(tokio::spawn({
                        let found_once = Arc::clone(&found_once);

                        async move {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("Try to remove oar dir: {:?}\n", &oar_dir);
                            fs::remove_dir_all(oar_dir).await?;
                            // # Ordering validity:
                            // Use `AcqRel` to `happened before relationship`(form a memory read/write order between threads) of cas(compare_and_swap),
                            // so that other threads read after writing true to memory to prevent unnecessary file writing.
                            // - In case of cas failure, use `Relaxed` because the order is unimportant.
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
        progress_fn(idx);
    }

    for task_handle in task_handles {
        task_handle.await??;
    }

    match found_once.load(Ordering::Relaxed) {
        true => Ok(()),
        false => Err(ConvertError::NotFoundOarDir),
    }
}

#[cfg(feature = "tracing")]
#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use temp_dir::TempDir;
    use tokio::fs::{create_dir_all, File};

    macro_rules! sender {
        () => {
            |idx: usize| tracing::debug!("{}", idx)
        };
    }

    #[tokio::test]
    #[quick_tracing::try_init(file = "../logs/unhide_dar.log", level = "ERROR")]
    async fn should_unhide_dar_files() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let test_dir = temp_dir
            .path()
            .join("TestMod/meshes/actors/character/animations/DynamicAnimationReplacer/100");
        create_dir_all(test_dir.as_path()).await?;
        let _ = File::create(test_dir.join("_condition.txt.mohidden")).await?;

        assert!(unhide_dar(temp_dir.path(), sender!()).await.is_ok());
        Ok(())
    }

    #[tokio::test]
    #[quick_tracing::init(file = "../logs/remove_oar.log", level = "ERROR")]
    async fn should_remove_oar_dir() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let test_dir = temp_dir
            .path()
            .join("TestMod/meshes/actors/character/animations/OpenAnimationReplacer/1000");
        let oar_dir_path = temp_dir.path().join(test_dir);
        create_dir_all(&oar_dir_path).await?;

        assert!(remove_oar(temp_dir.path(), sender!()).await.is_ok());
        Ok(())
    }
}
