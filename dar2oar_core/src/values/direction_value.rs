//! Actor's Direction
use serde::de::Unexpected;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_untagged::UntaggedEnumVisitor;

/// Actor's Direction
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DirectionValue {
    /// Actor's Direction value
    pub(crate) value: Direction,
}

/// Actor's Direction
#[derive(Debug, Clone, Default, PartialEq)]
pub enum Direction {
    /// 0.0
    #[default]
    None = 0,
    /// 1.0
    Forward,
    /// 2.0
    Right,
    /// 3.0
    Back,
    /// 4.0
    Left,
}

impl TryFrom<f64> for Direction {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(match value {
            x if (0.0..1.0).contains(&x) => Self::None,
            x if (1.0..2.0).contains(&x) => Self::Forward,
            x if (2.0..3.0).contains(&x) => Self::Right,
            x if (3.0..4.0).contains(&x) => Self::Back,
            x if (4.0..5.0).contains(&x) => Self::Left,
            _ => return Err("Invalid value for Direction"),
        })
    }
}

impl From<&Direction> for f64 {
    fn from(value: &Direction) -> Self {
        match *value {
            Direction::None => 0.0,
            Direction::Forward => 1.0,
            Direction::Right => 2.0,
            Direction::Back => 3.0,
            Direction::Left => 4.0,
        }
    }
}

impl Serialize for Direction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the variant as a floating-point number.
        let value: f64 = self.into();
        value.serialize(serializer)
    }
}

// NOTE: Numeric comparison with float(f32) does not work correctly, so cast from f64 to i64 and deserialize
// See: https://github.com/rust-lang/rust/issues/41620
impl<'de> Deserialize<'de> for Direction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        UntaggedEnumVisitor::new()
            .f64(|float| {
                float.try_into().map_err(|_err| {
                    serde::de::Error::invalid_value(Unexpected::Float(float), &r#"0.0..=4.0"#)
                })
            })
            .deserialize(deserializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_direction_value() -> Result<()> {
        let direction_value = DirectionValue {
            value: Direction::Back,
        };
        let serialized = serde_json::to_string_pretty(&direction_value)?;

        let expected = r#"{
  "value": 3.0
}"#;
        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_direction_value() -> Result<()> {
        let json_str = r#"{
            "value": 1.0
        }"#;

        let deserialized: DirectionValue = serde_json::from_str(json_str)?;
        let expected = DirectionValue {
            value: Direction::Forward,
        };

        assert_eq!(deserialized, expected);
        Ok(())
    }

    #[test]
    fn should_default_direction_value() {
        let default_direction_value = DirectionValue::default();

        let expected = DirectionValue {
            value: Direction::None,
        };

        assert_eq!(default_direction_value, expected);
    }

    #[test]
    fn should_serialize_direction() -> Result<()> {
        let direction = Direction::Right;

        let expected = "2.0";
        let serialized = serde_json::to_string(&direction)?;

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_direction() -> Result<()> {
        let json_str = "4.0";

        let deserialized: Direction = serde_json::from_str(json_str)?;
        let expected = Direction::Left;

        assert_eq!(deserialized, expected);
        Ok(())
    }
}
