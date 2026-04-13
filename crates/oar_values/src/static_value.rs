//! A static value within a certain range.

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
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StaticValue {
    // NOTE: Even if you change this to Cow<'a, str>, during JSON serialization
    // we cannot insert the value as a number without quotes.
    // Therefore, trying to avoid heap allocation by using Cow has no real benefit here.
    /// The value of the static value.
    pub value: f64,
}
