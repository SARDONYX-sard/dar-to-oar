use super::condition::Condition;
use crate::converter::values::PluginValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasPerk {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(default)]
    #[serde(rename = "Perk")]
    pub perk: PluginValue,
}

impl Default for HasPerk {
    fn default() -> Self {
        Self {
            condition: Condition::new("HasPerk"),
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
        let has_perk = HasPerk {
            condition: Condition::new("HasPerk"),
            perk: PluginValue::default(),
        };

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
  "Perk": {
    "pluginName": "SomePlugin",
    "formID": "12345"
  }
}"#;

        let deserialized: HasPerk = serde_json::from_str(json_str).unwrap();
        let expected = HasPerk {
            condition: Condition::new("HasPerk"),
            perk: PluginValue {
                plugin_name: "SomePlugin".to_string(),
                form_id: "12345".into(),
            },
        };

        assert_eq!(deserialized, expected);
    }

    #[test]
    fn should_default_has_perk() {
        let default_has_perk = HasPerk::default();

        let expected = HasPerk {
            condition: Condition::new("HasPerk"),
            perk: PluginValue::default(),
        };

        assert_eq!(default_has_perk, expected);
    }
}
