//! A set of f32 | `PluginValue` | Form | Pair str, number
use super::{actor_value::ActorValue, static_value::StaticValue, FormValue, GraphValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// f32 | `PluginValue` | Form | Pair str, number
///
/// In fact, it can be variously accepted rather than Numeric,
/// but the GUI description of OAR says Numeric Value, so we follow it.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum NumericValue {
    /// Just f32 value
    StaticValue(StaticValue),
    /// Pair plugin name & ID
    GlobalVariable(FormValue),
    /// Person and its internal value
    ActorValue(ActorValue),
    /// Pair str & Int | Float | Bool
    GraphVariable(GraphValue),
}

impl Default for NumericValue {
    fn default() -> Self {
        Self::StaticValue(StaticValue::default())
    }
}

impl<'de> Deserialize<'de> for NumericValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// Macro to change from [`serde_json`] error to [`serde`] custom error
        macro_rules! deserialize_json {
            ($value:expr) => {
                serde_json::from_value($value).map_err(|e| serde::de::Error::custom(e))
            };
        }

        let value: Value = Deserialize::deserialize(deserializer)?;

        if let Value::Object(map) = &value {
            if map.contains_key("value") {
                // If the "value" field is present, assume it's a StaticValue
                let static_value: StaticValue = deserialize_json!(value)?;
                Ok(NumericValue::StaticValue(static_value))
            } else if map.contains_key("form") {
                let global_variable = deserialize_json!(value)?;
                Ok(NumericValue::GlobalVariable(global_variable))
            } else if map.contains_key("actorValue") {
                let actor_value: ActorValue = deserialize_json!(value)?;
                Ok(NumericValue::ActorValue(actor_value))
            } else if map.contains_key("graphValue") {
                Ok(NumericValue::GraphVariable(deserialize_json!(value)?))
            } else {
                Err(serde::de::Error::custom(
                    "Unable to determine NumericValue variant",
                ))
            }
        } else {
            Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Other("Expected an object"),
                &"a map",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::PluginValue;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_numeric_value_static() -> Result<()> {
        let numeric_value = NumericValue::StaticValue(StaticValue::default());
        let serialized = serde_json::to_string_pretty(&numeric_value)?;

        let expected = r#"{
  "value": 0.0
}"#;
        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_serialize_numeric_value_global_variable() -> Result<()> {
        let numeric_value = NumericValue::GlobalVariable(FormValue::default());

        let expected = r#"{
  "form": {
    "pluginName": "",
    "formID": ""
  }
}"#;
        let serialized = serde_json::to_string_pretty(&numeric_value)?;
        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_numeric_value_static() -> Result<()> {
        let json_str = r#"{
            "value": 42.0
        }"#;

        let deserialized: NumericValue = serde_json::from_str(json_str)?;
        let expected = NumericValue::StaticValue(StaticValue { value: 42.0 });

        assert_eq!(deserialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_numeric_value_global_variable() -> Result<()> {
        let json_str = r#"{
          "form": {
            "pluginName": "MyPlugin",
            "formID": "0x12345"
          }
        }"#;

        let deserialized: NumericValue = serde_json::from_str(json_str)?;
        let expected = NumericValue::GlobalVariable(
            PluginValue {
                plugin_name: "MyPlugin".into(),
                form_id: "0x12345".into(),
            }
            .into(),
        );

        assert_eq!(deserialized, expected);
        Ok(())
    }

    #[test]
    fn should_fail_deserialize_numeric_value_unknown_type() {
        let json_str = r#"{
            "invalid_key": 42.0
        }"#;

        let result: Result<NumericValue, _> = serde_json::from_str(json_str);
        assert!(result.is_err());
    }
}
