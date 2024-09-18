//! A static value within a certain range.
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
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StaticValue {
    /// The value of the static value.
    pub value: f32,
}

impl From<f32> for StaticValue {
    fn from(value: f32) -> Self {
        Self { value }
    }
}
