//! Actor's Direction
use serde::de::{Error, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

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

impl TryFrom<u64> for Direction {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::None,
            1 => Self::Forward,
            2 => Self::Right,
            3 => Self::Back,
            4 => Self::Left,
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
        /// Inner struct for deserialization
        struct DirectionVisitor;

        impl<'de> Visitor<'de> for DirectionVisitor {
            type Value = Direction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid Direction value")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Direction::try_from(value).map_or_else(
                    |_err| {
                        Err(Error::unknown_variant(
                            &value.to_string(),
                            &["0", "1", "2", "3", "4"],
                        ))
                    },
                    |value| Ok(value),
                )
            }
        }

        // Deserialize from a JSON value.
        let value: Value = Deserialize::deserialize(deserializer)?;
        match value {
            Value::Number(num) => {
                let err = || {
                    Error::invalid_type(Unexpected::Other("WeaponType parse f64"), &"a valid f64")
                };
                let direction = num.as_u64().unwrap_or(num.as_f64().ok_or_else(err)? as u64);
                let err = |err| Error::invalid_type(Unexpected::Other(err), &"a valid u64 or f64");
                let direction = Self::try_from(direction).map_err(err)?;
                Ok(direction)
            }
            Value::String(s) => {
                let t = s.parse::<f64>().map_err(|_err| {
                    Error::invalid_type(
                        Unexpected::Other("Couldn't parse float value"),
                        &"a valid Direction value",
                    )
                })?;
                DirectionVisitor.visit_f64(t)
            }
            _ => Err(Error::invalid_type(
                Unexpected::Other("not a valid value for Direction"),
                &"a valid Direction value",
            )),
        }
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

        let expected = r#"{
  "value": 3.0
}"#;
        let serialized = serde_json::to_string_pretty(&direction_value)?;
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
