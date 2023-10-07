use super::{condition::default_required_version, is_false};
use crate::values::PluginValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasMagicEffect {
    /// Condition name "HasMagicEffect"
    pub condition: String,
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    #[serde(default)]
    #[serde(rename = "Magic effect")]
    pub magic_effect: PluginValue,
    #[serde(default)]
    #[serde(rename = "Active effects only")]
    pub active_effects_only: bool,
}

impl Default for HasMagicEffect {
    fn default() -> Self {
        Self {
            condition: "HasMagicEffect".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            magic_effect: PluginValue::default(),
            active_effects_only: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_has_magic_effect() {
        let has_magic_effect = HasMagicEffect::default();

        let expected = r#"{
  "condition": "HasMagicEffect",
  "requiredVersion": "1.0.0.0",
  "Magic effect": {
    "pluginName": "",
    "formID": ""
  },
  "Active effects only": false
}"#;
        let serialized = serde_json::to_string_pretty(&has_magic_effect).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize_has_magic_effect() {
        let json_str = r#"{
            "condition": "HasMagicEffect",
            "requiredVersion": "1.0.0.0",
            "Magic effect": {
                "pluginName": "Skyrim.esm",
                "formID": "7"
            },
            "Active effects only": true
        }"#;

        let deserialized: HasMagicEffect = serde_json::from_str(json_str).unwrap();
        let expected = HasMagicEffect {
            magic_effect: PluginValue {
                plugin_name: "Skyrim.esm".to_string(),
                form_id: "7".into(), // This is player
            },
            active_effects_only: true,
            ..Default::default()
        };

        assert_eq!(deserialized, expected);
    }
}
