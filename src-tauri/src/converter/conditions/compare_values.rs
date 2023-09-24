use super::{condition::default_required_version, is_false};
use crate::converter::values::{Cmp, NumericValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompareValues {
    /// Condition name "CompareValues"
    pub condition: String,
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    #[serde(default)]
    #[serde(rename = "Value A")]
    pub value_a: NumericValue,
    #[serde(rename = "Comparison")]
    #[serde(default)]
    /// == | != | > | >= | < | <=
    pub comparison: Cmp,
    #[serde(default)]
    #[serde(rename = "Value B")]
    pub value_b: NumericValue,
}

impl Default for CompareValues {
    fn default() -> Self {
        Self {
            condition: "CompareValues".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            value_a: NumericValue::default(),
            comparison: Cmp::Eq,
            value_b: NumericValue::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::converter::values::{
        ActorValue, ActorValueType, GraphValue, GraphVariableType, NumericLiteral, NumericValue,
        PluginValue, StaticValue,
    };

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_stringify_compare_values() {
        let compare_values = CompareValues {
            value_a: NumericValue::StaticValue(StaticValue { value: 42.0 }),
            value_b: NumericValue::StaticValue(StaticValue { value: 42.0 }),
            ..Default::default()
        };

        let expected = r#"{
  "condition": "CompareValues",
  "requiredVersion": "1.0.0.0",
  "Value A": {
    "value": 42.0
  },
  "Comparison": "==",
  "Value B": {
    "value": 42.0
  }
}"#;
        let serialized = serde_json::to_string_pretty(&compare_values).unwrap();
        assert_eq!(expected, serialized);
    }

    #[test]
    fn should_stringify_compare_values_with_actor_value() {
        let compare_values = CompareValues {
            value_a: NumericValue::ActorValue(ActorValue {
                actor_value: NumericLiteral::Decimal(123),
                actor_value_type: ActorValueType::Base,
            }),
            value_b: NumericValue::ActorValue(ActorValue {
                actor_value: NumericLiteral::Decimal(456),
                actor_value_type: ActorValueType::Max,
            }),
            comparison: Cmp::Ge,
            ..Default::default()
        };

        let expected = r#"{
  "condition": "CompareValues",
  "requiredVersion": "1.0.0.0",
  "Value A": {
    "actorValue": 123,
    "actorValueType": "Base"
  },
  "Comparison": ">=",
  "Value B": {
    "actorValue": 456,
    "actorValueType": "Max"
  }
}"#;
        let serialized = serde_json::to_string_pretty(&compare_values).unwrap();
        assert_eq!(expected, serialized);
    }

    #[test]
    fn should_stringify_compare_values_with_graph_variable() {
        let compare_values = CompareValues {
            value_a: NumericValue::GraphVariable(GraphValue {
                graph_variable: "true".to_string(),
                graph_variable_type: GraphVariableType::Bool,
            }),
            value_b: NumericValue::GraphVariable(GraphValue {
                // This is invalid as an Int, but valid as a syntax (any string will do since text is expected).
                graph_variable: "another_variable".to_string(),
                graph_variable_type: GraphVariableType::Int,
            }),
            comparison: Cmp::Ne,
            ..Default::default()
        };

        let expected = r#"{
  "condition": "CompareValues",
  "requiredVersion": "1.0.0.0",
  "Value A": {
    "graphVariable": "true",
    "graphVariableType": "Bool"
  },
  "Comparison": "!=",
  "Value B": {
    "graphVariable": "another_variable",
    "graphVariableType": "Int"
  }
}"#;
        let serialized = serde_json::to_string_pretty(&compare_values).unwrap();
        assert_eq!(expected, serialized);
    }

    #[test]
    fn should_stringify_compare_values_with_global_variable() {
        let compare_values = CompareValues {
            value_a: NumericValue::GlobalVariable(PluginValue {
                plugin_name: "my_plugin.esm".to_string(),
                form_id: 1usize.into(),
            }),
            value_b: NumericValue::GlobalVariable(PluginValue {
                plugin_name: "another_plugin.esp".to_string(),
                form_id: "2".into(),
            }),
            comparison: Cmp::Gt,
            ..Default::default()
        };

        let expected = r#"{
  "condition": "CompareValues",
  "requiredVersion": "1.0.0.0",
  "Value A": {
    "pluginName": "my_plugin.esm",
    "formID": "1"
  },
  "Comparison": ">",
  "Value B": {
    "pluginName": "another_plugin.esp",
    "formID": "2"
  }
}"#;
        let serialized = serde_json::to_string_pretty(&compare_values).unwrap();
        assert_eq!(expected, serialized);
    }
}
