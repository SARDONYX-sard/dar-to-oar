//! Type conversion definitions to change from DAR syntax to OAR json

use crate::{
    dar_syntax::syntax::{FnArg, NumberLiteral},
    values::{
        Direction, DirectionValue, FormValue, Keyword, NumericLiteral, NumericValue, PluginValue,
        StaticValue,
    },
};

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum ParseError {
    /// - 1st arg: Expected value
    /// - 2nd arg: Actual value
    #[error("Expected {0}. but got {1}")]
    UnexpectedValue(String, String),
}

impl From<NumberLiteral> for NumericLiteral {
    fn from(value: NumberLiteral) -> Self {
        match value {
            NumberLiteral::Hex(num) => NumericLiteral::Hex(num),
            NumberLiteral::Decimal(num) => NumericLiteral::Decimal(num),
            NumberLiteral::Float(num) => NumericLiteral::Float(num),
        }
    }
}
impl From<&NumberLiteral> for NumericLiteral {
    fn from(value: &NumberLiteral) -> Self {
        match *value {
            NumberLiteral::Hex(num) => NumericLiteral::Hex(num),
            NumberLiteral::Decimal(num) => NumericLiteral::Decimal(num),
            NumberLiteral::Float(num) => NumericLiteral::Float(num),
        }
    }
}

impl TryFrom<FnArg<'_>> for NumericLiteral {
    type Error = ParseError;

    fn try_from(value: FnArg) -> Result<Self, Self::Error> {
        match value {
            FnArg::PluginValue {
                plugin_name,
                form_id,
            } => Err(ParseError::UnexpectedValue(
                "Float(e.g. 3.0)".to_string(),
                format!(
                    "\"PluginValue\": {{ plugin_name: \"{}\", form_id: {} }}",
                    plugin_name, form_id
                ),
            )),
            FnArg::Number(num) => Ok(num.into()),
        }
    }
}

impl TryFrom<&FnArg<'_>> for NumericLiteral {
    type Error = ParseError;

    fn try_from(value: &FnArg) -> Result<Self, Self::Error> {
        match value {
            FnArg::PluginValue {
                plugin_name,
                form_id,
            } => Err(ParseError::UnexpectedValue(
                "Float(e.g. 3.0)".to_string(),
                format!(
                    "\"PluginValue\": {{ plugin_name: \"{}\", form_id: {} }}",
                    plugin_name, form_id
                ),
            )),
            FnArg::Number(num) => Ok(num.into()),
        }
    }
}

impl From<&FnArg<'_>> for NumericValue {
    fn from(value: &FnArg) -> Self {
        match value {
            FnArg::PluginValue {
                plugin_name,
                form_id,
            } => Self::GlobalVariable(PluginValue {
                plugin_name: plugin_name.to_string(),
                form_id: NumericLiteral::from(form_id).into(),
            }),
            FnArg::Number(num) => match num {
                NumberLiteral::Float(value) => Self::StaticValue((*value).into()),
                NumberLiteral::Decimal(value) => Self::StaticValue((*value as f32).into()),
                NumberLiteral::Hex(value) => Self::StaticValue((*value as f32).into()),
            },
        }
    }
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
                format!(
                    "\"PluginValue\": {{ plugin_name: \"{}\", form_id: {} }}",
                    plugin_name, form_id
                ),
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

impl TryFrom<&FnArg<'_>> for StaticValue {
    type Error = ParseError;

    fn try_from(value: &FnArg) -> Result<Self, Self::Error> {
        match value {
            FnArg::PluginValue {
                plugin_name,
                form_id,
            } => Err(ParseError::UnexpectedValue(
                "Float(e.g. 3.0)".to_string(),
                format!(
                    "\"PluginValue\": {{ plugin_name: \"{}\", form_id: {} }}",
                    plugin_name, form_id
                ),
            )),
            FnArg::Number(num) => match num {
                NumberLiteral::Float(value) => Ok(Self { value: *value }),
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
                form_id: NumericLiteral::from(form_id).into(),
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

impl TryFrom<&FnArg<'_>> for PluginValue {
    type Error = ParseError;

    fn try_from(value: &FnArg) -> Result<Self, Self::Error> {
        match value {
            FnArg::PluginValue {
                plugin_name,
                form_id,
            } => Ok(Self {
                plugin_name: plugin_name.to_string(),
                form_id: NumericLiteral::from(form_id).into(),
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

impl From<&FnArg<'_>> for Keyword {
    fn from(value: &FnArg<'_>) -> Self {
        match value {
            FnArg::PluginValue {
                plugin_name,
                form_id,
            } => Self::Form(FormValue {
                form: PluginValue {
                    plugin_name: plugin_name.to_string(),
                    form_id: NumericLiteral::from(form_id).into(),
                },
            }),
            FnArg::Number(num) => match num {
                num => Self::Literal(crate::values::LiteralValue {
                    editor_id: NumericLiteral::from(num).to_string(),
                }),
            },
        }
    }
}

impl TryFrom<&FnArg<'_>> for Direction {
    type Error = ParseError;

    fn try_from(value: &FnArg<'_>) -> Result<Self, Self::Error> {
        match value {
            FnArg::PluginValue {
                plugin_name,
                form_id,
            } => Err(ParseError::UnexpectedValue(
                "Float(e.g. 3.0)".to_string(),
                format!(
                    "\"PluginValue\": {{ plugin_name: \"{}\", form_id: {} }}",
                    plugin_name, form_id
                ),
            )),
            FnArg::Number(num) => Ok(match *num {
                NumberLiteral::Hex(num) => (num as u64)
                    .try_into()
                    .map_err(|e: &str| ParseError::UnexpectedValue(e.into(), "0..=4".into()))?,
                NumberLiteral::Decimal(num) => (num as u64)
                    .try_into()
                    .map_err(|e: &str| ParseError::UnexpectedValue(e.into(), "0..=4".into()))?,
                NumberLiteral::Float(num) => (num as u64)
                    .try_into()
                    .map_err(|e: &str| ParseError::UnexpectedValue(e.into(), "0..=4".into()))?,
            }),
        }
    }
}

impl TryFrom<&FnArg<'_>> for DirectionValue {
    type Error = ParseError;

    fn try_from(value: &FnArg<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            value: value.try_into()?,
        })
    }
}
