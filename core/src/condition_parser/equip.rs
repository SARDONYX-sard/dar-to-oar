//! Parses equipment-related conditions based on the provided arguments and condition name.
use super::errors::{ParseError, Result};
use crate::{
    conditions::{ConditionSet, IsEquipped, IsEquippedHasKeyword, IsEquippedShout, IsEquippedType},
    dar_syntax::syntax::FnArg,
    values::{NumericLiteral, TypeValue},
};

/// Parses equipment-related conditions based on the provided arguments and condition name.
///
/// # Errors
/// If parsing fails.
pub(super) fn parse_equip<'a>(
    condition_name: &'a str,
    mut args: Vec<FnArg<'a>>,
    negated: bool,
) -> Result<ConditionSet<'a>> {
    if args.is_empty() {
        return Err(ParseError::UnexpectedValue(
            "At least 1 argument is required, but got 0".into(),
            "".into(),
        ));
    }

    Ok(match condition_name {
        "IsEquippedRight" | "IsEquippedLeft" => ConditionSet::IsEquipped(IsEquipped {
            negated,
            form: args.swap_remove(0).try_into()?,
            left_hand: condition_name == "IsEquippedLeft",
            ..Default::default()
        }),
        "IsEquippedRightType" | "IsEquippedLeftType" => {
            let numeric_value: NumericLiteral = args.swap_remove(0).try_into()?;
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
                keyword: args.swap_remove(0).into(),
                ..Default::default()
            })
        }
        "IsEquippedShout" => ConditionSet::IsEquippedShout(IsEquippedShout {
            shout: args.swap_remove(0).try_into()?,
            negated,
            ..Default::default()
        }),
        _ => {
            return Err(ParseError::UnexpectedValue(
                "`IsEquipped` prefix condition: ".into(),
                condition_name.into(),
            ))
        }
    })
}
