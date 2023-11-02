use serde::{Deserialize, Serialize};

/// name space config.json
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MainConfig<'a> {
    #[serde(default)]
    pub name: &'a str,
    #[serde(default)]
    pub description: &'a str,
    #[serde(default)]
    pub author: &'a str,
}
