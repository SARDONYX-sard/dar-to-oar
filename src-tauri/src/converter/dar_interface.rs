use super::dar_syntax::{FnArg, NumberLiteral};
use super::values::{PluginValue, StaticValue};

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    /// - 1st arg: Expected value
    /// - 2nd arg: Actual value
    #[error("Expected {0}. but got {1}")]
    UnexpectedValue(String, String),
}

impl TryFrom<FnArg<'_>> for StaticValue {
    type Error = ParseError;

    fn try_from(value: FnArg) -> Result<Self, Self::Error> {
        match value {
            FnArg::PluginValue {
                plugin_name,
                form_id,
            } => Err(ParseError::UnexpectedValue(
                "Float(e.g. 3.0)".to_string(),
                "Plugin name, id".to_string(),
            )),
            FnArg::Number(num) => match num {
                NumberLiteral::Float(value) => Ok(Self { value }),
                num => Err(ParseError::UnexpectedValue(
                    "Float(e.g. 3.0)".to_string(),
                    num.to_string(),
                )),
            },
        }
    }
}

impl TryFrom<FnArg<'_>> for PluginValue {
    type Error = ParseError;

    fn try_from(value: FnArg) -> Result<Self, Self::Error> {
        match value {
            FnArg::PluginValue {
                plugin_name,
                form_id,
            } => Ok(Self {
                plugin_name: plugin_name.to_string(),
                form_id: form_id.to_string(),
            }),
            FnArg::Number(num) => match num {
                num => Err(ParseError::UnexpectedValue(
                    "Float(e.g. 3.0".to_string(),
                    num.to_string(),
                )),
            },
        }
    }
}
