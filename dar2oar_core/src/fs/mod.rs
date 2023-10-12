mod mapping_table;
pub mod parallel;
pub mod path_changer;
mod sequential;

pub use mapping_table::read_mapping_table;
pub use sequential::convert_dar_to_oar;

use crate::conditions::{ConditionsConfig, MainConfig};
use anyhow::Context as _;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Default, PartialEq)]
pub struct ConvertOptions<'a, P: AsRef<Path>> {
    /// DAR source dir path
    pub dar_dir: P,
    /// OAR destination dir path(If not, it is inferred from src)
    pub oar_dir: Option<PathBuf>,
    /// mod name in config.json & directory name(If not, it is inferred from src)
    pub mod_name: Option<&'a str>,
    /// mod author in config.json
    pub author: Option<&'a str>,
    /// path to section name table
    pub section_table: Option<HashMap<String, String>>,
    /// path to section name table(For _1st_person)
    pub section_1person_table: Option<HashMap<String, String>>,
    /// After converting to OAR, add mohidden to the DAR directory before conversion to treat it as a hidden directory. (for MO2 users)
    pub hide_dar: bool,
}

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

/// # Returns
/// Report which dirs have been restored
///
/// # NOTE
/// It is currently used only in GUI, but is implemented in Core as an API.
pub fn restore_dar(dar_dir: impl AsRef<Path>) -> anyhow::Result<String> {
    let mut restored_dar = None;
    let mut restored_1st_dar = None;
    for entry in walkdir::WalkDir::new(dar_dir) {
        let entry = entry?;
        let path = entry.path();
        let (dar_root, _, is_1st_person, _, _, _) =
            match path_changer::parse_dar_path(path, Some("DynamicAnimationReplacer.mohidden")) {
                Ok(data) => data,
                Err(_) => continue, // NOTE: The first search is skipped because it does not yet lead to the DAR file.
            };

        if restored_dar.is_none() && path.is_dir() {
            restored_dar = Some(dar_root);
            continue;
        }
        if restored_1st_dar.is_none() && path.is_dir() && is_1st_person {
            restored_1st_dar = Some(dar_root);
        }
    }

    let mut msg = String::new();
    if let Some(dar_root) = restored_dar.as_ref() {
        let dist = dar_root
            .as_os_str()
            .to_string_lossy()
            .replace(".mohidden", "");
        fs::rename(dar_root.clone(), dist)?;
        msg = format!("{}- Restored 3rd_person", msg);
    }
    if let Some(dar_root) = restored_1st_dar.as_ref() {
        let dist = dar_root
            .as_os_str()
            .to_string_lossy()
            .replace(".mohidden", "");
        fs::rename(dar_root.clone(), dist)?;
        msg = format!("{}\n- Restored 1rd_person", msg);
    }

    if restored_dar.is_none() && restored_1st_dar.is_none() {
        anyhow::bail!("Neither 1st or 3rd person DynamicAnimationReplacer.mohidden found.")
    } else {
        Ok(msg)
    }
}

/// # NOTE
/// It is currently used only in GUI, but is implemented in Core as an API.
pub fn remove_oar(dar_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let mut restored_dar = None;
    let mut restored_1st_dar = None;
    for entry in walkdir::WalkDir::new(dar_dir) {
        let entry = entry?;
        let path = entry.path();
        // NOTE: The OAR root obtained by parse fn is calculated and not guaranteed to exist.
        let (dar_root, oar_name_space_path, is_1st_person, _, _, _) =
            match path_changer::parse_dar_path(path, Some("DynamicAnimationReplacer.mohidden")) {
                Ok(data) => data,
                Err(_) => {
                    match path_changer::parse_dar_path(path, Some("DynamicAnimationReplacer")) {
                        Ok(data) => data,
                        Err(_) => continue, // NOTE: The first search is skipped because it does not yet lead to the DAR file.
                    }
                } // NOTE: The first search is skipped because it does not yet lead to the DAR file.
            };

        if restored_dar.is_none() && path.is_dir() {
            restored_dar = Some((dar_root, oar_name_space_path));
            continue;
        }
        if restored_1st_dar.is_none() && path.is_dir() && is_1st_person {
            restored_1st_dar = Some((dar_root, oar_name_space_path));
        }
    }

    if let Some((_, oar_root)) = restored_dar {
        dbg!(&oar_root);
        if oar_root.exists() {
            fs::remove_dir_all(oar_root)?;
        }
    }
    if let Some((_, oar_root)) = restored_1st_dar {
        dbg!(&oar_root);
        if oar_root.exists() {
            fs::remove_dir_all(oar_root)?;
        }
    }
    Ok(())
}

#[ignore]
#[test]
fn should_restore_dar() {
    let dar_dir = "../test/data/UNDERDOG Animations";
    remove_oar(dar_dir).unwrap();
}
