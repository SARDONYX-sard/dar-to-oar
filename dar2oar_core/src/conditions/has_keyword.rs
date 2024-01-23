use super::{condition::default_required_version, is_false};
use crate::values::Keyword;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasKeyword {
    /// Condition name "HasKeyword"
    pub condition: CompactString,
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: CompactString,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    #[serde(default)]
    #[serde(rename = "Keyword")]
    pub keyword: Keyword,
}

impl Default for HasKeyword {
    fn default() -> Self {
        Self {
            condition: "HasKeyword".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            keyword: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::LiteralValue;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_has_keyword() {
        let has_keyword = HasKeyword::default();
        let serialized = serde_json::to_string_pretty(&has_keyword).unwrap();

        let expected = r#"{
  "condition": "HasKeyword",
  "requiredVersion": "1.0.0.0",
  "Keyword": {
    "editorID": ""
  }
}"#;

        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize_has_keyword() {
        let json_str = r#"{
  "condition": "HasKeyword",
  "requiredVersion": "1.0.0.0",
  "Keyword": {
    "editorID": "SomeKeyword"
  }
}"#;
        let deserialized: HasKeyword = serde_json::from_str(json_str).unwrap();

        let expected = HasKeyword {
            keyword: Keyword::Literal(LiteralValue {
                editor_id: "SomeKeyword".to_string(),
            }),
            ..Default::default()
        };

        assert_eq!(deserialized, expected);
    }
}
