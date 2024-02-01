//! Person and its internal value
use super::numeric_literal::NumericLiteral;
use serde::{Deserialize, Serialize};

/// Person and its internal value
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
/// default: `ActorValue`
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum ActorValueType {
    /// Value
    #[default]
    ActorValue,
    /// Base
    Base,
    /// Max value
    Max,
    /// %
    Percentage,
}

impl TryFrom<&str> for ActorValueType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "Value" => Self::ActorValue,
            "Base" => Self::Base,
            "Max" => Self::Max,
            "Percentage" => Self::Percentage,
            _ => return Err("Invalid actor value type"),
        })
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
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
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
    fn test_serialize() -> Result<()> {
        let value = ActorValueType::Max;
        let result = serde_json::to_string(&value)?;
        assert_eq!(result, r#""Max""#);
        Ok(())
    }

    #[test]
    fn test_deserialize_valid() -> Result<()> {
        let json = r#""Percentage""#;
        let result: ActorValueType = serde_json::from_str(json)?;
        assert_eq!(result, ActorValueType::Percentage);
        Ok(())
    }

    #[test]
    fn test_deserialize_invalid() {
        let json = r#""InvalidType""#;
        let result: Result<ActorValueType, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
