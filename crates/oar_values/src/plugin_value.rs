//! A combination of the plugin name and the ID in it.
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

impl<'a> FormID<'a> {
    /// # Safety
    /// non prefix hexadecimal
    #[inline]
    pub const unsafe fn new_unchecked(cow: Cow<'a, str>) -> Self {
        Self(cow)
    }

    /// # Errors
    /// If prefix hexadecimal
    pub fn new(s: &'a str) -> Result<Self, &'static str> {
        fn is_valid_hex(s: &str) -> bool {
            !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit())
        }

        if is_valid_hex(s) {
            Ok(Self(Cow::Borrowed(s)))
        } else {
            Err("invalid hex string")
        }
    }
}

impl<'a> From<&'a str> for FormID<'a> {
    /// Clone into
    /// - NOTE: non cast to hex
    fn from(value: &'a str) -> Self {
        Self(value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_plugin_value() -> Result<(), serde_json::Error> {
        let actual = serde_json::to_string_pretty(&PluginValue {
            plugin_name: "MyPlugin".into(),
            form_id: FormID(Cow::Borrowed("3039")),
        })?;

        // NOTE: 0x3039 == 12345
        let expected = r#"{
  "pluginName": "MyPlugin",
  "formID": "3039"
}"#;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_plugin_value() -> Result<(), serde_json::Error> {
        let actual: PluginValue = serde_json::from_str(
            r#"{
  "pluginName": "AnotherPlugin",
  "formID": "fff"
}"#,
        )?;

        let expected = PluginValue {
            plugin_name: "AnotherPlugin".into(),
            form_id: "fff".into(),
        };

        assert_eq!(actual, expected);
        Ok(())
    }
}
