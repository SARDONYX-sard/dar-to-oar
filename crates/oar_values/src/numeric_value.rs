//! A set of f32 | `PluginValue` | Form | Pair str, number
use super::{FormValue, GraphValue, actor_value::ActorValue, static_value::StaticValue};
use serde::{Deserialize, Serialize};

/// f32 | `PluginValue` | Form | Pair str, number
///
/// In fact, it can be variously accepted rather than Numeric,
/// but the GUI description of OAR says Numeric Value, so we follow it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumericValue<'a> {
    /// Just f32 value
    StaticValue(StaticValue),
    /// Pair plugin name & ID
    GlobalVariable(FormValue<'a>),
    /// Person and its internal value
    ActorValue(ActorValue),
    /// Pair str & Int | Float | Bool
    GraphVariable(GraphValue<'a>),
}

impl From<StaticValue> for NumericValue<'_> {
    #[inline]
    fn from(value: StaticValue) -> Self {
        Self::StaticValue(value)
    }
}

impl Default for NumericValue<'_> {
    fn default() -> Self {
        Self::StaticValue(StaticValue::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PluginValue;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_numeric_value_static() -> Result<(), serde_json::Error> {
        let numeric_value = NumericValue::StaticValue(StaticValue::default());
        let serialized = serde_json::to_string_pretty(&numeric_value)?;

        let expected = r#"{
  "value": 0.0
}"#;
        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_serialize_numeric_value_global_variable() -> Result<(), serde_json::Error> {
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
    fn should_deserialize_numeric_value_static() -> Result<(), serde_json::Error> {
        let json_str = r#"{
            "value": 42.0
        }"#;

        let deserialized: NumericValue = serde_json::from_str(json_str)?;
        let expected = NumericValue::StaticValue(StaticValue { value: 42.0 });

        assert_eq!(deserialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_numeric_value_global_variable() -> Result<(), serde_json::Error> {
        let json_str = r#"{
          "form": {
            "pluginName": "MyPlugin",
            "formID": "12345"
          }
        }"#;

        let deserialized: NumericValue = serde_json::from_str(json_str)?;
        let expected = NumericValue::GlobalVariable(
            PluginValue {
                plugin_name: "MyPlugin".into(),
                form_id: "12345".into(),
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
