use super::{condition::default_required_version, is_false};
use crate::values::DirectionValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsMovementDirection {
    /// Condition name "IsMovementDirection"
    pub condition: String,
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    #[serde(default)]
    #[serde(rename = "Direction")]
    pub direction: DirectionValue,
}

impl Default for IsMovementDirection {
    fn default() -> Self {
        Self {
            condition: "IsMovementDirection".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            direction: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::Direction;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_is_movement_direction() {
        let is_movement_direction = IsMovementDirection {
            direction: DirectionValue {
                value: Direction::Left,
            },
            ..Default::default()
        };

        let expected = r#"{
  "condition": "IsMovementDirection",
  "requiredVersion": "1.0.0.0",
  "Direction": {
    "value": 4.0
  }
}"#;
        let serialized = serde_json::to_string_pretty(&is_movement_direction).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize_is_movement_direction() {
        // This is the actual json output by OAR.
        let json_str = r#"{
            "condition": "IsMovementDirection",
            "requiredVersion": "1.0.0.0",
            "negated": true,
            "Direction": {
                "value": 2.0
            }
        }"#;

        let deserialized: IsMovementDirection = serde_json::from_str(json_str).unwrap();
        let expected = IsMovementDirection {
            negated: true,
            direction: DirectionValue {
                value: Direction::Right,
            },
            ..Default::default()
        };

        assert_eq!(deserialized, expected);
    }
}
