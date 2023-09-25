use crate::converter::condition_parser::parse_dar2oar;
use crate::converter::conditions::ConditionSet;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self};
use std::io::Write;
use std::path::{Path, PathBuf};


/// root block in each config.json
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConditionsConfig {
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    priority: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_animations_folder: Option<String>,
    #[serde(default)]
    conditions: Vec<ConditionSet>,
}

/// name space config.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MainConfig {
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    author: String,
}

/// Path from just under the `data` directory in the Skyrim root directory to `actor`.
/// So, in `"Skyrim SpecialEdition/data/"`
const ACTOR_PATH: &str = "meshes\\actors";
const FIRST_PERSON_DIR_NAME: &str = "_1stperson";
const ANIM_DIR_NAME: &str = "animations";
const DAR_DIR_NAME: &str = "DynamicAnimationReplacer";
const DAR_CONDITIONS_DIR_NAME: &str = "_CustomConditions";
const OAR_DIR_NAME: &str = "OpenAnimationReplacer";
const DAR_CONDITIONS_FILE_NAME: &str = "_conditions.txt";
const CONFIG_FILE_NAME: &str = "config.json";

/// # Parameters
/// - mod_name: If this item is None, the trailing directory name of oar_mod_folder is used.
///     - e.g. mod_name: None, oar_mod_folder: "./test/oar" => mod_name: "oar"
/// - oar_config_path: Intermediate path to config.json (NOTE: oar_mod_folder is root dir)
fn generate_main_config_file(
    dar_actor_folder: &Path,
    oar_mod_folder: &Path,
    oar_config_path: &str,
    mod_name: Option<&str>,
    author: Option<&str>,
) -> Result<PathBuf, Box<dyn Error>> {
    let mod_name =
        mod_name.unwrap_or_else(|| oar_mod_folder.file_name().unwrap().to_str().unwrap());
    let name = format!(
        "{}-{}",
        mod_name,
        dar_actor_folder.file_name().unwrap().to_str().unwrap()
    );
    let oar_mod_path = oar_mod_folder.join(oar_config_path).join(&name);

    fs::create_dir_all(&oar_mod_path)?;

    let mc = MainConfig {
        author: author.unwrap_or_default().to_string(),
        name: name.to_string(),
        description: String::default(),
    };

    let file_path = oar_mod_path.join(CONFIG_FILE_NAME);
    // Serialize and write the MainConfig to the config file
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_path)?;
    let json = serde_json::to_string_pretty(&mc)?;
    file.write_all(json.as_bytes())?;

    Ok(file_path)
}

fn build_oar_directory(
    dar_actor_folder: &Path,
    oar_mod_folder: &Path,
    oar_config_path: &str,
    overwrite: bool,
    mod_name: Option<&str>,
    mod_author: Option<&str>,
) -> Result<PathBuf, Box<dyn Error>> {
    let oar_animations_folder = generate_main_config_file(
        dar_actor_folder,
        oar_mod_folder,
        oar_config_path,
        mod_name,
        mod_author,
    )?;
    let dar_conditions_folder = dar_actor_folder
        .join(ANIM_DIR_NAME)
        .join(DAR_DIR_NAME)
        .join(DAR_CONDITIONS_DIR_NAME);

    copy_directory(
        &dar_conditions_folder,
        &oar_animations_folder,
        overwrite,
        true,
    )?;

    Ok(oar_animations_folder)
}

fn copy_directory(
    src: &Path,
    dst: &Path,
    overwrite: bool,
    recursive: bool,
) -> Result<(), Box<dyn Error>> {
    if !src.exists() {
        return Err(format!("Source directory '{}' does not exist.", src.display()).into());
    }

    if !dst.exists() {
        fs::create_dir_all(&dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_type = entry.file_type()?;
        let entry_path = entry.path();
        let new_entry_path = dst.join(entry_path.file_name().unwrap());

        if entry_type.is_dir() {
            if recursive {
                copy_directory(&entry_path, &new_entry_path, overwrite, recursive)?;
            }
        } else if entry_type.is_file() {
            if new_entry_path.exists() && !overwrite {
                continue;
            }

            fs::copy(&entry_path, &new_entry_path)?;
        }
    }

    Ok(())
}

fn build_oar_directories(
    oar_mod_folder: &Path,
    dar_mod_folder: &Path,
    overwrite: bool,
    mod_name: Option<&str>,
    mod_author: Option<&str>,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let dar_directory_info = dar_mod_folder.join(ACTOR_PATH);
    let dar_actor_folders = fs::read_dir(&dar_directory_info)?
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                Some(entry.path())
            } else {
                None
            }
        })
        .filter(|path| path.is_dir());

    let mut animations_directories = Vec::new();

    for dar_actor_folder in dar_actor_folders {
        if dar_actor_folder.join(FIRST_PERSON_DIR_NAME).exists() {
            let oar_1st_person_config_path = Path::new(ACTOR_PATH)
                .join(dar_actor_folder.file_name().unwrap_or_default())
                .join(FIRST_PERSON_DIR_NAME)
                .join(ANIM_DIR_NAME)
                .join(OAR_DIR_NAME);

            let first_person_dar_directory = dar_actor_folder.join(FIRST_PERSON_DIR_NAME);
            let oar_animations_1st_person_folder = build_oar_directory(
                &first_person_dar_directory,
                oar_mod_folder,
                &oar_1st_person_config_path.to_string_lossy(),
                overwrite,
                mod_name,
                mod_author,
            )?;
            animations_directories.push(oar_animations_1st_person_folder);
        }

        let oar_config_path = Path::new(ACTOR_PATH)
            .join(dar_actor_folder.file_name().unwrap_or_default())
            .join(ANIM_DIR_NAME)
            .join(OAR_DIR_NAME);

        let oar_animations_folder = build_oar_directory(
            &dar_actor_folder,
            oar_mod_folder,
            &oar_config_path.to_string_lossy(),
            true,
            mod_name,
            mod_author,
        )?;
        animations_directories.push(oar_animations_folder);
    }

    Ok(animations_directories)
}

fn generate_conditions_config_file(conditions_file: &Path) -> Result<(), Box<dyn Error>> {
    let priority = conditions_file
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|dir_name| dir_name.to_str())
        .map(|name| name.parse::<i32>().unwrap_or(0))
        .unwrap_or(0);

    let name = conditions_file
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|dir_name| dir_name.to_str())
        .unwrap_or_default();

    let conditions_folder = conditions_file
        .parent()
        .and_then(|parent| parent.to_str())
        .unwrap_or_default();

    let binding = fs::read_to_string(conditions_file)?;
    println!("Parsing conditions at path: {}", conditions_folder);

    let conditions_list = parse_dar2oar(&binding)?;
    let config = ConditionsConfig {
        name: name.to_string(),
        description: String::default(),
        priority,
        override_animations_folder: Some(conditions_folder.to_string()),
        conditions: conditions_list,
    };

    let config_file_path = Path::new(conditions_folder).join("config.json");

    let config_file = fs::File::create(config_file_path)?;
    serde_json::to_writer_pretty(config_file, &config)?;

    fs::remove_file(conditions_file)?;

    Ok(())
}

fn convert_conditions(oar_conditions_folders: &Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    for folder in oar_conditions_folders {
        let conditions_files = fs::read_dir(&folder)?
            .filter_map(|entry| {
                if let Ok(entry) = entry {
                    Some(entry.path())
                } else {
                    None
                }
            })
            .filter(|path| {
                path.is_file() && path.file_name().unwrap_or_default() == DAR_CONDITIONS_FILE_NAME
            });

        for conditions_file in conditions_files {
            generate_conditions_config_file(&conditions_file)?;
        }
    }

    Ok(())
}

pub fn convert_dar_to_oar(
    dar_mod_folder: &Path,
    oar_mod_folder: &Path,
    mod_name: Option<&str>,
    mod_author: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(oar_mod_folder)?;

    let directories =
        build_oar_directories(&oar_mod_folder, dar_mod_folder, true, mod_name, mod_author)?;
    convert_conditions(&directories)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_create_config_file() {
        let dar_actor_folder = Path::new("../test/Smooth Moveset");
        let oar_mod_folder = Path::new("../test/Smooth Moveset");
        let oar_config_path = "config.json";
        let mod_name = Some("Smooth Moveset");
        let author = None;
        dbg!(generate_main_config_file(
            dar_actor_folder,
            oar_mod_folder,
            oar_config_path,
            mod_name,
            author,
        ));
    }
}
