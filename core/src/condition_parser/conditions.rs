//! Parses a high-level condition set based on the provided syntax.
use super::actor::parse_actor;
use super::compare::parse_compare;
use super::equip::parse_equip;
use super::errors::{ParseError, Result};
use super::faction::parse_faction;
use super::has::parse_has;
use super::macros::gen_cond;
use crate::conditions::{
    And, Condition, ConditionSet, CurrentGameTime, CurrentWeather, IsClass, IsCombatStyle,
    IsInLocation, IsMovementDirection, IsParentCell, IsRace, IsVoiceType, IsWorldSpace, IsWorn,
    IsWornHasKeyword, Level, Or, RandomCondition,
};
use crate::dar_syntax::syntax::{self, Expression};
use crate::values::{Cmp, DirectionValue};

/// Parses a high-level condition set based on the provided syntax.
/// # Errors
/// Parsing failed.
pub fn parse_conditions(input: syntax::Condition) -> Result<ConditionSet> {
    Ok(match input {
        syntax::Condition::And(conditions) => {
            let mut inner_conditions = vec![];
            for condition in conditions {
                inner_conditions.push(parse_conditions(condition)?);
            }
            ConditionSet::And(And {
                conditions: inner_conditions,
                ..Default::default()
            })
        }
        syntax::Condition::Or(conditions) => {
            let mut inner_conditions = vec![];
            for condition in conditions {
                inner_conditions.push(parse_conditions(condition)?);
            }
            ConditionSet::Or(Or {
                conditions: inner_conditions,
                ..Default::default()
            })
        }
        syntax::Condition::Exp(expression) => parse_condition(expression)?,
    })
}

/// Parses a conditional expression and translates it into a corresponding [`ConditionSet`].
/// # Errors
/// Parsing failed.
fn parse_condition(condition: Expression) -> Result<ConditionSet> {
    let Expression {
        negated,
        fn_name,
        mut args,
    } = condition;

    Ok(match fn_name {
        "CurrentGameTimeLessThan" => {
            if args.is_empty() {
                return Err(ParseError::UnexpectedValue(
                    "At least 1 argument is required, but got 0".into(),
                    "".into(),
                ));
            }

            ConditionSet::CurrentGameTime(CurrentGameTime {
                negated,
                comparison: Cmp::Lt,
                numeric_value: args.swap_remove(0).into(),
                ..Default::default()
            })
        }
        "CurrentWeather" => gen_cond!(
            CurrentWeather(weather, negated),
            args,
            "PluginValue for weather"
        ),
        "IsClass" => gen_cond!(IsClass(class, negated), args, "PluginValue for IsClass"),
        "IsCombatStyle" => {
            gen_cond!(
                IsCombatStyle(combat_style, negated),
                args,
                "PluginValue for IsCombatStyle"
            )
        }
        actor if fn_name.starts_with("IsActor") => parse_actor(actor, args, negated)?,
        equip if fn_name.starts_with("IsEquipped") => parse_equip(equip, args, negated)?,
        "IsInFaction" | "IsFactionRankEqualTo" | "IsFactionRankLessThan" => {
            parse_faction(fn_name, args, negated)?
        }
        "IsInLocation" => gen_cond!(IsInLocation(location, negated), args, "IsInLocation"),
        "IsLevelLessThan" => {
            if args.is_empty() {
                return Err(ParseError::UnexpectedValue(
                    "At least 1 argument is required, but got 0".into(),
                    "".into(),
                ));
            }

            ConditionSet::Level(Level {
                negated,
                comparison: Cmp::Lt,
                numeric_value: args.swap_remove(0).into(),
                ..Default::default()
            })
        }
        "IsParentCell" => gen_cond!(
            IsParentCell(cell, negated),
            args,
            "PluginValue for IsParentCell"
        ),
        "IsMovementDirection" => {
            if args.is_empty() {
                return Err(ParseError::UnexpectedValue(
                    "At least 1 argument is required, but got 0".into(),
                    "".into(),
                ));
            }

            ConditionSet::IsDirectionMovement(IsMovementDirection {
                negated,
                direction: DirectionValue {
                    value: args.swap_remove(0).try_into()?,
                },
                ..Default::default()
            })
        }
        "IsRace" => gen_cond!(IsRace(race, negated), args, "PluginValue for IsRace"),
        "IsVoiceType" => {
            gen_cond!(
                IsVoiceType(voice_type, negated),
                args,
                "PluginValue for IsVoiceType"
            )
        }
        "IsWorldSpace" => gen_cond!(
            IsWorldSpace(world_space, negated),
            args,
            "PluginValue for IsWorldSpace"
        ),
        "IsWorn" => gen_cond!(IsWorn(form, negated), args, "IsWorn"),
        "IsWornHasKeyword" => gen_cond!(
            IsWornHasKeyword(keyword, negated),
            args,
            "IsWornHasKeyword",
            into
        ),
        has_cond if fn_name.starts_with("Has") => parse_has(has_cond, args, negated)?,
        "Random" => {
            if args.is_empty() {
                return Err(ParseError::UnexpectedValue(
                    "At least 1 argument is required, but got 0".into(),
                    "".into(),
                ));
            }

            ConditionSet::RandomCondition(RandomCondition {
                negated,
                comparison: Cmp::Le,
                numeric_value: args.swap_remove(0).into(),
                ..Default::default()
            })
        }
        "ValueEqualTo" | "ValueLessThan" => parse_compare(fn_name, args, negated)?,

        // Conditional expressions without any arguments
        // This enumeration order is the same as the DAR docs.
        "IsFemale" | "IsChild" | "IsPlayerTeammate" | "IsInInterior" | "IsUnique"
        | "IsAttacking" | "IsRunning" | "IsSneaking" | "IsSprinting" | "IsInAir" | "IsInCombat"
        | "IsWeaponDrawn" => ConditionSet::Condition(Condition {
            condition: fn_name.into(),
            negated,
            ..Default::default()
        }),
        unknown_condition => {
            return Err(ParseError::UnexpectedValue(
                "Unknown condition: ".into(),
                unknown_condition.into(),
            ))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
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
                form_id: NumberLiteral::Hex(0x0000_0007),
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

        let input = syntax::Condition::And(vec![
            syntax::Condition::Or(vec![
                syntax::Condition::Exp(actor),
                syntax::Condition::Exp(player),
            ]),
            syntax::Condition::Or(vec![
                syntax::Condition::Exp(equip_r3),
                syntax::Condition::Exp(equip_r4),
            ]),
        ]);

        let conditions = parse_conditions(input);

        let expected: ConditionSet = ConditionSet::And(And {
            conditions: vec![
                ConditionSet::Or(Or {
                    conditions: vec![
                        ConditionSet::IsActorBase(IsActorBase {
                            negated: false,
                            actor_base: PluginValue {
                                plugin_name: "Skyrim.esm".into(),
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
                            negated: false,
                            type_value: TypeValue {
                                value: WeaponType::WarAxe,
                            },
                            left_hand: true,
                            ..Default::default()
                        }),
                        ConditionSet::IsEquippedType(IsEquippedType {
                            negated: true,
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
            ..Default::default()
        });

        assert_eq!(conditions, Ok(expected));
    }
}
