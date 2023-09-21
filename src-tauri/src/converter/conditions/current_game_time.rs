use super::condition::Condition;
use crate::converter::values::{Cmp, NumericValue};
use serde::{Deserialize, Serialize};

/// Compare current game time and numeric value.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CurrentGameTime {
    pub condition: Condition,
    pub comparison: Cmp,
    pub numeric_value: NumericValue,
}
