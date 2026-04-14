//! DAR function
pub mod arg_types;
pub mod ident;

use self::ident::ident;
use crate::{
    ast::{Function, HandType},
    parser::function::{
        arg_types::{
            number::{direction, static_value, weapon_type},
            plugin::parse_plugin_value,
        },
        ident::fn_kind::FnKind,
    },
};
use oar_values::{PluginValue, StaticValue};
use winnow::{
    Parser,
    combinator::{alt, delimited, opt, separated_pair, seq},
    error::{ModalResult, StrContext, StrContextValue},
};
use winnow_ext::delimited_multispace0;

/// Parse a DAR function call into a typed `Function`.
///
/// This parser is strict:
/// - Function name must be a known `FnKind`
/// - Arguments must match the expected `ArgPattern`
///
/// # Expected Syntax Examples
/// ```txt
/// ; Pattern1
/// IsActorBase("Skyrim.esm" | 0x00000007)
///
/// ; Pattern2
/// IsActorValueEqualTo(0x00000007, 30)
/// ```
pub fn function<'i>(input: &mut &'i str) -> ModalResult<Function<'i>> {
    let kind = ident
        .verify_map(|name| FnKind::parse(name).ok())
        .context_with(|| {
            [StrContext::Expected(StrContextValue::Description(
                FnKind::expected_str(),
            ))]
            .into_iter()
        })
        .parse_next(input)?;

    parse_by_kind(kind, input)
}

/// Dispatch parser based on function kind and its argument pattern.
///
/// Each function is parsed strictly according to its predefined pattern.
/// This guarantees:
/// - No invalid argument combinations
/// - No runtime ambiguity
fn parse_by_kind<'i>(kind: FnKind, input: &mut &'i str) -> ModalResult<Function<'i>> {
    match kind {
        // ---------------- no arg ----------------
        FnKind::IsFemale => {
            opt(parse_paren(())).parse_next(input)?;
            Ok(Function::IsFemale)
        }
        FnKind::IsChild => {
            opt(parse_paren(())).parse_next(input)?;
            Ok(Function::IsChild)
        }
        FnKind::IsPlayerTeammate => {
            opt(parse_paren(())).parse_next(input)?;
            Ok(Function::IsPlayerTeammate)
        }
        FnKind::IsInInterior => {
            opt(parse_paren(())).parse_next(input)?;
            Ok(Function::IsInInterior)
        }
        FnKind::IsUnique => {
            opt(parse_paren(())).parse_next(input)?;
            Ok(Function::IsUnique)
        }
        FnKind::IsAttacking => {
            opt(parse_paren(())).parse_next(input)?;
            Ok(Function::IsAttacking)
        }
        FnKind::IsRunning => {
            opt(parse_paren(())).parse_next(input)?;
            Ok(Function::IsRunning)
        }
        FnKind::IsSneaking => {
            opt(parse_paren(())).parse_next(input)?;
            Ok(Function::IsSneaking)
        }
        FnKind::IsSprinting => {
            opt(parse_paren(())).parse_next(input)?;
            Ok(Function::IsSprinting)
        }
        FnKind::IsInAir => {
            opt(parse_paren(())).parse_next(input)?;
            Ok(Function::IsInAir)
        }
        FnKind::IsInCombat => {
            opt(parse_paren(())).parse_next(input)?;
            Ok(Function::IsInCombat)
        }
        FnKind::IsWeaponDrawn => {
            opt(parse_paren(())).parse_next(input)?;
            Ok(Function::IsWeaponDrawn)
        }

        // ---------------- number1 ----------------
        FnKind::CurrentGameTimeLessThan => parse_paren(static_value)
            .map(|v| Function::CurrentGameTimeLessThan { value: v })
            .parse_next(input),

        FnKind::IsLevelLessThan => parse_paren(static_value)
            .map(|v| Function::IsLevelLessThan { level: v })
            .parse_next(input),

        FnKind::Random => parse_paren(static_value)
            .map(|v| Function::Random { value: v })
            .parse_next(input),

        FnKind::IsMovementDirection => parse_paren(direction)
            .map(|direction| Function::IsMovementDirection { direction })
            .parse_next(input),

        // ---------------- plugin1 ----------------
        FnKind::IsActorBase => parse_paren(parse_plugin_value)
            .map(|v| Function::IsActorBase { actor_base: v })
            .parse_next(input),

        FnKind::IsClass => parse_paren(parse_plugin_value)
            .map(|v| Function::IsClass { class: v })
            .parse_next(input),

        FnKind::IsCombatStyle => parse_paren(parse_plugin_value)
            .map(|v| Function::IsCombatStyle { combat_style: v })
            .parse_next(input),

        FnKind::CurrentWeather => parse_paren(parse_plugin_value)
            .map(|v| Function::CurrentWeather { weather: v })
            .parse_next(input),

        FnKind::IsRace => parse_paren(parse_plugin_value)
            .map(|v| Function::IsRace { race: v })
            .parse_next(input),

        FnKind::IsVoiceType => parse_paren(parse_plugin_value)
            .map(|v| Function::IsVoiceType { voice_type: v })
            .parse_next(input),

        FnKind::IsWorldSpace => parse_paren(parse_plugin_value)
            .map(|v| Function::IsWorldSpace { world_space: v })
            .parse_next(input),

        FnKind::IsParentCell => parse_paren(parse_plugin_value)
            .map(|v| Function::IsParentCell { cell: v })
            .parse_next(input),

        FnKind::IsWorn => parse_paren(parse_plugin_value)
            .map(|v| Function::IsWorn { form: v })
            .parse_next(input),

        FnKind::IsEquippedShout => parse_paren(parse_plugin_value)
            .map(|v| Function::IsEquippedShout { shout: v })
            .parse_next(input),

        FnKind::IsInLocation => parse_paren(parse_plugin_value)
            .map(|v| Function::IsInLocation { location: v })
            .parse_next(input),

        FnKind::HasPerk => parse_paren(parse_plugin_value)
            .map(|v| Function::HasPerk { perk: v })
            .parse_next(input),

        FnKind::HasSpell => parse_paren(parse_plugin_value)
            .map(|v| Function::HasSpell { spell: v })
            .parse_next(input),

        FnKind::HasMagicEffect => parse_paren(parse_plugin_value)
            .map(|v| Function::HasMagicEffect { magic_effect: v })
            .parse_next(input),

        // ---------------- keyword1 ----------------
        FnKind::HasKeyword => parse_paren(parse_plugin_value)
            .map(|v| Function::HasKeyword { keyword: v })
            .parse_next(input),

        FnKind::HasRefType => parse_paren(parse_plugin_value)
            .map(|v| Function::HasRefType {
                location_ref_type: v,
            })
            .parse_next(input),

        FnKind::IsWornHasKeyword => parse_paren(parse_plugin_value)
            .map(|v| Function::IsWornHasKeyword { keyword: v })
            .parse_next(input),

        FnKind::HasMagicEffectWithKeyword => parse_paren(parse_plugin_value)
            .map(|v| Function::HasMagicEffectWithKeyword { keyword: v })
            .parse_next(input),

        // ---------------- number2 ----------------
        FnKind::IsActorValueEqualTo => parse_paren(number_pair)
            .map(|(a, b)| Function::IsActorValueEqualTo {
                actor_value: a,
                value: b,
            })
            .parse_next(input),

        FnKind::IsActorValueLessThan => parse_paren(number_pair)
            .map(|(a, b)| Function::IsActorValueLessThan {
                actor_value: a,
                value: b,
            })
            .parse_next(input),

        FnKind::IsActorValueBaseLessThan => parse_paren(number_pair)
            .map(|(a, b)| Function::IsActorValueBaseLessThan {
                actor_value: a,
                value: b,
            })
            .parse_next(input),

        FnKind::IsActorValueMaxEqualTo => parse_paren(number_pair)
            .map(|(a, b)| Function::IsActorValueMaxEqualTo {
                actor_value: a,
                value: b,
            })
            .parse_next(input),

        FnKind::IsActorValueMaxLessThan => parse_paren(number_pair)
            .map(|(a, b)| Function::IsActorValueMaxLessThan {
                actor_value: a,
                value: b,
            })
            .parse_next(input),

        FnKind::IsActorValuePercentageEqualTo => parse_paren(number_pair)
            .map(|(a, b)| Function::IsActorValuePercentageEqualTo {
                actor_value: a,
                value: b,
            })
            .parse_next(input),

        FnKind::IsActorValuePercentageLessThan => parse_paren(number_pair)
            .map(|(a, b)| Function::IsActorValuePercentageLessThan {
                actor_value: a,
                value: b,
            })
            .parse_next(input),

        FnKind::ValueEqualTo => parse_paren(number_pair)
            .map(|(lhs, rhs)| Function::ValueEqualTo { lhs, rhs })
            .parse_next(input),

        FnKind::ValueLessThan => parse_paren(number_pair)
            .map(|(lhs, rhs)| Function::ValueLessThan { lhs, rhs })
            .parse_next(input),

        // ---------------- plugin + number ----------------
        FnKind::IsFactionRankEqualTo => parse_paren(plugin_number)
            .map(
                |PluginNumber { plugin, number }| Function::IsFactionRankEqualTo {
                    faction: plugin,
                    rank: number,
                },
            )
            .parse_next(input),

        FnKind::IsFactionRankLessThan => parse_paren(plugin_number)
            .map(
                |PluginNumber { plugin, number }| Function::IsFactionRankLessThan {
                    faction: plugin,
                    rank: number,
                },
            )
            .parse_next(input),

        // IsInFaction(plugin)
        FnKind::IsInFaction => parse_paren(parse_plugin_value)
            .map(|v| Function::IsInFaction { faction: v })
            .parse_next(input),

        // ---------------- left/right plugin ----------------
        FnKind::IsEquippedRight | FnKind::IsEquippedLeft => {
            let left = matches!(kind, FnKind::IsEquippedLeft);

            parse_paren(parse_plugin_value)
                .map(move |form| Function::IsEquipped {
                    form,
                    hand_type: if left {
                        HandType::Left
                    } else {
                        HandType::Right
                    },
                })
                .parse_next(input)
        }

        // ---------------- left/right number ----------------
        FnKind::IsEquippedRightType | FnKind::IsEquippedLeftType => {
            let left = matches!(kind, FnKind::IsEquippedLeftType);

            parse_paren(weapon_type)
                .map(move |weapon_type| Function::IsEquippedType {
                    weapon_type,
                    hand_type: if left {
                        HandType::Left
                    } else {
                        HandType::Right
                    },
                })
                .parse_next(input)
        }

        // ---------------- left/right keyword ----------------
        FnKind::IsEquippedRightHasKeyword | FnKind::IsEquippedLeftHasKeyword => {
            let left = matches!(kind, FnKind::IsEquippedLeftHasKeyword);

            parse_paren(parse_plugin_value)
                .map(move |keyword| Function::IsEquippedHasKeyword {
                    keyword,
                    hand_type: if left {
                        HandType::Left
                    } else {
                        HandType::Right
                    },
                })
                .parse_next(input)
        }
    }
}

fn parse_paren<'i, Output, Error, ParseNext>(
    parser: ParseNext,
) -> impl Parser<&'i str, Output, Error>
where
    Error: winnow::error::ParserError<&'i str>,
    ParseNext: Parser<&'i str, Output, Error>,
{
    delimited(
        delimited_multispace0("("),
        parser,
        delimited_multispace0(")"),
    )
}

struct PluginNumber<'i> {
    plugin: PluginValue<'i>,
    number: StaticValue,
}

fn plugin_number<'i>(input: &mut &'i str) -> ModalResult<PluginNumber<'i>> {
    alt((
        seq! {
            PluginNumber {
                plugin: parse_plugin_value,
                _: delimited_multispace0(","),
                number: static_value,
            }
        },
        seq! {
            PluginNumber {
                number: static_value,
                _: delimited_multispace0(","),
                plugin: parse_plugin_value,
            }
        },
    ))
    .context(StrContext::Label(
        "PluginValue + Number arguments",
    ))
    .context(StrContext::Expected(StrContextValue::Description(
        r#"(PluginValue, Number)/(Number, PluginValue): e.g. `("Skyrim.esm" | 0x007`, 10), (30.0, "Skyrim.esm" | 0x007`)"#,
    )))
    .parse_next(input)
}

fn number_pair(input: &mut &str) -> ModalResult<(StaticValue, StaticValue)> {
    separated_pair(static_value, delimited_multispace0(","), static_value)
        .context(StrContext::Label("Number + Number arguments"))
        .context(StrContext::Expected(StrContextValue::Description(
            r#"(Number, Number): e.g. `(30.0, 10)`, `(63, 50)`"#,
        )))
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use crate::parse_assert;

    use super::*;

    #[test]
    fn should_parse_fn_call() {
        let input = r#"IsActorValueLessThan(30, 60)"#;
        let expected = Function::IsActorValueLessThan {
            actor_value: StaticValue { value: 30.0 },
            value: StaticValue { value: 60.0 },
        };

        parse_assert!(function(input), expected);
    }
}
