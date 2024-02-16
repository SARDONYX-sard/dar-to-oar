//! Pair str & Int | Float | Bool
use serde::{Deserialize, Serialize};

/// Pair str & Int | Float | Bool
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphValue {
    /// string
    ///
    /// TODO: Unknown variable
    #[serde(rename = "graphVariable")]
    pub graph_variable: String,
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
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_graph_value() -> Result<()> {
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
    fn should_deserialize_graph_value() -> Result<()> {
        let actual = r#"{
  "graphVariable": "",
  "graphVariableType": "Int"
}"#;
        let actual: GraphValue = serde_json::from_str(actual)?;

        let expected = GraphValue {
            graph_variable: "".into(),
            graph_variable_type: GraphVariableType::Int,
        };

        assert_eq!(actual, expected);
        Ok(())
    }
}
