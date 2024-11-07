//! Represents a condition to test the reference's faction rank against a specified rank.
use super::{condition::default_required_version, is_false};
use crate::values::{Cmp, NumericValue, PluginValue};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents a condition to test the reference's faction rank against a specified rank.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FactionRank<'a> {
    /// The name of the condition, which is "`FactionRank`".
    pub condition: Cow<'a, str>,
    /// The required version for this condition.
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: Cow<'a, str>,
    /// Indicates whether the condition is negated or not.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    /// The faction to test against the reference's faction rank.
    #[serde(default)]
    #[serde(rename = "Faction")]
    pub faction: PluginValue<'a>,
    /// The comparison operator to use in the faction rank comparison.
    #[serde(default)]
    #[serde(rename = "Comparison")]
    pub comparison: Cmp,
    /// The numeric value to compare the faction rank against.
    #[serde(rename = "Numeric value")]
    pub numeric_value: NumericValue<'a>,
}

impl Default for FactionRank<'_> {
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
    use crate::error::Result;
    use crate::values::StaticValue;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_faction_rank_with_custom_values() -> Result<()> {
        let faction_rank = FactionRank {
            faction: PluginValue {
                plugin_name: "CustomPlugin".into(),
                form_id: "54321".into(),
            },
            comparison: Cmp::Lt,
            numeric_value: NumericValue::StaticValue(StaticValue { value: 75.0 }),
            ..Default::default()
        };
        let serialized = serde_json::to_string_pretty(&faction_rank)?;

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
        Ok(())
    }

    #[test]
    fn should_deserialize_faction_rank() -> Result<()> {
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
        let deserialized: FactionRank = serde_json::from_str(json_str)?;

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
        Ok(())
    }
}
