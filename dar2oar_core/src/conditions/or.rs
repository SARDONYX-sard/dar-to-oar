use super::{condition::default_required_version, is_false, ConditionSet};
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// - OAR: OR
/// - DAR: fn_name() OR
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Or {
    /// Condition name "OR"
    pub condition: CompactString,
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: CompactString,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    #[serde(rename = "Conditions")]
    pub conditions: Vec<ConditionSet>,
}

impl Default for Or {
    fn default() -> Self {
        Self {
            condition: "OR".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            conditions: Default::default(),
        }
    }
}
