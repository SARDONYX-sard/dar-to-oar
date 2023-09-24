use super::{condition::default_required_version, is_false};
use crate::converter::values::PluginValue;
use serde::{Deserialize, Serialize};

/// - OAR: IsEquipped
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsEquipped {
    /// Condition name "IsEquipped"
    pub condition: String,
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    #[serde(default)]
    #[serde(rename = "Form")]
    pub form: PluginValue,
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
    use pretty_assertions::assert_eq;
    use serde_json;

    #[test]
    fn should_serialize_is_equipped() {
        let is_equipped = IsEquipped {
            form: PluginValue {
                plugin_name: "MyPlugin".to_string(),
                form_id: "12345".into(),
            },
            left_hand: true,
            ..Default::default()
        };

        let expected = r#"{
  "condition": "IsEquipped",
  "requiredVersion": "1.0.0.0",
  "Form": {
    "pluginName": "MyPlugin",
    "formID": "12345"
  },
  "Left hand": true
}"#;
        let serialized = serde_json::to_string_pretty(&is_equipped).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize_is_equipped() {
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

        let deserialized: IsEquipped = serde_json::from_str(json_str).unwrap();
        let expected = IsEquipped {
            condition: "IsEquipped".into(),
            negated: true,
            form: PluginValue {
                plugin_name: "Skyrim.esm".to_string(),
                form_id: "7".into(),
            },
            left_hand: false,
            ..Default::default()
        };

        assert_eq!(deserialized, expected);
    }
}
