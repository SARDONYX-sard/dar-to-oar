//! Parses a comparison-based condition for plugin values.
use super::errors::{ParseError, Result};
use crate::{
    conditions::{CompareValues, ConditionSet},
    dar_syntax::FnArgs,
    values::{Cmp, NumericValue, PluginValue},
};

/// Parses a comparison-based condition for plugin values.
/// `ValueEqualTo` | `ValueLessThan`
///
/// # Errors
/// Parsing failed.
pub(super) fn parse_compare<'a>(
    condition_name: &'a str,
    mut args: FnArgs<'a>,
    negated: bool,
) -> Result<ConditionSet<'a>> {
    let plugin_value: PluginValue = args.pop_front()?.try_into()?;
    let static_value = args.pop_front()?.try_into()?;

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
            return Err(ParseError::UnexpectedValue {
                expected: "ValueEqualTo or ValueLessThan".into(),
                actual: condition_name.into(),
            })
        }
    })
}
