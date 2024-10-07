//! Trigger keywords
use super::{FormValue, LiteralValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Trigger keywords
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum Keyword {
    /// Single numeric type
    Literal(LiteralValue),
    /// plugin value
    Form(FormValue),
}

impl Default for Keyword {
    fn default() -> Self {
        Self::Literal(LiteralValue::default())
    }
}

impl<'de> Deserialize<'de> for Keyword {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;

        if let Value::Object(map) = &value {
            if map.contains_key("editorID") {
                // If the "editorID" field is present, assume it's a Literal
                let keyword_value: LiteralValue = match serde_json::from_value(value) {
                    Ok(keyword) => keyword,
                    Err(err) => return Err(serde::de::Error::custom(err)),
                };
                Ok(Self::Literal(keyword_value))
            } else if map.contains_key("form") {
                // If both "pluginName" and "formID" fields are present, assume it's a Form
                let form_value: FormValue = match serde_json::from_value(value) {
                    Ok(form) => form,
                    Err(err) => return Err(serde::de::Error::custom(err)),
                };
                Ok(Self::Form(form_value))
            } else {
                Err(serde::de::Error::custom(
                    "Unable to determine Keyword variant",
                ))
            }
        } else {
            Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Other("Expected an object"),
                &"a map",
            ))
        }
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
            editor_id: "SomeKeyword".to_string(),
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
