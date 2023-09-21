use super::condition::Condition;
use crate::converter::values::Keyword;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IsEquippedHasKeyword {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(rename = "Keyword")]
    #[serde(default)]
    pub keyword: Keyword,
    #[serde(default)]
    #[serde(rename = "Left hand")]
    pub left_hand: bool,
}

impl Default for IsEquippedHasKeyword {
    fn default() -> Self {
        Self {
            condition: Condition::new("IsEquipped"),
            keyword: Default::default(),
            left_hand: false,
        }
    }
}
