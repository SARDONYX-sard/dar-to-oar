mod mapping_table;
mod path_changer;

pub use mapping_table::read_mapping_table;

use crate::condition_parser::parse_dar2oar;
use crate::conditions::{ConditionsConfig, MainConfig};
use crate::fs::path_changer::parse_dar_path;
use anyhow::{Context as _, Result};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn read_file<P>(file_path: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = fs::File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn write_section_config<P>(oar_dir: P, config_json: ConditionsConfig) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let target_path = oar_dir.as_ref().join("config.json");
    let mut config_file = fs::File::create(&target_path).with_context(|| {
        let msg = format!("writing section config target: {:?}", target_path);
        log::error!("{}", msg);
        msg
    })?;
    let json = serde_json::to_string_pretty(&config_json)?;
    config_file.write_all(json.as_bytes())?;
    Ok(())
}

/// If there is no name_space_config file, create one.
/// If it exists, do nothing. (This behavior is intended to facilitate the creation of config files
/// for 1st_person and 3rd_person.)
fn write_name_space_config<P>(
    oar_name_space_path: P,
    mod_name: &str,
    author: Option<&str>,
) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let target_file = oar_name_space_path.as_ref().join("config.json");
    if target_file.exists() {
        return Ok(());
    }

    let config_json = MainConfig {
        name: mod_name.into(),
        author: author.unwrap_or_default().into(),
        ..Default::default()
    };
    fs::create_dir_all(&oar_name_space_path)?;
    let mut config_file = fs::File::create(target_file)?;
    let json = serde_json::to_string_pretty(&config_json)?;
    config_file.write_all(json.as_bytes())?;
    Ok(())
}

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
    for entry in WalkDir::new(dar_dir) {
        let entry = entry?;
        let path = entry.path();
        let (oar_name_space_path, is_1st_person, parsed_mod_name, priority, remain) =
            match parse_dar_path(&path) {
                Ok(data) => data,
                Err(_) => continue, // NOTE: The first search is skipped because it does not yet lead to the DAR file.
            };
        let parsed_mod_name = mod_name
            .and_then(|s| Some(s.to_string()))
            .unwrap_or(parsed_mod_name.unwrap_or("Unknown".into()));
        let oar_name_space_path = oar_dir
            .as_ref()
            .and_then(|path| {
                Some(match is_1st_person {
                    true => path.join(
                        "meshes/actors/character/_1stperson/animations/OpenAnimationReplacer",
                    ),
                    false => path.join("meshes/actors/character/animations/OpenAnimationReplacer"),
                })
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
                match read_file(&path) {
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

                write_name_space_config(&oar_name_space_path, &parsed_mod_name, author)
                    .with_context(|| {
                        format!(
                            "Failed to write name space config to: {:?}",
                            oar_name_space_path
                        )
                    })?;
            } else {
                // maybe motion files(.hex)
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

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fs::mapping_table::read_mapping_table;

    #[ignore]
    #[test]
    fn should_traverse() -> anyhow::Result<()> {
        let config = simple_log::LogConfigBuilder::builder()
            .path("../convert.log")
            .size(1 * 100)
            .roll_count(10)
            .level("error")
            .output_file()
            .output_console()
            .build();
        simple_log::new(config).unwrap();

        let table_content = "../test/settings/mapping_table.txt";
        let mapping = read_mapping_table(table_content)?;
        convert_dar_to_oar(
            "../test/data/Smooth Moveset",
            None,
            None,
            None,
            Some(mapping),
            None,
        )
    }
}
