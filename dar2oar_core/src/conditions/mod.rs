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
use serde::{Deserialize, Serialize};

pub(super) fn is_false(t: &bool) -> bool {
    *t == false
}

/// Generate structures that have only condition, Comparison and NumericValue
macro_rules! gen_cmp_num_struct {
    ($($(#[$attr:meta])* $name:ident),+ $(,)?) => {
      $(
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct $name {
            pub condition: String,
            #[serde(default = "default_required_version")]
            #[serde(rename = "requiredVersion")]
            pub required_version: String,
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

/// generate structures that have only condition and PluginValue
#[macro_export]
macro_rules! gen_one_plugin_struct {
    ($($(#[$attr:meta])* $name:ident, $field:ident => $rename_field:literal),+ $(,)?) => {
        $(
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct $name {
            pub condition: String,
            #[serde(default = "default_required_version")]
            #[serde(rename = "requiredVersion")]
            pub required_version: String,
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
  IsEquippedLeft, form => "Form",  // DAR only function
  IsEquippedRight, form => "Form", // DAR only function
  IsEquippedShout, shout => "Shout",
  IsInFaction, faction => "Faction",
  IsInLocation, location => "Location",
  IsParentCell, cell => "Cell",
  IsRace, race => "Race",
  IsVoiceType, voice_type => "Voice type",
  IsWorldSpace, world_space => "WorldSpace",
  IsWorn, form => "Form",
);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConditionSet {
    And(And),
    Condition(Condition),
    CompareValues(CompareValues),
    CurrentGameTime(CurrentGameTime),
    CurrentWeather(CurrentWeather),
    FactionRank(FactionRank),
    HasKeyword(HasKeyword),
    HasMagicEffect(HasMagicEffect),
    HasMagicEffectWithKeyword(HasMagicEffectWithKeyword),
    HasPerk(HasPerk),
    HasRefType(HasRefType),
    HasSpell(HasSpell),
    IsActorBase(IsActorBase),
    IsClass(IsClass),
    IsCombatStyle(IsCombatStyle),
    IsEquipped(IsEquipped),
    IsEquippedHasKeyword(IsEquippedHasKeyword),
    IsEquippedLeft(IsEquippedLeft),
    IsEquippedRight(IsEquippedRight),
    IsEquippedShout(IsEquippedShout),
    IsEquippedType(IsEquippedType),
    IsInFaction(IsInFaction),
    IsInLocation(IsInLocation),
    IsParentCell(IsParentCell),
    IsRace(IsRace),
    IsVoiceType(IsVoiceType),
    IsWorldSpace(IsWorldSpace),
    IsWorn(IsWorn),
    IsWornHasKeyword(IsWornHasKeyword),
    IsDirectionMovement(IsMovementDirection),
    Level(Level),
    Or(Or),
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

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum ConditionError {
    // Couldn't cast
    #[error("Only And or Or can be converted to Vec.")]
    CastError,
}
