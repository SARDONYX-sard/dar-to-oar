//! Represents a logical AND condition set.
use super::{condition::default_required_version, is_false, ConditionSet};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents a logical AND condition set.
///
/// - OAR: AND
/// - DAR: `fn_name() AND`
///
/// # NOTE
/// Fields other than conditions are never used in DAR to OAR.
/// In DAR, AND is pushed up to the root conditions.
/// The non-conditions definitions exist in anticipation of future OAR parsing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct And<'a> {
    /// The name of the condition, which is "AND".
    pub condition: Cow<'a, str>,
    /// The required version for this condition.
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: Cow<'a, str>,
    /// Indicates whether the condition is negated or not.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    /// The list of conditions forming the logical AND.
    #[serde(rename = "Conditions")]
    pub conditions: Vec<ConditionSet<'a>>,
}

impl Default for And<'_> {
    fn default() -> Self {
        Self {
            condition: "AND".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            conditions: Default::default(),
        }
    }
}
