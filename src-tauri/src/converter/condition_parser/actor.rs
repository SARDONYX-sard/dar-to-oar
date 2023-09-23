use super::dar_interface::ParseError;
use crate::{
    converter::{
        conditions::{CompareValues, Condition, ConditionSet, IsActorBase},
        dar_syntax::syntax::FnArg,
        values::{ActorValue, ActorValueType, Cmp, NumericValue},
    },
    get_try_into,
};

pub(super) fn parse_actor(
    condition_name: &str,
    args: Vec<FnArg<'_>>,
    is_negated: bool,
) -> Result<ConditionSet, ParseError> {
    let create_actor_cond =
        |comparison: Cmp, actor_value_type: ActorValueType| -> Result<ConditionSet, ParseError> {
            Ok(ConditionSet::CompareValues(CompareValues {
                condition: Condition {
                    negated: is_negated,
                    condition: "CompareValues".into(),
                    ..Default::default()
                },
                value_a: NumericValue::ActorValue(ActorValue {
                    actor_value: get_try_into!(args[0], "Hex | Decimal | Float")?,
                    actor_value_type,
                }),
                comparison,
                value_b: NumericValue::StaticValue(get_try_into!(args[1], "Float")?),
            }))
        };

    Ok(match condition_name {
        "IsActorValueEqualTo" => create_actor_cond(Cmp::Eq, ActorValueType::default())?,
        "IsActorValueBaseLessThan" => create_actor_cond(Cmp::Le, ActorValueType::Base)?,
        "IsActorValueMaxEqualTo" => create_actor_cond(Cmp::Eq, ActorValueType::Max)?,
        "IsActorValueMaxLessThan" => create_actor_cond(Cmp::Lt, ActorValueType::Max)?,
        "IsActorValuePercentageEqualTo" => create_actor_cond(Cmp::Eq, ActorValueType::Percentage)?,
        "IsActorValuePercentageLessThan" => create_actor_cond(Cmp::Lt, ActorValueType::Percentage)?,
        "IsActorBase" => ConditionSet::IsActorBase(IsActorBase {
            condition: Condition {
                negated: is_negated,
                condition: condition_name.into(),
                ..Default::default()
            },
            actor_base: get_try_into!(args[0], "PluginValue")?,
        }),
        _ => unreachable!(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converter::{
        dar_syntax::syntax::NumberLiteral,
        values::{ActorValue, Cmp, NumericLiteral, NumericValue, PluginValue, StaticValue},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_actor_is_actor_value_equal_to() {
        // test inputs
        let condition_name = "IsActorValueEqualTo";
        let args = vec![
            FnArg::Number(NumberLiteral::Float(3.3)), // actor_value
            FnArg::Number(NumberLiteral::Float(3.5)), // compare value
        ];
        let is_negated = false;

        let result = parse_actor(condition_name, args, is_negated);

        let expected = ConditionSet::CompareValues(CompareValues {
            condition: Condition {
                negated: false,
                condition: "CompareValues".into(),
                ..Default::default()
            },
            value_a: NumericValue::ActorValue(ActorValue {
                actor_value: NumericLiteral::Float(3.3),
                ..Default::default() // NOTE: The DAR contains the actor_type in the function name.
            }),
            comparison: Cmp::Eq,
            value_b: NumericValue::StaticValue(StaticValue { value: 3.5 }),
        });

        match result {
            Ok(result) => {
                assert_eq!(result, expected);
            }
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn test_parse_actor_is_actor_base() {
        let condition_name = "IsActorBase";
        let args = vec![FnArg::PluginValue {
            plugin_name: "Skyrim.esm",
            form_id: NumberLiteral::Hex(0x00000007),
        }];
        let is_negated = true;

        let result = parse_actor(condition_name, args, is_negated);

        let expected = Ok(ConditionSet::IsActorBase(IsActorBase {
            condition: Condition {
                negated: true,
                condition: "IsActorBase".into(),
                ..Default::default()
            },
            actor_base: PluginValue {
                plugin_name: "Skyrim.esm".into(),
                form_id: "7".into(),
            },
        }));

        assert_eq!(result, expected);
    }
}
