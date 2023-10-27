use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GraphValue {
    #[serde(rename = "graphVariable")]
    pub graph_variable: String,
    #[serde(rename = "graphVariableType")]
    pub graph_variable_type: GraphVariableType, // Use the enum type here
}

/// Float | Int | Bool
#[derive(Debug, Clone, Default, PartialEq)]
pub enum GraphVariableType {
    #[default]
    Float,
    Int,
    Bool,
}

impl Serialize for GraphVariableType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let type_str = match self {
            GraphVariableType::Float => "Float",
            GraphVariableType::Int => "Int",
            GraphVariableType::Bool => "Bool",
        };
        serializer.serialize_str(type_str)
    }
}

impl<'de> Deserialize<'de> for GraphVariableType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let type_str = String::deserialize(deserializer)?;
        match type_str.as_str() {
            "Float" => Ok(GraphVariableType::Float),
            "Int" => Ok(GraphVariableType::Int),
            "Bool" => Ok(GraphVariableType::Bool),
            _ => Err(serde::de::Error::custom("Invalid graph variable type")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_current_weather() {
        let graph_value = GraphValue::default();

        let expected = r#"{
  "graphVariable": "",
  "graphVariableType": "Float"
}"#;
        let serialized = serde_json::to_string_pretty(&graph_value).unwrap();
        assert_eq!(serialized, expected);
    }
}
