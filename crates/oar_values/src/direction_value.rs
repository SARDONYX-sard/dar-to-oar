//! Actor's Direction
use serde::de::Unexpected;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_untagged::UntaggedEnumVisitor;

use super::ValueError;

/// Actor's Direction
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DirectionValue {
    /// Actor's Direction value
    pub value: Direction,
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
    type Error = ValueError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(match value {
            x if (0.0..1.0).contains(&x) => Self::None,
            x if (1.0..2.0).contains(&x) => Self::Forward,
            x if (2.0..3.0).contains(&x) => Self::Right,
            x if (3.0..4.0).contains(&x) => Self::Back,
            x if (4.0..5.0).contains(&x) => Self::Left,
            invalid_value => {
                return Err(ValueError::CastError {
                    expected: "1.0..=5.0".into(),
                    actual: invalid_value.to_string(),
                });
            }
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
        serializer.serialize_str(match self {
            Self::None => "0.0",
            Self::Forward => "1.0",
            Self::Right => "2.0",
            Self::Back => "3.0",
            Self::Left => "4.0",
        })
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
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_direction_value() -> Result<(), serde_json::Error> {
        let actual = serde_json::to_string_pretty(&DirectionValue {
            value: Direction::Back,
        })?;

        let expected = r#"{
  "value": 3.0
}"#;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_direction_value() -> Result<(), serde_json::Error> {
        let actual: DirectionValue = serde_json::from_str(
            r#"{
            "value": 1.0
        }"#,
        )?;

        let expected = DirectionValue {
            value: Direction::Forward,
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn should_default_direction_value() {
        assert_eq!(
            DirectionValue::default(),
            DirectionValue {
                value: Direction::None
            }
        );
    }

    #[test]
    fn should_serialize_direction() -> Result<(), serde_json::Error> {
        assert_eq!(serde_json::to_string(&Direction::Right)?, "2.0");
        Ok(())
    }

    #[test]
    fn should_deserialize_direction() -> Result<(), serde_json::Error> {
        assert_eq!(serde_json::from_str::<Direction>("4.0")?, Direction::Left);
        Ok(())
    }
}
