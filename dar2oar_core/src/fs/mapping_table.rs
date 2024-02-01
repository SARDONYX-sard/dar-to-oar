//! Module for handling mapping tables in the conversion process.
//!
//! This module provides functions to read and parse mapping tables from files asynchronously.
use crate::error::{ConvertError, Result};
use compact_str::CompactString;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs::read_to_string;

/// Get mapping table from path
pub async fn get_mapping_table(
    mapping_path: Option<impl AsRef<Path>>,
) -> Option<HashMap<CompactString, String>> {
    match mapping_path {
        Some(table_path) => read_mapping_table(table_path).await.ok(),
        None => None,
    }
}

/// Try to read mapping table from path
///
/// # Errors
/// Path is not exist.
pub async fn read_mapping_table(
    table_path: impl AsRef<Path>,
) -> Result<HashMap<CompactString, String>> {
    let table_path = table_path.as_ref();
    if !table_path.exists() {
        return Err(ConvertError::NonExistPath(format!("{table_path:?}")));
    };

    let contents = read_to_string(table_path).await?;
    Ok(parse_mapping_table(&contents))
}

/// Parse the mapping table from a string.
///
/// This function takes a string representing the mapping table and parses it into a [`HashMap`].
/// It handles sequential numbering of duplicate keys when no key is available.
///
/// # Information
/// The key can only be up to [`f32`]`::MAX` due to DAR specifications.
/// Therefore, [`CompactString`] is used to fit into 24 bytes.
fn parse_mapping_table(table: &str) -> HashMap<CompactString, String> {
    let mut map = HashMap::new();

    // Sequential numbering of duplicate keys when no key is available.
    let mut current_section_name = "";
    let mut idx = 0;
    for line in table.lines() {
        if line.starts_with("//") {
            continue;
        };

        let mapping = line.find(' ').map_or((line, None), |sep_idx| {
            let (key, val) = line.split_at(sep_idx);
            (key, Some(val))
        });
        let section_name = match mapping.1 {
            None => {
                idx += 1;
                format!("{current_section_name}_{idx}")
            }
            Some(val) => {
                current_section_name = val.trim();
                idx = 0;
                current_section_name.to_string()
            }
        };

        let _ = match mapping.0 {
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
        let input = "
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
8001020  Sword+Shield";

        let result = parse_mapping_table(input);

        let expected = [
            ("8000000".into(), "Combat".into()),
            ("8000001".into(), "Base".into()),
            ("8000002".into(), "Base_1".into()),
            ("8000005".into(), "Female".into()),
            ("8001000".into(), "Unarmed".into()),
            ("8001010".into(), "Sword".into()),
            ("8001020".into(), "Sword+Shield".into()),
        ]
        .into();
        assert_eq!(result, expected);
    }
}
