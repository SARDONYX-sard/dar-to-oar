use super::{LiteralValue, PluginValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Keyword {
    Literal(LiteralValue),
    Form(FormValue),
}

/// Wrapper for wrapping pluginValue with a key called "form"
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FormValue {
    pub form: PluginValue,
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
                let keyword_value: LiteralValue = serde_json::from_value(value).unwrap();
                Ok(Keyword::Literal(keyword_value))
            } else if map.contains_key("form") {
                // If both "pluginName" and "formID" fields are present, assume it's a Form
                let form_value: FormValue = serde_json::from_value(value).unwrap();
                Ok(Keyword::Form(form_value))
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
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_keyword_default() {
        let keyword_enum = Keyword::default();
        let serialized = serde_json::to_string_pretty(&keyword_enum).unwrap();
        let expected = r#"{
  "editorID": ""
}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize_keyword_enum_literal() {
        let input = r#"{
  "editorID": "SomeKeyword"
}"#;
        let deserialized: Keyword = serde_json::from_str(input).unwrap();
        let expected = Keyword::Literal(LiteralValue {
            editor_id: "SomeKeyword".to_string(),
        });

        assert_eq!(deserialized, expected);
    }

    #[test]
    fn should_serialize_keyword_enum_form() {
        let keyword_enum = Keyword::Form(FormValue::default());

        let expected = r#"{
  "form": {
    "pluginName": "",
    "formID": ""
  }
}"#;
        let serialized = serde_json::to_string_pretty(&keyword_enum).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize_keyword_enum_form() {
        let input = r#"{
  "form": {
    "pluginName": "MyPlugin",
    "formID": "12345"
  }
}"#;

        let deserialized: Keyword = serde_json::from_str(input).unwrap();
        let expected = Keyword::Form(FormValue {
            form: PluginValue {
                plugin_name: "MyPlugin".to_string(),
                form_id: "12345".into(),
            },
        });

        assert_eq!(deserialized, expected);
    }
}
