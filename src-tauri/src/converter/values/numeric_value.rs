use super::{
    actor_value::ActorValue, graph_value::GraphValue, plugin_value::PluginValue,
    static_value::StaticValue,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// In fact, it can be variously accepted rather than Numeric,
/// but the GUI description of OAR says Numeric Value, so we follow it.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum NumericValue {
    StaticValue(StaticValue),
    GlobalVariable(PluginValue),
    ActorValue(ActorValue),
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
        let value: Value = Deserialize::deserialize(deserializer)?;

        if let Value::Object(map) = &value {
            if map.contains_key("value") {
                // If the "value" field is present, assume it's a StaticValue
                let static_value: StaticValue = serde_json::from_value(value).unwrap();
                Ok(NumericValue::StaticValue(static_value))
            } else if map.contains_key("pluginName") && map.contains_key("formID") {
                // If both "pluginName" and "formID" fields are present, assume it's a GlobalVariable
                let global_variable: PluginValue = serde_json::from_value(value).unwrap();
                Ok(NumericValue::GlobalVariable(global_variable))
            } else if map.contains_key("actorValue") {
                // If the "actorValue" field is present, assume it's an ActorValue
                let actor_value: ActorValue = serde_json::from_value(value).unwrap();
                Ok(NumericValue::ActorValue(actor_value))
            } else if map.contains_key("graphValue") {
                // If the "graphValue" field is present, assume it's a GraphVariable
                let graph_variable: GraphValue = serde_json::from_value(value).unwrap();
                Ok(NumericValue::GraphVariable(graph_variable))
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
    use pretty_assertions::assert_eq;
    use serde_json;

    #[test]
    fn should_serialize_numeric_value_static() {
        let numeric_value = NumericValue::StaticValue(StaticValue::default());
        let serialized = serde_json::to_string_pretty(&numeric_value).unwrap();

        let expected = r#"{
  "value": 0.0
}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_serialize_numeric_value_global_variable() {
        let numeric_value = NumericValue::GlobalVariable(PluginValue::default());

        let expected = r#"{
  "pluginName": "",
  "formID": ""
}"#;
        let serialized = serde_json::to_string_pretty(&numeric_value).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize_numeric_value_static() {
        let json_str = r#"{
            "value": 42.0
        }"#;

        let deserialized: NumericValue = serde_json::from_str(json_str).unwrap();
        let expected = NumericValue::StaticValue(StaticValue { value: 42.0 });

        assert_eq!(deserialized, expected);
    }

    #[test]
    fn should_deserialize_numeric_value_global_variable() {
        let json_str = r#"{
            "pluginName": "MyPlugin",
            "formID": "0x12345"
        }"#;

        let deserialized: NumericValue = serde_json::from_str(json_str).unwrap();
        let expected = NumericValue::GlobalVariable(PluginValue {
            plugin_name: "MyPlugin".into(),
            form_id: "0x12345".into(),
        });

        assert_eq!(deserialized, expected);
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
