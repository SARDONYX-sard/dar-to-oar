//! Comparison
use serde::{Deserialize, Serialize};

/// Comparison
/// - Eq: Equal("==") <- default
/// - Ne: Not Equal("!=")
/// - Gt: Greater than(">")
/// - Lt: Less than("<")
/// - Ge: Greater than or equal(">=")
/// - Le: Lesser than or equal("<="),
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cmp {
    #[serde(rename = "==")]
    #[default]
    /// Equal("==")
    Eq,
    #[serde(rename = "!=")]
    /// Not Equal("!=")
    Ne,
    #[serde(rename = ">")]
    /// Greater than(">")
    Gt,
    #[serde(rename = "<")]
    /// Less than("<")
    Lt,
    #[serde(rename = ">=")]
    /// Greater than or equal(">=")
    Ge,
    #[serde(rename = "<=")]
    /// Lesser than or equal("<=")
    Le,
}
