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

pub(super) fn parse_mapping_table(table: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Sequential numbering of duplicate keys when no key is available.
    let mut current_section_name = String::new();
    let mut idx = 0;
    for line in table.lines() {
        if line.starts_with("//") {
            continue;
        };

        let mapping = match line.find(' ') {
            Some(idx) => line.split_at(idx),
            None => (line, ""),
        };
        let section_name = match mapping.1 {
            "" => {
                idx += 1;
                format!("{}_{}", current_section_name, idx)
            }
            val => {
                current_section_name = val.trim().to_string();
                idx = 0;
                current_section_name.clone()
            }
        };

        match mapping.0 {
            "" | "\r\n" | "\n" => continue, // Skip blank lines.
            key => map.insert(key.to_string(), section_name.clone()),
        };
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_mapping_table() {
        let input = r#"
8000000  Combat
8000001
8000001  Base
// This is a line comment
8000002
8000005
8000005  Female
8001000
8001000  Unarmed
8001010
8001010  Sword
8001020
8001020  Sword+Shield"#;

        let result = parse_mapping_table(input);

        let mut expected = HashMap::new();
        expected.insert("8000000".to_string(), "Combat".to_string());
        expected.insert("8000001".to_string(), "Base".to_string());
        expected.insert("8000002".to_string(), "Base_1".to_string());
        expected.insert("8000005".to_string(), "Female".to_string());
        expected.insert("8001000".to_string(), "Unarmed".to_string());
        expected.insert("8001010".to_string(), "Sword".to_string());
        expected.insert("8001020".to_string(), "Sword+Shield".to_string());

        assert_eq!(result, expected);
    }
}
