//! Parses has-prefix conditions based on the provided arguments and condition name.
use super::dar_interface::ParseError;
use super::macros::{gen_cond, get_try_into, GetArg as _};
use crate::conditions::{
    ConditionSet, HasKeyword, HasMagicEffect, HasMagicEffectWithKeyword, HasPerk, HasRefType,
    HasSpell,
};
use crate::dar_syntax::syntax::FnArg;

/// Parses has-prefix conditions based on the provided arguments and condition name.
///
/// # Errors
/// If parsing fails.
pub(super) fn parse_has(
    condition_name: &str,
    args: Vec<FnArg<'_>>,
    negated: bool,
) -> Result<ConditionSet, ParseError> {
    Ok(match condition_name {
        "HasKeyword" => ConditionSet::HasKeyword(HasKeyword {
            keyword: args.try_get(0, "keyword in HasKeyword")?.into(),
            negated,
            ..Default::default()
        }),
        "HasPerk" => gen_cond!(HasPerk(perk, negated), args, "PluginValue in HasPerk"),
        "HasSpell" => gen_cond!(HasSpell(spell, negated), args, "PluginValue in HasSpell"),
        "HasMagicEffect" => ConditionSet::HasMagicEffect(HasMagicEffect {
            magic_effect: get_try_into!(args[0], "PluginValue in HasMagicEffect")?,
            negated,
            ..Default::default()
        }),
        "HasMagicEffectWithKeyword" => gen_cond!(
            HasMagicEffectWithKeyword(keyword, negated),
            args,
            "PluginValue in HasMagicEffectWithKeyword",
            into
        ),
        "HasRefType" => gen_cond!(
            HasRefType(location_ref_type, negated),
            args,
            "PluginValue in HasRefType",
            into
        ),
        unknown_condition => {
            return Err(ParseError::UnexpectedValue(
                "HasKeyword|HasPerk|HasSpell|HasMagicEffect|HasMagicEffectWithKeyword|HasRefType"
                    .into(),
                unknown_condition.into(),
            ))
        }
    })
}
