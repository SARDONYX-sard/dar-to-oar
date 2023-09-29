use serde::{Deserialize, Serialize};

/// name space config.json
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MainConfig {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: String,
}
