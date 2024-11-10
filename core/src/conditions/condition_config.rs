//! Represents the configuration for each animation root specified in a `config.json` file.
use super::ConditionSet;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents the configuration for each animation root specified in a `config.json` file.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConditionsConfig<'a> {
    /// An arbitrary name given by the user (value in the mapping table).
    ///
    /// # Note
    /// The name will probably exceed 24 bytes, so it should not be a [`CompactString`].
    #[serde(default)]
    pub name: Cow<'a, str>,
    /// The description associated with the animation root configuration.
    #[serde(default)]
    pub description: Cow<'a, str>,
    /// The priority of the animation root.
    #[serde(default)]
    pub priority: i32,
    /// An optional override for the animations folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "overrideAnimationsFolder")]
    pub override_animations_folder: Option<Cow<'a, str>>,
    /// A vector containing the conditions associated with the animation root.
    pub conditions: Vec<ConditionSet<'a>>,
}
