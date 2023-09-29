use super::{condition::default_required_version, is_false};
use crate::values::Keyword;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsEquippedHasKeyword {
    /// Condition name "IsEquippedHasKeyword"
    pub condition: String,
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    #[serde(default)]
    #[serde(rename = "Keyword")]
    pub keyword: Keyword,
    #[serde(default)]
    #[serde(rename = "Left hand")]
    pub left_hand: bool,
}

impl Default for IsEquippedHasKeyword {
    fn default() -> Self {
        Self {
            condition: "IsEquippedHasKeyword".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            keyword: Default::default(),
            left_hand: false,
        }
    }
}

// {
//     "condition": "IsEquippedHasKeyword",
//     "requiredVersion": "1.0.0.0",
//     "negated": true,
//     "Keyword": {
//         "editorID": "ExampleKeyword"
//     },
//     "Left hand": true
// }
