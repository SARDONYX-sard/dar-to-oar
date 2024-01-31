//! Represents a condition to check if an entity has a specific perk.
use super::{condition::default_required_version, is_false};
use crate::values::PluginValue;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// Represents a condition to check if an entity has a specific perk.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasPerk {
    /// The name of the condition, which is "HasPerk".
    pub condition: CompactString,
    /// The required version for this condition.
    #[serde(default = "default_required_version")]
    #[serde(rename = "requiredVersion")]
    pub required_version: CompactString,
    /// Indicates whether the condition is negated or not.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub negated: bool,

    /// The perk to check for in the entity.
    #[serde(default)]
    #[serde(rename = "Perk")]
    pub perk: PluginValue,
}

impl Default for HasPerk {
    fn default() -> Self {
        Self {
            condition: "HasPerk".into(),
            required_version: default_required_version(),
            negated: Default::default(),
            perk: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_has_perk() -> Result<()> {
        let has_perk = HasPerk::default();
        let serialized = serde_json::to_string_pretty(&has_perk)?;

        let expected = r#"{
  "condition": "HasPerk",
  "requiredVersion": "1.0.0.0",
  "Perk": {
    "pluginName": "",
    "formID": ""
  }
}"#;

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_has_perk() -> Result<()> {
        let json_str = r#"{
  "condition": "HasPerk",
  "requiredVersion": "1.0.0.0",
  "negated": true,
  "Perk": {
    "pluginName": "SomePlugin",
    "formID": "12345"
  }
}"#;
        let deserialized: HasPerk = serde_json::from_str(json_str)?;

        let expected = HasPerk {
            negated: true,
            perk: PluginValue {
                plugin_name: "SomePlugin".into(),
                form_id: "12345".into(),
            },
            ..Default::default()
        };

        assert_eq!(deserialized, expected);
        Ok(())
    }
}
