use super::{condition::default_required_version, is_false};
use crate::values::Keyword;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsEquippedHasKeyword {
    /// Condition name "IsEquippedHasKeyword"
    pub condition: CompactString,
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: CompactString,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::{FormValue, PluginValue};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_is_equipped_has_keyword() {
        let is_equipped_has_keyword = IsEquippedHasKeyword::default();
        let serialized = serde_json::to_string_pretty(&is_equipped_has_keyword).unwrap();

        let expected = r#"{
  "condition": "IsEquippedHasKeyword",
  "requiredVersion": "1.0.0.0",
  "Keyword": {
    "editorID": ""
  },
  "Left hand": false
}"#;

        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize_is_equipped_has_keyword() {
        let json_str = r#"{
            "condition": "IsEquippedHasKeyword",
            "requiredVersion": "1.0.0.0",
            "Keyword": {
              "form": {
                "pluginName": "Skyrim.esm",
                "formID": "7"
              }
            },
            "Left hand": true
        }"#;
        let deserialized: IsEquippedHasKeyword = serde_json::from_str(json_str).unwrap();

        let expected = IsEquippedHasKeyword {
            keyword: Keyword::Form(FormValue {
                form: PluginValue {
                    plugin_name: "Skyrim.esm".into(),
                    form_id: "7".into(),
                },
            }),
            left_hand: true,
            ..Default::default()
        };

        assert_eq!(deserialized, expected);
    }
}
