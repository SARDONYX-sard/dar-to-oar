mod and;
mod compare_values;
mod condition;
mod current_game_time;
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
mod level;
mod or;
mod random;

use crate::converter::values::PluginValue;
use serde::{Deserialize, Serialize};

pub use self::{
    and::And, compare_values::CompareValues, condition::Condition,
    current_game_time::CurrentGameTime, current_weather::CurrentWeather, faction_rank::FactionRank,
    has_keyword::HasKeyword, has_magic_effect::HasMagicEffect,
    has_magic_effect_with_keyword::HasMagicEffectWithKeyword, has_perk::HasPerk,
    has_ref_type::HasRefType, is_equipped::IsEquipped,
    is_equipped_has_keyword::IsEquippedHasKeyword, is_equipped_type::IsEquippedType,
    is_movement_direction::IsMovementDirection, is_worn_has_keyword::IsWornHasKeyword,
    level::Level, or::Or, random::RandomCondition,
};

pub(super) fn is_false(t: &bool) -> bool {
    *t == false
}

/// Macro for automatic generation of structures that have only condition and PluginValue
#[macro_export]
macro_rules! create_two_field_condition {
    ($($name:ident, $field:ident => $rename_field:literal),+ $(,)?) => {
        $(
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct $name {
            #[serde(flatten)]
            pub condition: Condition,
            #[serde(rename = $rename_field)]
            #[serde(default)]
            pub $field: PluginValue,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    condition: Condition::new(stringify!($name)),
                    $field: Default::default(),
                }
            }
        }
        )+
    };
}

create_two_field_condition!(
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
