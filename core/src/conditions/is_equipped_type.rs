//! Represents a condition to check if a specific type is equipped.
use super::{condition::default_required_version, is_false};
use crate::values::TypeValue;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents a condition to check if a specific type is equipped.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsEquippedType<'a> {
    /// The name of the condition, which is "`IsEquippedType`".
    pub condition: Cow<'a, str>,
    /// The required version for this condition.
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: Cow<'a, str>,
    /// Indicates whether the condition is negated or not.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    /// The type value to check for equipment.
    #[serde(default)]
    #[serde(rename = "Type")]
    pub type_value: TypeValue,
    /// Indicates whether the equipment should be in the left hand.
    #[serde(default)]
    #[serde(rename = "Left hand")]
    pub left_hand: bool,
}

impl Default for IsEquippedType<'_> {
    fn default() -> Self {
        Self {
            condition: "IsEquippedType".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            type_value: Default::default(),
            left_hand: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;
    use crate::values::WeaponType;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_is_equipped_type() -> Result<()> {
        let is_equipped_type = IsEquippedType {
            negated: true,
            type_value: TypeValue {
                value: WeaponType::Bow,
            },
            ..Default::default()
        };
        let serialized = serde_json::to_string_pretty(&is_equipped_type)?;

        let expected = r#"{
  "condition": "IsEquippedType",
  "requiredVersion": "1.0.0.0",
  "negated": true,
  "Type": {
    "value": 7.0
  },
  "Left hand": false
}"#;

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_is_equipped_type() -> Result<()> {
        let json_str = r#"{
            "condition": "IsEquippedType",
            "requiredVersion": "1.0.0.0",
            "Type": {
                "value": 13.0
            },
            "Left hand": false
        }"#;
        let deserialized: IsEquippedType = serde_json::from_str(json_str)?;

        let expected = IsEquippedType {
            type_value: TypeValue {
                value: WeaponType::IllusionSpell,
            },
            left_hand: false,
            ..Default::default()
        };

        assert_eq!(deserialized, expected);
        Ok(())
    }

    #[test]
    fn should_default_is_equipped_type() {
        let default_is_equipped_type = IsEquippedType::default();

        let expected = IsEquippedType {
            type_value: TypeValue {
                value: WeaponType::Unarmed,
            },
            left_hand: false,
            ..Default::default()
        };

        assert_eq!(default_is_equipped_type, expected);
    }
}
