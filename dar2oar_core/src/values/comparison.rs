//! Comparison
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// Comparison
/// - Eq: Equal("==") <- default
/// - Ne: Not Equal("!=")
/// - Gt: Greater than(">")
/// - Lt: Less than("<")
/// - Ge: Greater than or equal(">=")
/// - Le: Lesser than or equal("<="),
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Cmp {
    #[default]
    /// Equal("==")
    Eq,
    /// Not Equal("!=")
    Ne,
    /// Greater than(">")
    Gt,
    /// Less than("<")
    Lt,
    /// Greater than or equal(">=")
    Ge,
    /// Lesser than or equal("<=")
    Le,
}

// Implement Display trait for pretty printing
impl fmt::Display for Cmp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Eq => write!(f, "=="),
            Self::Ne => write!(f, "!="),
            Self::Gt => write!(f, ">"),
            Self::Lt => write!(f, "<"),
            Self::Ge => write!(f, ">="),
            Self::Le => write!(f, "<="),
        }
    }
}

// Implement TryFrom<&str> to parse from a string
impl TryFrom<&str> for Cmp {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "==" => Self::Eq,
            "!=" => Self::Ne,
            ">" => Self::Gt,
            "<" => Self::Lt,
            ">=" => Self::Ge,
            "<=" => Self::Le,
            _ => {
                return Err(
                    "Invalid comparison operator. Expected '==', '!=', '>', '<', '>=' or '<='",
                )
            }
        })
    }
}

impl From<Cmp> for &str {
    fn from(value: Cmp) -> Self {
        match value {
            Cmp::Eq => "==",
            Cmp::Ne => "!=",
            Cmp::Gt => ">",
            Cmp::Lt => "<",
            Cmp::Ge => ">=",
            Cmp::Le => "<=",
        }
    }
}

impl Serialize for Cmp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.clone().into())
    }
}

impl<'de> Deserialize<'de> for Cmp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let cmp_str = String::deserialize(deserializer)?;
        cmp_str
            .as_str()
            .try_into()
            .map_err(serde::de::Error::custom)
    }
}
