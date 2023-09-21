use super::condition::Condition;
use crate::converter::values::{Cmp, DirectionValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsMovementDirection {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(rename = "Comparison")]
    #[serde(default)]
    pub comparison: Cmp,
    #[serde(rename = "Direction")]
    #[serde(default)]
    pub direction: DirectionValue,
}

impl Default for IsMovementDirection {
    fn default() -> Self {
        Self {
            condition: Condition::new("IsMovementDirection"),
            comparison: Default::default(),
            direction: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::converter::values::Direction;

    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json;

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
  "Comparison": "==",
  "Direction": {
    "value": 4.0
  }
}"#;
        let serialized = serde_json::to_string_pretty(&is_movement_direction).unwrap();
        assert_eq!(expected, serialized);
    }

    #[test]
    fn should_deserialize_is_movement_direction() {
        let json_str = r#"{
            "condition": "IsMovementDirection",
            "requiredVersion": "1.0.0.0",
            "Comparison": "!=",
            "Direction": {
                "value": 2.0
            }
        }"#;

        let deserialized: IsMovementDirection = serde_json::from_str(json_str).unwrap();
        let expected = IsMovementDirection {
            comparison: Cmp::Ne,
            direction: DirectionValue {
                value: Direction::Right,
            },
            ..Default::default()
        };

        assert_eq!(expected, deserialized);
    }

    #[test]
    fn should_default_is_movement_direction() {
        let default_is_movement_direction = IsMovementDirection::default();

        let expected = IsMovementDirection {
            condition: Condition::new("IsMovementDirection"),
            comparison: Default::default(),
            direction: Default::default(),
        };

        assert_eq!(expected, default_is_movement_direction);
    }
}
