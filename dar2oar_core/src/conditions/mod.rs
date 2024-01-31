//! Module for representing conditions used in DAR files.
mod and;
mod compare_values;
mod condition;
mod condition_config;
mod current_weather;
mod faction_rank;
mod has_keyword;
mod has_magic_effect;
mod has_magic_effect_with_keyword;
mod has_perk;
mod has_ref_type;
mod is_equipped;
mod is_equipped_has_keyword;
mod is_equipped_type;
mod is_movement_direction;
mod is_worn_has_keyword;
mod namespace_config;
mod or;
mod random;

pub use self::{
    and::And, compare_values::CompareValues, condition::Condition,
    condition_config::ConditionsConfig, current_weather::CurrentWeather, faction_rank::FactionRank,
    has_keyword::HasKeyword, has_magic_effect::HasMagicEffect,
    has_magic_effect_with_keyword::HasMagicEffectWithKeyword, has_perk::HasPerk,
    has_ref_type::HasRefType, is_equipped::IsEquipped,
    is_equipped_has_keyword::IsEquippedHasKeyword, is_equipped_type::IsEquippedType,
    is_movement_direction::IsMovementDirection, is_worn_has_keyword::IsWornHasKeyword,
    namespace_config::MainConfig, or::Or, random::RandomCondition,
};

use self::condition::default_required_version;
use crate::values::{Cmp, NumericValue, PluginValue};
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// Returns `true` if the provided boolean value is `false`, otherwise `false`.
///
/// This function is used as a predicate for serialization purposes to skip fields
/// that have a default value of `false`.
#[inline]
pub(super) const fn is_false(t: &bool) -> bool {
    !(*t)
}

/// Generate structures that have only condition, Comparison and [`NumericValue`]
macro_rules! gen_cmp_num_struct {
    ($($(#[$attr:meta])* $name:ident),+ $(,)?) => {
      $(
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct $name {
            pub condition: CompactString,
            #[serde(default = "default_required_version")]
            #[serde(rename = "requiredVersion")]
            pub required_version: CompactString,
            #[serde(default)]
            #[serde(skip_serializing_if = "is_false")]
            pub negated: bool,

            /// == | != | > | >= | < | <=
            #[serde(default)]
            #[serde(rename = "Comparison")]
            pub comparison: Cmp,
            #[serde(default)]
            #[serde(rename = "Numeric value")]
            pub numeric_value: NumericValue,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    condition: stringify!($name).into(),
                    required_version: default_required_version(),
                    negated: Default::default(),
                    comparison: Default::default(),
                    numeric_value: Default::default(),
                }
            }
        }
      )+
    };
}

gen_cmp_num_struct!(
    /// - OAR: Level
    /// - Condition name "Level"
    Level,
    /// Compare current game time and numeric value.
    /// - Condition name "CurrentGameTime"
    CurrentGameTime
);

/// generate structures that have only condition and [`PluginValue`]
#[macro_export]
macro_rules! gen_one_plugin_struct {
    ($($(#[$attr:meta])* $name:ident, $field:ident => $rename_field:literal),+ $(,)?) => {
        $(
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct $name {
            pub condition: CompactString,
            #[serde(default = "default_required_version")]
            #[serde(rename = "requiredVersion")]
            pub required_version: CompactString,
            #[serde(default)]
            #[serde(skip_serializing_if = "is_false")]
            pub negated: bool,

            #[serde(rename = $rename_field)]
            #[serde(default)]
            pub $field: PluginValue,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    condition: stringify!($name).into(),
                    required_version: default_required_version(),
                    negated: Default::default(),
                    $field: Default::default(),
                }
            }
        }
        )+
    };
}

gen_one_plugin_struct!(
  HasSpell, spell => "Spell",
  IsActorBase, actor_base => "Actor base",
  IsClass, class => "Class",
  IsCombatStyle, combat_style => "Combat style",
  IsEquippedShout, shout => "Shout",
  IsInFaction, faction => "Faction",
  IsInLocation, location => "Location",
  IsParentCell, cell => "Cell",
  IsRace, race => "Race",
  IsVoiceType, voice_type => "Voice type",
  IsWorldSpace, world_space => "WorldSpace",
  IsWorn, form => "Form",
);

/// Represents a set of conditions that can be serialized to the OAR of functions present in the DAR.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConditionSet {
    /// Represents a logical AND operation between conditions.
    And(And),

    /// Represents a single condition.
    Condition(Condition),

    /// Represents a comparison between values.
    CompareValues(CompareValues),

    /// Represents a condition based on the current game time.
    CurrentGameTime(CurrentGameTime),

    /// Represents a condition based on the current weather in the game.
    CurrentWeather(CurrentWeather),

    /// Represents a condition based on the faction rank of an entity.
    FactionRank(FactionRank),

    /// Represents a condition based on whether an entity has a certain keyword.
    HasKeyword(HasKeyword),

    /// Represents a condition based on whether an entity has a specific magic effect.
    HasMagicEffect(HasMagicEffect),

    /// Represents a condition based on whether an entity has a magic effect with a certain keyword.
    HasMagicEffectWithKeyword(HasMagicEffectWithKeyword),

    /// Represents a condition based on whether an entity has a specific perk.
    HasPerk(HasPerk),

    /// Represents a condition based on the reference type of an entity.
    HasRefType(HasRefType),

    /// Represents a condition based on whether an entity has a specific spell.
    HasSpell(HasSpell),

    /// Represents a condition based on the actor base of an entity.
    IsActorBase(IsActorBase),

    /// Represents a condition based on the class of an entity.
    IsClass(IsClass),

    /// Represents a condition based on the combat style of an entity.
    IsCombatStyle(IsCombatStyle),

    /// Represents a condition based on whether an entity is equipped with something.
    IsEquipped(IsEquipped),

    /// Represents a condition based on whether an equipped item has a certain keyword.
    IsEquippedHasKeyword(IsEquippedHasKeyword),

    /// Represents a condition based on whether a shout is equipped.
    IsEquippedShout(IsEquippedShout),

    /// Represents a condition based on the equipped type of an entity.
    IsEquippedType(IsEquippedType),

    /// Represents a condition based on whether an entity is in a faction.
    IsInFaction(IsInFaction),

    /// Represents a condition based on whether an entity is in a specific location.
    IsInLocation(IsInLocation),

    /// Represents a condition based on the parent cell of an entity.
    IsParentCell(IsParentCell),

    /// Represents a condition based on the race of an entity.
    IsRace(IsRace),

    /// Represents a condition based on the voice type of an entity.
    IsVoiceType(IsVoiceType),

    /// Represents a condition based on the world space of an entity.
    IsWorldSpace(IsWorldSpace),

    /// Represents a condition based on whether an entity is worn.
    IsWorn(IsWorn),

    /// Represents a condition based on whether a worn item has a certain keyword.
    IsWornHasKeyword(IsWornHasKeyword),

    /// Represents a condition based on the movement direction of an entity.
    IsDirectionMovement(IsMovementDirection),

    /// Represents a condition based on the level of an entity.
    Level(Level),

    /// Represents a logical OR operation between conditions.
    Or(Or),

    /// Represents a random condition.
    RandomCondition(RandomCondition),
}

impl TryFrom<ConditionSet> for Vec<ConditionSet> {
    type Error = ConditionError;

    fn try_from(value: ConditionSet) -> Result<Self, Self::Error> {
        Ok(match value {
            ConditionSet::And(and) => and.conditions,
            ConditionSet::Or(or) => or.conditions,
            _ => return Err(ConditionError::CastError),
        })
    }
}

/// Represents an error that can occur while working with conditions.
#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum ConditionError {
    /// Error indicating failure to cast to Vec.
    #[error("Only And or Or can be converted to Vec.")]
    CastError,
}
