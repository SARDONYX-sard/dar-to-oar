//! Parses a comparison-based condition for plugin values.
use super::errors::{ParseError, Result};
use crate::{
    conditions::{CompareValues, ConditionSet},
    dar_syntax::syntax::FnArg,
    values::{Cmp, NumericValue, PluginValue},
};

/// Parses a comparison-based condition for plugin values.
/// `ValueEqualTo` | `ValueLessThan`
///
/// # Errors
/// Parsing failed.
pub(super) fn parse_compare<'a>(
    condition_name: &'a str,
    mut args: Vec<FnArg<'a>>,
    negated: bool,
) -> Result<ConditionSet<'a>> {
    let args_len = args.len();
    if args_len < 2 {
        return Err(ParseError::UnexpectedValue(
            "[Plugin value, float: in ValueEqualTo | ValueLessThan] At least 2 argument is required, but got {arg_len}".into(),
            "".into(),
        ));
    }

    let static_value = args
        .pop()
        .ok_or(ParseError::NotEnoughArguments {
            expected: 2,
            actual: args_len,
        })?
        .try_into()?;
    let plugin_value: PluginValue = args
        .pop()
        .ok_or(ParseError::NotEnoughArguments {
            expected: 2,
            actual: args_len,
        })?
        .try_into()?;

    let create_compare = |comparison: Cmp| {
        ConditionSet::CompareValues(CompareValues {
            negated,
            value_a: NumericValue::GlobalVariable(plugin_value.into()),
            comparison,
            value_b: NumericValue::StaticValue(static_value),
            ..Default::default()
        })
    };

    Ok(match condition_name {
        "ValueEqualTo" => create_compare(Cmp::Eq),
        "ValueLessThan" => create_compare(Cmp::Lt),
        _ => {
            return Err(ParseError::UnexpectedValue(
                "ValueEqualTo or ValueLessThan".into(),
                condition_name.into(),
            ))
        }
    })
}
