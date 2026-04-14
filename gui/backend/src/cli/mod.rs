pub(super) mod error;
pub(super) mod handler;

use std::collections::HashMap;
use tauri_plugin_cli::ArgData;

use crate::cli::error::CliError;

fn get_string(map: &HashMap<String, ArgData>, key: &str) -> Result<String, CliError> {
    map.get(key)
        .and_then(|v| v.value.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| CliError::MissingArg {
            key: key.to_string(),
        })
}

fn get_opt_string(map: &HashMap<String, ArgData>, key: &str) -> Option<String> {
    map.get(key)
        .and_then(|v| v.value.as_str())
        .map(|s| s.to_string())
}
