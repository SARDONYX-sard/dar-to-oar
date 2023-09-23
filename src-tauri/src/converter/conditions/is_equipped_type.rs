use super::condition::Condition;
use crate::converter::values::TypeValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsEquippedType {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(default)]
    #[serde(rename = "Type")]
    pub type_value: TypeValue,
    #[serde(default)]
    #[serde(rename = "Left hand")]
    pub left_hand: bool,
}

impl Default for IsEquippedType {
    fn default() -> Self {
        Self {
            condition: Condition::new("IsEquippedType"),
            type_value: Default::default(),
            left_hand: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::converter::values::WeaponType;

    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json;

    #[test]
    fn should_serialize_is_equipped_type() {
        let is_equipped_type = IsEquippedType {
            type_value: TypeValue {
                value: WeaponType::Bow,
            },
            ..Default::default()
        };

        let expected = r#"{
  "condition": "IsEquippedType",
  "requiredVersion": "1.0.0.0",
  "Type": {
    "value": 7.0
  },
  "Left hand": false
}"#;
        let serialized = serde_json::to_string_pretty(&is_equipped_type).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize_is_equipped_type() {
        let json_str = r#"{
            "condition": "IsEquippedType",
            "requiredVersion": "1.0.0.0",
            "Type": {
                "value": 13.0
            },
            "Left hand": false
        }"#;

        let deserialized: IsEquippedType = serde_json::from_str(json_str).unwrap();
        let expected = IsEquippedType {
            condition: Condition::new("IsEquippedType"),
            type_value: TypeValue {
                value: WeaponType::IllusionSpell,
            },
            left_hand: false,
        };

        assert_eq!(deserialized, expected);
    }

    #[test]
    fn should_default_is_equipped_type() {
        let default_is_equipped_type = IsEquippedType::default();

        let expected = IsEquippedType {
            condition: Condition::new("IsEquippedType"),
            type_value: TypeValue {
                value: WeaponType::Unarmed,
            },
            left_hand: false,
        };

        assert_eq!(default_is_equipped_type, expected);
    }
}
