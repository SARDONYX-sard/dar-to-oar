//! Represents a condition based on whether an entity is equipped with a specific form.
use super::{condition::default_required_version, is_false};
use crate::values::PluginValue;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// Represents a condition based on whether an entity is equipped with a specific form.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsEquipped {
    /// The name of the condition, which is "IsEquipped".
    pub condition: CompactString,
    /// The required version for compatibility with this condition.
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: CompactString,
    /// Indicates whether the condition is negated.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    /// The form associated with the condition.
    #[serde(default)]
    #[serde(rename = "Form")]
    pub form: PluginValue,
    /// Indicates whether the entity is equipped in the left hand.
    #[serde(default)]
    #[serde(rename = "Left hand")]
    pub left_hand: bool,
}

impl Default for IsEquipped {
    fn default() -> Self {
        Self {
            condition: "IsEquipped".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            form: PluginValue::default(),
            left_hand: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_is_equipped() -> Result<()> {
        let is_equipped = IsEquipped {
            form: PluginValue {
                plugin_name: "MyPlugin".into(),
                form_id: "12345".into(),
            },
            left_hand: true,
            ..Default::default()
        };
        let serialized = serde_json::to_string_pretty(&is_equipped)?;

        let expected = r#"{
  "condition": "IsEquipped",
  "requiredVersion": "1.0.0.0",
  "Form": {
    "pluginName": "MyPlugin",
    "formID": "12345"
  },
  "Left hand": true
}"#;

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_is_equipped() -> Result<()> {
        // This is the actual json output by OAR.
        let json_str = r#"{
            "condition": "IsEquipped",
            "requiredVersion": "1.0.0.0",
            "negated": true,
            "Form": {
                "pluginName": "Skyrim.esm",
                "formID": "7"
            },
            "Left hand": false
        }"#;
        let deserialized: IsEquipped = serde_json::from_str(json_str)?;

        let expected = IsEquipped {
            condition: "IsEquipped".into(),
            negated: true,
            form: PluginValue {
                plugin_name: "Skyrim.esm".into(),
                form_id: "7".into(),
            },
            left_hand: false,
            ..Default::default()
        };

        assert_eq!(deserialized, expected);
        Ok(())
    }
}
