mod constants;
mod visiter;
mod write;

use super::condition_parser::parse_dar2oar;
use super::conditions::ConditionSet;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Each animation root config.json
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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

fn write_configs<P>(file_path: P, content: &str) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let mut mapping = HashMap::new();
    mapping.insert("8213000", "Hey".to_string());

    let file_path_str = file_path.as_ref().to_str().unwrap_or_default();

    if let Some(idx) = file_path_str.find("DynamicAnimationReplacer") {
        let priority = file_path
            .as_ref()
            .ancestors()
            .skip_while(|path| path.file_name() == Some(OsStr::new("_conditions.txt")))
            .next()
            .unwrap()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let name: String = mapping
            .get(priority.as_str())
            .unwrap_or(&"".to_owned())
            .to_string();

        let target_path = Path::new(&file_path_str[0..idx]).join("OpenAnimationReplacer");

        // parse _condition.txt
        let config_json = ConditionsConfig {
            name,
            priority: priority.parse().unwrap_or_default(),
            conditions: parse_dar2oar(&content).unwrap(),
            ..Default::default()
        };

        fs::create_dir_all(&target_path)?;

        // config.jsonを生成した新しいパスに書き込む
        let mut config_file = fs::File::create(&target_path.join("config.json")).unwrap();
        let json = serde_json::to_string_pretty(&config_json).unwrap();
        config_file.write_all(json.as_bytes()).unwrap();
    }
    Ok(())
}
