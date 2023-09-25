use anyhow::bail;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct ParseSettings {
    src_dir: String,
    dist_dir: String,
    mod_name: String,
    mod_author: String,
    mapping_table: HashMap<String, String>,
}

pub fn read_mapping_table(table_path: impl AsRef<Path>) -> anyhow::Result<HashMap<String, String>> {
    let mut file_contents = String::new();
    match File::open(table_path) {
        Ok(mut file) => match file.read_to_string(&mut file_contents) {
            Ok(_) => Ok(parse_mapping_table(&file_contents)),
            Err(e) => bail!("Error reading file: {}", e),
        },
        Err(e) => bail!("Error opening file: {}", e),
    }
}

fn parse_mapping_table(table: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    let mut current_key = String::new();
    for line in table.lines() {
        let mapping: Vec<&str> = line.split_whitespace().collect();
        let value = match mapping.get(1) {
            Some(key) => {
                current_key = key.to_string();
                key
            }
            None => current_key.as_str(),
        };
        map.insert(mapping[0].to_string(), value.to_string());
    }

    map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_read_settings() {
        let table_content = include_str!("../../../../test/settings/mapping_table.txt");
        dbg!(parse_mapping_table(table_content));
    }
}
