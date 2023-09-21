use super::condition::Condition;
use crate::converter::values::PluginValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IsEquipped {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(rename = "Form")]
    #[serde(default)]
    pub form: PluginValue,
    #[serde(default)]
    #[serde(rename = "Left hand")]
    pub left_hand: bool,
}

impl Default for IsEquipped {
    fn default() -> Self {
        Self {
            condition: Condition::new("IsEquipped"),
            form: PluginValue::default(),
            left_hand: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json;

    #[test]
    fn should_serialize_is_equipped() {
        let is_equipped = IsEquipped {
            form: PluginValue {
                plugin_name: "MyPlugin".to_string(),
                form_id: "12345".to_string(),
            },
            left_hand: true,
            ..Default::default()
        };

        let expected = r#"{
  "condition": "IsEquipped",
  "requiredVersion": "1.0.0.0",
  "Form": {
    "pluginName": "MyPlugin",
    "formID": "12345"
  },
  "Left hand": true
}"#;
        let serialized = serde_json::to_string_pretty(&is_equipped).unwrap();
        assert_eq!(expected, serialized);
    }

    #[test]
    fn should_deserialize_is_equipped() {
        let json_str = r#"{
            "condition": "IsEquipped",
            "requiredVersion": "1.0.0.0",
            "Form": {
                "pluginName": "AnotherPlugin",
                "formID": "54321"
            },
            "Left hand": false
        }"#;

        let deserialized: IsEquipped = serde_json::from_str(json_str).unwrap();
        let expected = IsEquipped {
            condition: Condition::new("IsEquipped"),
            form: PluginValue {
                plugin_name: "AnotherPlugin".to_string(),
                form_id: "54321".to_string(),
            },
            left_hand: false,
        };

        assert_eq!(expected, deserialized);
    }

    #[test]
    fn should_default_is_equipped() {
        let default_is_equipped = IsEquipped::default();

        let expected = IsEquipped {
            condition: Condition::new("IsEquipped"),
            form: PluginValue::default(),
            left_hand: false,
        };

        assert_eq!(expected, default_is_equipped);
    }
}
