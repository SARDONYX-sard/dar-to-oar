//! Wrapper for wrapping pluginValue with a key called `form`
use super::PluginValue;
use serde::{Deserialize, Serialize};

/// Wrapper for wrapping pluginValue with a key called `form`
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormValue {
    /// A combination of the plugin name and the ID in it.
    pub form: PluginValue,
}

impl From<PluginValue> for FormValue {
    fn from(value: PluginValue) -> Self {
        Self { form: value }
    }
}
