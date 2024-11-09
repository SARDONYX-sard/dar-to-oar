//! Parses faction-related conditions based on the provided arguments and condition name.
use super::errors::{ParseError, Result};
use crate::{
    conditions::{ConditionSet, FactionRank, IsInFaction},
    dar_syntax::FnArgs,
    values::Cmp,
};

/// Parses faction-related conditions based on the provided arguments and condition name.
///
/// # Errors
/// If parsing fails.
pub(super) fn parse_faction<'a>(
    condition_name: &'a str,
    mut args: FnArgs<'a>,
    negated: bool,
) -> Result<ConditionSet<'a>> {
    let mut create_cond = |comparison: Cmp| -> Result<ConditionSet, ParseError> {
        let faction = args.pop_front()?.try_into()?;
        let numeric_value = args.pop_front()?.into();

        Ok(ConditionSet::FactionRank(FactionRank {
            negated,
            faction,
            comparison,
            numeric_value,
            ..Default::default()
        }))
    };

    Ok(match condition_name {
        "IsInFaction" => ConditionSet::IsInFaction(IsInFaction {
            negated,
            faction: args.pop_front()?.try_into()?,
            ..Default::default()
        }),
        "IsFactionRankEqualTo" => create_cond(Cmp::Eq)?,
        "IsFactionRankLessThan" => create_cond(Cmp::Lt)?,
        _ => {
            return Err(ParseError::UnexpectedValue {
                expected: "IsInFaction, IsFactionRankEqualTo or IsFactionRankLessThan".to_string(),
                actual: condition_name.to_string(),
            })
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dar_syntax::ast::fn_args::fn_args;
    use crate::dar_syntax::{FnArg, NumberLiteral};
    use crate::values::{NumericValue, PluginValue};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_is_in_faction() {
        let condition_name = "IsInFaction";
        let args = fn_args![
            FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Decimal(7),
            },
            FnArg::Number(NumberLiteral::Decimal(3)),
        ];
        let is_negated = false;

        let result = parse_faction(condition_name, args, is_negated);

        let expected = Ok(ConditionSet::IsInFaction(IsInFaction {
            negated: false,
            faction: PluginValue {
                plugin_name: "Skyrim.esm".into(),
                form_id: "7".into(),
            },
            ..Default::default()
        }));

        assert_eq!(result, expected);
    }

    #[test]
    fn should_parse_is_faction_rank_equal_to() {
        let condition_name = "IsFactionRankEqualTo";
        let args = fn_args![
            FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Decimal(7),
            },
            FnArg::Number(NumberLiteral::Float(0.3)),
        ];
        let is_negated = true;

        let result = parse_faction(condition_name, args, is_negated);

        let expected = Ok(ConditionSet::FactionRank(FactionRank {
            negated: true,
            faction: PluginValue {
                plugin_name: "Skyrim.esm".into(),
                form_id: "7".into(),
            },
            comparison: Cmp::Eq,
            numeric_value: NumericValue::StaticValue(0.3.into()),
            ..Default::default()
        }));

        assert_eq!(result, expected);
    }
}
