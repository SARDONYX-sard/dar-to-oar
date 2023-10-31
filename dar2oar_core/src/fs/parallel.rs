use crate::condition_parser::parse_dar2oar;
use crate::conditions::ConditionsConfig;
use crate::fs::path_changer::parse_dar_path;
use crate::fs::{read_file, write_name_space_config, write_section_config, ConvertOptions};
use anyhow::{bail, Context as _, Result};
use jwalk::WalkDir;
use std::path::Path;
use tokio::fs;

/// multi thread converter
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
    let entires = WalkDir::new(&dar_dir).into_iter();

    let mut walk_len = 0;
    if let Some(sender) = sender {
        walk_len = WalkDir::new(dar_dir).into_iter().count(); // Lower performance cost when sender is None
        log::debug!("Dir & File Counts: {}", walk_len);
        sender.send(walk_len).await?;
    }

    for (idx, entry) in entires.enumerate() {
        if let Some(sender) = sender {
            log::debug!("Converted: {}/{}", idx, walk_len);
            sender.send(idx).await?;
        }

        let entry = entry?;
        let path = entry.path(); // Separate this for binding
        let path = path.as_path();

        let (dar_root, oar_name_space_path, is_1st_person, parsed_mod_name, priority, remain) =
            match parse_dar_path(path, None) {
                Ok(data) => data,
                Err(_) => continue, // NOTE: The first search is skipped because it does not yet lead to the DAR file.
            };
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
            log::debug!("Dir: {:?}", path);
        } else if path.extension().is_some() {
            log::debug!("File: {:?}", path);
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
            log::trace!("section root: {:?}", section_root);
            fs::create_dir_all(&section_root).await?;
            if file_name == "_conditions.txt" {
                match read_file(path).await {
                    Ok(content) => {
                        log::trace!("Content:\n{}", content);

                        let config_json = ConditionsConfig {
                            name: section_name.into(),
                            priority: priority.parse()?,
                            conditions: parse_dar2oar(&content)?,
                            ..Default::default()
                        };

                        write_section_config(section_root, config_json).await?
                    }
                    Err(err) => log::error!("Error reading file {path:?}: {err}"),
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
                    fs::create_dir_all(&non_leaf_dir).await?;
                    fs::copy(path, &non_leaf_dir.join(file_name)).await?;
                } else {
                    fs::copy(path, section_root.join(file_name)).await?;
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
            Ok(msg)
        }
        false => bail!("DynamicAnimationReplacer dir was never found"),
    }
}
