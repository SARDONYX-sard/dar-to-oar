use super::ConditionSet;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// Each animation root config.json
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConditionsConfig {
    /// # NOTE
    /// An arbitrary name given by the user (value in the mapping table) will probably exceed 24bytes.
    /// Therefore, it should not be a [CompactString].
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: CompactString,
    #[serde(default)]
    pub priority: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "overrideAnimationsFolder")]
    pub override_animations_folder: Option<CompactString>,
    #[serde(default)]
    pub conditions: Vec<ConditionSet>,
}
