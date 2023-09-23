use super::condition::Condition;
use crate::converter::values::{FormValue, Keyword};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasMagicEffectWithKeyword {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(rename = "Keyword")]
    pub keyword: Keyword,
    #[serde(rename = "Active effects only")]
    #[serde(default)]
    pub active_effects_only: bool,
}

impl Default for HasMagicEffectWithKeyword {
    fn default() -> Self {
        Self {
            condition: Condition::new("HasMagicEffect"),
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
  "condition": "HasMagicEffect",
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
            "condition": "HasMagicEffect",
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
