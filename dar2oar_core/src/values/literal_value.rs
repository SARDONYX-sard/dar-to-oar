use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LiteralValue {
    #[serde(rename = "editorID")]
    #[serde(default)]
    pub editor_id: String,
}
