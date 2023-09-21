use super::condition::Condition;
use crate::converter::values::PluginValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasRefType {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(rename = "Location ref type")]
    #[serde(default)]
    pub location_ref_type: PluginValue,
}

impl Default for HasRefType {
    fn default() -> Self {
        Self {
            condition: Condition::new("HasRefType"),
            location_ref_type: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json;

    #[test]
    fn should_serialize_has_ref_type() {
        let has_ref_type = HasRefType {
            condition: Condition::new("HasRefType"),
            location_ref_type: PluginValue {
                plugin_name: "MyPlugin".to_string(),
                form_id: "12345".to_string(),
            },
        };

        let expected = r#"{
  "condition": "HasRefType",
  "requiredVersion": "1.0.0.0",
  "Location ref type": {
    "pluginName": "MyPlugin",
    "formID": "12345"
  }
}"#;
        let serialized = serde_json::to_string_pretty(&has_ref_type).unwrap();
        assert_eq!(expected, serialized);
    }

    #[test]
    fn should_deserialize_has_ref_type() {
        let json_str = r#"{
  "condition": "HasRefType",
  "requiredVersion": "1.0.0.0",
  "Location ref type": {
    "pluginName": "AnotherPlugin",
    "formID": "54321"
  }
}"#;

        let deserialized: HasRefType = serde_json::from_str(json_str).unwrap();
        let expected = HasRefType {
            condition: Condition::new("HasRefType"),
            location_ref_type: PluginValue {
                plugin_name: "AnotherPlugin".to_string(),
                form_id: "54321".to_string(),
            },
        };

        assert_eq!(expected, deserialized);
    }

    #[test]
    fn should_default_has_ref_type() {
        let default_has_ref_type = HasRefType::default();

        let expected = HasRefType {
            condition: Condition::new("HasRefType"),
            location_ref_type: Default::default(),
        };

        assert_eq!(expected, default_has_ref_type);
    }
}
