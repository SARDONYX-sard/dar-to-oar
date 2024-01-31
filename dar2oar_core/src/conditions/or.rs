//! OR condition
use super::{condition::default_required_version, is_false, ConditionSet};
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// Represents the "OR" condition in the OAR of functions in the DAR.
///
/// - OAR: OR
/// - DAR: `fn_name() OR`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Or {
    /// The name of the condition, which is "OR".
    pub condition: CompactString,
    /// The required version for compatibility with this condition.
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: CompactString,
    /// Indicates whether the condition is negated.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    /// A vector containing the sub-conditions for the "OR" condition.
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
