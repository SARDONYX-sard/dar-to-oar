use super::ConditionSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct And {
    pub conditions: Vec<ConditionSet>,
}

impl Default for And {
    fn default() -> Self {
        Self {
            conditions: Default::default(),
        }
    }
}
