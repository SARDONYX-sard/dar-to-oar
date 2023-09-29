use super::numeric_literal::NumericLiteral;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ActorValue {
    /// default: 0
    #[serde(default)]
    #[serde(rename = "actorValue")]
    pub actor_value: NumericLiteral,
    /// OAR GUI selection => config.json value
    /// - Actor Value => "Value"
    /// - Base Actor Value => "Base"
    /// - Max Actor Value => "Max"
    /// - Actor Value Percentage (0-1) => "Percentage"
    #[serde(rename = "actorValueType")]
    pub actor_value_type: ActorValueType,
}

/// OAR GUI selection => config.json value
/// - Actor Value => "Value"
/// - Base Actor Value => "Base"
/// - Max Actor Value => "Max"
/// - Actor Value Percentage (0-1) => "Percentage"
///
/// default: ActorValue
#[derive(Debug, Clone, Default, PartialEq)]
pub enum ActorValueType {
    #[default]
    ActorValue,
    Base,
    Max,
    Percentage,
}

impl TryFrom<&str> for ActorValueType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Value" => Ok(ActorValueType::ActorValue),
            "Base" => Ok(ActorValueType::Base),
            "Max" => Ok(ActorValueType::Max),
            "Percentage" => Ok(ActorValueType::Percentage),
            _ => Err("Invalid actor value type"),
        }
    }
}

impl From<ActorValueType> for &str {
    fn from(value: ActorValueType) -> Self {
        match value {
            ActorValueType::ActorValue => "Value",
            ActorValueType::Base => "Base",
            ActorValueType::Max => "Max",
            ActorValueType::Percentage => "Percentage",
        }
    }
}

impl Serialize for ActorValueType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.clone().into())
    }
}

impl<'de> Deserialize<'de> for ActorValueType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let type_str = String::deserialize(deserializer)?;
        type_str
            .as_str()
            .try_into()
            .map_err(|err| serde::de::Error::custom(err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_try_from_str_valid() {
        let value = "Value";
        let result = ActorValueType::try_from(value);
        assert_eq!(result, Ok(ActorValueType::ActorValue));
    }

    #[test]
    fn test_try_from_str_invalid() {
        let value = "InvalidValue";
        let result = ActorValueType::try_from(value);
        assert_eq!(result, Err("Invalid actor value type"));
    }

    #[test]
    fn test_into_str() {
        let value = ActorValueType::Base;
        let result: &str = value.into();
        assert_eq!(result, "Base");
    }

    #[test]
    fn test_serialize() {
        let value = ActorValueType::Max;
        let result = serde_json::to_string(&value).unwrap();
        assert_eq!(result, "\"Max\"");
    }

    #[test]
    fn test_deserialize_valid() {
        let json = "\"Percentage\"";
        let result: ActorValueType = serde_json::from_str(json).unwrap();
        assert_eq!(result, ActorValueType::Percentage);
    }

    #[test]
    fn test_deserialize_invalid() {
        let json = "\"InvalidType\"";
        let result: Result<ActorValueType, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
