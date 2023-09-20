use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use conditions::ConditionSet;
use parser::parse_conditions;

mod conditions;
mod parser;
mod values;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConditionsConfig {
    name: Option<String>,
    description: Option<String>,
    priority: Option<i32>,
    override_animations_folder: Option<String>,
    conditions: Option<Vec<ConditionSet>>, // Assuming conditions can be of various types
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MainConfig {
    name: Option<String>,
    description: Option<String>,
    author: Option<String>,
}

const ACTOR_FOLDER: &str = "meshes\\actors";
const FIRST_PERSON_FOLDER: &str = "_1stperson";
const ANIMATION_FOLDER: &str = "animations";
const DAR_FOLDER: &str = "DynamicAnimationReplacer";
const DAR_CONDITIONS_FOLDER: &str = "_CustomConditions";
const OAR_FOLDER: &str = "OpenAnimationReplacer";
const DAR_CONDITIONS_FILE_NAME: &str = "_conditions.txt";
const CONFIG_FILE_NAME: &str = "config.json";

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
    let file_path = oar_mod_path.join(CONFIG_FILE_NAME);

    fs::create_dir_all(&oar_mod_path)?;

    let mc = MainConfig {
        author: author.map(|a| a.to_string()),
        name: Some(name.to_string()),
        description: None,
    };

    // Serialize and write the MainConfig to the config file
    let mut file = File::create(&file_path)?;
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
        .join(ANIMATION_FOLDER)
        .join(DAR_FOLDER)
        .join(DAR_CONDITIONS_FOLDER);

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
    let dar_directory_info = dar_mod_folder.join(ACTOR_FOLDER);
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
        if dar_actor_folder.join(FIRST_PERSON_FOLDER).exists() {
            let oar_1st_person_config_path = Path::new(ACTOR_FOLDER)
                .join(dar_actor_folder.file_name().unwrap_or_default())
                .join(FIRST_PERSON_FOLDER)
                .join(ANIMATION_FOLDER)
                .join(OAR_FOLDER);

            let first_person_dar_directory = dar_actor_folder.join(FIRST_PERSON_FOLDER);
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

        let oar_config_path = Path::new(ACTOR_FOLDER)
            .join(dar_actor_folder.file_name().unwrap_or_default())
            .join(ANIMATION_FOLDER)
            .join(OAR_FOLDER);

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
    let conditions = binding
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>();

    println!("Parsing conditions at path: {}", conditions_folder);

    let conditions_list = parse_conditions(&conditions);
    let config = ConditionsConfig {
        name: Some(name.to_string()),
        description: None,
        priority: Some(priority),
        override_animations_folder: Some(conditions_folder.to_string()),
        conditions: Some(conditions_list),
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
