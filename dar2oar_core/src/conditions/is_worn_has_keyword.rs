use super::{condition::default_required_version, is_false};
use crate::values::Keyword;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsWornHasKeyword {
    /// Condition name "IsWornHasKeyword"
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
}

impl Default for IsWornHasKeyword {
    fn default() -> Self {
        Self {
            condition: "IsWornHasKeyword".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            keyword: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::{FormValue, PluginValue};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_is_worn_has_keyword() {
        let is_worn_has_keyword = IsWornHasKeyword::default();
        let serialized = serde_json::to_string_pretty(&is_worn_has_keyword).unwrap();

        let expected = r#"{
  "condition": "IsWornHasKeyword",
  "requiredVersion": "1.0.0.0",
  "Keyword": {
    "editorID": ""
  }
}"#;

        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize_is_worn_has_keyword() {
        let json_str = r#"{
            "condition": "IsWornHasKeyword",
            "requiredVersion": "1.0.0.0",
            "Keyword": {
              "form": {
                "pluginName": "Skyrim.esm",
                "formID": "7"
              }
            }
        }"#;
        let deserialized: IsWornHasKeyword = serde_json::from_str(json_str).unwrap();

        let expected = IsWornHasKeyword {
            keyword: Keyword::Form(FormValue {
                form: PluginValue {
                    plugin_name: "Skyrim.esm".into(),
                    form_id: "7".into(),
                },
            }),
            ..Default::default()
        };

        assert_eq!(deserialized, expected);
    }
}
