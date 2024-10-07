//! Represents a condition based on the movement direction of an entity.
use super::{condition::default_required_version, is_false};
use crate::values::DirectionValue;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// Represents a condition based on the movement direction of an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsMovementDirection {
    /// The name of the condition, which is "`IsMovementDirection`".
    pub condition: CompactString,
    /// The required version for compatibility with this condition.
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: CompactString,
    /// Indicates whether the condition is negated.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    /// The movement direction associated with the condition.
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
    use crate::error::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_is_movement_direction() -> Result<()> {
        let is_movement_direction = IsMovementDirection {
            direction: DirectionValue {
                value: Direction::Left,
            },
            ..Default::default()
        };
        let serialized = serde_json::to_string_pretty(&is_movement_direction)?;

        let expected = r#"{
  "condition": "IsMovementDirection",
  "requiredVersion": "1.0.0.0",
  "Direction": {
    "value": 4.0
  }
}"#;

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_is_movement_direction() -> Result<()> {
        // This is the actual json output by OAR.
        let json_str = r#"{
            "condition": "IsMovementDirection",
            "requiredVersion": "1.0.0.0",
            "negated": true,
            "Direction": {
                "value": 2.0
            }
        }"#;
        let deserialized: IsMovementDirection = serde_json::from_str(json_str)?;

        let expected = IsMovementDirection {
            negated: true,
            direction: DirectionValue {
                value: Direction::Right,
            },
            ..Default::default()
        };

        assert_eq!(deserialized, expected);
        Ok(())
    }
}
