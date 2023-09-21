use super::{condition::Condition, ConditionSet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Or {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(rename = "conditions")]
    pub conditions: Vec<ConditionSet>,
}

impl Default for Or {
    fn default() -> Self {
        Self {
            condition: Condition::new("OR"),
            conditions: Default::default(),
        }
    }
}
