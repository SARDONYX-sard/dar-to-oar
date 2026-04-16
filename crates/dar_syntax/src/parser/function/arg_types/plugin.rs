//! DAR Plugin value type

use std::borrow::Cow;

use crate::ast::{ActorArgs, GlobalVariable};

use super::{
    number::{form_id, static_value},
    string::string,
};

use oar_values::{ActorValue, ActorValueType, PluginValue, WeaponType};
use winnow::{
    Parser,
    ascii::float,
    combinator::{alt, seq},
    error::{
        ModalResult,
        StrContext::{self, Expected, Label},
        StrContextValue::{self, Description},
    },
};
use winnow_ext::delimited_multispace0;

/// Parse plugin value(e.g. `"Skyrim.esm" | 0x007`)
pub(crate) fn plugin_value<'i>(input: &mut &'i str) -> ModalResult<PluginValue<'i>> {
    seq! {
        PluginValue {
            plugin_name: delimited_multispace0(string).map(Cow::Borrowed),
            _: "|",
            form_id: delimited_multispace0(form_id)
        }
    }
    .context(Label("Plugin"))
    .context(Expected(Description(
        r#"Plugin: e.g. `"Skyrim.esm" | 0x007`"#,
    )))
    .parse_next(input)
}

fn global_variable<'i>(input: &mut &'i str) -> ModalResult<GlobalVariable<'i>> {
    alt((
        plugin_value.map(GlobalVariable::Plugin),
        static_value.map(GlobalVariable::StaticValue),
    ))
    .parse_next(input)
}

pub(crate) fn global_pair<'i>(
    input: &mut &'i str,
) -> ModalResult<(GlobalVariable<'i>, GlobalVariable<'i>)> {
    seq! {
        global_variable,
        _: delimited_multispace0(","),
        global_variable
    }
    .context(StrContext::Label("GlobalVariables"))
    .context(StrContext::Expected(StrContextValue::Description(
        r#"(GlobalVariable, GlobalVariable): e.g. `("Skyrim.esm" | 0x007, 10)`, `(30.0, 10)`"#,
    )))
    .parse_next(input)
}

/// (global_variable, plugin) / (plugin, global_variable)
pub(crate) fn actor_args<'i, Error>(
    actor_value_type: ActorValueType,
) -> impl Parser<&'i str, ActorArgs<'i>, Error>
where
    Error: winnow::error::ParserError<&'i str>
        + winnow::error::AddContext<&'i str, winnow::error::StrContext>
        + std::convert::From<winnow::error::ErrMode<winnow::error::ContextError>>,
{
    move |input: &mut &'i str| {
        alt((
            seq! {
                ActorArgs::ActorFirst {
                    value_a: static_value.map(|v| ActorValue {
                        actor_value: v.value as i64,
                        actor_value_type,
                    }),
                    _: delimited_multispace0(","),
                    value_b: global_variable,
                }
            },
            seq! {
                ActorArgs::GlobalFirst {
                    value_a: global_variable,
                    _: delimited_multispace0(","),
                    value_b: static_value.map(|v| ActorValue {
                        actor_value: v.value as i64,
                        actor_value_type,
                    }),
                }
            },
        ))
        .context(StrContext::Label("Actor arguments"))
        .context(StrContext::Expected(StrContextValue::Description(
            r#"(ActorValue, PluginValue/Number)/(PluginValue/Number, ActorValue): e.g. `(30, 0.5)`, `(0.1, 10)`, "#,
        )))
        .parse_next(input)
    }
}

pub(crate) struct FactionArgs<'i> {
    pub(crate) faction: PluginValue<'i>,
    pub(crate) rank: GlobalVariable<'i>,
}

/// (global_variable, plugin)
pub(crate) fn faction_args<'i>(input: &mut &'i str) -> ModalResult<FactionArgs<'i>> {
    alt((
        seq! {
            FactionArgs {
                faction: plugin_value,
                _: delimited_multispace0(","),
                rank: global_variable,
            }
        },
        seq! {
            FactionArgs {
                rank: global_variable,
                _: delimited_multispace0(","),
                faction: plugin_value,
            }
        },
    ))
    .context(StrContext::Label("Faction arguments"))
    .context(StrContext::Expected(StrContextValue::Description(
        r#"(PluginValue, Number)/(Number, PluginValue): e.g. `("Skyrim.esm" | 0x007, 10)`, `(30.0, "Skyrim.esm" | 0x007)`"#,
    )))
    .parse_next(input)
}

pub(crate) fn weapon_type(input: &mut &str) -> ModalResult<WeaponType> {
    float
        .verify_map(|value: f64| {
            Some(match value {
                -1.0..0.0 => WeaponType::Other,
                x if (0.0..1.0).contains(&x) => WeaponType::Unarmed,
                x if (1.0..2.0).contains(&x) => WeaponType::Sword,
                x if (2.0..3.0).contains(&x) => WeaponType::Dagger,
                x if (3.0..4.0).contains(&x) => WeaponType::WarAxe,
                x if (4.0..5.0).contains(&x) => WeaponType::Mace,
                x if (5.0..6.0).contains(&x) => WeaponType::Greatsword,
                x if (6.0..7.0).contains(&x) => WeaponType::Battleaxe,
                x if (7.0..8.0).contains(&x) => WeaponType::Bow,
                x if (8.0..9.0).contains(&x) => WeaponType::Staff,
                x if (9.0..10.0).contains(&x) => WeaponType::Crossbow,
                x if (10.0..11.0).contains(&x) => WeaponType::Warhammer,
                x if (11.0..12.0).contains(&x) => WeaponType::Shield,
                x if (12.0..13.0).contains(&x) => WeaponType::AlterationSpell,
                x if (13.0..14.0).contains(&x) => WeaponType::IllusionSpell,
                x if (14.0..15.0).contains(&x) => WeaponType::DestructionSpell,
                x if (15.0..16.0).contains(&x) => WeaponType::ConjurationSpell,
                x if (16.0..17.0).contains(&x) => WeaponType::RestorationSpell,
                x if (17.0..18.0).contains(&x) => WeaponType::Scroll,
                x if (18.0..19.0).contains(&x) => WeaponType::Torch,
                _ => return None,
            })
        })
        .context(Label("WeaponType"))
        .context(Expected(Description("-1.0..=18.0")))
        .parse_next(input)
}
