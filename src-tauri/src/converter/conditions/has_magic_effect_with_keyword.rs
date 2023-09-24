use super::{condition::default_required_version, is_false};
use crate::converter::values::{FormValue, Keyword};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasMagicEffectWithKeyword {
    /// Condition name "HasMagicEffectWithKeyword"
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
    #[serde(rename = "Active effects only")]
    pub active_effects_only: bool,
}

impl Default for HasMagicEffectWithKeyword {
    fn default() -> Self {
        Self {
            condition: "HasMagicEffectWithKeyword".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            keyword: Keyword::Form(FormValue::default()),
            active_effects_only: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::converter::values::PluginValue;

    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json;

    #[test]
    fn should_serialize_has_magic_effect() {
        let has_magic_effect = HasMagicEffectWithKeyword::default();

        let expected = r#"{
  "condition": "HasMagicEffectWithKeyword",
  "requiredVersion": "1.0.0.0",
  "Keyword": {
    "form": {
      "pluginName": "",
      "formID": ""
    }
  },
  "Active effects only": false
}"#;
        let serialized = serde_json::to_string_pretty(&has_magic_effect).unwrap();
        assert_eq!(expected, serialized);
    }

    #[test]
    fn should_deserialize_has_magic_effect() {
        let json_str = r#"{
            "condition": "HasMagicEffectWithKeyword",
            "requiredVersion": "1.0.0.0",
            "Keyword": {
              "form": {
                "pluginName": "Skyrim.esm",
                "formID": "7"
              }
            },
            "Active effects only": true
        }"#;

        let deserialized: HasMagicEffectWithKeyword = serde_json::from_str(json_str).unwrap();
        let expected = HasMagicEffectWithKeyword {
            keyword: Keyword::Form(FormValue {
                form: PluginValue {
                    plugin_name: "Skyrim.esm".into(),
                    form_id: "7".into(), // This is player
                },
            }),
            active_effects_only: true,
            ..Default::default()
        };

        assert_eq!(expected, deserialized);
    }
}
