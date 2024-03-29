//! Parses equipment-related conditions based on the provided arguments and condition name.
use super::macros::{gen_cond, get_try_into};
use super::{dar_interface::ParseError, macros::GetArg as _};
use crate::{
    conditions::{ConditionSet, IsEquipped, IsEquippedHasKeyword, IsEquippedShout, IsEquippedType},
    dar_syntax::syntax::FnArg,
    values::{NumericLiteral, TypeValue},
};

/// Parses equipment-related conditions based on the provided arguments and condition name.
///
/// # Errors
/// If parsing fails.
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
                value: numeric_value.try_into().map_err(|_err| {
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
                keyword: args.try_get(0, "Keyword")?.into(),
                ..Default::default()
            })
        }
        "IsEquippedShout" => gen_cond!(
            IsEquippedShout(shout, negated),
            args,
            "shout(PluginValue) in IsEquippedShout"
        ),
        _ => {
            return Err(ParseError::UnexpectedValue(
                "IsEquipped prefix condition unexpected to come in: ".into(),
                condition_name.into(),
            ))
        }
    })
}
