use super::values::{KeywordValue, NumericValue, PluginValue, RandomValue, TypeValue, ValueSet};
use serde::{Deserialize, Serialize};

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    *t == Default::default()
}

const REQUIRED_VERSION: &str = "1.0.0.0";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Condition {
    pub required_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Condition name (e.g. IsWornHasKeyword)
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub negated: bool,
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            required_version: REQUIRED_VERSION.to_owned(),
            condition: None,
            negated: false,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct CompareValues {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(rename = "Value A")]
    pub value_a: ValueSet,
    #[serde(rename = "Comparison")]
    /// == | != | > | >= | < | <=
    pub comparison: String,
    #[serde(rename = "Value B")]
    pub value_b: ValueSet,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct CurrentGameTime {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct CurrentWeather {
    #[serde(flatten)]
    pub condition: Condition,
    pub weather: Option<PluginValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct FactionRank {
    #[serde(flatten)]
    pub condition: Condition,
    pub faction: Option<PluginValue>,
    pub comparison: Option<String>,
    #[serde(rename = "Numeric value")]
    pub numeric_value: Option<NumericValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct HasKeyword {
    #[serde(flatten)]
    pub condition: Condition,
    pub keyword: Option<KeywordValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct HasMagicEffect {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(rename = "Magic effect")]
    pub magic_effect: PluginValue,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct HasMagicEffectWithKeyword {
    #[serde(flatten)]
    pub condition: Condition,
    pub keyword: Option<KeywordValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct HasPerk {
    #[serde(flatten)]
    pub condition: Condition,
    pub perk: Option<PluginValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct HasRefType {
    #[serde(flatten)]
    pub condition: Condition,
    pub location_ref_type: Option<PluginValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct HasSpell {
    #[serde(flatten)]
    pub condition: Condition,
    pub spell: Option<PluginValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct IsActorBase {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(rename = "Actor base")]
    pub actor_base: Option<PluginValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct IsClass {
    #[serde(flatten)]
    pub condition: Condition,
    pub class: Option<PluginValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct IsCombatStyle {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(rename = "Combat style")]
    pub combat_style: Option<PluginValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct IsEquipped {
    #[serde(flatten)]
    pub condition: Condition,
    pub form: Option<PluginValue>,
    #[serde(default)]
    #[serde(rename = "Left hand")]
    pub left_hand: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct IsEquippedHasKeyword {
    #[serde(flatten)]
    pub condition: Condition,
    pub keyword: Option<KeywordValue>,
    #[serde(default)]
    #[serde(rename = "Left hand")]
    pub left_hand: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct IsEquippedLeft {
    #[serde(flatten)]
    pub condition: Condition,
    pub form: Option<PluginValue>,
}

impl Default for IsEquippedLeft {
    fn default() -> Self {
        Self {
            condition: Condition {
                condition: Some(String::from("IsEquippedLeft")),
                ..Default::default()
            },
            form: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct IsEquippedRight {
    #[serde(flatten)]
    pub condition: Condition,
    pub form: Option<PluginValue>,
}

impl Default for IsEquippedRight {
    fn default() -> Self {
        Self {
            condition: Condition {
                condition: Some(String::from("IsEquippedRight")),
                ..Default::default()
            },
            form: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct IsEquippedShout {
    #[serde(flatten)]
    pub condition: Condition,
    pub shout: Option<PluginValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct IsEquippedType {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(rename = "Type")]
    pub type_value: Option<TypeValue>,
    #[serde(default)]
    #[serde(rename = "Left hand")]
    pub left_hand: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct IsInFaction {
    #[serde(flatten)]
    pub condition: Condition,
    pub faction: Option<PluginValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct IsInLocation {
    #[serde(flatten)]
    pub condition: Condition,
    pub location: Option<PluginValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct IsParentCell {
    #[serde(flatten)]
    pub condition: Condition,
    pub cell: Option<PluginValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct IsRace {
    #[serde(flatten)]
    pub condition: Condition,
    pub race: Option<PluginValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct IsVoiceType {
    #[serde(flatten)]
    pub condition: Condition,
    #[serde(rename = "Voice type")]
    pub voice_type: Option<PluginValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct IsWorldSpace {
    #[serde(flatten)]
    pub condition: Condition,
    pub worldspace: Option<PluginValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct IsWorn {
    #[serde(flatten)]
    pub condition: Condition,
    pub form: Option<PluginValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct IsWornHasKeyword {
    #[serde(flatten)]
    pub condition: Condition,
    pub keyword: Option<KeywordValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct NumericComparison {
    #[serde(flatten)]
    pub condition: Condition,
    pub comparison: Option<String>,
    #[serde(rename = "Numeric value")]
    pub numeric_value: Option<NumericValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Or {
    #[serde(rename = "condition")]
    pub condition: String,
    #[serde(rename = "conditions")]
    pub conditions: Vec<ConditionSet>,
}

impl Default for Or {
    fn default() -> Self {
        Self {
            condition: String::from("OR"),
            conditions: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct RandomCondition {
    #[serde(flatten)]
    pub condition: Condition,
    pub comparison: Option<String>,
    #[serde(rename = "Random value")]
    pub random_value: Option<RandomValue>,
    #[serde(rename = "Numeric value")]
    pub numeric_value: Option<NumericValue>,
    pub keep_random_results_on_loop: bool,
}

impl Default for RandomCondition {
    fn default() -> Self {
        Self {
            condition: Default::default(),
            comparison: Default::default(),
            random_value: Default::default(),
            numeric_value: Default::default(),
            keep_random_results_on_loop: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum ConditionSet {
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
    NumericComparison(NumericComparison),
    Or(Or),
    RandomCondition(RandomCondition),
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::to_string;

    #[test]
    fn test_serde_compare_values() {
        let compare_values = CompareValues {
            value_a: ValueSet::NumericValue(NumericValue { value: 42.0 }),
            comparison: String::from("=="),
            value_b: ValueSet::NumericValue(NumericValue { value: 42.0 }),
            condition: Condition::default(),
        };

        let expected =
            r#"{"requiredVersion":"1.0.0.0","Value A":42.0,"Comparison":"<","Value B":42.0}"#;
        let serialized = to_string(&compare_values).unwrap();
        assert_eq!(expected, serialized);

        let deserialized: CompareValues = serde_json::from_str(expected).unwrap();
        assert_eq!(compare_values, deserialized);
    }
}
