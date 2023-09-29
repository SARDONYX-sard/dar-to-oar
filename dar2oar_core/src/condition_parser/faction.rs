use super::dar_interface::ParseError;
use crate::{
    conditions::{ConditionSet, FactionRank, IsInFaction},
    dar_syntax::syntax::FnArg,
    get_into, get_try_into,
    values::Cmp,
};

pub(super) fn parse_faction(
    condition_name: &str,
    args: Vec<FnArg<'_>>,
    negated: bool,
) -> Result<ConditionSet, ParseError> {
    let create_cond = |comparison: Cmp| -> Result<ConditionSet, ParseError> {
        Ok(ConditionSet::FactionRank(FactionRank {
            negated,
            faction: get_try_into!(args[0], "PluginValue")?,
            comparison,
            numeric_value: get_into!(args[1], "NumericValue"),
            ..Default::default()
        }))
    };

    Ok(match condition_name {
        "IsInFaction" => ConditionSet::IsInFaction(IsInFaction {
            negated,
            faction: get_try_into!(args[0], "PluginValue")?,
            ..Default::default()
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
    use crate::dar_syntax::syntax::NumberLiteral;
    use crate::values::{NumericValue, PluginValue};
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
