use super::dar_interface::ParseError;
use crate::{
    conditions::{ConditionSet, IsEquipped, IsEquippedHasKeyword, IsEquippedType},
    dar_syntax::syntax::FnArg,
    get_into, get_try_into,
    values::{NumericLiteral, TypeValue},
};

pub(super) fn parse_equip(
    condition_name: &str,
    args: Vec<FnArg<'_>>,
    negated: bool,
) -> Result<ConditionSet, ParseError> {
    Ok(match condition_name {
        "IsEquippedRight" | "IsEquippedLeft" => ConditionSet::IsEquipped(IsEquipped {
            negated,
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
                negated,
                left_hand: condition_name == "IsEquippedLeftType",
                type_value,
                ..Default::default()
            })
        }
        "IsEquippedRightHasKeyword" | "IsEquippedLeftHasKeyword" => {
            ConditionSet::IsEquippedHasKeyword(IsEquippedHasKeyword {
                negated,
                left_hand: condition_name == "IsEquippedLeftHasKeyword",
                keyword: get_into!(args[0], "Keyword"),
                ..Default::default()
            })
        }
        _ => unreachable!(),
    })
}
