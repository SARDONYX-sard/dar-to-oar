use crate::condition_parser::parse_dar2oar;
use crate::conditions::ConditionsConfig;
use crate::fs::path_changer::parse_dar_path;
use crate::fs::{read_file, write_name_space_config, write_section_config};
use anyhow::{Context as _, Result, bail};
use jwalk::WalkDir;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// multi thread converter
pub fn convert_dar_to_oar<P>(
    dar_dir: P,
    oar_dir: Option<PathBuf>,
    mod_name: Option<&str>,
    author: Option<&str>,
    section_table: Option<HashMap<String, String>>,
    section_1person_table: Option<HashMap<String, String>>,
) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut is_converted_once = false;

    for entry in WalkDir::new(dar_dir) {
        let entry = entry?;
        let path = entry.path(); // Separate this for binding
        let path = path.as_path();
        let (oar_name_space_path, is_1st_person, parsed_mod_name, priority, remain) =
            match parse_dar_path(path) {
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
            fs::create_dir_all(&section_root)?;
            if file_name == "_conditions.txt" {
                match read_file(path) {
                    Ok(content) => {
                        log::trace!("Content:\n{}", content);

                        let config_json = ConditionsConfig {
                            name: section_name.into(),
                            priority: priority.parse()?,
                            conditions: parse_dar2oar(&content)?,
                            ..Default::default()
                        };

                        write_section_config(section_root, config_json)?
                    }
                    Err(err) => log::error!("Error reading file {path:?}: {err}"),
                }

                if !is_converted_once {
                    is_converted_once = true;
                    write_name_space_config(&oar_name_space_path, &parsed_mod_name, author)
                        .with_context(|| {
                            format!(
                                "Failed to write name space config to: {:?}",
                                oar_name_space_path
                            )
                        })?;
                }
            } else {
                // maybe motion files(.hkx)
                if let Some(remain) = remain {
                    let non_leaf_dir = section_root.join(remain);
                    fs::create_dir_all(&non_leaf_dir)?;
                    fs::copy(path, &non_leaf_dir.join(file_name))?;
                } else {
                    fs::copy(path, section_root.join(file_name))?;
                }
            }
        }
    }

    match is_converted_once {
        true => Ok(()),
        false => bail!("DynamicAnimationReplacer dir was never found"),
    }
}
