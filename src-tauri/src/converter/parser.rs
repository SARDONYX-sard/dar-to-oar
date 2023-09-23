use super::conditions::{
    CompareValues, Condition, ConditionSet, CurrentGameTime, CurrentWeather, FactionRank,
    HasKeyword, HasMagicEffect, HasMagicEffectWithKeyword, HasPerk, HasRefType, HasSpell,
    IsActorBase, IsClass, IsCombatStyle, IsEquipped, IsEquippedHasKeyword, IsEquippedType,
    IsInFaction, IsInLocation, IsMovementDirection, IsParentCell, IsRace, IsVoiceType,
    IsWorldSpace, IsWorn, IsWornHasKeyword, Level, Or, RandomCondition,
};
use super::values::{
    ActorValue, Cmp, Direction, DirectionValue, FormValue, Keyword, NumericValue, PluginValue,
    StaticValue, TypeValue, WeaponType,
};

fn get_plugin_value(condition: &str) -> Option<PluginValue> {
    let condition_split: Vec<&str> = condition.split('|').map(|s| s.trim()).collect();
    if condition_split.len() == 2 {
        Some(PluginValue {
            plugin_name: condition_split[0].replace("\"", ""),
            form_id: condition_split[1][2..].trim_start_matches('0').into(),
        })
    } else {
        None
    }
}

fn get_keyword_value(condition: &str) -> Option<Keyword> {
    match get_plugin_value(condition) {
        Some(form) => Some(Keyword::Form(FormValue {
            form,
            ..Default::default()
        })),
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
        "IsLevelLessThan" => ConditionSet::Level(Level {
            comparison: Cmp::Lt,
            numeric_value: NumericValue::StaticValue(StaticValue {
                value: condition_set[1].parse().unwrap(),
            }),
            ..Default::default()
        }),
        "CurrentWeather" => ConditionSet::CurrentWeather(CurrentWeather {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            weather: get_plugin_value(condition_set[1]).unwrap_or_default(),
        }),
        "IsRace" => ConditionSet::IsRace(IsRace {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            race: get_plugin_value(condition_set[1]).unwrap_or_default(),
        }),
        "IsClass" => ConditionSet::IsClass(IsClass {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            class: get_plugin_value(condition_set[1]).unwrap_or_default(),
        }),
        "IsCombatStyle" => ConditionSet::IsCombatStyle(IsCombatStyle {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            combat_style: get_plugin_value(condition_set[1]).unwrap_or_default(),
        }),
        "IsVoiceType" => ConditionSet::IsVoiceType(IsVoiceType {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            voice_type: get_plugin_value(condition_set[1]).unwrap_or_default(),
        }),
        "IsParentCell" => ConditionSet::IsParentCell(IsParentCell {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            cell: get_plugin_value(condition_set[1]).unwrap_or_default(),
        }),
        "IsWorldSpace" => ConditionSet::IsWorldSpace(IsWorldSpace {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            world_space: get_plugin_value(condition_set[1]).unwrap_or_default(),
        }),
        "IsMovementDirection" => ConditionSet::IsDirectionMovement(IsMovementDirection {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            direction: DirectionValue {
                value: Direction::try_from(
                    condition_set[1].parse::<f64>().unwrap_or_default() as u64
                )
                .unwrap_or_default(),
            },
            ..Default::default()
        }),
        "IsInLocation" => ConditionSet::IsInLocation(IsInLocation {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            location: get_plugin_value(condition_set[1]).unwrap_or_default(),
        }),
        "IsWorn" => ConditionSet::IsWorn(IsWorn {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            form: get_plugin_value(condition_set[1]).unwrap_or_default(),
        }),
        "IsWornHasKeyword" => ConditionSet::IsWornHasKeyword(IsWornHasKeyword {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            keyword: get_keyword_value(condition_set[1]).unwrap_or_default(),
        }),
        has_cond if condition_name.starts_with("Has") => {
            parse_has(has_cond, condition_set[1], is_negated)
        }
        "Random" => {
            let numeric_value = match condition_set[1].parse().ok() {
                Some(value) => Some(NumericValue::StaticValue(StaticValue { value })),
                None => None,
            };

            ConditionSet::RandomCondition(RandomCondition {
                condition: Condition {
                    negated: is_negated,
                    ..Default::default()
                },
                comparison: Cmp::Le,
                numeric_value: numeric_value.unwrap_or_default(),
                ..Default::default()
            })
        }
        "CurrentGameTimeLessThan" => {
            let value = condition_set[1].parse().unwrap_or_default();
            ConditionSet::CurrentGameTime(CurrentGameTime {
                condition: Condition {
                    negated: is_negated,
                    ..Default::default()
                },
                comparison: Cmp::Lt,
                numeric_value: NumericValue::StaticValue(StaticValue { value }),
            })
        }
        _ => {
            log::warn!("Condition({condition_name}) has no explicit mapping.");

            ConditionSet::Condition(Condition {
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
    let value_b = StaticValue {
        value: values[1].parse().unwrap(),
    };

    let create_compare = |comparison: &str| {
        ConditionSet::CompareValues(CompareValues {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            value_a: NumericValue::GlobalVariable(value_a),
            comparison: comparison.try_into().unwrap_or_default(),
            value_b: NumericValue::StaticValue(value_b),
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
    let value_b = StaticValue {
        value: values[1].parse().unwrap(),
    };

    let create_actor_cond = |comparison: Cmp, actor_type: &str| {
        let value_a = ActorValue {
            actor_value: super::values::NumericLiteral::Float(
                values[0].parse::<f32>().unwrap_or_default(),
            ),
            actor_value_type: actor_type.try_into().unwrap_or_default(),
        };

        ConditionSet::CompareValues(CompareValues {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            value_a: NumericValue::ActorValue(value_a),
            comparison,
            value_b: NumericValue::StaticValue(value_b),
        })
    };

    match condition_name {
        "IsActorValueEqualTo" => create_actor_cond(Cmp::Eq, ""),
        "IsActorValueBaseLessThan" => create_actor_cond(Cmp::Le, "Base"),
        "IsActorValueMaxEqualTo" => create_actor_cond(Cmp::Eq, "Max"),
        "IsActorValueMaxLessThan" => create_actor_cond(Cmp::Lt, "Max"),
        "IsActorValuePercentageEqualTo" => create_actor_cond(Cmp::Eq, "Percentage"),
        "IsActorValuePercentageLessThan" => create_actor_cond(Cmp::Lt, "Percentage"),
        "IsActorBase" => ConditionSet::IsActorBase(IsActorBase {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            actor_base: get_plugin_value(condition_name).unwrap_or_default(),
        }),
        _ => unreachable!(),
    }
}

fn parse_faction(condition_name: &str, arg1: &str, is_negated: bool) -> ConditionSet {
    let values: Vec<&str> = arg1.split(',').map(|s| s.trim()).collect();
    let numeric_value = NumericValue::StaticValue(StaticValue {
        value: values[1].parse().unwrap_or_default(),
    });

    let create_cond = |comparison: Cmp| {
        ConditionSet::FactionRank(FactionRank {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            faction: get_plugin_value(values[0]).unwrap_or_default(),
            comparison,
            numeric_value,
        })
    };

    match condition_name {
        "IsInFaction" => ConditionSet::IsInFaction(IsInFaction {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            faction: get_plugin_value(condition_name).unwrap_or_default(),
        }),
        "IsFactionRankEqualTo" => create_cond(Cmp::Eq),
        "IsFactionRankLessThan" => create_cond(Cmp::Lt),
        _ => unreachable!(),
    }
}

fn parse_equip(condition_name: &str, arg1: &str, is_negated: bool) -> ConditionSet {
    match condition_name {
        "IsEquippedRight" | "IsEquippedLeft" => ConditionSet::IsEquipped(IsEquipped {
            condition: Condition {
                negated: is_negated,
                ..Default::default()
            },
            form: get_plugin_value(arg1).unwrap_or_default(),
            left_hand: condition_name.ends_with("Left"),
            ..Default::default()
        }),

        "IsEquippedRightType" | "IsEquippedLeftType" => {
            let type_value = TypeValue {
                value: WeaponType::try_from(arg1.parse::<f64>().unwrap_or_default() as i64)
                    .unwrap_or_default(),
            };
            ConditionSet::IsEquippedType(IsEquippedType {
                condition: Condition {
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
                    negated: is_negated,
                    ..Default::default()
                },
                left_hand: condition_name == "IsEquippedLeftHasKeyword",
                keyword: get_keyword_value(arg1).unwrap_or_default(),
            })
        }
        _ => unreachable!(),
    }
}

fn parse_has(condition_name: &str, arg1: &str, is_negated: bool) -> ConditionSet {
    let condition = Condition {
        condition: condition_name.to_string(),
        negated: is_negated,
        ..Default::default()
    };

    match condition_name {
        "HasKeyword" => ConditionSet::HasKeyword(HasKeyword {
            condition,
            keyword: get_keyword_value(arg1).unwrap_or_default(),
        }),
        "HasPerk" => ConditionSet::HasPerk(HasPerk {
            condition,
            perk: get_plugin_value(arg1).unwrap_or_default(),
        }),
        "HasSpell" => ConditionSet::HasSpell(HasSpell {
            condition,
            spell: get_plugin_value(arg1).unwrap_or_default(),
        }),
        "HasMagicEffect" => ConditionSet::HasMagicEffect(HasMagicEffect {
            condition,
            magic_effect: get_plugin_value(arg1).expect("Not found plugin argument"),
            ..Default::default()
        }),
        "HasMagicEffectWithKeyword" => {
            ConditionSet::HasMagicEffectWithKeyword(HasMagicEffectWithKeyword {
                condition,
                keyword: get_keyword_value(arg1).unwrap_or_default(),
                ..Default::default()
            })
        }
        "HasRefType" => ConditionSet::HasRefType(HasRefType {
            condition,
            location_ref_type: get_plugin_value(arg1).unwrap_or_default(),
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

        match or.as_mut() {
            Some(or_ref) => {
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
            }
            None => {
                result.push(parse_condition(
                    cleaned.trim_start_matches("NOT"),
                    is_negated,
                ));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_conditions() {
        let input: Vec<&str> = r#"
            IsActorBase("Skyrim.esm" | 0x00000007) OR
            IsPlayerTeammate() AND
            IsEquippedRightType(3) OR
            IsEquippedRightType(4)
        "#
        .lines()
        .collect();

        let conditions = parse_conditions(&input);

        let expected: Vec<ConditionSet> = vec![
            ConditionSet::Or(Or {
                conditions: vec![
                    ConditionSet::IsActorBase(IsActorBase {
                        actor_base: PluginValue {
                            plugin_name: "Skyrim.esm".to_string(),
                            form_id: "0x00000007".into(),
                        },
                        ..Default::default()
                    }),
                    ConditionSet::Condition(Condition::new("IsPlayerTeammate")),
                ],
                ..Default::default()
            }),
            ConditionSet::Or(Or {
                conditions: vec![
                    ConditionSet::IsEquippedType(IsEquippedType {
                        type_value: TypeValue {
                            value: WeaponType::WarAxe,
                        },
                        ..Default::default()
                    }),
                    ConditionSet::IsEquippedType(IsEquippedType {
                        type_value: TypeValue {
                            value: WeaponType::Mace,
                        },
                        left_hand: true,
                        ..Default::default()
                    }),
                ],
                ..Default::default()
            }),
        ];

        assert_eq!(expected, conditions);
    }
}
