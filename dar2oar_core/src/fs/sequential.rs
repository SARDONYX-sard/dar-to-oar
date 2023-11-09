use crate::condition_parser::parse_dar2oar;
use crate::conditions::ConditionsConfig;
use crate::error::{ConvertError, Result};
use crate::fs::path_changer::parse_dar_path;
use crate::fs::section_writer::{read_file, write_name_space_config, write_section_config};
use crate::fs::{ConvertOptions, ConvertedReport};
use async_walkdir::WalkDir;
use core::future::Future;
use std::path::Path;
use tokio::fs;
use tokio_stream::StreamExt;

/// Single thread converter
///
/// # Parameters
/// - `options`: Convert options
/// - `async_fn`: For progress async callback(1st time: max contents count, 2nd~: index)
///
/// # Return
/// Complete info
pub async fn convert_dar_to_oar<Fut, O>(
    options: ConvertOptions<'_, impl AsRef<Path>>,
    mut async_fn: impl FnMut(usize) -> Fut,
) -> Result<ConvertedReport>
where
    Fut: Future<Output = O> + Send + 'static,
    O: Send + 'static,
{
    let ConvertOptions {
        dar_dir,
        oar_dir,
        mod_name,
        author,
        section_table,
        section_1person_table,
        hide_dar,
    } = options;
    let mut is_converted_once = false;
    let mut dar_namespace = None; // To need rename to hidden
    let mut dar_1st_namespace = None; // To need rename to hidden(For _1stperson)

    let walk_len = WalkDir::new(&dar_dir).collect::<Vec<_>>().await.len(); // Lower performance cost when sender is None.
    tracing::trace!("Send all dirs & files counts: {}", walk_len);
    tokio::spawn(async_fn(walk_len));

    let mut entries = WalkDir::new(dar_dir);
    let mut idx = 0usize;
    while let Some(entry) = entries.next().await {
        tracing::trace!("Send Dir or file index: {}", idx);
        tokio::spawn(async_fn(idx));
        idx += 1;

        let path = entry?.path();
        let path = path.as_path();

        let (dar_root, oar_name_space_path, is_1st_person, parsed_mod_name, priority, remain) =
            match parse_dar_path(path, None) {
                Ok(data) => data,
                Err(_) => continue, // NOTE: The first search is skipped because it does not yet lead to the DAR file.
            };

        tracing::debug!(
            "[parsed Path]\ndar_root: {:?}, oar_name_space_path: {:?}, is_1st_person: {:?}, parsed_mod_name: {:?}, priority: {:?}, remain_dir: {:?}",
            dar_root, oar_name_space_path, is_1st_person, parsed_mod_name, priority, remain
        );
        let parsed_mod_name = mod_name
            .map(|s| s.to_string())
            .unwrap_or(parsed_mod_name.unwrap_or("Unknown".into()));
        let oar_name_space_path = oar_dir
            .as_ref()
            .map(|path| match is_1st_person {
                true => {
                    path.join("meshes/actors/character/_1stperson/animations/OpenAnimationReplacer")
                }
                false => path.join("meshes/actors/character/animations/OpenAnimationReplacer"),
            })
            .unwrap_or(oar_name_space_path)
            .join(mod_name.unwrap_or(&parsed_mod_name));

        if path.is_dir() {
            tracing::debug!("Dir: {:?}", path);
        } else if path.extension().is_some() {
            tracing::debug!("File: {:?}", path);
            let file_name = path
                .file_name()
                .ok_or_else(|| ConvertError::NotFoundFileName)?
                .to_str()
                .ok_or_else(|| ConvertError::InvalidUtf8)?;

            // files that do not have a priority dir, i.e., files on the same level as the priority dir,
            // are copied to the name space folder location.
            // for this reason, an empty string should be put in the name space folder.
            let priority = &priority.unwrap_or_default();

            let section_name = match is_1st_person {
                true => section_1person_table
                    .as_ref()
                    .and_then(|table| table.get(priority)),
                false => section_table.as_ref().and_then(|table| table.get(priority)),
            }
            .unwrap_or(priority);

            let section_root = oar_name_space_path.join(section_name);
            fs::create_dir_all(&section_root).await?;
            if file_name == "_conditions.txt" {
                match read_file(path).await {
                    Ok(content) => {
                        tracing::debug!("_conditions.txt Content:\n{}", content);

                        let config_json = ConditionsConfig {
                            name: section_name.into(),
                            priority: priority.parse()?,
                            conditions: parse_dar2oar(&content)?,
                            ..Default::default()
                        };

                        write_section_config(section_root, config_json).await?
                    }
                    Err(err) => tracing::error!("Error reading file {path:?}: {err}"),
                }

                if is_1st_person {
                    if dar_1st_namespace.is_none() {
                        dar_1st_namespace = Some(dar_root);
                    }
                } else if dar_namespace.is_none() {
                    dar_namespace = Some(dar_root);
                }
                if !is_converted_once {
                    is_converted_once = true;
                    write_name_space_config(&oar_name_space_path, &parsed_mod_name, author).await?;
                }
            } else {
                // maybe motion files(.kkx)
                if let Some(remain) = remain {
                    let non_leaf_dir = section_root.join(remain);
                    let file = &non_leaf_dir.join(file_name);
                    tracing::debug!("Create dirs: {:?}", &non_leaf_dir);
                    fs::create_dir_all(&non_leaf_dir).await?;
                    tracing::debug!("Remain + Copy:\nfrom: {path:?}\nto: {file:?}");
                    fs::copy(path, file).await?;
                } else {
                    let file = section_root.join(file_name);
                    tracing::debug!("Copy:\nfrom: {path:?}\nto: {file:?}");
                    fs::copy(path, file).await?;
                }
            }
        }
    }

    async fn rename_dir(dir: Option<&std::path::PathBuf>) -> Result<()> {
        if let Some(dar_namespace) = dir {
            let mut dist = dar_namespace.clone();
            dist.as_mut_os_string().push(".mohidden");
            fs::rename(dar_namespace, dist).await?;
        }
        Ok(())
    }

    match is_converted_once {
        true => {
            if hide_dar {
                rename_dir(dar_namespace.as_ref()).await?;
                rename_dir(dar_1st_namespace.as_ref()).await?;

                match (dar_namespace, dar_1st_namespace) {
                    (Some(_), Some(_)) => Ok(ConvertedReport::Renamed1rdAnd3rdPersonDar),
                    (Some(_), None) => Ok(ConvertedReport::Renamed3rdPersonDar),
                    (None, Some(_)) => Ok(ConvertedReport::Renamed1rdPersonDar),
                    _ => Err(ConvertError::NotFoundDarDir),
                }
            } else {
                Ok(ConvertedReport::Complete)
            }
        }
        false => Err(ConvertError::NotFoundDarDir),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    const DAR_DIR: &str = "../test/data/UNDERDOG Animations";
    const TABLE_PATH: &str = "../test/settings/UnderDog Animations_v1.9.6_mapping_table.txt";
    const LOG_PATH: &str = "../convert.log";

    /// NOTE: It is a macro because it must be called at the root of a function to function.
    macro_rules! logger_init {
        () => {
            let (non_blocking, _guard) =
                tracing_appender::non_blocking(std::fs::File::create(LOG_PATH)?);
            tracing_subscriber::fmt()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_max_level(tracing::Level::DEBUG)
                .init();
        };
    }

    async fn create_options<'a>() -> Result<ConvertOptions<'a, &'a str>> {
        Ok(ConvertOptions {
            dar_dir: DAR_DIR,
            // cannot use include_str!
            section_table: Some(crate::read_mapping_table(TABLE_PATH).await?),
            ..Default::default()
        })
    }

    /// 14.75s
    #[ignore]
    #[tokio::test]
    async fn convert_non_mpsc() -> Result<()> {
        logger_init!();
        convert_dar_to_oar(create_options().await?, |_| async {}).await?;
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn convert_with_mpsc() -> Result<()> {
        use once_cell::sync::Lazy;
        use std::sync::atomic::AtomicUsize;
        use std::sync::atomic::Ordering;

        logger_init!();
        let (tx, mut rx) = tokio::sync::mpsc::channel(500);

        //? NOTE: Since recv does not seem to be possible until io is finished, send is used to see the output.
        let sender = move |idx: usize| {
            let tx = tx.clone();
            async move {
                static NUM: Lazy<AtomicUsize> = Lazy::new(AtomicUsize::default);
                let num = NUM.load(Ordering::Acquire);
                if num != 0 {
                    println!("[sender] Converted: {}/{}", idx, num);
                } else {
                    NUM.store(idx, Ordering::Release);
                    println!("[sender] Converted: {}", idx);
                }
                tx.send(idx).await.unwrap_or_default();
            }
        };

        let _ = tokio::spawn(convert_dar_to_oar(create_options().await?, sender)).await?;
        while let Some(i) = rx.recv().await {
            println!("[recv] Converted: {}", i);
        }
        Ok(())
    }
}
