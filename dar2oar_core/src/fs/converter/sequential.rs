use crate::error::Result;
use crate::fs::converter::common::{convert_inner, handle_conversion_results};
use crate::fs::converter::ConvertOptions;
use crate::fs::path_changer::parse_dar_path;
use async_walkdir::{Filtering, WalkDir};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio_stream::StreamExt;

/// Single thread converter
///
/// # Parameters
/// - `options`: Convert options
/// - `progress_fn`: For progress callback(1st time: max contents count, 2nd~: index)
///
/// # Return
/// Complete info
pub async fn convert_dar_to_oar(
    options: ConvertOptions,
    mut progress_fn: impl FnMut(usize),
) -> Result<()> {
    let dar_dir = options.dar_dir.as_str();

    let walk_len = get_dar_file_count(dar_dir).await?;
    tracing::info!("Sequential Converter/DAR file counts: {}", walk_len);
    progress_fn(walk_len);

    let is_converted_once = AtomicBool::new(false);
    let mut entries = get_dar_files(dar_dir).await;
    let mut idx = 0usize;
    while let Some(entry) = entries.next().await {
        let path = entry?.path();
        let path = path.as_path();
        if !path.is_file() {
            continue;
        }
        let parsed_path = match parse_dar_path(path) {
            Ok(p) => p,
            Err(_) => continue,
        };

        tracing::debug!("[Start {}th conversion]\n{:?}", idx, &parsed_path);
        convert_inner(&options, path, &parsed_path, &is_converted_once).await?;

        progress_fn(idx);
        tracing::debug!("[End {}th conversion]\n\n", idx);
        idx += 1;
    }

    // # Ordering validity:
    // The order is irrelevant because `tokio::spawn` is not used in the while loop.
    // Therefore, there is no problem in using `Relaxed`.
    handle_conversion_results(is_converted_once.load(Ordering::Relaxed))
}

async fn get_dar_files(root: impl AsRef<Path>) -> WalkDir {
    WalkDir::new(root).filter(move |entry| async move {
        if let Ok(file_type) = entry.file_type().await {
            match file_type.is_dir() {
                true => Filtering::Ignore,
                false => Filtering::Continue,
            }
        } else {
            // NOTE: Non-existent, non-authoritative, and I/O errors will ignore.
            // Reason
            // - Because if there is no entry in a higher-level function, it will cause an error.
            // - In async closure, Result and ? operators cannot be used.
            Filtering::Ignore
        }
    })
}

/// # NOTE
/// I thought this would make performance very bad, but it only gets worse by a few tens of milliseconds.
async fn get_dar_file_count(root: impl AsRef<Path>) -> Result<usize> {
    #[inline]
    pub fn is_contain_dar(path: impl AsRef<Path>) -> Option<usize> {
        path.as_ref()
            .iter()
            .position(|os_str| os_str == std::ffi::OsStr::new("DynamicAnimationReplacer"))
    }

    let mut walk_len = 0;
    let mut entries = get_dar_files(root).await;
    while let Some(entry) = entries.next().await {
        let path = entry?.path();
        tracing::trace!("Calculate walk_len[{}]: {:?}", walk_len, &path);
        match is_contain_dar(path) {
            Some(_) => walk_len += 1,
            None => continue,
        }
    }
    Ok(walk_len)
}
