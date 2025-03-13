//! Parses a comparison-based condition for plugin values.
use super::errors::{ParseError, Result};
use crate::{
    conditions::{CompareValues, ConditionSet},
    dar_syntax::FnArgs,
    values::Cmp,
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
    let value_a = args.pop_front()?;
    let value_b = args.pop_front()?;

    let create_compare = |comparison: Cmp| {
        ConditionSet::CompareValues(CompareValues {
            negated,
            value_a: value_a.into(),
            comparison,
            value_b: value_b.into(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        dar_syntax::{ast::fn_args::fn_args, FnArg, NumberLiteral},
        values::{Cmp, NumericValue, PluginValue, StaticValue},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_value_less_than() {
        // test inputs
        let condition_name = "ValueLessThan";
        let args = fn_args![
            FnArg::Number(NumberLiteral::Float(3.5)), // compare value
            FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Hex(0x0000_0007),
            }
        ];
        let is_negated = false;

        let result = parse_compare(condition_name, args, is_negated);

        let expected = ConditionSet::CompareValues(CompareValues {
            negated: false,
            value_a: NumericValue::StaticValue(StaticValue { value: 3.5 }),
            comparison: Cmp::Le,
            value_b: NumericValue::GlobalVariable(
                PluginValue {
                    plugin_name: "Skyrim.esm".into(),
                    form_id: "7".into(),
                }
                .into(),
            ),
            ..Default::default()
        });

        match result {
            Ok(result) => {
                assert_eq!(result, expected);
            }
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn test_parse_less_than_inverse() {
        let condition_name = "ValueEqualTo";
        let args = fn_args![
            FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Hex(0x0000_0007),
            },
            FnArg::Number(NumberLiteral::Float(3.5)) // compare value
        ];

        let is_negated = true;

        let result = parse_compare(condition_name, args, is_negated);

        let expected = Ok(ConditionSet::CompareValues(CompareValues {
            negated: true,
            value_a: NumericValue::GlobalVariable(
                PluginValue {
                    plugin_name: "Skyrim.esm".into(),
                    form_id: "7".into(),
                }
                .into(),
            ),
            comparison: Cmp::Eq,
            value_b: NumericValue::StaticValue(StaticValue { value: 3.5 }),
            ..Default::default()
        }));

        assert_eq!(result, expected);
    }
}
