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
use crate::dar_syntax::{Condition as DarCondition, Expression};
use crate::values::{Cmp, DirectionValue};

/// Parses a high-level condition set based on the provided syntax.
/// # Errors
/// Parsing failed.
pub fn parse_conditions(input: DarCondition) -> Result<ConditionSet> {
    Ok(match input {
        DarCondition::And(conditions) => {
            let mut inner_conditions = vec![];
            for condition in conditions {
                inner_conditions.push(parse_conditions(condition)?);
            }
            ConditionSet::And(And {
                conditions: inner_conditions,
                ..Default::default()
            })
        }
        DarCondition::Or(conditions) => {
            let mut inner_conditions = vec![];
            for condition in conditions {
                inner_conditions.push(parse_conditions(condition)?);
            }
            ConditionSet::Or(Or {
                conditions: inner_conditions,
                ..Default::default()
            })
        }
        DarCondition::Exp(expression) => parse_condition(expression)?,
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
        "CurrentGameTimeLessThan" => ConditionSet::CurrentGameTime(CurrentGameTime {
            negated,
            comparison: Cmp::Lt,
            numeric_value: args.pop_front()?.into(),
            ..Default::default()
        }),
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
        "IsLevelLessThan" => ConditionSet::Level(Level {
            negated,
            comparison: Cmp::Lt,
            numeric_value: args.pop_front()?.into(),
            ..Default::default()
        }),
        "IsParentCell" => gen_cond!(
            IsParentCell(cell, negated),
            args,
            "PluginValue for IsParentCell"
        ),
        "IsMovementDirection" => ConditionSet::IsDirectionMovement(IsMovementDirection {
            negated,
            direction: DirectionValue {
                value: args.pop_front()?.try_into()?,
            },
            ..Default::default()
        }),
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
        "Random" => ConditionSet::RandomCondition(RandomCondition {
            negated,
            comparison: Cmp::Le,
            numeric_value: args.pop_front()?.into(),
            ..Default::default()
        }),
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
            return Err(ParseError::UnexpectedValue {
                expected: "Unknown condition: ".into(),
                actual: unknown_condition.into(),
            })
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        conditions::{And, IsActorBase, IsEquippedType},
        dar_syntax::{ast::fn_args::fn_args, FnArg, NumberLiteral},
        values::{PluginValue, TypeValue, WeaponType},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_conditions() {
        let actor = Expression {
            negated: false,
            fn_name: "IsActorBase",
            args: fn_args![FnArg::PluginValue {
                plugin_name: "Skyrim.esm",
                form_id: NumberLiteral::Hex(0x0000_0007),
            }],
        };
        let player = Expression {
            negated: false,
            fn_name: "IsPlayerTeammate",
            args: fn_args![],
        };
        let equip_r3 = Expression {
            negated: false,
            fn_name: "IsEquippedLeftType",
            args: fn_args![FnArg::Number(NumberLiteral::Decimal(3))],
        };
        let equip_r4 = Expression {
            negated: true,
            fn_name: "IsEquippedRightType",
            args: fn_args![FnArg::Number(NumberLiteral::Decimal(4))],
        };

        let input = DarCondition::And(vec![
            DarCondition::Or(vec![DarCondition::Exp(actor), DarCondition::Exp(player)]),
            DarCondition::Or(vec![
                DarCondition::Exp(equip_r3),
                DarCondition::Exp(equip_r4),
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
