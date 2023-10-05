use super::is_false;
use serde::{Deserialize, Serialize};

pub const REQUIRED_VERSION: &str = "1.0.0.0";

pub fn default_required_version() -> String {
    REQUIRED_VERSION.to_string()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    /// Condition name (e.g. IsWornHasKeyword)
    #[serde(default)]
    pub condition: String,
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    /// The required version for this condition.
    pub required_version: String,
    /// condition to **Not** (default is `false`).
    #[serde(default)]
    // NOTE: There is code written under the assumption that it is skipped when false (e.g. IsEquipped).
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            condition: String::default(),
            required_version: REQUIRED_VERSION.to_string(),
            negated: false,
        }
    }
}

impl Condition {
    /// Creates a new `Condition` with the specified name.
    pub fn new(condition: &str) -> Self {
        Self {
            condition: condition.to_string(),
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
            condition: "SomeCondition".to_string(),
            required_version: REQUIRED_VERSION.to_string(),
            negated: true,
        };

        let expected_json = r#"{
  "condition": "SomeCondition",
  "requiredVersion": "1.0.0.0",
  "negated": true
}"#;

        let serialized_json = serde_json::to_string_pretty(&condition).unwrap();
        assert_eq!(expected_json, serialized_json);
    }

    #[test]
    fn deserialize_condition() {
        let json_str = r#"{
            "condition": "AnotherCondition",
            "requiredVersion": "1.0.0.0",
            "negated": false
        }"#;

        let expected_condition = Condition {
            condition: "AnotherCondition".to_string(),
            required_version: REQUIRED_VERSION.to_string(),
            negated: false,
        };

        let deserialized_condition: Condition = serde_json::from_str(json_str).unwrap();
        assert_eq!(expected_condition, deserialized_condition);
    }
}
