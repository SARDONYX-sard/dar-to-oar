use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RandomValue {
    #[serde(default)]
    pub min: f32,
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

const fn max_percent() -> f32 {
    1.0
}
