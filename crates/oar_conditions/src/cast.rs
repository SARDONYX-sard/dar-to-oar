//! Parses a high-level condition set based on the provided syntax.
use crate::{
    conditions::{
        And, CompareValues, Condition, CurrentGameTime, CurrentWeather, IsClass, IsCombatStyle,
        IsInLocation, IsMovementDirection, IsParentCell, IsRace, IsVoiceType, IsWorldSpace, IsWorn,
        IsWornHasKeyword, Level, Oar, Or, RandomCondition,
    },
    error::Error,
};
use dar_syntax::ast::{Dar, Expression, Function, HandType};
use oar_values::{ActorValue, ActorValueType, Cmp, DirectionValue, NumericValue};
use rayon::prelude::*;

impl<'input> TryFrom<Dar<'input>> for Oar<'input> {
    type Error = Error;

    fn try_from(dar: Dar<'input>) -> Result<Self, Self::Error> {
        fn oar_to<'input>(conditions: Vec<Dar<'input>>) -> Result<Vec<Oar<'input>>, Error> {
            let (oar, errors): (Vec<Oar<'input>>, Vec<Error>) = conditions
                .into_par_iter()
                .partition_map(|condition| match condition.try_into() {
                    Ok(oar) => rayon::iter::Either::Left(oar),
                    Err(err) => rayon::iter::Either::Right(err),
                });

            if errors.is_empty() {
                return Ok(oar);
            }

            Err(Error::NestedError { errors })
        }

        Ok(match dar {
            Dar::And(conditions) => Oar::And(And {
                conditions: oar_to(conditions)?,
                ..Default::default()
            }),
            Dar::Or(conditions) => Oar::Or(Or {
                conditions: oar_to(conditions)?,
                ..Default::default()
            }),
            Dar::Exp(expression) => expr_to_oar(expression)?,
        })
    }
}

fn expr_to_oar(expr: Expression) -> Result<Oar, Error> {
    let Expression { negated, function } = expr;

    Ok(match function {
        // ---------------- simple number ----------------
        Function::CurrentGameTimeLessThan { value } => Oar::CurrentGameTime(CurrentGameTime {
            negated,
            comparison: Cmp::Lt,
            numeric_value: value.into(),
            ..Default::default()
        }),
        Function::Random { value } => Oar::RandomCondition(RandomCondition {
            negated,
            comparison: Cmp::Le,
            numeric_value: value.into(),
            ..Default::default()
        }),
        Function::IsLevelLessThan { level } => Oar::Level(Level {
            negated,
            comparison: Cmp::Lt,
            numeric_value: level.into(),
            ..Default::default()
        }),
        Function::IsMovementDirection { direction } => {
            Oar::IsDirectionMovement(IsMovementDirection {
                negated,
                direction: DirectionValue { value: direction },
                ..Default::default()
            })
        }

        // ---------------- plugin ----------------
        Function::CurrentWeather { weather } => Oar::CurrentWeather(CurrentWeather {
            negated,
            weather,
            ..Default::default()
        }),
        Function::IsClass { class } => Oar::IsClass(IsClass {
            negated,
            class,
            ..Default::default()
        }),
        Function::IsCombatStyle { combat_style } => Oar::IsCombatStyle(IsCombatStyle {
            negated,
            combat_style,
            ..Default::default()
        }),
        Function::IsRace { race } => Oar::IsRace(IsRace {
            negated,
            race,
            ..Default::default()
        }),
        Function::IsVoiceType { voice_type } => Oar::IsVoiceType(IsVoiceType {
            negated,
            voice_type,
            ..Default::default()
        }),
        Function::IsWorldSpace { world_space } => Oar::IsWorldSpace(IsWorldSpace {
            negated,
            world_space,
            ..Default::default()
        }),
        Function::IsParentCell { cell } => Oar::IsParentCell(IsParentCell {
            negated,
            cell,
            ..Default::default()
        }),
        Function::IsWorn { form } => Oar::IsWorn(IsWorn {
            negated,
            form,
            ..Default::default()
        }),
        Function::IsWornHasKeyword { keyword } => Oar::IsWornHasKeyword(IsWornHasKeyword {
            negated,
            keyword: keyword.into(),
            ..Default::default()
        }),
        Function::IsInLocation { location } => Oar::IsInLocation(IsInLocation {
            negated,
            location,
            ..Default::default()
        }),

        // ---------------- actor ----------------
        Function::IsActorBase { actor_base } => Oar::IsActorBase(crate::conditions::IsActorBase {
            negated,
            actor_base,
            ..Default::default()
        }),

        Function::IsActorValueEqualTo { actor_value, value } => Oar::CompareValues(CompareValues {
            negated,
            value_a: actor_value.into(),
            comparison: Cmp::Eq,
            value_b: value.into(),
            ..Default::default()
        }),
        Function::IsActorValueLessThan { actor_value, value } => {
            Oar::CompareValues(CompareValues {
                negated,
                value_a: NumericValue::ActorValue(ActorValue {
                    actor_value: oar_values::NumericLiteral::Float(actor_value.value),
                    actor_value_type: ActorValueType::ActorValue,
                }),
                comparison: Cmp::Lt,
                value_b: value.into(),
                ..Default::default()
            })
        }
        Function::IsActorValueBaseLessThan { actor_value, value } => {
            Oar::CompareValues(CompareValues {
                negated,
                value_a: NumericValue::ActorValue(ActorValue {
                    actor_value: oar_values::NumericLiteral::Float(actor_value.value),
                    actor_value_type: ActorValueType::Base,
                }),
                comparison: Cmp::Lt,
                value_b: value.into(),
                ..Default::default()
            })
        }
        Function::IsActorValueMaxEqualTo { actor_value, value } => {
            Oar::CompareValues(CompareValues {
                negated,
                value_a: NumericValue::ActorValue(ActorValue {
                    actor_value: oar_values::NumericLiteral::Float(actor_value.value),
                    actor_value_type: ActorValueType::Max,
                }),
                comparison: Cmp::Eq,
                value_b: value.into(),
                ..Default::default()
            })
        }
        Function::IsActorValueMaxLessThan { actor_value, value } => {
            Oar::CompareValues(CompareValues {
                negated,
                value_a: NumericValue::ActorValue(ActorValue {
                    actor_value: oar_values::NumericLiteral::Float(actor_value.value),
                    actor_value_type: ActorValueType::Max,
                }),
                comparison: Cmp::Lt,
                value_b: value.into(),
                ..Default::default()
            })
        }
        Function::IsActorValuePercentageEqualTo { actor_value, value } => {
            Oar::CompareValues(CompareValues {
                negated,
                value_a: NumericValue::ActorValue(ActorValue {
                    actor_value: oar_values::NumericLiteral::Float(actor_value.value),
                    actor_value_type: ActorValueType::Percentage,
                }),
                comparison: Cmp::Eq,
                value_b: value.into(),
                ..Default::default()
            })
        }
        Function::IsActorValuePercentageLessThan { actor_value, value } => {
            Oar::CompareValues(CompareValues {
                negated,
                value_a: NumericValue::ActorValue(ActorValue {
                    actor_value: oar_values::NumericLiteral::Float(actor_value.value),
                    actor_value_type: ActorValueType::Percentage,
                }),
                comparison: Cmp::Lt,
                value_b: value.into(),
                ..Default::default()
            })
        }

        // ---------------- equipped ----------------
        Function::IsEquipped { form, hand_type } => {
            Oar::IsEquipped(crate::conditions::IsEquipped {
                negated,
                form,
                left_hand: hand_type == HandType::Left,
                ..Default::default()
            })
        }
        Function::IsEquippedType {
            weapon_type,
            hand_type,
        } => Oar::IsEquippedType(crate::conditions::IsEquippedType {
            negated,
            type_value: weapon_type.into(),
            left_hand: hand_type == HandType::Left,
            ..Default::default()
        }),
        Function::IsEquippedHasKeyword { keyword, hand_type } => {
            Oar::IsEquippedHasKeyword(crate::conditions::IsEquippedHasKeyword {
                negated,
                keyword: keyword.into(),
                left_hand: hand_type == HandType::Left,
                ..Default::default()
            })
        }
        Function::IsEquippedShout { shout } => {
            Oar::IsEquippedShout(crate::conditions::IsEquippedShout {
                negated,
                shout,
                ..Default::default()
            })
        }

        // ---------------- faction ----------------
        Function::IsInFaction { faction } => Oar::IsInFaction(crate::conditions::IsInFaction {
            negated,
            faction,
            ..Default::default()
        }),
        Function::IsFactionRankEqualTo { faction, rank } => {
            Oar::FactionRank(crate::conditions::FactionRank {
                negated,
                faction,
                comparison: Cmp::Eq,
                numeric_value: rank.into(),
                ..Default::default()
            })
        }
        Function::IsFactionRankLessThan { faction, rank } => {
            Oar::FactionRank(crate::conditions::FactionRank {
                negated,
                faction,
                comparison: Cmp::Lt,
                numeric_value: rank.into(),
                ..Default::default()
            })
        }

        // ---------------- has ----------------
        Function::HasKeyword { keyword } => Oar::HasKeyword(crate::conditions::HasKeyword {
            negated,
            keyword: keyword.into(),
            ..Default::default()
        }),
        Function::HasPerk { perk } => Oar::HasPerk(crate::conditions::HasPerk {
            negated,
            perk,
            ..Default::default()
        }),
        Function::HasSpell { spell } => Oar::HasSpell(crate::conditions::HasSpell {
            negated,
            spell,
            ..Default::default()
        }),
        Function::HasMagicEffect { magic_effect } => {
            Oar::HasMagicEffect(crate::conditions::HasMagicEffect {
                negated,
                magic_effect,
                ..Default::default()
            })
        }
        Function::HasMagicEffectWithKeyword { keyword } => {
            Oar::HasMagicEffectWithKeyword(crate::conditions::HasMagicEffectWithKeyword {
                negated,
                keyword: keyword.into(),
                ..Default::default()
            })
        }
        Function::HasRefType { location_ref_type } => {
            Oar::HasRefType(crate::conditions::HasRefType {
                negated,
                location_ref_type: location_ref_type.into(),
                ..Default::default()
            })
        }

        // ---------------- misc compare ----------------
        Function::ValueEqualTo { lhs, rhs } => Oar::CompareValues(CompareValues {
            negated,
            value_a: lhs.into(),
            comparison: Cmp::Eq,
            value_b: rhs.into(),
            ..Default::default()
        }),
        Function::ValueLessThan { lhs, rhs } => Oar::CompareValues(CompareValues {
            negated,
            value_a: lhs.into(),
            comparison: Cmp::Lt,
            value_b: rhs.into(),
            ..Default::default()
        }),

        // ---------------- no arg ----------------
        Function::IsFemale => Oar::Condition(Condition {
            condition: "IsFemale".into(),
            negated,
            ..Default::default()
        }),
        Function::IsChild => Oar::Condition(Condition {
            condition: "IsChild".into(),
            negated,
            ..Default::default()
        }),
        Function::IsPlayerTeammate => Oar::Condition(Condition {
            condition: "IsPlayerTeammate".into(),
            negated,
            ..Default::default()
        }),
        Function::IsInInterior => Oar::Condition(Condition {
            condition: "IsInInterior".into(),
            negated,
            ..Default::default()
        }),
        Function::IsUnique => Oar::Condition(Condition {
            condition: "IsUnique".into(),
            negated,
            ..Default::default()
        }),
        Function::IsAttacking => Oar::Condition(Condition {
            condition: "IsAttacking".into(),
            negated,
            ..Default::default()
        }),
        Function::IsRunning => Oar::Condition(Condition {
            condition: "IsRunning".into(),
            negated,
            ..Default::default()
        }),
        Function::IsSneaking => Oar::Condition(Condition {
            condition: "IsSneaking".into(),
            negated,
            ..Default::default()
        }),
        Function::IsSprinting => Oar::Condition(Condition {
            condition: "IsSprinting".into(),
            negated,
            ..Default::default()
        }),
        Function::IsInAir => Oar::Condition(Condition {
            condition: "IsInAir".into(),
            negated,
            ..Default::default()
        }),
        Function::IsInCombat => Oar::Condition(Condition {
            condition: "IsInCombat".into(),
            negated,
            ..Default::default()
        }),
        Function::IsWeaponDrawn => Oar::Condition(Condition {
            condition: "IsWeaponDrawn".into(),
            negated,
            ..Default::default()
        }),
    })
}
