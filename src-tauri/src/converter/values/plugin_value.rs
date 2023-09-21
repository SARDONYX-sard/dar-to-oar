use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PluginValue {
    #[serde(rename = "pluginName")]
    #[serde(default)]
    pub plugin_name: String,
    /// hexadecimal representation except prefix(0x)
    #[serde(rename = "formID")]
    #[serde(default)]
    pub form_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json;

    #[test]
    fn should_serialize_plugin_value() {
        let plugin_value = PluginValue {
            plugin_name: "MyPlugin".to_string(),
            form_id: "12345".to_string(),
        };

        let expected = r#"{
  "pluginName": "MyPlugin",
  "formID": "12345"
}"#;
        let serialized = serde_json::to_string_pretty(&plugin_value).unwrap();
        assert_eq!(expected, serialized);
    }

    #[test]
    fn should_deserialize_plugin_value() {
        let json_str = r#"{
  "pluginName": "AnotherPlugin",
  "formID": "54321"
}"#;

        let deserialized: PluginValue = serde_json::from_str(json_str).unwrap();
        let expected = PluginValue {
            plugin_name: "AnotherPlugin".to_string(),
            form_id: "54321".to_string(),
        };

        assert_eq!(expected, deserialized);
    }

    #[test]
    fn should_default_plugin_value() {
        let default_plugin_value = PluginValue::default();

        let expected = PluginValue {
            plugin_name: "".to_string(),
            form_id: "".to_string(),
        };

        assert_eq!(expected, default_plugin_value);
    }
}
