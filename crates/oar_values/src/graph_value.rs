//! Pair str & Int | Float | Bool
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Pair str & Int | Float | Bool
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphValue<'a> {
    /// string (`hkbBehaviorGraphStringData.variableNames`)
    #[serde(rename = "graphVariable")]
    pub graph_variable: Cow<'a, str>,
    /// Float | Int | Bool
    #[serde(rename = "graphVariableType")]
    pub graph_variable_type: GraphVariableType,
}

/// Float | Int | Bool
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum GraphVariableType {
    /// Floating point number
    #[default]
    Float,
    /// Integer
    Int,
    /// Boolean
    Bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_graph_value() -> Result<(), serde_json::Error> {
        let graph_value = GraphValue::default();

        let expected = r#"{
  "graphVariable": "",
  "graphVariableType": "Float"
}"#;
        let serialized = serde_json::to_string_pretty(&graph_value)?;
        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_graph_value() -> Result<(), serde_json::Error> {
        let actual = r#"{
  "graphVariable": "FNISaa_sprint",
  "graphVariableType": "Int"
}"#;
        let actual: GraphValue = serde_json::from_str(actual)?;

        let expected = GraphValue {
            graph_variable: "FNISaa_sprint".into(),
            graph_variable_type: GraphVariableType::Int,
        };

        assert_eq!(actual, expected);
        Ok(())
    }
}
