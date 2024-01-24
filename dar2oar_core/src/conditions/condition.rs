use super::is_false;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

pub const REQUIRED_VERSION: &str = "1.0.0.0";

pub fn default_required_version() -> CompactString {
    REQUIRED_VERSION.into()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    /// Condition name (e.g. IsWornHasKeyword)
    #[serde(default)]
    pub condition: CompactString,
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    /// The required version for this condition.
    pub required_version: CompactString,
    /// condition to **Not** (default is `false`).
    #[serde(default)]
    // NOTE: There is code written under the assumption that it is skipped when false (e.g. IsEquipped).
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            condition: CompactString::default(),
            required_version: default_required_version(),
            negated: false,
        }
    }
}

impl Condition {
    /// Creates a new `Condition` with the specified name.
    pub fn new(condition: &str) -> Self {
        Self {
            condition: condition.into(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn default_condition() {
        let default_condition = Condition::default();
        assert_eq!(default_condition.condition, "");
        assert_eq!(default_condition.required_version, REQUIRED_VERSION);
        assert_eq!(default_condition.negated, false);
    }

    #[test]
    fn create_condition() {
        let condition_name = "IsWornHasKeyword";
        let condition = Condition::new(condition_name);
        assert_eq!(condition.condition, condition_name);
        assert_eq!(condition.required_version, REQUIRED_VERSION);
        assert_eq!(condition.negated, false);
    }

    #[test]
    fn serialize_condition() {
        let condition = Condition {
            condition: "SomeCondition".into(),
            required_version: REQUIRED_VERSION.into(),
            negated: true,
        };
        let serialized_json = serde_json::to_string_pretty(&condition).unwrap();

        let expected_json = r#"{
  "condition": "SomeCondition",
  "requiredVersion": "1.0.0.0",
  "negated": true
}"#;

        assert_eq!(serialized_json, expected_json);
    }

    #[test]
    fn deserialize_condition() {
        let json_str = r#"{
            "condition": "AnotherCondition",
            "requiredVersion": "1.0.0.0",
            "negated": false
        }"#;
        let deserialized: Condition = serde_json::from_str(json_str).unwrap();

        let expected = Condition {
            condition: "AnotherCondition".into(),
            required_version: REQUIRED_VERSION.into(),
            negated: false,
        };

        assert_eq!(deserialized, expected);
    }
}
