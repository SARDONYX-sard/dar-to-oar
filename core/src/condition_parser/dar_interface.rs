//! Type conversion definitions to change from DAR syntax to OAR json

use super::errors::{ParseError, Result};
use crate::{
    dar_syntax::syntax::{FnArg, NumberLiteral},
    values::{
        Direction, FormValue, Keyword, LiteralValue, NumericLiteral, NumericValue, PluginValue,
        StaticValue,
    },
};

impl From<NumberLiteral> for NumericLiteral {
    fn from(value: NumberLiteral) -> Self {
        match value {
            NumberLiteral::Decimal(num) => Self::Decimal(num),
            NumberLiteral::Float(num) => Self::Float(num),
            NumberLiteral::Hex(num) => Self::Hex(num),
        }
    }
}

impl From<&NumberLiteral> for NumericLiteral {
    fn from(value: &NumberLiteral) -> Self {
        //! Note: omitting the definition using owned into will result in a circular loop.
        match *value {
            NumberLiteral::Decimal(num) => Self::Decimal(num),
            NumberLiteral::Float(num) => Self::Float(num),
            NumberLiteral::Hex(num) => Self::Hex(num),
        }
    }
}

impl TryFrom<FnArg<'_>> for NumericLiteral {
    type Error = ParseError;

    fn try_from(value: FnArg) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&FnArg<'_>> for NumericLiteral {
    type Error = ParseError;

    fn try_from(value: &FnArg) -> Result<Self, Self::Error> {
        match value {
            FnArg::Number(num) => Ok(num.into()),
            other @ FnArg::PluginValue { .. } => Err(ParseError::UnexpectedValue(
                "Number(e.g. 3.0)".into(),
                format!("{other:?}",),
            )),
        }
    }
}

impl<'a> From<FnArg<'a>> for NumericValue<'a> {
    fn from(value: FnArg<'a>) -> Self {
        match value {
            FnArg::PluginValue {
                plugin_name,
                form_id,
            } => Self::GlobalVariable(
                PluginValue {
                    plugin_name: (*plugin_name).into(),
                    form_id: NumericLiteral::from(form_id).into(),
                }
                .into(),
            ),
            FnArg::Number(num) => match num {
                NumberLiteral::Float(value) => Self::StaticValue(value.into()),
                NumberLiteral::Decimal(value) => Self::StaticValue((value as f32).into()),
                NumberLiteral::Hex(value) => Self::StaticValue((value as f32).into()),
            },
        }
    }
}

impl From<&NumberLiteral> for StaticValue {
    fn from(value: &NumberLiteral) -> Self {
        match *value {
            NumberLiteral::Float(value) => Self { value },
            NumberLiteral::Decimal(value) => Self {
                value: value as f32,
            },
            NumberLiteral::Hex(value) => Self {
                value: value as f32,
            },
        }
    }
}

impl TryFrom<FnArg<'_>> for StaticValue {
    type Error = ParseError;

    fn try_from(value: FnArg) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&FnArg<'_>> for StaticValue {
    type Error = ParseError;

    fn try_from(value: &FnArg) -> Result<Self, Self::Error> {
        match value {
            FnArg::Number(num) => Ok(num.into()),
            other @ FnArg::PluginValue { .. } => Err(ParseError::UnexpectedValue(
                "StaticValue(e.g. 3.0)".to_owned(),
                format!("{other:?}",),
            )),
        }
    }
}

impl<'a> TryFrom<FnArg<'a>> for PluginValue<'a> {
    type Error = ParseError;

    fn try_from(value: FnArg<'a>) -> Result<Self, Self::Error> {
        match value {
            FnArg::PluginValue {
                plugin_name,
                form_id,
            } => Ok(Self {
                plugin_name: plugin_name.into(),
                form_id: NumericLiteral::from(form_id).into(),
            }),
            FnArg::Number(num) => Err(ParseError::UnexpectedValue(
                "plugin_name, form_id (in cast FnArg to PluginValue)".into(),
                num.to_string(),
            )),
        }
    }
}

impl<'a> From<FnArg<'a>> for Keyword<'a> {
    fn from(value: FnArg<'a>) -> Self {
        match value {
            FnArg::PluginValue {
                plugin_name,
                form_id,
            } => Self::Form(FormValue {
                form: PluginValue {
                    plugin_name: (*plugin_name).into(),
                    form_id: NumericLiteral::from(form_id).into(),
                },
            }),
            FnArg::Number(num) => Self::Literal(LiteralValue {
                editor_id: NumericLiteral::from(num).to_string().into(),
            }),
        }
    }
}

impl TryFrom<FnArg<'_>> for Direction {
    type Error = ParseError;

    fn try_from(value: FnArg<'_>) -> Result<Self, Self::Error> {
        match value {
            FnArg::Number(num) => Ok(match num {
                NumberLiteral::Hex(num) => (num as f64).try_into()?,
                NumberLiteral::Decimal(num) => (num as f64).try_into()?,
                NumberLiteral::Float(num) => (num as f64).try_into()?,
            }),
            other @ FnArg::PluginValue { .. } => Err(ParseError::UnexpectedValue(
                "1..=4(in Cast &FnArg to Direction)".into(),
                format!("{other:?}"),
            )),
        }
    }
}
