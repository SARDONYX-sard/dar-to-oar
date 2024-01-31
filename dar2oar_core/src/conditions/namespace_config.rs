//! Specifically for the 'config.json' namespace.
use serde::{Deserialize, Serialize};

/// Represents the configuration structure for the 'config.json' namespace.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MainConfig<'a> {
    /// The name associated with the configuration.
    #[serde(default)]
    pub name: &'a str,

    /// The description associated with the configuration.
    #[serde(default)]
    pub description: &'a str,

    /// The author associated with the configuration.
    #[serde(default)]
    pub author: &'a str,
}
