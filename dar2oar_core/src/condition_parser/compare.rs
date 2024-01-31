//! Parses a comparison-based condition for plugin values.
use super::dar_interface::ParseError;
use super::macros::get_try_into;
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
pub(super) fn parse_compare(
    condition_name: &str,
    args: Vec<FnArg<'_>>,
    negated: bool,
) -> Result<ConditionSet, ParseError> {
    let plugin_value: PluginValue = get_try_into!(
        args[0],
        "Plugin value: in ValueEqualTo | ValueLessThan 1st arg",
        "None"
    )?;
    let static_value = get_try_into!(
        args[1],
        " float(e.g. 1.0): in ValueEqualTo | ValueLessThan 2nd arg",
        "None"
    )?;

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
                "ValueEqualTo or ValueLessThan".to_string(),
                condition_name.to_string(),
            ))
        }
    })
}
