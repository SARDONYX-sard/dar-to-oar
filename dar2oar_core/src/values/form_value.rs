use super::PluginValue;
use serde::{Deserialize, Serialize};

/// Wrapper for wrapping pluginValue with a key called "form"
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FormValue {
    pub form: PluginValue,
}

impl From<PluginValue> for FormValue {
    fn from(value: PluginValue) -> Self {
        Self { form: value }
    }
}
