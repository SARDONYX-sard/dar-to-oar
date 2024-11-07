//! Represents a condition to check if an equipped item has a specific keyword.
use super::{condition::default_required_version, is_false};
use crate::values::Keyword;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents a condition to check if an equipped item has a specific keyword.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsEquippedHasKeyword<'a> {
    /// The name of the condition, which is "`IsEquippedHasKeyword`".
    pub condition: Cow<'a, str>,
    /// The required version for this condition.
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: Cow<'a, str>,
    /// Indicates whether the condition is negated or not.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    /// The keyword to check for in the equipped item.
    #[serde(default)]
    #[serde(rename = "Keyword")]
    pub keyword: Keyword<'a>,
    /// Indicates whether the equipped item should be in the left hand.
    #[serde(default)]
    #[serde(rename = "Left hand")]
    pub left_hand: bool,
}

impl Default for IsEquippedHasKeyword<'_> {
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
    use crate::error::Result;
    use crate::values::{FormValue, PluginValue};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_is_equipped_has_keyword() -> Result<()> {
        let is_equipped_has_keyword = IsEquippedHasKeyword::default();
        let serialized = serde_json::to_string_pretty(&is_equipped_has_keyword)?;

        let expected = r#"{
  "condition": "IsEquippedHasKeyword",
  "requiredVersion": "1.0.0.0",
  "Keyword": {
    "editorID": ""
  },
  "Left hand": false
}"#;

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_is_equipped_has_keyword() -> Result<()> {
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
        let deserialized: IsEquippedHasKeyword = serde_json::from_str(json_str)?;

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
        Ok(())
    }
}
