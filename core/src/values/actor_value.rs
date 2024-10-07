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
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActorValueType {
    /// Value
    #[default]
    #[serde(rename = "Value")]
    ActorValue,
    /// Base
    Base,
    /// Max value
    Max,
    /// %
    Percentage,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;
    use pretty_assertions::assert_eq;

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
