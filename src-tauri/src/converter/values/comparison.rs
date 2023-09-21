use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone, Default, PartialEq)]
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
            Cmp::Eq => write!(f, "=="),
            Cmp::Ne => write!(f, "!="),
            Cmp::Gt => write!(f, ">"),
            Cmp::Lt => write!(f, "<"),
            Cmp::Ge => write!(f, ">="),
            Cmp::Le => write!(f, "<="),
        }
    }
}

// Implement TryFrom<&str> to parse from a string
impl TryFrom<&str> for Cmp {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "==" => Cmp::Eq,
            "!=" => Cmp::Ne,
            ">" => Cmp::Gt,
            "<" => Cmp::Lt,
            ">=" => Cmp::Ge,
            "<=" => Cmp::Le,
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
            .map_err(|err| serde::de::Error::custom(err))
    }
}
