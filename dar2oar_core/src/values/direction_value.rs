use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DirectionValue {
    pub(crate) value: Direction,
}

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
            0 => Direction::None,
            1 => Direction::Forward,
            2 => Direction::Right,
            3 => Direction::Back,
            4 => Direction::Left,
            _ => return Err("Invalid value for Direction"),
        })
    }
}

impl From<&Direction> for f64 {
    fn from(value: &Direction) -> f64 {
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
                match Direction::try_from(value) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(Error::unknown_variant(
                        &value.to_string(),
                        &["0", "1", "2", "3", "4"],
                    )),
                }
            }
        }

        // Deserialize from a JSON value.
        let value: Value = Deserialize::deserialize(deserializer)?;
        match value {
            Value::Number(num) => {
                let direction =
                    Direction::try_from(num.as_u64().unwrap_or(num.as_f64().unwrap() as u64))
                        .map_err(|err| {
                            Error::invalid_type(
                                serde::de::Unexpected::Other(err),
                                &"a valid i64 or f64",
                            )
                        })?;
                Ok(direction)
            }
            Value::String(s) => {
                let t = s.parse::<f64>().map_err(|_| {
                    Error::invalid_type(
                        serde::de::Unexpected::Other("Couldn't parse float value"),
                        &"a valid Direction value",
                    )
                })?;
                DirectionVisitor.visit_f64(t)
            }
            _ => Err(Error::invalid_type(
                serde::de::Unexpected::Other("not a valid value for Direction"),
                &"a valid Direction value",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_direction_value() {
        let direction_value = DirectionValue {
            value: Direction::Back,
        };

        let expected = r#"{
  "value": 3.0
}"#;
        let serialized = serde_json::to_string_pretty(&direction_value).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize_direction_value() {
        let json_str = r#"{
            "value": 1.0
        }"#;

        let deserialized: DirectionValue = serde_json::from_str(json_str).unwrap();
        let expected = DirectionValue {
            value: Direction::Forward,
        };

        assert_eq!(deserialized, expected);
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
    fn should_serialize_direction() {
        let direction = Direction::Right;

        let expected = "2.0";
        let serialized = serde_json::to_string(&direction).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize_direction() {
        let json_str = "4.0";

        let deserialized: Direction = serde_json::from_str(json_str).unwrap();
        let expected = Direction::Left;

        assert_eq!(deserialized, expected);
    }
}
