//! OR condition
use super::{condition::default_required_version, is_false, ConditionSet};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents the "OR" condition in the OAR of functions in the DAR.
///
/// - OAR: OR
/// - DAR: `fn_name() OR`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Or<'a> {
    /// The name of the condition, which is "OR".
    pub condition: Cow<'a, str>,
    /// The required version for this condition.
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: Cow<'a, str>,
    /// Indicates whether the condition is negated or not.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    /// A vector containing the sub-conditions for the "OR" condition.
    #[serde(rename = "Conditions")]
    pub conditions: Vec<ConditionSet<'a>>,
}

impl Default for Or<'_> {
    fn default() -> Self {
        Self {
            condition: "OR".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            conditions: Default::default(),
        }
    }
}
