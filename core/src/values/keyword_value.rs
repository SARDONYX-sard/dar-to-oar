//! Trigger keywords
use super::{FormValue, LiteralValue};
use serde::{Deserialize, Serialize};

// NOTE: Changing the order of enums will cause Deserialize to error.
/// Trigger keywords
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Keyword<'a> {
    /// plugin value
    Form(FormValue<'a>),
    /// Single numeric type
    Literal(LiteralValue<'a>),
}

impl Default for Keyword<'_> {
    fn default() -> Self {
        Self::Literal(LiteralValue::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;
    use crate::values::PluginValue;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_keyword_default() -> Result<()> {
        let keyword_enum = Keyword::default();
        let serialized = serde_json::to_string_pretty(&keyword_enum)?;
        let expected = r#"{
  "editorID": ""
}"#;
        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_keyword_enum_literal() -> Result<()> {
        let input = r#"{
  "editorID": "SomeKeyword"
}"#;
        let deserialized: Keyword = serde_json::from_str(input)?;
        let expected = Keyword::Literal(LiteralValue {
            editor_id: "SomeKeyword".into(),
        });

        assert_eq!(deserialized, expected);
        Ok(())
    }

    #[test]
    fn should_serialize_keyword_enum_form() -> Result<()> {
        let keyword_enum = Keyword::Form(FormValue::default());

        let expected = r#"{
  "form": {
    "pluginName": "",
    "formID": ""
  }
}"#;
        let serialized = serde_json::to_string_pretty(&keyword_enum)?;
        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_keyword_enum_form() -> Result<()> {
        let input = r#"{
  "form": {
    "pluginName": "MyPlugin",
    "formID": "12345"
  }
}"#;

        let deserialized: Keyword = serde_json::from_str(input)?;
        let expected = Keyword::Form(FormValue {
            form: PluginValue {
                plugin_name: "MyPlugin".into(),
                form_id: "12345".into(),
            },
        });

        assert_eq!(deserialized, expected);
        Ok(())
    }
}
