use super::condition::Condition;
use crate::converter::values::Keyword;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IsWornHasKeyword {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(rename = "Keyword")]
    #[serde(default)]
    pub keyword: Keyword,
}

impl Default for IsWornHasKeyword {
    fn default() -> Self {
        Self {
            condition: Condition::new("IsEquipped"),
            keyword: Default::default(),
        }
    }
}
