use super::{condition::default_required_version, is_false};
use crate::converter::values::PluginValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentWeather {
    /// Condition name "CurrentWeather"
    pub condition: String,
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    #[serde(default)]
    #[serde(rename = "Weather")]
    pub weather: PluginValue,
}

impl Default for CurrentWeather {
    fn default() -> Self {
        Self {
            condition: "CurrentWeather".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            weather: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json;

    #[test]
    fn should_serialize_current_weather() {
        let current_weather = CurrentWeather::default();

        let expected = r#"{
  "condition": "CurrentWeather",
  "requiredVersion": "1.0.0.0",
  "Weather": {
    "pluginName": "",
    "formID": ""
  }
}"#;
        let serialized = serde_json::to_string_pretty(&current_weather).unwrap();
        assert_eq!(expected, serialized);
    }

    #[test]
    fn should_deserialize_current_weather() {
        let json_str = r#"{
            "condition": "CurrentWeather",
            "requiredVersion": "1.0.0.0",
            "Weather": {
                "pluginName": "",
                "formID": ""
            }
        }"#;

        let expected = CurrentWeather::default();
        let deserialized: CurrentWeather = serde_json::from_str(json_str).unwrap();
        assert_eq!(expected, deserialized);
    }
}
