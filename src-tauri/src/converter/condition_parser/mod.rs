mod actor;
mod compare;
mod dar_interface;
mod equip;
mod faction;
mod has;

use self::actor::parse_actor;
use self::compare::parse_compare;
use self::dar_interface::ParseError;
use self::equip::parse_equip;
use self::faction::parse_faction;
use self::has::parse_has;
use super::conditions::{
    And, Condition, ConditionSet, CurrentGameTime, CurrentWeather, IsClass, IsCombatStyle,
    IsInLocation, IsMovementDirection, IsParentCell, IsRace, IsVoiceType, IsWorldSpace, IsWorn,
    IsWornHasKeyword, Level, Or, RandomCondition,
};
use super::values::{Cmp, DirectionValue, NumericValue};
use crate::converter::dar_syntax::syntax::{Condition as Condition_, Expression};

fn parse_conditions(input: Condition_) -> Result<ConditionSet, ParseError> {
    Ok(match input {
        Condition_::And(conditions) => {
            let mut inner_conditions = vec![];
            for condition in conditions {
                inner_conditions.push(parse_conditions(condition)?);
            }
            ConditionSet::And(And {
                conditions: inner_conditions,
                ..Default::default()
            })
        }
        Condition_::Or(conditions) => {
            let mut inner_conditions = vec![];
            for condition in conditions {
                inner_conditions.push(parse_conditions(condition)?);
            }
            ConditionSet::Or(Or {
                conditions: inner_conditions,
                ..Default::default()
            })
        }
        Condition_::Exp(expression) => parse_condition(expression)?,
    })
}

#[macro_export]
macro_rules! get_arg {
    ($args:ident[$index:literal], $expected:literal) => {
        $args.get($index).ok_or(ParseError::UnexpectedValue(
            $expected.to_string(),
            format!("None in args[{}]", $index),
        ))
    };
    ($args:ident[$index:literal], $expected:literal, $actual:literal) => {
        $args.get($index).ok_or(ParseError::UnexpectedValue(
            $expected.to_string(),
            $actual.to_string(),
        ))
    };
}

// Return early if [$index]th of args cannot be obtained. After that, do into.
#[macro_export]
macro_rules! get_into {
    ($args:ident[$index:literal], $expected:literal) => {
        crate::get_arg!($args[$index], $expected)?.into()
    };
    ($args:ident[$index:literal], $expected:literal, $actual:literal) => {
        crate::get_arg!($args[$index], $expected, $actual)?.into()
    };
}

/// # Examples
/// ```rust
/// let plugin_value = get_try_into!(
///     args[0], // slice & index
///     "float(e.g. 1.0): in ValueEqualTo | ValueLessThan 2nd arg", // Expected
///     "None" // Actual(This is option)
/// )?;
/// ```
#[macro_export]
macro_rules! get_try_into {
    ($args:ident[$index:literal], $expected:literal) => {
        crate::get_arg!($args[$index], $expected)?.try_into()
    };
    ($args:ident[$index:literal], $expected:literal, $actual:literal) => {
        crate::get_arg!($args[$index], $expected, $actual)?.try_into()
    };
}

#[macro_export]
/// $id:ident, $field_name:ident, $args:ident $negated:expr, $expected:literal
macro_rules! gen_cond {
    ($id:ident($field_name:ident, $negated:ident), $args:ident, $expected:literal) => {
        ConditionSet::$id($id {
            condition: Condition {
                condition: stringify!($id).into(),
                negated: $negated,
                ..Default::default()
            },
            $field_name: crate::get_try_into!($args[0], $expected)?,
        })
    };
    ($id:ident($field_name:ident, $negated:ident), $args:ident, $expected:literal, into) => {
        ConditionSet::$id($id {
            condition: Condition {
                negated: $negated,
                ..Default::default()
            },
            $field_name: crate::get_into!($args[0], $expected),
        })
    };
}

fn parse_condition<'a>(condition: Expression<'a>) -> Result<ConditionSet, ParseError> {
    let Expression {
        negated,
        fn_name,
        args,
    } = condition;

    Ok(match fn_name {
        "ValueEqualTo" | "ValueLessThan" => parse_compare(fn_name, args, negated)?,
        actor if fn_name.starts_with("IsActor") => parse_actor(actor, args, negated)?,
        faction if fn_name.starts_with("IsFaction") => parse_faction(faction, args, negated)?,
        equip if fn_name.starts_with("IsEquipped") => parse_equip(equip, args, negated)?,
        "IsLevelLessThan" => ConditionSet::Level(Level {
            comparison: Cmp::Lt,
            numeric_value: get_into!(args[1], "NumericValue"),
            condition: Condition {
                condition: "Level".into(),
                negated,
                ..Default::default()
            },
            ..Default::default()
        }),
        "CurrentWeather" => gen_cond!(
            CurrentWeather(weather, negated),
            args,
            "PluginValue for weather"
        ),
        "IsRace" => gen_cond!(IsRace(race, negated), args, "PluginValue for IsRace"),
        "IsClass" => gen_cond!(IsClass(class, negated), args, "PluginValue for IsClass"),
        "IsCombatStyle" => {
            gen_cond!(
                IsCombatStyle(combat_style, negated),
                args,
                "PluginValue for IsCombatStyle"
            )
        }
        "IsVoiceType" => {
            gen_cond!(
                IsVoiceType(voice_type, negated),
                args,
                "PluginValue for IsVoiceType"
            )
        }
        "IsParentCell" => gen_cond!(
            IsParentCell(cell, negated),
            args,
            "PluginValue for IsParentCell"
        ),
        "IsWorldSpace" => gen_cond!(
            IsWorldSpace(world_space, negated),
            args,
            "PluginValue for IsWorldSpace"
        ),
        "IsMovementDirection" => ConditionSet::IsDirectionMovement(IsMovementDirection {
            condition: Condition {
                negated,
                ..Default::default()
            },
            direction: DirectionValue {
                value: get_try_into!(args[1], "Direction: 0..=4")?,
            },
            ..Default::default()
        }),
        "IsInLocation" => gen_cond!(IsInLocation(location, negated), args, "IsInLocation"),
        "IsWorn" => gen_cond!(IsWorn(form, negated), args, "IsWorn"),
        "IsWornHasKeyword" => gen_cond!(
            IsWornHasKeyword(keyword, negated),
            args,
            "IsWornHasKeyword",
            into
        ),
        has_cond if fn_name.starts_with("Has") => parse_has(has_cond, args, negated)?,
        "Random" => ConditionSet::RandomCondition(RandomCondition {
            condition: Condition {
                negated,
                ..Default::default()
            },
            comparison: Cmp::Le,
            numeric_value: get_into!(args[0], "NumericValue in Random"),
            ..Default::default()
        }),
        "CurrentGameTimeLessThan" => ConditionSet::CurrentGameTime(CurrentGameTime {
            condition: Condition {
                negated,
                ..Default::default()
            },
            comparison: Cmp::Lt,
            numeric_value: NumericValue::StaticValue(args[0].clone().try_into().unwrap()),
        }),
        _ => {
            log::warn!("Condition({fn_name}) has no explicit mapping.");

            ConditionSet::Condition(Condition {
                negated,
                condition: fn_name.to_string(),
                ..Default::default()
            })
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converter::{
        conditions::{And, IsActorBase, IsEquippedType},
        dar_syntax::syntax::{FnArg, NumberLiteral},
        values::{PluginValue, TypeValue, WeaponType},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_conditions() {
        let actor = Expression {
            negated: false,
            fn_name: "IsActorBase",
            args: vec![FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Hex(0x00000007),
            }],
        };
        let player = Expression {
            negated: false,
            fn_name: "IsPlayerTeammate",
            args: vec![],
        };
        let equip_r3 = Expression {
            negated: false,
            fn_name: "IsEquippedLeftType",
            args: vec![FnArg::Number(NumberLiteral::Decimal(3))],
        };
        let equip_r4 = Expression {
            negated: true,
            fn_name: "IsEquippedRightType",
            args: vec![FnArg::Number(NumberLiteral::Decimal(4))],
        };

        let input = Condition_::And(vec![
            Condition_::Or(vec![Condition_::Exp(actor), Condition_::Exp(player)]),
            Condition_::Or(vec![Condition_::Exp(equip_r3), Condition_::Exp(equip_r4)]),
        ]);

        let conditions = parse_conditions(input);

        let expected: ConditionSet = ConditionSet::And(And {
            conditions: vec![
                ConditionSet::Or(Or {
                    conditions: vec![
                        ConditionSet::IsActorBase(IsActorBase {
                            condition: Condition {
                                condition: "IsActorBase".into(),
                                negated: false,
                                ..Default::default()
                            },
                            actor_base: PluginValue {
                                plugin_name: "Skyrim.esm".to_string(),
                                form_id: "7".into(),
                            },
                            ..Default::default()
                        }),
                        ConditionSet::Condition(Condition::new("IsPlayerTeammate")),
                    ],
                    ..Default::default()
                }),
                ConditionSet::Or(Or {
                    conditions: vec![
                        ConditionSet::IsEquippedType(IsEquippedType {
                            condition: Condition {
                                condition: "IsEquippedType".into(),
                                negated: false,
                                ..Default::default()
                            },
                            type_value: TypeValue {
                                value: WeaponType::WarAxe,
                            },
                            left_hand: true,
                            ..Default::default()
                        }),
                        ConditionSet::IsEquippedType(IsEquippedType {
                            condition: Condition {
                                condition: "IsEquippedType".into(),
                                negated: true,
                                ..Default::default()
                            },
                            type_value: TypeValue {
                                value: WeaponType::Mace,
                            },
                            left_hand: false,
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
        });

        assert_eq!(conditions, Ok(expected));
    }
}
