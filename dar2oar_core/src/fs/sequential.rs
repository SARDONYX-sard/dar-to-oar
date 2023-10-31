use crate::condition_parser::parse_dar2oar;
use crate::conditions::ConditionsConfig;
use crate::fs::path_changer::parse_dar_path;
use crate::fs::{read_file, write_name_space_config, write_section_config, ConvertOptions};
use anyhow::{bail, Context as _, Result};
use async_walkdir::WalkDir;
use std::path::Path;
use tokio::fs;
use tokio_stream::StreamExt;

/// Single thread converter
///
/// # Return
/// Complete info
pub async fn convert_dar_to_oar(options: ConvertOptions<'_, impl AsRef<Path>>) -> Result<String> {
    let ConvertOptions {
        dar_dir,
        oar_dir,
        mod_name,
        author,
        section_table,
        section_1person_table,
        hide_dar,
        sender,
    } = options;
    let mut is_converted_once = false;
    let mut dar_namespace = None; // To need rename to hidden
    let mut dar_1st_namespace = None; // To need rename to hidden(For _1stperson)

    let sender = sender.as_ref(); // Borrowing ownership here prevents move errors in the loop.
    let mut walk_len = 0;
    if let Some(sender) = sender {
        walk_len = WalkDir::new(&dar_dir).collect::<Vec<_>>().await.len(); // Lower performance cost when sender is None.
        log::debug!("Dir & File Counts: {}", walk_len);
        sender.send(walk_len).await?;
    }

    let mut entries = WalkDir::new(dar_dir);
    let mut idx = 0usize;
    while let Some(entry) = entries.next().await {
        if let Some(sender) = sender {
            log::debug!("Converted: {}/{}", idx, walk_len);
            sender.send(idx).await?;
        }
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
                .context("Not found file name")?
                .to_str()
                .context("This file isn't valid utf8")?;

            // Files that do not have a priority dir, i.e., files on the same level as the priority dir,
            // are copied to the name space folder location.
            // For this reason, an empty string should be put in the name space folder.
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
                    write_name_space_config(&oar_name_space_path, &parsed_mod_name, author)
                        .await
                        .with_context(|| {
                            format!(
                                "Failed to write name space config to: {:?}",
                                oar_name_space_path
                            )
                        })?;
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

    match is_converted_once {
        true => {
            let mut msg = "Conversion Completed.".to_string();
            if hide_dar {
                if let Some(dar_namespace) = dar_namespace {
                    let mut dist = dar_namespace.clone();
                    dist.as_mut_os_string().push(".mohidden");
                    fs::rename(dar_namespace, dist).await?;
                    msg = format!("{}\n- 3rdPerson DAR dir was renamed", msg);
                };

                if let Some(dar_1st_namespace) = dar_1st_namespace {
                    let mut dist = dar_1st_namespace.clone();
                    dist.as_mut_os_string().push(".mohidden");
                    fs::rename(dar_1st_namespace, dist).await?;
                    msg = format!("{}\n- 1stPerson DAR dir was renamed", msg);
                };
            }
            tracing::debug!(msg);
            Ok(msg)
        }
        false => bail!("DynamicAnimationReplacer dir was never found"),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tracing::Level;

    /// 14.75s
    #[ignore]
    #[tokio::test]
    async fn convert_non_mpsc() -> anyhow::Result<()> {
        let (non_blocking, _guard) =
            tracing_appender::non_blocking(std::fs::File::create("../convert.log")?);
        tracing_subscriber::fmt()
            .with_writer(non_blocking)
            .with_ansi(false)
            .with_max_level(Level::DEBUG)
            .init();

        // cannot use include_str!
        let table = crate::read_mapping_table(
            "../test/settings/UnderDog Animations_v1.9.6_mapping_table.txt",
        )
        .await
        .unwrap();

        let span = tracing::info_span!("converting");
        let _guard = span.enter();
        convert_dar_to_oar(ConvertOptions {
            dar_dir: "../test/data/UNDERDOG Animations",
            section_table: Some(table),
            // sender: Some(tx),
            ..Default::default()
        })
        .await?;
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn convert_with_mpsc() -> anyhow::Result<()> {
        use tokio::sync::mpsc;

        let (non_blocking, _guard) =
            tracing_appender::non_blocking(std::fs::File::create("../convert.log")?);
        tracing_subscriber::fmt()
            .with_writer(non_blocking)
            .with_ansi(false)
            .with_max_level(Level::ERROR)
            .init();

        // cannot use include_str!
        let table = crate::read_mapping_table(
            "../test/settings/UnderDog Animations_v1.9.6_mapping_table.txt",
        )
        .await
        .unwrap();

        let span = tracing::info_span!("converting");
        let _guard = span.enter();

        let (tx, mut rx) = mpsc::channel(1500);

        tokio::spawn(convert_dar_to_oar(ConvertOptions {
            dar_dir: "../test/data/UNDERDOG Animations",
            section_table: Some(table),
            sender: Some(tx),
            ..Default::default()
        }));

        let mut end = None;
        while let Some(i) = rx.recv().await {
            match end {
                Some(end) => println!("completed {}/{}", i, end),
                _ => end = Some(i),
            }
        }
        Ok(())
    }
}
