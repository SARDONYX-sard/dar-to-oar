//! Parses a high-level condition set based on the provided syntax.
use crate::conditions::{
    And, CompareValues, Condition, CurrentGameTime, CurrentWeather, FactionRank, HasKeyword,
    HasMagicEffect, HasMagicEffectWithKeyword, HasPerk, HasRefType, HasSpell, IsActorBase, IsClass,
    IsCombatStyle, IsEquipped, IsEquippedHasKeyword, IsEquippedShout, IsEquippedType, IsInFaction,
    IsInLocation, IsMovementDirection, IsParentCell, IsRace, IsVoiceType, IsWorldSpace, IsWorn,
    IsWornHasKeyword, Level, Oar, Or, RandomCondition,
};
use dar_syntax::ast::{Dar, Expression, Function};
use oar_values::{ActorValueType, Cmp, DirectionValue};
use rayon::prelude::*;

impl<'input> From<Dar<'input>> for Oar<'input> {
    #[inline]
    fn from(dar: Dar<'input>) -> Self {
        match dar {
            Dar::And(conditions) => Oar::And(And {
                conditions: conditions.into_par_iter().map(Into::into).collect(),
                ..Default::default()
            }),
            Dar::Or(conditions) => Oar::Or(Or {
                conditions: conditions.into_par_iter().map(Into::into).collect(),
                ..Default::default()
            }),
            Dar::Exp(expression) => expr_to_oar(expression),
        }
    }
}

fn expr_to_oar(expr: Expression) -> Oar {
    let Expression { negated, function } = expr;

    match function {
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
        Function::IsActorBase { actor_base } => Oar::IsActorBase(IsActorBase {
            negated,
            actor_base,
            ..Default::default()
        }),

        Function::IsActorValueEqualTo { id, value } => Oar::CompareValues(CompareValues {
            negated,
            value_a: id.into(),
            comparison: Cmp::Eq,
            value_b: value.into(),
            ..Default::default()
        }),
        Function::IsActorValueLessThan { id, value } => Oar::CompareValues(CompareValues {
            negated,
            value_a: id.into_actor_value(ActorValueType::ActorValue),
            comparison: Cmp::Lt,
            value_b: value.into(),
            ..Default::default()
        }),
        Function::IsActorValueBaseLessThan { id, value } => Oar::CompareValues(CompareValues {
            negated,
            value_a: id.into_actor_value(ActorValueType::Base),
            comparison: Cmp::Lt,
            value_b: value.into(),
            ..Default::default()
        }),
        Function::IsActorValueMaxEqualTo { id, value } => Oar::CompareValues(CompareValues {
            negated,
            value_a: id.into_actor_value(ActorValueType::Max),
            comparison: Cmp::Eq,
            value_b: value.into(),
            ..Default::default()
        }),
        Function::IsActorValueMaxLessThan { id, value } => Oar::CompareValues(CompareValues {
            negated,
            value_a: id.into_actor_value(ActorValueType::Max),
            comparison: Cmp::Lt,
            value_b: value.into(),
            ..Default::default()
        }),
        Function::IsActorValuePercentageEqualTo { id, value } => {
            Oar::CompareValues(CompareValues {
                negated,
                value_a: id.into_actor_value(ActorValueType::Percentage),
                comparison: Cmp::Eq,
                value_b: value.into(),
                ..Default::default()
            })
        }
        Function::IsActorValuePercentageLessThan { id, value } => {
            Oar::CompareValues(CompareValues {
                negated,
                value_a: id.into_actor_value(ActorValueType::Percentage),
                comparison: Cmp::Lt,
                value_b: value.into(),
                ..Default::default()
            })
        }

        // ---------------- equipped ----------------
        Function::IsEquipped {
            form,
            is_left: left_hand,
        } => Oar::IsEquipped(IsEquipped {
            negated,
            form,
            left_hand,
            ..Default::default()
        }),
        Function::IsEquippedType {
            value: weapon_type,
            is_left: left_hand,
        } => Oar::IsEquippedType(IsEquippedType {
            negated,
            type_value: weapon_type.into(),
            left_hand,
            ..Default::default()
        }),
        Function::IsEquippedHasKeyword {
            keyword,
            is_left: left_hand,
        } => Oar::IsEquippedHasKeyword(IsEquippedHasKeyword {
            negated,
            keyword: keyword.into(),
            left_hand,
            ..Default::default()
        }),
        Function::IsEquippedShout { shout } => Oar::IsEquippedShout(IsEquippedShout {
            negated,
            shout,
            ..Default::default()
        }),

        // ---------------- faction ----------------
        Function::IsInFaction { faction } => Oar::IsInFaction(IsInFaction {
            negated,
            faction,
            ..Default::default()
        }),
        Function::IsFactionRankEqualTo { faction, rank } => Oar::FactionRank(FactionRank {
            negated,
            faction,
            comparison: Cmp::Eq,
            numeric_value: rank.into(),
            ..Default::default()
        }),
        Function::IsFactionRankLessThan { faction, rank } => Oar::FactionRank(FactionRank {
            negated,
            faction,
            comparison: Cmp::Lt,
            numeric_value: rank.into(),
            ..Default::default()
        }),

        // ---------------- has ----------------
        Function::HasKeyword { keyword } => Oar::HasKeyword(HasKeyword {
            negated,
            keyword: keyword.into(),
            ..Default::default()
        }),
        Function::HasPerk { perk } => Oar::HasPerk(HasPerk {
            negated,
            perk,
            ..Default::default()
        }),
        Function::HasSpell { spell } => Oar::HasSpell(HasSpell {
            negated,
            spell,
            ..Default::default()
        }),
        Function::HasMagicEffect { magic_effect } => Oar::HasMagicEffect(HasMagicEffect {
            negated,
            magic_effect,
            ..Default::default()
        }),
        Function::HasMagicEffectWithKeyword { keyword } => {
            Oar::HasMagicEffectWithKeyword(HasMagicEffectWithKeyword {
                negated,
                keyword: keyword.into(),
                ..Default::default()
            })
        }
        Function::HasRefType { location_ref_type } => Oar::HasRefType(HasRefType {
            negated,
            location_ref_type: location_ref_type.into(),
            ..Default::default()
        }),

        // ---------------- misc compare ----------------
        Function::ValueEqualTo { value_a, value_b } => Oar::CompareValues(CompareValues {
            negated,
            value_a: value_a.into(),
            comparison: Cmp::Eq,
            value_b: value_b.into(),
            ..Default::default()
        }),
        Function::ValueLessThan { value_a, value_b } => Oar::CompareValues(CompareValues {
            negated,
            value_a: value_a.into(),
            comparison: Cmp::Lt,
            value_b: value_b.into(),
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
    }
}
