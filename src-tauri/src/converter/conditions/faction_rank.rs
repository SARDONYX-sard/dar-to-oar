use super::condition::Condition;
use crate::converter::values::{Cmp, NumericValue, PluginValue};
use serde::{Deserialize, Serialize};

/// Tests the ref's faction rank against the specified rank.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FactionRank {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(default)]
    pub faction: PluginValue,
    #[serde(default)]
    pub comparison: Cmp,
    #[serde(rename = "Numeric value")]
    #[serde(default)]
    pub numeric_value: NumericValue,
}

impl Default for FactionRank {
    fn default() -> Self {
        Self {
            condition: Condition::new("FactionRank"),
            faction: Default::default(),
            comparison: Default::default(),
            numeric_value: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::converter::values::StaticValue;

    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json;

    #[test]
    fn default_faction_rank() {
        let faction_rank = FactionRank::default();

        let expected = r#"{
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
}"#;
        let serialized = serde_json::to_string_pretty(&faction_rank).unwrap();
        assert_eq!(expected, serialized);
    }

    #[test]
    fn should_serialize_faction_rank_with_custom_values() {
        let faction_rank = FactionRank {
            faction: PluginValue {
                plugin_name: "CustomPlugin".to_string(),
                form_id: "54321".into(),
            },
            comparison: Cmp::Lt,
            numeric_value: NumericValue::StaticValue(StaticValue { value: 75.0 }),
            ..Default::default()
        };

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
        let serialized = serde_json::to_string_pretty(&faction_rank).unwrap();
        assert_eq!(expected, serialized);
    }

    #[test]
    fn should_deserialize_faction_rank() {
        let json_str = r#"{
  "condition": "FactionRank",
  "Faction": {
      "pluginName": "MyPlugin",
      "formID": "12345"
  },
  "Comparison": ">=",
  "Numeric value": {
      "value": 50.0
  }
}"#;

        let deserialized: FactionRank = serde_json::from_str(json_str).unwrap();
        let expected = FactionRank {
            condition: Condition::new("FactionRank"),
            faction: PluginValue {
                plugin_name: "MyPlugin".to_string(),
                form_id: "12345".into(),
            },
            comparison: Cmp::Ge,
            numeric_value: NumericValue::StaticValue(StaticValue { value: 50.0 }),
        };

        assert_eq!(expected, deserialized);
    }
}
