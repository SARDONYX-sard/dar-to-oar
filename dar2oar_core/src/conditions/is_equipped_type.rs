use super::{condition::default_required_version, is_false};
use crate::values::TypeValue;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsEquippedType {
    /// Condition name "IsEquippedType"
    pub condition: CompactString,
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: CompactString,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

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
    use crate::values::WeaponType;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_is_equipped_type() {
        let is_equipped_type = IsEquippedType {
            negated: true,
            type_value: TypeValue {
                value: WeaponType::Bow,
            },
            ..Default::default()
        };
        let serialized = serde_json::to_string_pretty(&is_equipped_type).unwrap();

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
            type_value: TypeValue {
                value: WeaponType::IllusionSpell,
            },
            left_hand: false,
            ..Default::default()
        };

        assert_eq!(deserialized, expected);
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
