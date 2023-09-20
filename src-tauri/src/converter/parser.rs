use super::conditions::{
    CompareValues, Condition, ConditionSet, CurrentWeather, FactionRank, HasKeyword,
    HasMagicEffect, HasMagicEffectWithKeyword, HasPerk, HasRefType, HasSpell, IsActorBase, IsClass,
    IsCombatStyle, IsEquipped, IsEquippedHasKeyword, IsEquippedType, IsInFaction, IsInLocation,
    IsParentCell, IsRace, IsVoiceType, IsWorldSpace, IsWorn, IsWornHasKeyword, NumericComparison,
    Or, RandomCondition,
};
use super::values::{
    ActorValue, KeywordValue, NumericValue, PluginValue, RandomValue, TypeValue, ValueSet,
};

fn get_plugin_value(condition: &str) -> Option<PluginValue> {
    let condition_split: Vec<&str> = condition.split('|').map(|s| s.trim()).collect();
    if condition_split.len() == 2 {
        Some(PluginValue {
            plugin_name: condition_split[0].replace("\"", ""),
            form_id: condition_split[1][2..].trim_start_matches('0').to_string(),
        })
    } else {
        None
    }
}

fn get_keyword_value(condition: &str) -> Option<KeywordValue> {
    match get_plugin_value(condition) {
        Some(plugin_value) => Some(KeywordValue {
            form: Some(plugin_value),
            ..Default::default()
        }),
        None => None,
    }
}

fn parse_condition(condition: &str, is_negated: bool) -> ConditionSet {
    let condition_set: Vec<&str> = condition
        .split(|c| c == '(' || c == ')')
        .filter(|s| !s.is_empty())
        .collect();

    let condition_name = condition_set[0];

    match condition_name {
        "ValueEqualTo" | "ValueLessThan" => {
            parse_compare(condition_name, condition_set[1], is_negated)
        }
        actor if condition_name.starts_with("IsActor") => {
            parse_actor(actor, condition_set[1], is_negated)
        }
        faction if condition_name.starts_with("IsFaction") => {
            parse_faction(faction, condition_set[1], is_negated)
        }
        equip if condition_name.starts_with("IsEquipped") => {
            parse_equip(equip, condition_set[1], is_negated)
        }
        "IsLevelLessThan" => ConditionSet::NumericComparison(NumericComparison {
            condition: Condition {
                condition: Some("Level".to_string()),
                negated: is_negated,
                ..Default::default()
            },
            comparison: Some("<".to_string()),
            numeric_value: Some(NumericValue {
                value: condition_set[1].parse().unwrap(),
            }),
        }),
        "CurrentWeather" => ConditionSet::CurrentWeather(CurrentWeather {
            condition: Condition {
                condition: Some(condition_name.to_string()),
                negated: is_negated,
                ..Default::default()
            },
            weather: get_plugin_value(condition_set[1]),
        }),
        "IsRace" => ConditionSet::IsRace(IsRace {
            condition: Condition {
                condition: Some(condition_name.to_string()),
                negated: is_negated,
                ..Default::default()
            },
            race: get_plugin_value(condition_set[1]),
        }),
        "IsClass" => ConditionSet::IsClass(IsClass {
            condition: Condition {
                condition: Some(condition_name.to_string()),
                negated: is_negated,
                ..Default::default()
            },
            class: get_plugin_value(condition_set[1]),
        }),
        "IsCombatStyle" => ConditionSet::IsCombatStyle(IsCombatStyle {
            condition: Condition {
                condition: Some(condition_name.to_string()),
                negated: is_negated,
                ..Default::default()
            },
            combat_style: get_plugin_value(condition_set[1]),
        }),
        "IsVoiceType" => ConditionSet::IsVoiceType(IsVoiceType {
            condition: Condition {
                condition: Some(condition_name.to_string()),
                negated: is_negated,
                ..Default::default()
            },
            voice_type: get_plugin_value(condition_set[1]),
        }),
        "IsParentCell" => ConditionSet::IsParentCell(IsParentCell {
            condition: Condition {
                condition: Some(condition_name.to_string()),
                negated: is_negated,
                ..Default::default()
            },
            cell: get_plugin_value(condition_set[1]),
        }),
        "IsWorldSpace" => ConditionSet::IsWorldSpace(IsWorldSpace {
            condition: Condition {
                condition: Some(condition_name.to_string()),
                negated: is_negated,
                ..Default::default()
            },
            worldspace: get_plugin_value(condition_set[1]),
        }),
        "IsMovementDirection" => ConditionSet::NumericComparison(NumericComparison {
            condition: Condition {
                condition: Some(condition_name.to_string()),
                negated: is_negated,
                ..Default::default()
            },
            comparison: None,
            numeric_value: Some(NumericValue {
                value: condition_set[1].parse().unwrap(),
            }),
        }),
        "IsInLocation" => ConditionSet::IsInLocation(IsInLocation {
            condition: Condition {
                condition: Some(condition_name.to_string()),
                negated: is_negated,
                ..Default::default()
            },
            location: get_plugin_value(condition_set[1]),
        }),
        "IsWorn" => ConditionSet::IsWorn(IsWorn {
            condition: Condition {
                condition: Some(condition_name.to_string()),
                negated: is_negated,
                ..Default::default()
            },
            form: get_plugin_value(condition_set[1]),
        }),
        "IsWornHasKeyword" => ConditionSet::IsWornHasKeyword(IsWornHasKeyword {
            condition: Condition {
                condition: Some(condition_name.to_string()),
                negated: is_negated,
                ..Default::default()
            },
            keyword: get_keyword_value(condition_set[1]),
        }),
        has_cond if condition_name.starts_with("Has") => {
            parse_has(has_cond, condition_set[1], is_negated)
        }
        "Random" => {
            let numeric_value = match condition_set[1].parse().ok() {
                Some(num) => Some(NumericValue { value: num }),
                None => None,
            };

            ConditionSet::RandomCondition(RandomCondition {
                condition: Condition {
                    condition: Some(condition_name.to_string()),
                    negated: is_negated,
                    ..Default::default()
                },
                comparison: Some("<=".to_string()),
                random_value: Some(RandomValue { min: 0.0, max: 1.0 }),
                numeric_value,
                ..Default::default()
            })
        }
        "CurrentGameTimeLessThan" => {
            let numeric_value = match condition_set[1].parse().ok() {
                Some(num) => Some(NumericValue { value: num }),
                None => None,
            };
            ConditionSet::NumericComparison(NumericComparison {
                condition: Condition {
                    condition: Some("CurrentGameTime".to_string()),
                    negated: is_negated,
                    ..Default::default()
                },
                comparison: Some("<".to_string()),
                numeric_value,
            })
        }
        _ => {
            log::warn!("Condition({condition_name}) has no explicit mapping.");

            ConditionSet::Condition(Condition {
                condition: Some("CurrentGameTime".to_string()),
                negated: is_negated,
                ..Default::default()
            })
        }
    }
}

/// condition_name: "ValueEqualTo" | "ValueLessThan"
fn parse_compare(condition_name: &str, arg1: &str, is_negated: bool) -> ConditionSet {
    let values: Vec<&str> = arg1.split(',').map(|s| s.trim()).collect();
    let value_a = get_plugin_value(values[0]).unwrap();
    let value_b = NumericValue {
        value: values[1].parse().unwrap(),
    };

    let create_compare = |comparison: &str| {
        ConditionSet::CompareValues(CompareValues {
            condition: Condition {
                condition: Some("CompareValues".to_string()),
                negated: is_negated,
                ..Default::default()
            },
            value_a: ValueSet::PluginValue(value_a),
            comparison: comparison.to_string(),
            value_b: ValueSet::NumericValue(value_b),
        })
    };

    match condition_name {
        "ValueEqualTo" => create_compare("=="),
        "ValueLessThan" => create_compare("<"),
        _ => unreachable!(),
    }
}

fn parse_actor(condition_name: &str, arg1: &str, is_negated: bool) -> ConditionSet {
    let values: Vec<&str> = arg1.split(',').map(|s| s.trim()).collect();
    let value_b = NumericValue {
        value: values[1].parse().unwrap(),
    };

    let create_actor_cond = |comparison: &str, actor_type: &str| {
        let value_a = ActorValue {
            actor_value: Some(values[0].parse().unwrap()),
            actor_value_type: actor_type.to_string(),
        };

        ConditionSet::CompareValues(CompareValues {
            condition: Condition {
                condition: Some("CompareValues".to_string()),
                negated: is_negated,
                ..Default::default()
            },
            value_a: ValueSet::ActorValue(value_a),
            comparison: comparison.to_string(),
            value_b: ValueSet::NumericValue(value_b),
        })
    };

    match condition_name {
        "IsActorValueEqualTo" => create_actor_cond("==", ""),
        "IsActorValueBaseLessThan" => create_actor_cond("<", "Base"),
        "IsActorValueMaxEqualTo" => create_actor_cond("==", "Max"),
        "IsActorValueMaxLessThan" => create_actor_cond("<", "Max"),
        "IsActorValuePercentageEqualTo" => create_actor_cond("==", "Percentage"),
        "IsActorValuePercentageLessThan" => create_actor_cond("<", "Percentage"),
        "IsActorBase" => ConditionSet::IsActorBase(IsActorBase {
            condition: Condition {
                condition: Some(condition_name.to_string()),
                negated: is_negated,
                ..Default::default()
            },
            actor_base: get_plugin_value(condition_name),
        }),
        _ => unreachable!(),
    }
}

fn parse_faction(condition_name: &str, arg1: &str, is_negated: bool) -> ConditionSet {
    let values: Vec<&str> = arg1.split(',').map(|s| s.trim()).collect();
    let numeric_value = match values[1].parse().ok() {
        Some(num) => Some(NumericValue { value: num }),
        None => None,
    };

    let create_cond = |comparison: &str| {
        ConditionSet::FactionRank(FactionRank {
            condition: Condition {
                condition: Some("FactionRank".to_string()),
                negated: is_negated,
                ..Default::default()
            },
            faction: get_plugin_value(values[0]),
            comparison: Some(comparison.to_string()),
            numeric_value,
        })
    };

    match condition_name {
        "IsInFaction" => ConditionSet::IsInFaction(IsInFaction {
            condition: Condition {
                condition: Some(condition_name.to_string()),
                negated: is_negated,
                ..Default::default()
            },
            faction: get_plugin_value(condition_name),
        }),
        "IsFactionRankEqualTo" => create_cond("=="),
        "IsFactionRankLessThan" => create_cond("<"),
        _ => unreachable!(),
    }
}

fn parse_equip(condition_name: &str, arg1: &str, is_negated: bool) -> ConditionSet {
    match condition_name {
        "IsEquippedRight" | "IsEquippedLeft" => ConditionSet::IsEquipped(IsEquipped {
            condition: Condition {
                condition: Some(condition_name.to_string()),
                negated: is_negated,
                ..Default::default()
            },
            form: get_plugin_value(arg1),
            ..Default::default()
        }),

        "IsEquippedRightType" | "IsEquippedLeftType" => {
            let type_value = match arg1.parse().ok() {
                Some(value) => Some(TypeValue { value }),
                None => None,
            };
            ConditionSet::IsEquippedType(IsEquippedType {
                condition: Condition {
                    condition: Some("IsEquippedType".to_string()),
                    negated: is_negated,
                    ..Default::default()
                },
                left_hand: condition_name == "IsEquippedLeftType",
                type_value,
                ..Default::default()
            })
        }
        "IsEquippedRightHasKeyword" | "IsEquippedLeftHasKeyword" => {
            ConditionSet::IsEquippedHasKeyword(IsEquippedHasKeyword {
                condition: Condition {
                    condition: Some("IsEquippedType".to_string()),
                    negated: is_negated,
                    ..Default::default()
                },
                left_hand: condition_name == "IsEquippedLeftHasKeyword",
                keyword: get_keyword_value(arg1),
            })
        }
        _ => unreachable!(),
    }
}

fn parse_has(condition_name: &str, arg1: &str, is_negated: bool) -> ConditionSet {
    let condition = Condition {
        condition: Some(condition_name.to_string()),
        negated: is_negated,
        ..Default::default()
    };

    match condition_name {
        "HasKeyword" => ConditionSet::HasKeyword(HasKeyword {
            condition,
            keyword: get_keyword_value(arg1),
        }),
        "HasPerk" => ConditionSet::HasPerk(HasPerk {
            condition,
            perk: get_plugin_value(arg1),
        }),
        "HasSpell" => ConditionSet::HasSpell(HasSpell {
            condition,
            spell: get_plugin_value(arg1),
        }),
        "HasMagicEffect" => ConditionSet::HasMagicEffect(HasMagicEffect {
            condition,
            magic_effect: get_plugin_value(arg1).expect("Not found plugin argument"),
        }),
        "HasMagicEffectWithKeyword" => {
            ConditionSet::HasMagicEffectWithKeyword(HasMagicEffectWithKeyword {
                condition,
                keyword: get_keyword_value(arg1),
            })
        }
        "HasRefType" => ConditionSet::HasRefType(HasRefType {
            condition,
            location_ref_type: get_plugin_value(arg1),
        }),
        _ => unreachable!(),
    }
}

pub(crate) fn parse_conditions(conditions: &[&str]) -> Vec<ConditionSet> {
    let mut result: Vec<ConditionSet> = Vec::new();
    let mut or_conditions: Option<Vec<ConditionSet>> = None;
    let mut or: Option<Or> = None;
    #[allow(unused_assignments)]
    let mut in_or_loop = false;

    for condition in conditions {
        let cleaned = condition.replace(" ", "");
        let mut is_negated = false;

        if cleaned.ends_with("OR") {
            in_or_loop = true;
            if or.is_none() && or_conditions.is_none() {
                or_conditions = Some(Vec::new());
                or = Some(Or::default());
                result.push(ConditionSet::Or(or.as_ref().unwrap().clone()));
            }
        } else {
            in_or_loop = false;
        }

        if cleaned.starts_with("NOT") {
            is_negated = true;
        }

        if let Some(or_ref) = or.as_mut() {
            if let Some(ref mut or_conditions) = or_conditions {
                or_conditions.push(parse_condition(
                    cleaned.trim_start_matches("NOT"),
                    is_negated,
                ));
                if !in_or_loop {
                    or_ref.conditions = or_conditions.clone();
                    or_conditions.clear();
                    or = None;
                }
            }
        } else {
            result.push(parse_condition(
                cleaned.trim_start_matches("NOT"),
                is_negated,
            ));
        }
    }

    result
}
