//! A static value within a certain range.
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// A static value within a certain range.
/// # NOTE
/// The value is expected to be within the range -99999996802856924650656260769173209088.000
/// to 9.999999680285692e37.
///
/// If the value is out of this range (i.e., -inf or inf), it may cause issues with `config.json` serialization.
///
/// ```json:config.json
/// {
///   "condition": "CompareValues",
///   "requiredVersion": "1.0.0.0",
///   "Value A": {
///       "
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct StaticValue<'a> {
    /// The value of the static value.
    #[serde(borrow)]
    pub value: Cow<'a, str>,
}

impl<'de> serde::Deserialize<'de> for StaticValue<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: Cow<'de, str> = Deserialize::deserialize(deserializer)?;

        let v: f32 = s.parse().map_err(serde::de::Error::custom)?;
        if !v.is_finite() {
            return Err(serde::de::Error::custom("value must be finite"));
        }

        Ok(Self { value: s })
    }
}
