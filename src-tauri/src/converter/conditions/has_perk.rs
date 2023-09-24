use super::{condition::default_required_version, is_false};
use crate::converter::values::PluginValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasPerk {
    /// Condition name "HasRefType"
    pub condition: String,
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    #[serde(default)]
    #[serde(rename = "Perk")]
    pub perk: PluginValue,
}

impl Default for HasPerk {
    fn default() -> Self {
        Self {
            condition: "HasPerk".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            perk: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json;

    #[test]
    fn should_serialize_has_perk() {
        let has_perk = HasPerk::default();

        let expected = r#"{
  "condition": "HasPerk",
  "requiredVersion": "1.0.0.0",
  "Perk": {
    "pluginName": "",
    "formID": ""
  }
}"#;
        let serialized = serde_json::to_string_pretty(&has_perk).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize_has_perk() {
        let json_str = r#"{
  "condition": "HasPerk",
  "requiredVersion": "1.0.0.0",
  "negated": true,
  "Perk": {
    "pluginName": "SomePlugin",
    "formID": "12345"
  }
}"#;

        let deserialized: HasPerk = serde_json::from_str(json_str).unwrap();
        let expected = HasPerk {
            negated: true,
            perk: PluginValue {
                plugin_name: "SomePlugin".into(),
                form_id: "12345".into(),
            },
            ..Default::default()
        };

        assert_eq!(deserialized, expected);
    }
}
