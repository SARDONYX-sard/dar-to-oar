mod mapping_table;
pub mod parallel;
pub mod path_changer;
mod sequential;

pub use mapping_table::read_mapping_table;
pub use sequential::convert_dar_to_oar;

use crate::conditions::{ConditionsConfig, MainConfig};
use anyhow::Context as _;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

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

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn should_parallel_traverse() -> anyhow::Result<()> {
        let config = simple_log::LogConfigBuilder::builder()
            .path("../convert.log")
            .size(100)
            .roll_count(10)
            .level("error")
            .output_file()
            .output_console()
            .build();
        simple_log::new(config).unwrap();

        let table_content = "../test/settings/mapping_table.txt";
        let mapping = read_mapping_table(table_content)?;
        convert_dar_to_oar(
            "../test/data/Modern Female Sitting Animations Overhaul",
            None,
            None,
            None,
            Some(mapping),
            None,
        )
    }
}
