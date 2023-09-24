use super::dar_interface::ParseError;
use crate::converter::conditions::{
    ConditionSet, HasKeyword, HasMagicEffect, HasMagicEffectWithKeyword, HasPerk, HasRefType,
    HasSpell,
};
use crate::{converter::dar_syntax::syntax::FnArg, gen_cond, get_into, get_try_into};

pub(super) fn parse_has(
    condition_name: &str,
    args: Vec<FnArg<'_>>,
    negated: bool,
) -> Result<ConditionSet, ParseError> {
    Ok(match condition_name {
        "HasKeyword" => ConditionSet::HasKeyword(HasKeyword {
            keyword: get_into!(args[0], "keyword in HasKeyword"),
            ..Default::default()
        }),
        "HasPerk" => gen_cond!(HasPerk(perk, negated), args, "PluginValue in HasPerk"),
        "HasSpell" => gen_cond!(HasSpell(spell, negated), args, "PluginValue in HasSpell"),
        "HasMagicEffect" => ConditionSet::HasMagicEffect(HasMagicEffect {
            magic_effect: get_try_into!(args[0], "PluginValue in HasMagicEffect")?,
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
        _ => unreachable!(),
    })
}
