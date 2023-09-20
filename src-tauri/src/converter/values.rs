use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ActorValue {
    pub actor_value: Option<i32>,
    pub actor_value_type: String,
}

impl Default for ActorValue {
    fn default() -> Self {
        Self {
            actor_value: None,
            actor_value_type: String::from("Value"),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct KeywordValue {
    pub editor_id: Option<String>,
    pub form: Option<PluginValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NumericValue {
    pub value: f32,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PluginValue {
    pub plugin_name: String,
    pub form_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct RandomValue {
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct TypeValue {
    pub value: f32,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum ValueSet {
    ActorValue(ActorValue),
    KeywordValue(KeywordValue),
    NumericValue(NumericValue),
    PluginValue(PluginValue),
    RandomValue(RandomValue),
    TypeValue(TypeValue),
    #[default]
    Unknown,
}
