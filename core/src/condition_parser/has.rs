//! Parses has-prefix conditions based on the provided arguments and condition name.
use super::errors::{ParseError, Result};
use crate::conditions::{
    ConditionSet, HasKeyword, HasMagicEffect, HasMagicEffectWithKeyword, HasPerk, HasRefType,
    HasSpell,
};
use crate::dar_syntax::syntax::FnArg;

/// Parses has-prefix conditions based on the provided arguments and condition name.
///
/// # Errors
/// If parsing fails.
pub(super) fn parse_has<'a>(
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

    let arg = args.swap_remove(0);

    Ok(match condition_name {
        "HasKeyword" => ConditionSet::HasKeyword(HasKeyword {
            keyword: arg.into(),
            negated,
            ..Default::default()
        }),
        "HasPerk" => ConditionSet::HasPerk(HasPerk {
            perk: arg.try_into()?,
            negated,
            ..Default::default()
        }),
        "HasSpell" => ConditionSet::HasSpell(HasSpell {
            spell: arg.try_into()?,
            negated,
            ..Default::default()
        }),
        "HasMagicEffect" => ConditionSet::HasMagicEffect(HasMagicEffect {
            magic_effect: arg.try_into()?,
            negated,
            ..Default::default()
        }),
        "HasMagicEffectWithKeyword" => {
            ConditionSet::HasMagicEffectWithKeyword(HasMagicEffectWithKeyword {
                keyword: arg.into(),
                negated,
                ..Default::default()
            })
        }
        "HasRefType" => ConditionSet::HasRefType(HasRefType {
            location_ref_type: arg.into(),
            negated,
            ..Default::default()
        }),
        unknown_condition => {
            return Err(ParseError::UnexpectedValue(
                "HasKeyword|HasPerk|HasSpell|HasMagicEffect|HasMagicEffectWithKeyword|HasRefType"
                    .into(),
                unknown_condition.into(),
            ))
        }
    })
}
