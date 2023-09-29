use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiteralValue {
    #[serde(rename = "editorID")]
    #[serde(default)]
    pub editor_id: String,
}
