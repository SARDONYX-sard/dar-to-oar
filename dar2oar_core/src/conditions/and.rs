use super::{condition::default_required_version, is_false, ConditionSet};
use serde::{Deserialize, Serialize};

/// - OAR: AND
/// - DAR: fn_name() AND
///
/// - NOTE:
///   Fields other than Fields other than conditions are never used in DAR to OAR.
///   In DAR, AND is pushed up to the root conditions.
///   The non-conditions definitions exist in anticipation of future OAR parsing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct And {
    /// Condition name "AND"
    pub condition: String,
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    #[serde(rename = "Conditions")]
    pub conditions: Vec<ConditionSet>,
}

impl Default for And {
    fn default() -> Self {
        Self {
            condition: "AND".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            conditions: Default::default(),
        }
    }
}
