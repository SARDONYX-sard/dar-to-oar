use crate::error::{ConvertError, Result};
use crate::fs::converter::common::{convert_inner, handle_conversion_results};
use crate::fs::converter::ConvertOptions;
use crate::fs::path_changer::parse_dar_path;
use jwalk::WalkDirGeneric;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Multi thread converter
///
/// # Parameters
/// - `options`: Convert options
/// - `progress_fn`: For progress callback(1st time: max contents count, 2nd~: index)
///
/// # Return
/// Complete info
///
/// # NOTE
/// For library reasons, you get the number of DAR dirs and files, not the number of DAR files only
/// (i.e., the count is different from the Sequential version)
pub async fn convert_dar_to_oar(
    options: ConvertOptions,
    mut progress_fn: impl FnMut(usize),
) -> Result<()> {
    let dar_dir = options.dar_dir.as_str();

    let walk_len = get_dar_files(dar_dir).into_iter().count();
    tracing::info!("Parallel Converter/DAR dir & file counts: {}", walk_len);
    progress_fn(walk_len);

    let entires = get_dar_files(dar_dir).into_iter();
    let options = Arc::new(options);
    let is_converted_once = Arc::new(AtomicBool::new(false));
    let mut task_handles: Vec<tokio::task::JoinHandle<Result<()>>> = Vec::new();

    for entry in entires {
        let path = entry.map_err(|_| ConvertError::NotFoundEntry)?.path();
        if !path.is_file() {
            continue;
        }
        let parsed_path = Arc::new(match parse_dar_path(&path) {
            Ok(p) => p,
            Err(_) => continue,
        });
        let path = Arc::new(path);

        task_handles.push(tokio::spawn({
            let path = Arc::clone(&path);
            let parsed_path = Arc::clone(&parsed_path);
            let options = Arc::clone(&options);
            let is_converted_once = Arc::clone(&is_converted_once);
            async move {
                convert_inner(
                    &options,
                    path.as_ref(),
                    parsed_path.as_ref(),
                    is_converted_once.as_ref(),
                )
                .await?;
                Ok(())
            }
        }));
    }

    for (idx, task_handle) in task_handles.into_iter().enumerate() {
        task_handle.await??;
        progress_fn(idx);
    }

    // # Ordering validity:
    // Since all processing threads are loaded after they have finished, ordering relationships are not a concern.
    // Therefore, there is no problem in using `Relaxed`.
    handle_conversion_results(is_converted_once.load(Ordering::Relaxed))
}

pub(crate) fn get_dar_files(root: impl AsRef<Path>) -> WalkDirGeneric<(usize, bool)> {
    WalkDirGeneric::<(usize, bool)>::new(root).process_read_dir(
        |_depth, _path, _read_dir_state, children| {
            // Custom filter
            children.retain(|dir_entry_result| {
                dir_entry_result
                    .as_ref()
                    .map(|dir_entry| {
                        let path = dir_entry.path();
                        // NOTE: If false is set at the dir stage, the internal file search is skipped,
                        // so only the file cannot be extracted.
                        is_contain_oar(path).is_none()
                    })
                    .unwrap_or(false)
            });
        },
    )
}

#[inline]
pub(super) fn is_contain_oar(path: impl AsRef<Path>) -> Option<usize> {
    path.as_ref()
        .iter()
        .position(|os_str| os_str == std::ffi::OsStr::new("OpenAnimationReplacer"))
}

pub(crate) fn get_oar(root: impl AsRef<Path>) -> WalkDirGeneric<(usize, bool)> {
    #[inline]
    fn is_contain_dar(path: impl AsRef<Path>) -> Option<usize> {
        path.as_ref()
            .iter()
            .position(|os_str| os_str == std::ffi::OsStr::new("DynamicAnimationReplacer"))
    }

    WalkDirGeneric::<(usize, bool)>::new(root).process_read_dir(
        |_depth, _path, _read_dir_state, children| {
            // Custom filter
            children.retain(|dir_entry_result| {
                dir_entry_result
                    .as_ref()
                    .map(|dir_entry| {
                        let path = dir_entry.path();
                        // NOTE: If false is set at the dir stage, the internal file search is skipped,
                        // so only the file cannot be extracted.
                        is_contain_dar(path).is_none()
                    })
                    .unwrap_or(false)
            });
        },
    )
}
