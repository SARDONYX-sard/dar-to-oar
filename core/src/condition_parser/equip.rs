//! Parses equipment-related conditions based on the provided arguments and condition name.
use super::errors::{ParseError, Result};
use crate::{
    conditions::{ConditionSet, IsEquipped, IsEquippedHasKeyword, IsEquippedShout, IsEquippedType},
    dar_syntax::FnArgs,
    values::{NumericLiteral, TypeValue},
};

/// Parses equipment-related conditions based on the provided arguments and condition name.
///
/// # Errors
/// If parsing fails.
pub(super) fn parse_equip<'a>(
    condition_name: &'a str,
    mut args: FnArgs<'a>,
    negated: bool,
) -> Result<ConditionSet<'a>> {
    Ok(match condition_name {
        "IsEquippedRight" | "IsEquippedLeft" => ConditionSet::IsEquipped(IsEquipped {
            negated,
            form: args.pop_front()?.try_into()?,
            left_hand: condition_name == "IsEquippedLeft",
            ..Default::default()
        }),
        "IsEquippedRightType" | "IsEquippedLeftType" => {
            let numeric_value: NumericLiteral = args.pop_front()?.try_into()?;
            let type_value = TypeValue {
                value: numeric_value.try_into()?,
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
                keyword: args.pop_front()?.into(),
                ..Default::default()
            })
        }
        "IsEquippedShout" => ConditionSet::IsEquippedShout(IsEquippedShout {
            shout: args.pop_front()?.try_into()?,
            negated,
            ..Default::default()
        }),
        _ => {
            return Err(ParseError::UnexpectedValue {
                expected: "`IsEquipped` prefix condition: ".into(),
                actual: condition_name.into(),
            })
        }
    })
}
