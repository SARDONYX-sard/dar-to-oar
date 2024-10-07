//! Represents a condition to check if an entity has a specific keyword.
use super::{condition::default_required_version, is_false};
use crate::values::Keyword;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// Represents a condition to check if an entity has a specific keyword.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasKeyword {
    /// The name of the condition, which is "`HasKeyword`".
    pub condition: CompactString,
    /// The required version for this condition.
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: CompactString,
    /// Indicates whether the condition is negated or not.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    /// The keyword to check for in the entity.
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
    use crate::error::Result;
    use crate::values::LiteralValue;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_has_keyword() -> Result<()> {
        let has_keyword = HasKeyword::default();
        let serialized = serde_json::to_string_pretty(&has_keyword)?;

        let expected = r#"{
  "condition": "HasKeyword",
  "requiredVersion": "1.0.0.0",
  "Keyword": {
    "editorID": ""
  }
}"#;

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_has_keyword() -> Result<()> {
        let json_str = r#"{
  "condition": "HasKeyword",
  "requiredVersion": "1.0.0.0",
  "Keyword": {
    "editorID": "SomeKeyword"
  }
}"#;
        let deserialized: HasKeyword = serde_json::from_str(json_str)?;

        let expected = HasKeyword {
            keyword: Keyword::Literal(LiteralValue {
                editor_id: "SomeKeyword".to_string(),
            }),
            ..Default::default()
        };

        assert_eq!(deserialized, expected);
        Ok(())
    }
}
