use anyhow::bail;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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

    // Sequential numbering of duplicate keys when no key is available.
    let mut current_section_name = String::new();
    let mut idx = 0;
    for line in table.lines() {
        let mapping: Vec<&str> = line.split_whitespace().collect();
        let section_name = match mapping.get(1) {
            Some(val) => {
                current_section_name = val.to_string();
                idx = 0;
                current_section_name.clone()
            }
            None => {
                idx += 1;
                format!("{}_{}", current_section_name, idx)
            }
        };

        map.insert(mapping[0].to_string(), section_name.clone());
    }

    map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_read_settings() {
        let table_content = include_str!("../../../test/settings/mapping_table.txt");
        dbg!(parse_mapping_table(table_content));
    }
}
