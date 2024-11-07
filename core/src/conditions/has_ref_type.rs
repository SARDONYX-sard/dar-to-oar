//! Represents a condition to check if a reference has a specific type.
use super::{condition::default_required_version, is_false};
use crate::values::Keyword;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents a condition to check if a reference has a specific type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasRefType<'a> {
    /// The name of the condition, which is "`HasRefType`".
    pub condition: Cow<'a, str>,
    /// The required version for this condition.
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: Cow<'a, str>,
    /// Indicates whether the condition is negated or not.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    /// The reference type to check for.
    #[serde(default)]
    #[serde(rename = "Location ref type")]
    pub location_ref_type: Keyword<'a>,
}

impl Default for HasRefType<'_> {
    fn default() -> Self {
        Self {
            condition: "HasRefType".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            location_ref_type: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;
    use crate::values::{FormValue, LiteralValue, PluginValue};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_has_ref_type() -> Result<()> {
        let has_ref_type = HasRefType {
            location_ref_type: Keyword::Form(FormValue {
                form: PluginValue {
                    plugin_name: "Skyrim.esm".into(),
                    form_id: "7".into(),
                },
            }),
            ..Default::default()
        };
        let serialized = serde_json::to_string_pretty(&has_ref_type)?;

        let expected = r#"{
  "condition": "HasRefType",
  "requiredVersion": "1.0.0.0",
  "Location ref type": {
    "form": {
      "pluginName": "Skyrim.esm",
      "formID": "7"
    }
  }
}"#;

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_has_ref_type() -> Result<()> {
        let json_str = r#"{
            "condition": "HasRefType",
            "requiredVersion": "1.0.0.0",
            "negated": true,
            "Location ref type": {
                "editorID": "ExampleLocationTyp"
            }
}"#;
        let deserialized: HasRefType = serde_json::from_str(json_str)?;

        let expected = HasRefType {
            negated: true,
            location_ref_type: Keyword::Literal(LiteralValue {
                editor_id: "ExampleLocationTyp".into(),
            }),
            ..Default::default()
        };

        assert_eq!(deserialized, expected);
        Ok(())
    }
}
