use super::condition::Condition;
use crate::converter::values::Keyword;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HasKeyword {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(default)]
    pub keyword: Keyword,
}

impl Default for HasKeyword {
    fn default() -> Self {
        Self {
            condition: Condition::new("HasKeyword"),
            keyword: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::converter::values::LiteralValue;

    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json;

    #[test]
    fn should_serialize_has_keyword() {
        let has_keyword = HasKeyword::default();

        let expected = r#"{
  "condition": "HasKeyword",
  "requiredVersion": "1.0.0.0",
  "Keyword": {
    "editorID": ""
  }
}"#;
        let serialized = serde_json::to_string_pretty(&has_keyword).unwrap();
        assert_eq!(expected, serialized);
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

        assert_eq!(expected, deserialized);
    }
}
