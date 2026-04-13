#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FnKind {
    /// `CurrentGameTimeLessThan`
    CurrentGameTimeLessThan,
    /// `CurrentWeather`
    CurrentWeather,
    /// `IsClass`
    IsClass,
    /// `IsCombatStyle`
    IsCombatStyle,

    /// `IsActorValueEqualTo`
    IsActorValueEqualTo,
    /// `IsActorValueLessThan`
    IsActorValueLessThan,
    /// `IsActorValueBaseLessThan`
    IsActorValueBaseLessThan,
    /// `IsActorValueMaxEqualTo`
    IsActorValueMaxEqualTo,
    /// `IsActorValueMaxLessThan`
    IsActorValueMaxLessThan,
    /// `IsActorValuePercentageEqualTo`
    IsActorValuePercentageEqualTo,
    /// `IsActorValuePercentageLessThan`
    IsActorValuePercentageLessThan,
    /// `IsActorBase`
    IsActorBase,

    /// `IsEquippedRight`
    IsEquippedRight,
    /// `IsEquippedLeft`
    IsEquippedLeft,
    /// `IsEquippedRightType`
    IsEquippedRightType,
    /// `IsEquippedLeftType`
    IsEquippedLeftType,
    /// `IsEquippedRightHasKeyword`
    IsEquippedRightHasKeyword,
    /// `IsEquippedLeftHasKeyword`
    IsEquippedLeftHasKeyword,
    /// `IsEquippedShout`
    IsEquippedShout,

    /// `IsInFaction`
    IsInFaction,
    /// `IsFactionRankEqualTo`
    IsFactionRankEqualTo,
    /// `IsFactionRankLessThan`
    IsFactionRankLessThan,
    /// `IsInLocation`
    IsInLocation,
    /// `IsLevelLessThan`
    IsLevelLessThan,
    /// `IsParentCell`
    IsParentCell,
    /// `IsMovementDirection`
    IsMovementDirection,
    /// `IsRace`
    IsRace,
    /// `IsVoiceType`
    IsVoiceType,
    /// `IsWorldSpace`
    IsWorldSpace,
    /// `IsWorn`
    IsWorn,
    /// `IsWornHasKeyword`
    IsWornHasKeyword,

    /// `HasKeyword`
    HasKeyword,
    /// `HasPerk`
    HasPerk,
    /// `HasSpell`
    HasSpell,
    /// `HasMagicEffect`
    HasMagicEffect,
    /// `HasMagicEffectWithKeyword`
    HasMagicEffectWithKeyword,
    /// `HasRefType`
    HasRefType,

    /// `Random`
    Random,
    /// `ValueEqualTo`
    ValueEqualTo,
    /// `ValueLessThan`
    ValueLessThan,

    /// `IsFemale`
    IsFemale,
    /// `IsChild`
    IsChild,
    /// `IsPlayerTeammate`
    IsPlayerTeammate,
    /// `IsInInterior`
    IsInInterior,
    /// `IsUnique`
    IsUnique,
    /// `IsAttacking`
    IsAttacking,
    /// `IsRunning`
    IsRunning,
    /// `IsSneaking`
    IsSneaking,
    /// `IsSprinting`
    IsSprinting,
    /// `IsInAir`
    IsInAir,
    /// `IsInCombat`
    IsInCombat,
    /// `IsWeaponDrawn`
    IsWeaponDrawn,
}

impl FnKind {
    /// Ignore ascii case parse.
    ///
    /// # Errors
    /// Invalid condition name.
    #[expect(clippy::cognitive_complexity)]
    pub fn parse(s: &str) -> Result<Self, String> {
        Ok(match s {
            _ if s.eq_ignore_ascii_case("CurrentGameTimeLessThan") => Self::CurrentGameTimeLessThan,
            _ if s.eq_ignore_ascii_case("CurrentWeather") => Self::CurrentWeather,
            _ if s.eq_ignore_ascii_case("IsClass") => Self::IsClass,
            _ if s.eq_ignore_ascii_case("IsCombatStyle") => Self::IsCombatStyle,

            // Actor value
            _ if s.eq_ignore_ascii_case("IsActorValueEqualTo") => Self::IsActorValueEqualTo,
            _ if s.eq_ignore_ascii_case("IsActorValueLessThan") => Self::IsActorValueLessThan,
            _ if s.eq_ignore_ascii_case("IsActorValueBaseLessThan") => {
                Self::IsActorValueBaseLessThan
            }
            _ if s.eq_ignore_ascii_case("IsActorValueMaxEqualTo") => Self::IsActorValueMaxEqualTo,
            _ if s.eq_ignore_ascii_case("IsActorValueMaxLessThan") => Self::IsActorValueMaxLessThan,
            _ if s.eq_ignore_ascii_case("IsActorValuePercentageEqualTo") => {
                Self::IsActorValuePercentageEqualTo
            }
            _ if s.eq_ignore_ascii_case("IsActorValuePercentageLessThan") => {
                Self::IsActorValuePercentageLessThan
            }
            _ if s.eq_ignore_ascii_case("IsActorBase") => Self::IsActorBase,

            // IsEquipped
            _ if s.eq_ignore_ascii_case("IsEquippedRight") => Self::IsEquippedRight,
            _ if s.eq_ignore_ascii_case("IsEquippedLeft") => Self::IsEquippedLeft,
            _ if s.eq_ignore_ascii_case("IsEquippedRightType") => Self::IsEquippedRightType,
            _ if s.eq_ignore_ascii_case("IsEquippedLeftType") => Self::IsEquippedLeftType,
            _ if s.eq_ignore_ascii_case("IsEquippedRightHasKeyword") => {
                Self::IsEquippedRightHasKeyword
            }
            _ if s.eq_ignore_ascii_case("IsEquippedLeftHasKeyword") => {
                Self::IsEquippedLeftHasKeyword
            }
            _ if s.eq_ignore_ascii_case("IsEquippedShout") => Self::IsEquippedShout,

            // Faction / Location
            _ if s.eq_ignore_ascii_case("IsInFaction") => Self::IsInFaction,
            _ if s.eq_ignore_ascii_case("IsFactionRankEqualTo") => Self::IsFactionRankEqualTo,
            _ if s.eq_ignore_ascii_case("IsFactionRankLessThan") => Self::IsFactionRankLessThan,
            _ if s.eq_ignore_ascii_case("IsInLocation") => Self::IsInLocation,

            _ if s.eq_ignore_ascii_case("IsLevelLessThan") => Self::IsLevelLessThan,
            _ if s.eq_ignore_ascii_case("IsParentCell") => Self::IsParentCell,
            _ if s.eq_ignore_ascii_case("IsMovementDirection") => Self::IsMovementDirection,
            _ if s.eq_ignore_ascii_case("IsRace") => Self::IsRace,
            _ if s.eq_ignore_ascii_case("IsVoiceType") => Self::IsVoiceType,
            _ if s.eq_ignore_ascii_case("IsWorldSpace") => Self::IsWorldSpace,
            _ if s.eq_ignore_ascii_case("IsWorn") => Self::IsWorn,
            _ if s.eq_ignore_ascii_case("IsWornHasKeyword") => Self::IsWornHasKeyword,

            // Has
            _ if s.eq_ignore_ascii_case("HasKeyword") => Self::HasKeyword,
            _ if s.eq_ignore_ascii_case("HasPerk") => Self::HasPerk,
            _ if s.eq_ignore_ascii_case("HasSpell") => Self::HasSpell,
            _ if s.eq_ignore_ascii_case("HasMagicEffect") => Self::HasMagicEffect,
            _ if s.eq_ignore_ascii_case("HasMagicEffectWithKeyword") => {
                Self::HasMagicEffectWithKeyword
            }
            _ if s.eq_ignore_ascii_case("HasRefType") => Self::HasRefType,

            _ if s.eq_ignore_ascii_case("Random") => Self::Random,
            _ if s.eq_ignore_ascii_case("ValueEqualTo") => Self::ValueEqualTo,
            _ if s.eq_ignore_ascii_case("ValueLessThan") => Self::ValueLessThan,

            // no-arg functions
            _ if s.eq_ignore_ascii_case("IsFemale") => Self::IsFemale,
            _ if s.eq_ignore_ascii_case("IsChild") => Self::IsChild,
            _ if s.eq_ignore_ascii_case("IsPlayerTeammate") => Self::IsPlayerTeammate,
            _ if s.eq_ignore_ascii_case("IsInInterior") => Self::IsInInterior,
            _ if s.eq_ignore_ascii_case("IsUnique") => Self::IsUnique,
            _ if s.eq_ignore_ascii_case("IsAttacking") => Self::IsAttacking,
            _ if s.eq_ignore_ascii_case("IsRunning") => Self::IsRunning,
            _ if s.eq_ignore_ascii_case("IsSneaking") => Self::IsSneaking,
            _ if s.eq_ignore_ascii_case("IsSprinting") => Self::IsSprinting,
            _ if s.eq_ignore_ascii_case("IsInAir") => Self::IsInAir,
            _ if s.eq_ignore_ascii_case("IsInCombat") => Self::IsInCombat,
            _ if s.eq_ignore_ascii_case("IsWeaponDrawn") => Self::IsWeaponDrawn,

            invalid => {
                return Err(format!(
                    "Invalid Condition: {invalid}. Expected one of [{}]",
                    Self::expected_str()
                ));
            }
        })
    }

    /// To str.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::CurrentGameTimeLessThan => "CurrentGameTimeLessThan",
            Self::CurrentWeather => "CurrentWeather",
            Self::IsClass => "IsClass",
            Self::IsCombatStyle => "IsCombatStyle",

            Self::IsActorValueEqualTo => "IsActorValueEqualTo",
            Self::IsActorValueLessThan => "IsActorValueLessThan",
            Self::IsActorValueBaseLessThan => "IsActorValueBaseLessThan",
            Self::IsActorValueMaxEqualTo => "IsActorValueMaxEqualTo",
            Self::IsActorValueMaxLessThan => "IsActorValueMaxLessThan",
            Self::IsActorValuePercentageEqualTo => "IsActorValuePercentageEqualTo",
            Self::IsActorValuePercentageLessThan => "IsActorValuePercentageLessThan",
            Self::IsActorBase => "IsActorBase",

            Self::IsEquippedRight => "IsEquippedRight",
            Self::IsEquippedLeft => "IsEquippedLeft",
            Self::IsEquippedRightType => "IsEquippedRightType",
            Self::IsEquippedLeftType => "IsEquippedLeftType",
            Self::IsEquippedRightHasKeyword => "IsEquippedRightHasKeyword",
            Self::IsEquippedLeftHasKeyword => "IsEquippedLeftHasKeyword",
            Self::IsEquippedShout => "IsEquippedShout",

            Self::IsInFaction => "IsInFaction",
            Self::IsFactionRankEqualTo => "IsFactionRankEqualTo",
            Self::IsFactionRankLessThan => "IsFactionRankLessThan",
            Self::IsInLocation => "IsInLocation",

            Self::IsLevelLessThan => "IsLevelLessThan",
            Self::IsParentCell => "IsParentCell",
            Self::IsMovementDirection => "IsMovementDirection",
            Self::IsRace => "IsRace",
            Self::IsVoiceType => "IsVoiceType",
            Self::IsWorldSpace => "IsWorldSpace",
            Self::IsWorn => "IsWorn",
            Self::IsWornHasKeyword => "IsWornHasKeyword",

            Self::HasKeyword => "HasKeyword",
            Self::HasPerk => "HasPerk",
            Self::HasSpell => "HasSpell",
            Self::HasMagicEffect => "HasMagicEffect",
            Self::HasMagicEffectWithKeyword => "HasMagicEffectWithKeyword",
            Self::HasRefType => "HasRefType",

            Self::Random => "Random",
            Self::ValueEqualTo => "ValueEqualTo",
            Self::ValueLessThan => "ValueLessThan",

            Self::IsFemale => "IsFemale",
            Self::IsChild => "IsChild",
            Self::IsPlayerTeammate => "IsPlayerTeammate",
            Self::IsInInterior => "IsInInterior",
            Self::IsUnique => "IsUnique",
            Self::IsAttacking => "IsAttacking",
            Self::IsRunning => "IsRunning",
            Self::IsSneaking => "IsSneaking",
            Self::IsSprinting => "IsSprinting",
            Self::IsInAir => "IsInAir",
            Self::IsInCombat => "IsInCombat",
            Self::IsWeaponDrawn => "IsWeaponDrawn",
        }
    }

    pub(crate) fn expected_str() -> &'static str {
        use std::sync::OnceLock;

        static EXPECTED: OnceLock<String> = OnceLock::new();

        EXPECTED.get_or_init(|| {
            Self::ALL_FN_KIND
                .iter()
                .map(|kind| kind.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        })
    }

    /// For Error reporting
    const ALL_FN_KIND: [Self; 52] = [
        Self::CurrentGameTimeLessThan,
        Self::CurrentWeather,
        Self::IsClass,
        Self::IsCombatStyle,
        Self::IsActorValueEqualTo,
        Self::IsActorValueLessThan,
        Self::IsActorValueBaseLessThan,
        Self::IsActorValueMaxEqualTo,
        Self::IsActorValueMaxLessThan,
        Self::IsActorValuePercentageEqualTo,
        Self::IsActorValuePercentageLessThan,
        Self::IsActorBase,
        Self::IsEquippedRight,
        Self::IsEquippedLeft,
        Self::IsEquippedRightType,
        Self::IsEquippedLeftType,
        Self::IsEquippedRightHasKeyword,
        Self::IsEquippedLeftHasKeyword,
        Self::IsEquippedShout,
        Self::IsInFaction,
        Self::IsFactionRankEqualTo,
        Self::IsFactionRankLessThan,
        Self::IsInLocation,
        Self::IsLevelLessThan,
        Self::IsParentCell,
        Self::IsMovementDirection,
        Self::IsRace,
        Self::IsVoiceType,
        Self::IsWorldSpace,
        Self::IsWorn,
        Self::IsWornHasKeyword,
        Self::HasKeyword,
        Self::HasPerk,
        Self::HasSpell,
        Self::HasMagicEffect,
        Self::HasMagicEffectWithKeyword,
        Self::HasRefType,
        Self::Random,
        Self::ValueEqualTo,
        Self::ValueLessThan,
        Self::IsFemale,
        Self::IsChild,
        Self::IsPlayerTeammate,
        Self::IsInInterior,
        Self::IsUnique,
        Self::IsAttacking,
        Self::IsRunning,
        Self::IsSneaking,
        Self::IsSprinting,
        Self::IsInAir,
        Self::IsInCombat,
        Self::IsWeaponDrawn,
    ];
}
