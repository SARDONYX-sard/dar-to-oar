use super::{condition::default_required_version, is_false};
use crate::values::{Cmp, NumericValue, PluginValue};
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// Tests the ref's faction rank against the specified rank.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FactionRank {
    /// Condition name "FactionRank"
    pub condition: CompactString,
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: CompactString,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    #[serde(default)]
    #[serde(rename = "Faction")]
    pub faction: PluginValue,
    #[serde(default)]
    #[serde(rename = "Comparison")]
    pub comparison: Cmp,
    #[serde(default)]
    #[serde(rename = "Numeric value")]
    pub numeric_value: NumericValue,
}

impl Default for FactionRank {
    fn default() -> Self {
        Self {
            condition: "FactionRank".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            faction: Default::default(),
            comparison: Default::default(),
            numeric_value: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::StaticValue;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_faction_rank_with_custom_values() {
        let faction_rank = FactionRank {
            faction: PluginValue {
                plugin_name: "CustomPlugin".into(),
                form_id: "54321".into(),
            },
            comparison: Cmp::Lt,
            numeric_value: NumericValue::StaticValue(StaticValue { value: 75.0 }),
            ..Default::default()
        };
        let serialized = serde_json::to_string_pretty(&faction_rank).unwrap();

        let expected = r#"{
  "condition": "FactionRank",
  "requiredVersion": "1.0.0.0",
  "Faction": {
    "pluginName": "CustomPlugin",
    "formID": "54321"
  },
  "Comparison": "<",
  "Numeric value": {
    "value": 75.0
  }
}"#;

        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize_faction_rank() {
        let json_str = r#"
        {
            "condition": "FactionRank",
            "requiredVersion": "1.0.0.0",
            "Faction": {
                "pluginName": "",
                "formID": ""
            },
            "Comparison": "==",
            "Numeric value": {
                "value": 0.0
            }
        }
"#;
        let deserialized: FactionRank = serde_json::from_str(json_str).unwrap();

        let expected = FactionRank {
            faction: PluginValue {
                plugin_name: "".into(),
                form_id: "".into(),
            },
            comparison: Cmp::Eq,
            numeric_value: NumericValue::StaticValue(StaticValue { value: 0.0 }),
            ..Default::default()
        };

        assert_eq!(deserialized, expected);
    }
}
