use super::dar_interface::ParseError;
use crate::{
    converter::{
        conditions::{Condition, ConditionSet, FactionRank, IsInFaction},
        dar_syntax::syntax::FnArg,
        values::Cmp,
    },
    get_into, get_try_into,
};

pub(super) fn parse_faction(
    condition_name: &str,
    args: Vec<FnArg<'_>>,
    is_negated: bool,
) -> Result<ConditionSet, ParseError> {
    let create_cond = |comparison: Cmp| -> Result<ConditionSet, ParseError> {
        Ok(ConditionSet::FactionRank(FactionRank {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            faction: get_try_into!(args[0], "PluginValue")?,
            comparison,
            numeric_value: get_into!(args[1], "NumericValue"),
        }))
    };

    Ok(match condition_name {
        "IsInFaction" => ConditionSet::IsInFaction(IsInFaction {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            faction: get_try_into!(args[0], "PluginValue")?,
        }),
        "IsFactionRankEqualTo" => create_cond(Cmp::Eq)?,
        "IsFactionRankLessThan" => create_cond(Cmp::Lt)?,
        _ => {
            return Err(ParseError::UnexpectedValue(
                "IsInFaction, IsFactionRankEqualTo or IsFactionRankLessThan".to_string(),
                condition_name.to_string(),
            ))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converter::dar_syntax::syntax::NumberLiteral;
    use crate::converter::values::{NumericValue, PluginValue};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_is_in_faction() {
        let condition_name = "IsInFaction";
        let args = vec![
            FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Decimal(7),
            },
            FnArg::Number(NumberLiteral::Decimal(3)),
        ];
        let is_negated = false;

        let result = parse_faction(condition_name, args, is_negated);

        let expected = Ok(ConditionSet::IsInFaction(IsInFaction {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            faction: PluginValue {
                plugin_name: "Skyrim.esm".into(),
                form_id: "7".into(),
            },
        }));

        assert_eq!(result, expected);
    }

    #[test]
    fn should_parse_is_faction_rank_equal_to() {
        let condition_name = "IsFactionRankEqualTo";
        let args = vec![
            FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Decimal(7),
            },
            FnArg::Number(NumberLiteral::Float(0.3)),
        ];
        let is_negated = true;

        let result = parse_faction(condition_name, args, is_negated);

        let expected = Ok(ConditionSet::FactionRank(FactionRank {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            faction: PluginValue {
                plugin_name: "Skyrim.esm".into(),
                form_id: "7".into(),
            },
            comparison: Cmp::Eq,
            numeric_value: NumericValue::StaticValue(0.3.into()),
        }));

        assert_eq!(result, expected);
    }
}
