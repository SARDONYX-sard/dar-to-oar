use oar_values::{Direction, PluginValue, StaticValue, WeaponType};

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
    /// `CurrentGameTimeLessThan(float)`
    CurrentGameTimeLessThan { value: StaticValue },

    /// `CurrentWeather(plugin)`
    CurrentWeather { weather: PluginValue<'input> },

    /// `IsClass(plugin)`
    IsClass { class: PluginValue<'input> },

    /// `IsCombatStyle(plugin)`
    IsCombatStyle { combat_style: PluginValue<'input> },

    // ---------------- Actor ----------------
    /// `IsActorValueEqualTo(actor_value, Number<'input>)`
    IsActorValueEqualTo {
        actor_value: StaticValue,
        value: StaticValue,
    },

    /// `IsActorValueLessThan(actor_value, Number<'input>)`
    IsActorValueLessThan {
        actor_value: StaticValue,
        value: StaticValue,
    },

    /// `IsActorValueBaseLessThan(actor_value, Number<'input>)`
    IsActorValueBaseLessThan {
        actor_value: StaticValue,
        value: StaticValue,
    },

    /// `IsActorValueMaxEqualTo(actor_value, Number<'input>)`
    IsActorValueMaxEqualTo {
        actor_value: StaticValue,
        value: StaticValue,
    },

    /// `IsActorValueMaxLessThan(actor_value, Number<'input>)`
    IsActorValueMaxLessThan {
        actor_value: StaticValue,
        value: StaticValue,
    },

    /// `IsActorValuePercentageEqualTo(actor_value, Number<'input>)`
    IsActorValuePercentageEqualTo {
        actor_value: StaticValue,
        value: StaticValue,
    },

    /// `IsActorValuePercentageLessThan(actor_value, Number<'input>)`
    IsActorValuePercentageLessThan {
        actor_value: StaticValue,
        value: StaticValue,
    },

    /// `IsActorBase(plugin)`
    IsActorBase { actor_base: PluginValue<'input> },

    // ---------------- Equipped ----------------
    /// `IsEquippedRight(plugin)`
    /// `IsEquippedLeft(plugin)`
    IsEquipped {
        form: PluginValue<'input>,
        hand_type: HandType,
    },

    /// - `IsEquippedRightType(Number<'input>)`
    /// - `IsEquippedLeftType(Number<'input>)`
    IsEquippedType {
        weapon_type: WeaponType,
        hand_type: HandType,
    },

    /// - `IsEquippedRightHasKeyword(keyword)`
    /// - `IsEquippedLeftHasKeyword(keyword)`
    IsEquippedHasKeyword {
        keyword: PluginValue<'input>,
        hand_type: HandType,
    },

    /// `IsEquippedShout(plugin)`
    IsEquippedShout { shout: PluginValue<'input> },

    // ---------------- Faction / Location ----------------
    /// `IsInFaction(plugin)`
    IsInFaction { faction: PluginValue<'input> },

    /// `IsFactionRankEqualTo(plugin, Number<'input>)`
    IsFactionRankEqualTo {
        faction: PluginValue<'input>,
        rank: StaticValue,
    },

    /// `IsFactionRankLessThan(plugin, Number<'input>)`
    IsFactionRankLessThan {
        faction: PluginValue<'input>,
        rank: StaticValue,
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
    /// `Random(Number<'input>)`
    Random { value: StaticValue },

    /// `ValueEqualTo(Number, Number)`
    ValueEqualTo { lhs: StaticValue, rhs: StaticValue },

    /// `ValueLessThan(Number, Number)`
    ValueLessThan { lhs: StaticValue, rhs: StaticValue },

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

#[derive(Debug, Clone, PartialEq)]
pub enum HandType {
    Left,
    Right,
}
