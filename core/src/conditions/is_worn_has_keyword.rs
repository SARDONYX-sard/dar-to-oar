//! Represents a condition based on whether an entity is worn and has a specific keyword.
use super::{condition::default_required_version, is_false};
use crate::values::Keyword;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents a condition based on whether an entity is worn and has a specific keyword.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsWornHasKeyword<'a> {
    /// The name of the condition, which is "`IsWornHasKeyword`".
    pub condition: Cow<'a, str>,
    /// The required version for this condition.
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: Cow<'a, str>,
    /// Indicates whether the condition is negated or not.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    /// The keyword associated with the condition.
    #[serde(default)]
    #[serde(rename = "Keyword")]
    pub keyword: Keyword<'a>,
}

impl Default for IsWornHasKeyword<'_> {
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
    use crate::error::Result;
    use crate::values::{FormValue, PluginValue};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_is_worn_has_keyword() -> Result<()> {
        let is_worn_has_keyword = IsWornHasKeyword::default();
        let serialized = serde_json::to_string_pretty(&is_worn_has_keyword)?;

        let expected = r#"{
  "condition": "IsWornHasKeyword",
  "requiredVersion": "1.0.0.0",
  "Keyword": {
    "editorID": ""
  }
}"#;

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_is_worn_has_keyword() -> Result<()> {
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
        let deserialized: IsWornHasKeyword = serde_json::from_str(json_str)?;

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
        Ok(())
    }
}
