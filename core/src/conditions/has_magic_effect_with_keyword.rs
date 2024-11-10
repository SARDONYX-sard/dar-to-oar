//! Represents a condition to check if an entity has a magic effect with a specific keyword.
use super::{condition::default_required_version, is_false};
use crate::values::{FormValue, Keyword};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents a condition to check if an entity has a magic effect with a specific keyword.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasMagicEffectWithKeyword<'a> {
    /// The name of the condition, which is "`HasMagicEffectWithKeyword`".
    ///
    /// # Note
    /// This condition name is 25 bytes.
    /// Optimization by [`CompactString`] is limited to 24 bytes, the size of a [`String`] structure.
    /// Therefore, this field cannot be SSO (Small String Optimization).
    pub condition: Cow<'a, str>,
    /// The required version for this condition.
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: Cow<'a, str>,
    /// Indicates whether the condition is negated or not.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    /// The keyword to check for in the magic effect.
    #[serde(default)]
    #[serde(rename = "Keyword")]
    pub keyword: Keyword<'a>,
    /// Indicates whether to consider only active effects.
    #[serde(default)]
    #[serde(rename = "Active effects only")]
    pub active_effects_only: bool,
}

impl Default for HasMagicEffectWithKeyword<'_> {
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
    use super::*;
    use crate::error::Result;
    use crate::values::PluginValue;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_has_magic_effect() -> Result<()> {
        let has_magic_effect = HasMagicEffectWithKeyword::default();
        let serialized = serde_json::to_string_pretty(&has_magic_effect)?;

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

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_has_magic_effect() -> Result<()> {
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
        let deserialized: HasMagicEffectWithKeyword = serde_json::from_str(json_str)?;

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

        assert_eq!(deserialized, expected);
        Ok(())
    }
}
