use super::dar_interface::ParseError;
use crate::{
    converter::{
        conditions::{
            Condition, ConditionSet, HasKeyword, HasMagicEffect, HasMagicEffectWithKeyword,
            HasPerk, HasRefType, HasSpell,
        },
        dar_syntax::syntax::FnArg,
    },
    gen_cond, get_into, get_try_into,
};

pub(super) fn parse_has(
    condition_name: &str,
    args: Vec<FnArg<'_>>,
    negated: bool,
) -> Result<ConditionSet, ParseError> {
    let condition = Condition {
        condition: condition_name.to_string(),
        negated,
        ..Default::default()
    };

    Ok(match condition_name {
        "HasKeyword" => ConditionSet::HasKeyword(HasKeyword {
            condition,
            keyword: get_into!(args[0], "keyword in HasKeyword"),
        }),
        "HasPerk" => gen_cond!(HasPerk(perk, negated), args, "PluginValue in HasPerk"),
        "HasSpell" => gen_cond!(HasSpell(spell, negated), args, "PluginValue in HasSpell"),
        "HasMagicEffect" => ConditionSet::HasMagicEffect(HasMagicEffect {
            condition,
            magic_effect: get_try_into!(args[0], "PluginValue in HasMagicEffect")?,
            ..Default::default()
        }),
        "HasMagicEffectWithKeyword" => {
            ConditionSet::HasMagicEffectWithKeyword(HasMagicEffectWithKeyword {
                condition,
                keyword: get_into!(args[0], "PluginValue in HasMagicEffectWithKeyword"),
                ..Default::default()
            })
        }
        "HasRefType" => gen_cond!(
            HasRefType(location_ref_type, negated),
            args,
            "PluginValue in HasRefType"
        ),
        _ => unreachable!(),
    })
}
