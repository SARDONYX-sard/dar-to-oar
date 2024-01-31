//! A value with a range, used for randomization.
use serde::{Deserialize, Serialize};

/// A value with a range, used for randomization.
///
/// This struct has `min` and `max` fields to define the range of the random value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RandomValue {
    /// The minimum value of the range.
    #[serde(default)]
    pub min: f32,
    /// The maximum value of the range, with a default value of 1.0 (100%).
    #[serde(default = "max_percent")]
    pub max: f32,
}

impl Default for RandomValue {
    fn default() -> Self {
        Self {
            min: Default::default(),
            max: max_percent(),
        }
    }
}

/// Returns the maximum percentage value (1.0) used as the default for `RandomValue::max`.
///
/// This function is used as the default value for the `max` field in `RandomValue`.
const fn max_percent() -> f32 {
    1.0
}
