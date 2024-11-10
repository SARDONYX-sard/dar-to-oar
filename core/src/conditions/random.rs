//! Represents a condition involving randomness.
use super::{condition::default_required_version, is_false};
use crate::values::{Cmp, NumericValue, RandomValue};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents a condition involving randomness.
///
/// - OAR (Object Arithmetic Representation): Random
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RandomCondition<'a> {
    /// The name of the condition, which is "Random".
    pub condition: Cow<'a, str>,
    /// The required version for this condition.
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: Cow<'a, str>,
    /// Indicates whether the condition is negated or not.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    /// The random value used in the condition.
    #[serde(default)]
    #[serde(rename = "Random value")]
    pub random_value: RandomValue,
    /// The comparison operator to use in the condition.
    #[serde(default)]
    #[serde(rename = "Comparison")]
    pub comparison: Cmp,
    /// The numeric value to compare against in the condition.
    #[serde(default)]
    #[serde(rename = "Numeric value")]
    pub numeric_value: NumericValue<'a>,
}

impl Default for RandomCondition<'_> {
    fn default() -> Self {
        Self {
            condition: "Random".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            random_value: Default::default(),
            comparison: Default::default(),
            numeric_value: Default::default(),
        }
    }
}
