use super::ConditionSet;
use serde::{Deserialize, Serialize};

/// Each animation root config.json
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConditionsConfig {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub priority: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "overrideAnimationsFolder")]
    pub override_animations_folder: Option<String>,
    #[serde(default)]
    pub conditions: Vec<ConditionSet>,
}
