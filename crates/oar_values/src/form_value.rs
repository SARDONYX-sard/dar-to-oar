//! Wrapper for wrapping pluginValue with a key called `form`
use super::PluginValue;
use serde::{Deserialize, Serialize};

/// Wrapper for wrapping pluginValue with a key called `form`
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormValue<'a> {
    /// A combination of the plugin name and the ID in it.
    pub form: PluginValue<'a>,
}

impl<'a> From<PluginValue<'a>> for FormValue<'a> {
    fn from(value: PluginValue<'a>) -> Self {
        Self { form: value }
    }
}
