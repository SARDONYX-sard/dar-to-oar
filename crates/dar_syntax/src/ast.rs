//! DAR abstract syntax tree
use oar_values::{
    ActorValue, Direction, FormValue, NumericValue, PluginValue, StaticValue, WeaponType,
};

/// Represents a top-level condition, which can be an AND combination, OR combination, or a leaf expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Dar<'input> {
    /// Represents an AND combination of multiple conditions.
    And(Vec<Self>),
    /// Represents an OR combination of multiple conditions.
    Or(Vec<Self>),
    /// Represents a leaf expression within the condition hierarchy.
    Exp(Expression<'input>),
}
impl Dar<'_> {
    /// push to inner vec.
    pub(crate) fn push(&mut self, expression: Self) -> Result<(), &'static str> {
        match self {
            Self::And(inner) | Self::Or(inner) => {
                inner.push(expression);
                Ok(())
            }
            Self::Exp(_) => Err("Expression cannot push"),
        }
    }
}

/// DAR One line representation
#[derive(Debug, Clone, PartialEq)]
pub struct Expression<'input> {
    /// function arguments
    pub function: Function<'input>,

    /// not condition
    pub negated: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Function<'input> {
    /// `CurrentGameTimeLessThan(time: float)`
    CurrentGameTimeLessThan { value: StaticValue },

    /// `CurrentWeather(plugin)`
    CurrentWeather { weather: PluginValue<'input> },

    /// `IsClass(plugin)`
    IsClass { class: PluginValue<'input> },

    /// `IsCombatStyle(plugin)`
    IsCombatStyle { combat_style: PluginValue<'input> },

    // ---------------- Actor ----------------
    /// `IsActorValueEqualTo(actor_value, Number<'input>)`
    IsActorValueEqualTo(ActorArgs<'input>),

    /// `IsActorValueLessThan(actor_value, Number<'input>)`
    IsActorValueLessThan(ActorArgs<'input>),

    /// `IsActorValueBaseLessThan(actor_value, Number<'input>)`
    IsActorValueBaseLessThan(ActorArgs<'input>),

    /// `IsActorValueMaxEqualTo(actor_value, Number<'input>)`
    IsActorValueMaxEqualTo(ActorArgs<'input>),

    /// `IsActorValueMaxLessThan(actor_value, Number<'input>)`
    IsActorValueMaxLessThan(ActorArgs<'input>),

    /// `IsActorValuePercentageEqualTo(actor_value, Number<'input>)`
    IsActorValuePercentageEqualTo(ActorArgs<'input>),

    /// `IsActorValuePercentageLessThan(actor_value, Number<'input>)`
    IsActorValuePercentageLessThan(ActorArgs<'input>),

    /// `IsActorBase(plugin)`
    IsActorBase { actor_base: PluginValue<'input> },

    // ---------------- Equipped ----------------
    /// `IsEquippedRight(plugin)`
    /// `IsEquippedLeft(plugin)`
    IsEquipped {
        form: PluginValue<'input>,
        is_left: bool,
    },

    /// - `IsEquippedRightType(Number<'input>)`
    /// - `IsEquippedLeftType(Number<'input>)`
    IsEquippedType { value: WeaponType, is_left: bool },

    /// - `IsEquippedRightHasKeyword(keyword)`
    /// - `IsEquippedLeftHasKeyword(keyword)`
    IsEquippedHasKeyword {
        keyword: PluginValue<'input>,
        is_left: bool,
    },

    /// `IsEquippedShout(plugin)`
    IsEquippedShout { shout: PluginValue<'input> },

    // ---------------- Faction / Location ----------------
    /// `IsInFaction(plugin)`
    IsInFaction { faction: PluginValue<'input> },

    /// `IsFactionRankEqualTo(variable, plugin)`
    IsFactionRankEqualTo {
        rank: GlobalVariable<'input>,
        faction: PluginValue<'input>,
    },

    /// `IsFactionRankLessThan(variable, plugin)`
    IsFactionRankLessThan {
        rank: GlobalVariable<'input>,
        faction: PluginValue<'input>,
    },

    /// `IsInLocation(plugin)`
    IsInLocation { location: PluginValue<'input> },

    /// `IsLevelLessThan(Number<'input>)`
    IsLevelLessThan { level: StaticValue },

    /// `IsParentCell(plugin)`
    IsParentCell { cell: PluginValue<'input> },

    /// `IsMovementDirection(Number<'input>)`
    IsMovementDirection { direction: Direction },

    /// `IsRace(plugin)`
    IsRace { race: PluginValue<'input> },

    /// `IsVoiceType(plugin)`
    IsVoiceType { voice_type: PluginValue<'input> },

    /// `IsWorldSpace(plugin)`
    IsWorldSpace { world_space: PluginValue<'input> },

    /// `IsWorn(plugin)`
    IsWorn { form: PluginValue<'input> },

    /// `IsWornHasKeyword(keyword)`
    IsWornHasKeyword { keyword: PluginValue<'input> },

    // ---------------- Has ----------------
    /// `HasKeyword(keyword)`
    HasKeyword { keyword: PluginValue<'input> },

    /// `HasPerk(plugin)`
    HasPerk { perk: PluginValue<'input> },

    /// `HasSpell(plugin)`
    HasSpell { spell: PluginValue<'input> },

    /// `HasMagicEffect(plugin)`
    HasMagicEffect { magic_effect: PluginValue<'input> },

    /// `HasMagicEffectWithKeyword(keyword)`
    HasMagicEffectWithKeyword { keyword: PluginValue<'input> },

    /// `HasRefType(keyword)`
    HasRefType {
        location_ref_type: PluginValue<'input>,
    },

    // ---------------- Misc ----------------
    /// `Random(Number<'input>)`: 0..=1
    Random { value: StaticValue },

    /// `ValueEqualTo(GlobalVariable, GlobalVariable)`
    ValueEqualTo {
        value_a: GlobalVariable<'input>,
        value_b: GlobalVariable<'input>,
    },

    /// `ValueLessThan(GlobalVariable, GlobalVariable)`
    ValueLessThan {
        value_a: GlobalVariable<'input>,
        value_b: GlobalVariable<'input>,
    },

    // ---------------- No-arg ----------------
    /// `IsFemale()`
    IsFemale,

    /// `IsChild()`
    IsChild,

    /// `IsPlayerTeammate()`
    IsPlayerTeammate,

    /// `IsInInterior()`
    IsInInterior,

    /// `IsUnique()`
    IsUnique,

    /// `IsAttacking()`
    IsAttacking,

    /// `IsRunning()`
    IsRunning,

    /// `IsSneaking()`
    IsSneaking,

    /// `IsSprinting()`
    IsSprinting,

    /// `IsInAir()`
    IsInAir,

    /// `IsInCombat()`
    IsInCombat,

    /// `IsWeaponDrawn()`
    IsWeaponDrawn,
}

/// Plugin / Number
#[derive(Debug, Clone, PartialEq)]
pub enum GlobalVariable<'i> {
    /// e.g., `"Skyrim.esm" | 0x00007`
    Plugin(PluginValue<'i>),
    /// e.g., 1.0
    StaticValue(StaticValue),
}

impl<'i> From<GlobalVariable<'i>> for NumericValue<'i> {
    #[inline]
    fn from(value: GlobalVariable<'i>) -> Self {
        match value {
            GlobalVariable::Plugin(plugin_value) => {
                NumericValue::GlobalVariable(FormValue { form: plugin_value })
            }
            GlobalVariable::StaticValue(static_value) => NumericValue::StaticValue(static_value),
        }
    }
}

/// fn(Plugin, Number) / fn(Number, Plugin)
#[derive(Debug, Clone, PartialEq)]
pub enum ActorArgs<'i> {
    /// Plugin, Number
    PluginFirst {
        value_a: PluginValue<'i>,
        value_b: ActorValue,
    },
    /// Number, Plugin
    NumberFirst {
        value_a: ActorValue,
        value_b: PluginValue<'i>,
    },
}

impl<'i> ActorArgs<'i> {
    /// Cast to (value_a, value_b)
    #[inline]
    pub fn into_numeric_values(self) -> (NumericValue<'i>, NumericValue<'i>) {
        match self {
            ActorArgs::PluginFirst { value_a, value_b } => (
                NumericValue::GlobalVariable(FormValue { form: value_a }),
                NumericValue::ActorValue(value_b),
            ),
            ActorArgs::NumberFirst { value_a, value_b } => (
                NumericValue::ActorValue(value_a),
                NumericValue::GlobalVariable(FormValue { form: value_b }),
            ),
        }
    }
}
