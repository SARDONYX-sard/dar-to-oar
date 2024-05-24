//! Structure comparing two A and two B
use super::{condition::default_required_version, is_false};
use crate::values::{Cmp, NumericValue};
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// Structure comparing A and B
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompareValues {
    /// Condition name "`CompareValues`"
    pub condition: CompactString,
    /// The required version for compatibility with this condition.
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: CompactString,
    /// Indicates whether the condition is negated.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    /// Comparison object A
    #[serde(default)]
    #[serde(rename = "Value A")]
    pub value_a: NumericValue,
    #[serde(rename = "Comparison")]
    #[serde(default)]
    /// == | != | > | >= | < | <=
    pub comparison: Cmp,
    /// Comparison object B
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
    use super::*;
    use crate::values::{
        ActorValue, ActorValueType, GraphValue, GraphVariableType, NumericLiteral, NumericValue,
        PluginValue, StaticValue,
    };
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_stringify_compare_values() -> Result<()> {
        let compare_values = CompareValues {
            value_a: NumericValue::StaticValue(StaticValue { value: 42.0 }),
            value_b: NumericValue::StaticValue(StaticValue { value: 42.0 }),
            ..Default::default()
        };
        let serialized = serde_json::to_string_pretty(&compare_values)?;

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

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_stringify_compare_values_with_actor_value() -> Result<()> {
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
        let serialized = serde_json::to_string_pretty(&compare_values)?;

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

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_stringify_compare_values_with_graph_variable() -> Result<()> {
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
        let serialized = serde_json::to_string_pretty(&compare_values)?;

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

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_stringify_compare_values_with_global_variable() -> Result<()> {
        let compare_values = CompareValues {
            value_a: NumericValue::GlobalVariable(
                PluginValue {
                    plugin_name: "my_plugin.esm".into(),
                    form_id: 1_usize.into(),
                }
                .into(),
            ),
            value_b: NumericValue::GlobalVariable(
                PluginValue {
                    plugin_name: "another_plugin.esp".into(),
                    form_id: "2".into(),
                }
                .into(),
            ),
            comparison: Cmp::Gt,
            ..Default::default()
        };
        let serialized = serde_json::to_string_pretty(&compare_values)?;

        let expected = r#"{
  "condition": "CompareValues",
  "requiredVersion": "1.0.0.0",
  "Value A": {
    "form": {
      "pluginName": "my_plugin.esm",
      "formID": "1"
    }
  },
  "Comparison": ">",
  "Value B": {
    "form": {
      "pluginName": "another_plugin.esp",
      "formID": "2"
    }
  }
}"#;

        assert_eq!(serialized, expected);
        Ok(())
    }
}
