use serde::{Deserialize, Serialize};

/// -99999996802856924650656260769173209088.000 <= value <= 9.999999680285692e37
///
/// when out of range(i.e. -inf or inf), break config.json. Example is here.
/// ```json:config.json
/// {
///   "condition": "CompareValues",
///   "requiredVersion": "1.0.0.0",
///   "Value A": {
///       "
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticValue {
    pub value: f32,
}
