use super::{condition::default_required_version, is_false};
use crate::converter::values::Keyword;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IsWornHasKeyword {
    /// Condition name "Random"
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
}

impl Default for IsWornHasKeyword {
    fn default() -> Self {
        Self {
            condition: "Level".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            keyword: Default::default(),
        }
    }
}

// actual json
// {
//    "condition": "Level",
//    "requiredVersion": "1.0.0.0",
//    "negated": true,
//    "Comparison": "==",
//    "Numeric value": {
//        "value": 0.0
//    }
//}
