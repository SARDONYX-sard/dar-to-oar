//! String Literal
use serde::{Deserialize, Serialize};

/// Wrapper `editor_id`
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiteralValue {
    /// Editor ID
    #[serde(rename = "editorID")]
    #[serde(default)]
    pub editor_id: String,
}
