use super::NumericLiteral;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PluginValue {
    #[serde(rename = "pluginName")]
    #[serde(default)]
    pub plugin_name: String,
    /// - OAR: Non prefix(0x) hexadecimal
    /// - DAR: Decimal or Hex
    ///
    /// Therefore, convert to hexadecimal all DAR number for ID.
    ///
    /// # NOTE:
    /// The actual OAR GUI input hex would look like this.
    /// ```json
    /// { "formID": "0\u0000\u0000\u0000\u0000\u0000" }, // If input 0
    /// { "formID": "0A\u0000\u0000\u0000\u0000" } // If input a
    /// ```
    /// However, this converter does not take it into account. The reasons are as follows.
    /// - Some DAR mods allow values larger than the allowed 6 digits to be entered.
    /// - OAR can read hexes that are not filled with zeros.
    #[serde(rename = "formID")]
    #[serde(default)]
    pub form_id: FormID,
}

/// Non prefix(0x) hexadecimal ID
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FormID(String);

impl From<&str> for FormID {
    /// Clone into
    /// - NOTE: non cast to hex
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

macro_rules! from {
    ($($_type:ident),+ $(,)?) => {
        $(
            impl From<$_type> for FormID {
                fn from(value: $_type) -> Self {
                    NumericLiteral::from(value).into()
                }
            }
        )+
    };
}

from!(usize, isize, f32);

impl From<NumericLiteral> for FormID {
    fn from(value: NumericLiteral) -> Self {
        Self(match value {
            NumericLiteral::Hex(hex_value) => {
                format!("{:x}", hex_value)
            }
            NumericLiteral::Decimal(decimal_value) => {
                if decimal_value == 0 {
                    String::default()
                } else {
                    format!("{:x}", decimal_value)
                }
            }
            NumericLiteral::Float(float_value) => {
                format!("{:x}", float_value as usize)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_plugin_value() {
        let plugin_value = PluginValue {
            plugin_name: "MyPlugin".into(),
            form_id: NumericLiteral::Decimal(12345).into(),
        };

        // NOTE: 0x3039 == 12345
        let expected = r#"{
  "pluginName": "MyPlugin",
  "formID": "3039"
}"#;
        let serialized = serde_json::to_string_pretty(&plugin_value).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize_plugin_value() {
        let json_str = r#"{
  "pluginName": "AnotherPlugin",
  "formID": "fff"
}"#;

        let deserialized: PluginValue = serde_json::from_str(json_str).unwrap();
        let expected = PluginValue {
            plugin_name: "AnotherPlugin".into(),
            form_id: "fff".into(),
        };

        assert_eq!(deserialized, expected);
    }

    #[test]
    fn should_default_plugin_value() {
        let default_plugin_value = PluginValue::default();

        let expected = PluginValue {
            plugin_name: "".into(),
            form_id: "".into(),
        };

        assert_eq!(default_plugin_value, expected);
    }
}
