use super::{condition::default_required_version, is_false};
use crate::converter::values::Keyword;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasRefType {
    /// Condition name "HasRefType"
    pub condition: String,
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    #[serde(default)]
    #[serde(rename = "Location ref type")]
    pub location_ref_type: Keyword,
}

impl Default for HasRefType {
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
    use crate::converter::values::FormValue;
    use pretty_assertions::assert_eq;
    use serde_json;

    #[test]
    fn should_serialize_has_ref_type() {
        let has_ref_type = HasRefType {
            location_ref_type: Keyword::Form(FormValue {
                form: crate::converter::values::PluginValue {
                    plugin_name: "Skyrim.esm".into(),
                    form_id: "7".into(),
                },
            }),
            ..Default::default()
        };

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
        let serialized = serde_json::to_string_pretty(&has_ref_type).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize_has_ref_type() {
        let json_str = r#"{
            "condition": "HasRefType",
            "requiredVersion": "1.0.0.0",
            "negated": true,
            "Location ref type": {
                "editorID": "ExampleLocationTyp"
            }
}"#;

        let deserialized: HasRefType = serde_json::from_str(json_str).unwrap();
        let expected = HasRefType {
            negated: true,
            location_ref_type: Keyword::Literal(crate::converter::values::LiteralValue {
                editor_id: "ExampleLocationTyp".into(),
            }),
            ..Default::default()
        };

        assert_eq!(deserialized, expected);
    }
}
