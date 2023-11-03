use std::collections::HashMap;
use std::path::Path;
use tokio::{fs::File, io::AsyncReadExt};

/// Get mapping table from path
pub async fn get_mapping_table(
    mapping_path: Option<impl AsRef<Path>>,
) -> Option<HashMap<String, String>> {
    match mapping_path {
        Some(table_path) => read_mapping_table(table_path).await.ok(),
        None => None,
    }
}

/// Try to read mapping table from path
pub async fn read_mapping_table(
    table_path: impl AsRef<Path>,
) -> anyhow::Result<HashMap<String, String>> {
    let mut file_contents = String::new();
    File::open(table_path)
        .await?
        .read_to_string(&mut file_contents)
        .await?;
    Ok(parse_mapping_table(&file_contents))
}

fn parse_mapping_table(table: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Sequential numbering of duplicate keys when no key is available.
    let mut current_section_name = "";
    let mut idx = 0;
    for line in table.lines() {
        if line.starts_with("//") {
            continue;
        };

        let mapping = match line.find(' ') {
            Some(idx) => {
                let (key, val) = line.split_at(idx);
                (key, Some(val))
            }
            None => (line, None),
        };
        let section_name = match mapping.1 {
            None => {
                idx += 1;
                format!("{}_{}", current_section_name, idx)
            }
            Some(val) => {
                current_section_name = val.trim();
                idx = 0;
                current_section_name.to_string()
            }
        };

        match mapping.0 {
            "" | "\r\n" | "\n" => continue, // Skip blank lines.
            key => map.insert(key.into(), section_name),
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
        expected.insert("8000000".into(), "Combat".to_string());
        expected.insert("8000001".into(), "Base".to_string());
        expected.insert("8000002".into(), "Base_1".to_string());
        expected.insert("8000005".into(), "Female".to_string());
        expected.insert("8001000".into(), "Unarmed".to_string());
        expected.insert("8001010".into(), "Sword".to_string());
        expected.insert("8001020".into(), "Sword+Shield".to_string());

        assert_eq!(result, expected);
    }
}
