//! Represents a generic condition.
use super::is_false;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Representing the default required version.
pub const REQUIRED_VERSION: &str = "1.0.0.0";

/// Create a default required version as a [`CompactString`].
#[inline]
pub const fn default_required_version() -> Cow<'static, str> {
    Cow::Borrowed(REQUIRED_VERSION)
}

/// Represents a generic condition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Condition<'a> {
    /// The name of the condition (e.g., "`IsWornHasKeyword`").
    #[serde(default)]
    pub condition: Cow<'a, str>,
    /// The required version for this condition.
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: Cow<'a, str>,
    /// Indicates whether the condition is negated or not (default: `false`).
    ///
    /// # NOTE
    /// There is code written under the assumption that it is skipped when false (e.g., `IsEquipped`).
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,
}

impl Default for Condition<'_> {
    fn default() -> Self {
        Self {
            condition: Default::default(),
            required_version: default_required_version(),
            negated: false,
        }
    }
}

impl<'a> Condition<'a> {
    /// Creates a new `Condition` with the specified name.
    pub fn new(condition: &'a str) -> Self {
        Self {
            condition: condition.into(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;
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
    fn serialize_condition() -> Result<()> {
        let condition = Condition {
            condition: "SomeCondition".into(),
            required_version: REQUIRED_VERSION.into(),
            negated: true,
        };
        let serialized_json = serde_json::to_string_pretty(&condition)?;

        let expected_json = r#"{
  "condition": "SomeCondition",
  "requiredVersion": "1.0.0.0",
  "negated": true
}"#;

        assert_eq!(serialized_json, expected_json);
        Ok(())
    }

    #[test]
    fn deserialize_condition() -> Result<()> {
        let json_str = r#"{
            "condition": "AnotherCondition",
            "requiredVersion": "1.0.0.0",
            "negated": false
        }"#;
        let deserialized: Condition = serde_json::from_str(json_str)?;

        let expected = Condition {
            condition: "AnotherCondition".into(),
            required_version: REQUIRED_VERSION.into(),
            negated: false,
        };

        assert_eq!(deserialized, expected);
        Ok(())
    }
}
