use super::{condition::Condition, is_false};
use crate::converter::values::{Cmp, NumericValue, RandomValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RandomCondition {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(default)]
    pub comparison: Cmp,
    #[serde(rename = "Random value")]
    #[serde(default)]
    pub random_value: RandomValue,
    #[serde(rename = "Numeric value")]
    #[serde(default)]
    pub numeric_value: NumericValue,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub keep_random_results_on_loop: bool,
}

impl Default for RandomCondition {
    fn default() -> Self {
        Self {
            condition: Condition::new("Random"),
            comparison: Default::default(),
            random_value: Default::default(),
            numeric_value: Default::default(),
            keep_random_results_on_loop: true,
        }
    }
}
