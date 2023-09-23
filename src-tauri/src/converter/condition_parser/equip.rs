use super::dar_interface::ParseError;
use crate::{
    converter::{
        conditions::{Condition, ConditionSet, IsEquipped, IsEquippedHasKeyword, IsEquippedType},
        dar_syntax::syntax::FnArg,
        values::{NumericLiteral, TypeValue},
    },
    get_into, get_try_into,
};

pub(super) fn parse_equip(
    condition_name: &str,
    args: Vec<FnArg<'_>>,
    negated: bool,
) -> Result<ConditionSet, ParseError> {
    dbg!(condition_name);
    dbg!(&args);
    Ok(match condition_name {
        "IsEquippedRight" | "IsEquippedLeft" => ConditionSet::IsEquipped(IsEquipped {
            condition: Condition {
                condition: "IsEquipped".into(),
                // NOTE: not exist negated in IsEquipped
                // Therefore, always set it to false and do not reflect it in json.
                ..Default::default()
            },
            form: get_try_into!(args[0], "PluginValue")?,
            left_hand: condition_name == "IsEquippedLeft",
            ..Default::default()
        }),
        "IsEquippedRightType" | "IsEquippedLeftType" => {
            let numeric_value: NumericLiteral = get_try_into!(args[0], "WeaponType -1..18")?;
            let type_value = TypeValue {
                value: numeric_value.try_into().map_err(|_: &str| {
                    ParseError::UnexpectedValue("-1..18".into(), "Unknown value".into())
                })?,
            };
            ConditionSet::IsEquippedType(IsEquippedType {
                condition: Condition {
                    negated,
                    condition: "IsEquippedType".into(),
                    ..Default::default()
                },
                left_hand: condition_name == "IsEquippedLeftType",
                type_value,
                ..Default::default()
            })
        }
        "IsEquippedRightHasKeyword" | "IsEquippedLeftHasKeyword" => {
            ConditionSet::IsEquippedHasKeyword(IsEquippedHasKeyword {
                condition: Condition {
                    negated,
                    condition: "IsEquippedHasKeyword".into(),
                    ..Default::default()
                },
                left_hand: condition_name == "IsEquippedLeftHasKeyword",
                keyword: get_into!(args[0], "Keyword"),
            })
        }
        _ => unreachable!(),
    })
}
