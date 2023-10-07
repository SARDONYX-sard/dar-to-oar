use super::NumericLiteral;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PluginValue {
    #[serde(rename = "pluginName")]
    #[serde(default)]
    pub plugin_name: String,
    /// hexadecimal representation except prefix(0x)
    /// - NOTE:
    /// OAR seems to only allow input of hexadecimal numbers, but also supports decimal numbers for DAR.
    /// But convert to hexadecimal when holding
    ///
    /// TODO: Support \u0000 pattern
    /// "formID": "0\u0000\u0000\u0000\u0000\u0000"
    /// "formID": "0A\u0000\u0000\u0000\u0000"
    #[serde(rename = "formID")]
    #[serde(default)]
    pub form_id: FormID,
}

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

        // NOTE: formID: 3039 is hex(12345)
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
  "formID": "0xfff"
}"#;

        let deserialized: PluginValue = serde_json::from_str(json_str).unwrap();
        let expected = PluginValue {
            plugin_name: "AnotherPlugin".into(),
            form_id: "0xfff".into(),
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
