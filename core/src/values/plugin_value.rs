//! A combination of the plugin name and the ID in it.
use super::NumericLiteral;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// A combination of the plugin name and the ID in it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginValue<'a> {
    /// e.g. `Skyrim.esm`
    #[serde(rename = "pluginName")]
    #[serde(default)]
    pub plugin_name: Cow<'a, str>,
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
    pub form_id: FormID<'a>,
}

/// Non prefix(0x) hexadecimal ID
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormID<'a>(Cow<'a, str>);

impl<'a> From<&'a str> for FormID<'a> {
    /// Clone into
    /// - NOTE: non cast to hex
    fn from(value: &'a str) -> Self {
        Self(value.into())
    }
}

/// Macro for type conversion of [`NumericLiteral`] and its internal numeric value to [`FormID`].
macro_rules! from {
    ($($_type:ident),+ $(,)?) => {
        $(
            impl From<$_type> for FormID<'_> {
                fn from(value: $_type) -> Self {
                    NumericLiteral::from(value).into()
                }
            }
        )+
    };
}

from!(usize, isize, f32);

impl From<NumericLiteral> for FormID<'_> {
    fn from(value: NumericLiteral) -> Self {
        Self(match value {
            NumericLiteral::Hex(hex_value) => format!("{hex_value:x}").into(),
            NumericLiteral::Decimal(decimal_value) => match decimal_value == 0 {
                true => Default::default(),
                false => format!("{decimal_value:x}").into(),
            },
            NumericLiteral::Float(float_value) => format!("{:x}", float_value as usize).into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_plugin_value() -> Result<()> {
        let plugin_value = PluginValue {
            plugin_name: "MyPlugin".into(),
            form_id: NumericLiteral::Decimal(12345).into(),
        };

        // NOTE: 0x3039 == 12345
        let expected = r#"{
  "pluginName": "MyPlugin",
  "formID": "3039"
}"#;
        let serialized = serde_json::to_string_pretty(&plugin_value)?;
        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_plugin_value() -> Result<()> {
        let json_str = r#"{
  "pluginName": "AnotherPlugin",
  "formID": "fff"
}"#;

        let deserialized: PluginValue = serde_json::from_str(json_str)?;
        let expected = PluginValue {
            plugin_name: "AnotherPlugin".into(),
            form_id: "fff".into(),
        };

        assert_eq!(deserialized, expected);
        Ok(())
    }
}
