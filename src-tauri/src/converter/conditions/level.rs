use super::condition::Condition;
use crate::converter::values::{Cmp, NumericValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Level {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(rename = "Comparison")]
    #[serde(default)]
    pub comparison: Cmp,
    #[serde(rename = "Numeric value")]
    #[serde(default)]
    pub numeric_value: NumericValue,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            condition: Condition::new("Level"),
            comparison: Default::default(),
            numeric_value: Default::default(),
        }
    }
}
