//! Represents a condition to check if the current weather matches a specified weather.
use super::{condition::default_required_version, is_false};
use crate::values::PluginValue;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// Represents a condition to check if the current weather matches a specified weather.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentWeather {
    /// The name of the condition, which is "`CurrentWeather`".
    pub condition: CompactString,
    /// The required version for this condition.
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: CompactString,
    /// Indicates whether the condition is negated or not.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    /// The specific weather condition to check for.
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
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_current_weather() -> Result<()> {
        let current_weather = CurrentWeather::default();
        let serialized = serde_json::to_string_pretty(&current_weather)?;

        let expected = r#"{
  "condition": "CurrentWeather",
  "requiredVersion": "1.0.0.0",
  "Weather": {
    "pluginName": "",
    "formID": ""
  }
}"#;

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_current_weather() -> Result<()> {
        let json_str = r#"{
            "condition": "CurrentWeather",
            "requiredVersion": "1.0.0.0",
            "Weather": {
                "pluginName": "",
                "formID": ""
            }
        }"#;
        let deserialized: CurrentWeather = serde_json::from_str(json_str)?;

        let expected = CurrentWeather::default();

        assert_eq!(deserialized, expected);
        Ok(())
    }
}
