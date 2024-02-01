//! Specifically for the 'config.json' namespace.
use serde::{Deserialize, Serialize};

/// Represents the configuration structure for the 'config.json' namespace.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MainConfig<'config> {
    /// The name associated with the configuration.
    #[serde(default)]
    pub name: &'config str,

    /// The description associated with the configuration.
    #[serde(default)]
    pub description: &'config str,

    /// The author associated with the configuration.
    #[serde(default)]
    pub author: &'config str,
}
