use crate::error::{ConvertError, Result};
use crate::fs::converter::common::{convert_inner, handle_conversion_results};
use crate::fs::converter::{ConvertOptions, ConvertedReport};
use crate::fs::path_changer::parse_dar_path;
use jwalk::WalkDirGeneric;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Multi thread converter
///
/// # Parameters
/// - `options`: Convert options
/// - `progress_fn`: For progress async callback(1st time: max contents count, 2nd~: index)
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
) -> Result<ConvertedReport> {
    let dar_dir = options.dar_dir.as_str();

    let walk_len = get_dar_files(dar_dir).into_iter().count();
    tracing::debug!("Parallel Converter/DAR dir & file counts: {}", walk_len);
    progress_fn(walk_len);

    let entires = get_dar_files(dar_dir).into_iter();
    let hide_dar = options.hide_dar;
    let options = Arc::new(options);

    let mut dar_1st_namespace = None; // To need rename to hidden(For _1stperson)
    let mut dar_namespace = None; // To need rename to hidden
    let is_converted_once = Arc::new(AtomicBool::new(false));
    let mut task_handles: Vec<tokio::task::JoinHandle<Result<()>>> = Vec::new();

    for (idx, entry) in entires.enumerate() {
        let path = entry.map_err(|_| ConvertError::NotFoundEntry)?.path();
        let parsed_path = Arc::new(match parse_dar_path(&path, None) {
            Ok(p) => p,
            Err(_) => continue,
        });
        let path = Arc::new(path);

        if dar_1st_namespace.is_none() && parsed_path.is_1st_person {
            dar_1st_namespace = Some(parsed_path.dar_root.clone());
        } else if dar_namespace.is_none() {
            dar_namespace = Some(parsed_path.dar_root.clone());
        }

        task_handles.push(tokio::spawn({
            let path = Arc::clone(&path);
            let parsed_path = Arc::clone(&parsed_path);
            let options = Arc::clone(&options);
            let is_converted_once = Arc::clone(&is_converted_once);
            async move {
                tracing::debug!("[Start {}th conversion]\n{:?}", idx, &parsed_path);
                convert_inner(
                    &options,
                    &path,
                    parsed_path.as_ref(),
                    is_converted_once.as_ref(),
                )
                .await?;
                tracing::debug!("[End {}th conversion]\n\n", idx);
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
    match is_converted_once.load(Ordering::Relaxed) {
        true => handle_conversion_results(hide_dar, &dar_namespace, &dar_1st_namespace).await,
        false => Err(ConvertError::NeverConverted),
    }
}

fn get_dar_files(root: impl AsRef<Path>) -> WalkDirGeneric<(usize, bool)> {
    #[inline]
    pub fn is_contain_oar(path: impl AsRef<Path>) -> Option<usize> {
        path.as_ref()
            .iter()
            .position(|os_str| os_str == std::ffi::OsStr::new("OpenAnimationReplacer"))
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
                        is_contain_oar(path).is_none()
                    })
                    .unwrap_or(false)
            });
        },
    )
}
