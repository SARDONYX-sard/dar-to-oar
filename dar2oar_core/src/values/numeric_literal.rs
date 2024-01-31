//! Numeric type tied to DAR
use core::fmt;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

/// This is almost identical to the DAR, but applied to the display for the OAR.
/// - The most noticeable difference is that it omits the hexadecimal 0x when made into a string
///
/// Hex | Decimal | Float
/// - Sum of Hex | Decimal | Float (The original DAR argument can have multiple notations, such as 0x007 or 0.1.
///
/// default: 0.0
#[derive(Debug, Clone, PartialEq)]
pub enum NumericLiteral {
    /// e.g. 0x007
    Hex(usize),
    /// e.g. 1
    Decimal(isize),
    /// e.g. 1.0
    Float(f32),
}

impl From<usize> for NumericLiteral {
    fn from(value: usize) -> Self {
        NumericLiteral::Hex(value)
    }
}

impl From<isize> for NumericLiteral {
    fn from(value: isize) -> Self {
        NumericLiteral::Decimal(value)
    }
}

impl From<f32> for NumericLiteral {
    fn from(value: f32) -> Self {
        NumericLiteral::Float(value)
    }
}

impl Default for NumericLiteral {
    fn default() -> Self {
        Self::Float(0.0)
    }
}

impl fmt::Display for NumericLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // NOTE: When converted to a string, omit the hexadecimal prefix (0x) and use the same hex notation as for OAR.
            NumericLiteral::Hex(hex) => write!(f, "{hex:x}"),
            NumericLiteral::Decimal(decimal) => write!(f, "{decimal}"),
            NumericLiteral::Float(float) => write!(f, "{float}"),
        }
    }
}

impl Serialize for NumericLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            NumericLiteral::Hex(hex) => serializer.serialize_str(&format!("{:x}", hex)), // NOTE: OAR non prefix(0x)
            NumericLiteral::Decimal(decimal) => serializer.serialize_i64(*decimal as i64),
            NumericLiteral::Float(float) => serializer.serialize_f32(*float),
        }
    }
}

impl<'de> Deserialize<'de> for NumericLiteral {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// Inner deserialization struct
        struct NumericLiteralVisitor;

        impl<'de> serde::de::Visitor<'de> for NumericLiteralVisitor {
            type Value = NumericLiteral;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string, integer, or floating-point number")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if value.starts_with("0x") {
                    // Parse hexadecimal value
                    let hex_value = value.trim_start_matches("0x");
                    usize::from_str_radix(hex_value, 16).map_or_else(
                        |_err| Err(E::custom(format!("Invalid hexadecimal value: {}", value))),
                        |hex| Ok(NumericLiteral::Hex(hex)),
                    )
                } else if let Ok(decimal) = value.parse::<isize>() {
                    Ok(NumericLiteral::Decimal(decimal))
                } else if let Ok(float) = value.parse::<f32>() {
                    Ok(NumericLiteral::Float(float))
                } else {
                    Err(E::custom(format!("Invalid numeric value: {}", value)))
                }
            }
        }

        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        match value {
            Value::Number(num) => NumericLiteralVisitor.visit_str(num.to_string().as_str()),
            Value::String(s) => NumericLiteralVisitor.visit_str(s.as_str()),
            _ => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Other(format!("Invalid numeric value: {}", value).as_str()),
                &"a string, integer, or floating-point number",
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
    fn test_serialize_non_prefix_hex() -> Result<()> {
        let value = NumericLiteral::Hex(0x2a);
        let serialized = serde_json::to_string(&value)?;
        assert_eq!(serialized, r#""2a""#);
        Ok(())
    }

    #[test]
    fn test_serialize_decimal() -> Result<()> {
        let value = NumericLiteral::Decimal(123);
        let serialized = serde_json::to_string(&value)?;
        assert_eq!(serialized, "123");
        Ok(())
    }

    #[test]
    fn test_serialize_float() -> Result<()> {
        let value = NumericLiteral::Float(3.12);
        let serialized = serde_json::to_string(&value)?;
        assert_eq!(serialized, "3.12");
        Ok(())
    }

    #[test]
    fn test_deserialize_hex() -> Result<()> {
        let json = r#""0x2a""#;
        let deserialized: NumericLiteral = serde_json::from_str(json)?;
        assert_eq!(deserialized, NumericLiteral::Hex(0x2a));
        Ok(())
    }

    #[test]
    fn test_deserialize_decimal() -> Result<()> {
        let json = "123";
        let deserialized: NumericLiteral = serde_json::from_str(json)?;
        assert_eq!(deserialized, NumericLiteral::Decimal(123));
        Ok(())
    }

    #[test]
    fn test_deserialize_float() -> Result<()> {
        let json = "3.12";
        let deserialized: NumericLiteral = serde_json::from_str(json)?;
        assert_eq!(deserialized, NumericLiteral::Float(3.12));
        Ok(())
    }

    #[test]
    fn test_deserialize_invalid() {
        let json = r#""invalid""#;
        let result: Result<NumericLiteral, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
