use super::{condition::default_required_version, is_false};
use crate::converter::values::{Cmp, NumericValue, RandomValue};
use serde::{Deserialize, Serialize};

/// - OAR: Random
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RandomCondition {
    /// Condition name "Random"
    pub condition: String,
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    #[serde(default)]
    #[serde(rename = "Random value")]
    pub random_value: RandomValue,
    #[serde(default)]
    #[serde(rename = "Comparison")]
    pub comparison: Cmp,
    #[serde(default)]
    #[serde(rename = "Numeric value")]
    pub numeric_value: NumericValue,
}

impl Default for RandomCondition {
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
