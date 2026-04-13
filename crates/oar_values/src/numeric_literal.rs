//! Numeric type tied to DAR
use std::borrow::Cow;

/// This is almost identical to the DAR, but applied to the display for the OAR.
/// - The most noticeable difference is that it omits the hexadecimal 0x when made into a string
///
/// Hex | Float
/// - Sum of Hex | Float (The original DAR argument can have multiple notations, such as 0x007 or 0.1.
///
/// default: 0.0
#[derive(Debug, Clone, PartialEq)]
pub enum NumericLiteral<'a> {
    /// Non prefix hex str
    /// - e.g., `0x007` -> `7`
    Hex(Cow<'a, str>),

    /// e.g. `12.3`, `30`
    Float(Cow<'a, str>),
}

impl<'i> NumericLiteral<'i> {
    /// # Errors
    /// Invalid number
    pub fn try_into_hex_str(self) -> Result<Cow<'i, str>, core::num::ParseFloatError> {
        Ok(match self {
            NumericLiteral::Hex(s) => s,
            NumericLiteral::Float(s) => Cow::Owned(format!("{:x}", s.parse::<f64>()? as i64)),
        })
    }

    /// # Errors
    /// Invalid number
    pub fn try_into_float_str(self) -> Result<Cow<'i, str>, core::num::ParseIntError> {
        Ok(match self {
            NumericLiteral::Float(s) => s,
            NumericLiteral::Hex(s) => {
                let v = i64::from_str_radix(&s, 16)? as f64;
                Cow::Owned(v.to_string())
            }
        })
    }
}
impl core::fmt::Display for NumericLiteral<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Hex(s) => write!(f, "0x{s}"),
            Self::Float(s) => write!(f, "{s}"),
        }
    }
}

impl Default for NumericLiteral<'_> {
    fn default() -> Self {
        Self::Float(Cow::Borrowed("0.0"))
    }
}

impl serde::Serialize for NumericLiteral<'_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Hex(s) | Self::Float(s) => serializer.serialize_str(s),
        }
    }
}

impl<'a, 'de: 'a> serde::Deserialize<'de> for NumericLiteral<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// Inner deserialization struct
        struct NumericLiteralVisitor<'a> {
            lifetime: core::marker::PhantomData<&'a str>,
        }
        impl<'de> serde::de::Visitor<'de> for NumericLiteralVisitor<'de> {
            type Value = NumericLiteral<'de>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string, integer, or floating-point number")
            }
            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.starts_with("0x") {
                    // Parse hexadecimal value
                    let hex_value = v.trim_start_matches("0x");
                    usize::from_str_radix(hex_value, 16).map_or_else(
                        |_err| Err(E::custom(format!("Invalid hexadecimal value: {v}"))),
                        |_hex| Ok(NumericLiteral::Hex(Cow::Borrowed(&v[2..]))),
                    )
                } else if let Ok(_float) = v.parse::<f32>() {
                    Ok(NumericLiteral::Float(Cow::Borrowed(v)))
                } else {
                    Err(E::custom(format!("Invalid numeric value: {v}")))
                }
            }
        }
        deserializer.deserialize_str(NumericLiteralVisitor {
            lifetime: core::marker::PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_serialize() -> Result<(), serde_json::Error> {
        let actual = serde_json::to_string(&NumericLiteral::Hex(Cow::Borrowed("2a")))?;
        assert_eq!(actual, r#""2a""#);

        let actual = serde_json::to_string(&NumericLiteral::Float(Cow::Borrowed("123")))?;
        assert_eq!(actual, "123");

        let actual = serde_json::to_string(&NumericLiteral::Float(Cow::Borrowed("3.12")))?;
        assert_eq!(actual, "3.12");
        Ok(())
    }

    #[test]
    fn test_deserialize() -> Result<(), serde_json::Error> {
        let actual: NumericLiteral = serde_json::from_str(r#""0x2a""#)?;
        assert_eq!(actual, NumericLiteral::Hex(Cow::Borrowed("2a")));

        let actual: NumericLiteral = serde_json::from_str("123")?;
        assert_eq!(actual, NumericLiteral::Float(Cow::Borrowed("123")));

        let actual: NumericLiteral = serde_json::from_str("3.12")?;
        assert_eq!(actual, NumericLiteral::Float(Cow::Borrowed("3.12")));
        Ok(())
    }

    #[test]
    fn test_deserialize_invalid() {
        assert!(serde_json::from_str::<NumericLiteral>(r#""invalid""#).is_err());
    }
}
