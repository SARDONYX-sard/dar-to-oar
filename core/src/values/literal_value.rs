//! String Literal
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Wrapper `editor_id`
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiteralValue<'a> {
    /// Editor ID
    #[serde(rename = "editorID")]
    #[serde(default)]
    pub editor_id: Cow<'a, str>,
}
