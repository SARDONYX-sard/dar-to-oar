//! Hex | Decimal | Float
use core::fmt;

/// Hex | Decimal | Float
#[derive(Clone, PartialEq)]
pub enum NumberLiteral {
    /// e.g. 0x007
    Hex(usize),
    /// e.g. 1
    Decimal(isize),
    /// e.g. 1.0
    Float(f32),
}

// Hex debugging display is displayed in hexadecimal notation because it is difficult to understand if it is in decimal.
impl fmt::Debug for NumberLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hex(arg0) => f.debug_tuple("Hex").field(&format!("{arg0:#x}")).finish(),
            Self::Decimal(arg0) => f.debug_tuple("Decimal").field(arg0).finish(),
            Self::Float(arg0) => f.debug_tuple("Float").field(arg0).finish(),
        }
    }
}

impl fmt::Display for NumberLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hex(hex) => write!(f, "0x{hex:x}"),
            Self::Decimal(decimal) => write!(f, "{decimal}"),
            Self::Float(float) => write!(f, "{float}"),
        }
    }
}
